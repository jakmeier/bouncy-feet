import { chromium } from 'playwright';
import pg from 'pg';

const environment = process.env.ENV || "local";
if (!["local", "staging", "production"].includes(environment))
    throw new Error("Invalid ENV=", environment);
const baseUrl = process.env.BASE_URL
    || environment === "local" ? "http://localhost:5173"
    : environment === "staging" ? "https://dev.bouncy-feet.ch"
        : environment === "production" ? "https://app.bouncy-feet.ch"
            : "";
const saveToPg = process.env.SAVE || false

// Checked without caching, essentially the installation of the PWA
const COLD_URL = '/';
// Checked after the PWA has pre-cached all assets
const WARM_URLS = ['/', '/users/', '/club/3/'];

async function auditPage(context, relativeUrl, precached) {
    const page = await context.newPage();
    const url = baseUrl + relativeUrl;

    // Set up LCP (Largest Contentful Paint) observer
    await page.addInitScript(() => {
        window.__lcp = 0;
        new PerformanceObserver(list => {
            const entries = list.getEntries();
            window.__lcp = entries.at(-1).startTime;
        }).observe({ type: 'largest-contentful-paint', buffered: true });
    });

    const requests = [];
    let totalBytes = 0;

    // Track all network requests
    page.on('request', req => {
        requests.push({
            url: req.url(),
            resourceType: req.resourceType(),
            method: req.method(),
        });
    });

    page.on('response', async res => {
        try {
            const headers = res.headers();
            const contentLength = parseInt(headers['content-length'] || '0');
            // content-length isn't always set, fall back to body size
            let size = contentLength;
            if (!size) {
                const body = await res.body().catch(() => Buffer.alloc(0));
                size = body.length;
            }
            totalBytes += size;
        } catch { }
    });

    const start = Date.now();
    await page.goto(url, { waitUntil: 'networkidle' });
    const loadTime = Date.now() - start;

    // Get performance timing from the browser itself
    const timing = await page.evaluate(() => {
        const t = performance.getEntriesByType('navigation')[0];
        return {
            dns: t.domainLookupEnd - t.domainLookupStart,
            tcp: t.connectEnd - t.connectStart,
            ttfb: t.responseStart - t.requestStart,
            domLoad: t.domContentLoadedEventEnd - t.startTime,
            fullLoad: t.loadEventEnd - t.startTime,
            lcp: window.__lcp,
        };
    });

    // Break down requests by type
    const byType = requests.reduce((acc, r) => {
        acc[r.resourceType] = (acc[r.resourceType] || 0) + 1;
        return acc;
    }, {});

    return {
        url,
        relativeUrl,
        loadTime,
        totalRequests: requests.length,
        totalKB: (totalBytes / 1024).toFixed(1),
        byType,
        timing,
        precached
    };
}

async function saveResults(pool, results, { environment, gitCommit } = {}) {
    const client = await pool.connect();
    try {
        await client.query('BEGIN');

        // Insert the run
        const { rows: [run] } = await client.query(
            `INSERT INTO audit_runs (environment, git_commit) VALUES ($1, $2) RETURNING id`,
            [environment, gitCommit ?? null]
        );

        for (const r of results) {
            // Insert page audit
            const { rows: [audit] } = await client.query(
                `INSERT INTO page_audits
          (run_id, url, load_time_ms, ttfb_ms, dns_ms, tcp_ms, dom_ready_ms, full_load_ms, total_requests, total_kb, lcp_ms, precached)
         VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12)
         RETURNING id`,
                [
                    run.id, r.relativeUrl, r.loadTime,
                    r.timing.ttfb, r.timing.dns, r.timing.tcp,
                    r.timing.domLoad, r.timing.fullLoad,
                    r.totalRequests, r.totalKB,
                    r.timing.lcp ?? null,
                    r.precached
                ]
            );

            // Insert request breakdown
            for (const [type, count] of Object.entries(r.byType)) {
                await client.query(
                    `INSERT INTO request_breakdown (page_audit_id, resource_type, request_count) VALUES ($1,$2,$3)`,
                    [audit.id, type, count]
                );
            }
        }

        await client.query('COMMIT');
        console.log(`Saved run #${run.id}`);
    } catch (e) {
        await client.query('ROLLBACK');
        throw e;
    } finally {
        client.release();
    }
}

async function run() {
    const browser = await chromium.launch({
        executablePath: process.env.PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH || '/usr/bin/chromium',
        args: ['--no-sandbox', '--disable-setuid-sandbox'], // required when running as root in a container
    });
    const results = [];

    // Cold: Default playwright already has that
    console.log(`Cold audit: ${COLD_URL}`);
    const coldContext = await browser.newContext();
    results.push(await auditPage(coldContext, COLD_URL, false));
    await coldContext.close();

    // Warm: Set up persistent context and prime the service worker
    const userDataDir = process.env.PROFILE_DIR || '/tmp/pw-profile';
    const warmContext = await chromium.launchPersistentContext(userDataDir, {
        executablePath: process.env.PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH || '/usr/bin/chromium',
        args: ['--no-sandbox', '--disable-setuid-sandbox'],
    });
    const primePage = await warmContext.newPage();
    await primePage.goto(baseUrl + '/', { waitUntil: 'networkidle' });
    await primePage.waitForLoadState('networkidle'); // let precache settle
    await primePage.close();

    for (const relativeUrl of WARM_URLS) {
        console.log(`Warm audit: ${relativeUrl}`);
        results.push(await auditPage(warmContext, relativeUrl, true));
    }

    await warmContext.close();
    await browser.close();

    console.log('\n=== Performance Audit ===\n');
    for (const r of results) {
        console.log(`${r.url}`);
        console.log(`   Load time:  ${r.loadTime}ms`);
        console.log(`   Requests:   ${r.totalRequests} total`);
        console.log(`   Data:       ${r.totalKB} KB`);
        console.log(`   TTFB:       ${r.timing.ttfb.toFixed(0)}ms`);
        console.log(`   DOM ready:  ${r.timing.domLoad.toFixed(0)}ms`);
        console.log(`   LCP:        ${r.timing.lcp?.toFixed(0) ?? 'n/a'}ms`);
        console.log(`   By type:    ${JSON.stringify(r.byType)}`);
        console.log('');
    }


    if (saveToPg) {
        if (!process.env.DATABASE_URL) {
            throw new Error("Cannot write to postgres, missing DATABASE_URL");
        }

        const { Pool } = pg;
        const pool = new Pool({ connectionString: process.env.DATABASE_URL });

        console.log('Storing to Postgres');
        saveResults(pool, results, { environment });
    }
}

run();
