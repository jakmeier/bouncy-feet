#!/usr/bin/env node
import { chromium } from '@playwright/test';
import { readFile, mkdir, writeFile, access } from 'node:fs/promises';
import path from 'node:path';
import process from 'node:process';
import { fileURLToPath } from 'node:url';

const frontendDir = path.resolve(
  path.dirname(fileURLToPath(import.meta.url)),
  '..'
);
const repoDir = path.resolve(frontendDir, '..');

function usage(message) {
  if (message) console.error(`error: ${message}\n`);
  console.error(`Usage: npm run export:challenge-keypoints -- <challenge.ron> [options]

Options:
  --all                 refresh existing fixtures (default: missing only)
  --challenge <id>      export only one parsed challenge
  --url <frontend-url>  harness server (default: http://localhost:5173)
  --start <ms>          override the selected challenge's range start
  --end <ms>            override the selected challenge's inclusive range end
  --video-url <url>     override video URL (requires --challenge)

The frontend dev server must already be running. Node never parses RON; the
application's Rust/WASM parser returns all set/challenge/video identifiers.`);
  process.exit(message ? 2 : 0);
}

function argumentsFrom(argv) {
  const options = { all: false, baseUrl: 'http://localhost:5173' };
  const take = (name) => {
    const value = argv.shift();
    if (!value) usage(`${name} requires a value`);
    return value;
  };
  while (argv.length) {
    const argument = argv.shift();
    if (argument === '--all') options.all = true;
    else if (argument === '--challenge') options.challenge = take(argument);
    else if (argument === '--url') options.baseUrl = take(argument);
    else if (argument === '--start') options.startMs = Number(take(argument));
    else if (argument === '--end') options.endMs = Number(take(argument));
    else if (argument === '--video-url') options.videoUrl = take(argument);
    else if (argument === '--help' || argument === '-h') usage();
    else if (argument.startsWith('-')) usage(`unknown option ${argument}`);
    else if (!options.ronPath) options.ronPath = argument;
    else usage(`unexpected argument ${argument}`);
  }
  if (!options.ronPath) usage('a challenge RON path is required');
  if (options.videoUrl && !options.challenge)
    usage('--video-url requires --challenge');
  for (const key of ['startMs', 'endMs']) {
    if (options[key] !== undefined && !Number.isFinite(options[key]))
      usage(`--${key} must be a number`);
  }
  return options;
}

async function exists(filename) {
  try {
    await access(filename);
    return true;
  } catch {
    return false;
  }
}

async function main() {
  const options = argumentsFrom(process.argv.slice(2));
  let ronPath = path.resolve(process.cwd(), options.ronPath);
  if (!(await exists(ronPath)))
    ronPath = path.resolve(repoDir, options.ronPath);
  const ron = await readFile(ronPath, 'utf8');
  let browser;
  try {
    browser = await chromium.launch({ headless: true });
    const page = await browser.newPage();
    const harnessUrl = new URL(
      '/dev/challenge-keypoint-export',
      options.baseUrl
    ).href;
    try {
      await page.goto(harnessUrl, { waitUntil: 'networkidle' });
    } catch (error) {
      throw new Error(
        `cannot load ${harnessUrl}; start the frontend with \`npm --prefix bouncy_frontend run dev\` (${error.message})`
      );
    }
    await page.waitForFunction(() => window.challengeKeypointExporter);
    const descriptor = await page.evaluate(
      (text) => window.challengeKeypointExporter.descriptors(text),
      ron
    );
    if (!/^[a-zA-Z0-9_-]+$/.test(descriptor.setId)) {
      throw new Error(`unsafe parsed challenge-set ID: ${descriptor.setId}`);
    }
    const selected = descriptor.challenges.filter((challenge) => {
      if (!/^[a-zA-Z0-9_-]+$/.test(challenge.id))
        throw new Error(`unsafe parsed challenge ID: ${challenge.id}`);
      return !options.challenge || challenge.id === options.challenge;
    });
    if (options.challenge && selected.length === 0)
      throw new Error(`parsed RON has no challenge ${options.challenge}`);

    const outputDir = path.join(
      repoDir,
      'bouncy_instructor',
      'tests',
      'data',
      'test_challenges',
      descriptor.setId
    );
    await mkdir(outputDir, { recursive: true });
    let generated = 0;
    for (const challenge of selected) {
      const fixture = path.join(outputDir, `${challenge.id}.ron`);
      if (!options.all && (await exists(fixture))) {
        console.log(`skip existing ${path.relative(repoDir, fixture)}`);
        continue;
      }
      console.log(
        `export ${challenge.id} from ${options.videoUrl || challenge.videoUrl}`
      );
      const result = await page.evaluate(
        ({ text, id, exportOptions }) =>
          window.challengeKeypointExporter.exportChallenge(
            text,
            id,
            exportOptions
          ),
        {
          text: ron,
          id: challenge.id,
          exportOptions: {
            startMs: options.startMs,
            endMs: options.endMs,
            videoUrl: options.videoUrl,
          },
        }
      );
      if (!result.ron || result.frameCount === 0)
        throw new Error(`refusing to write empty fixture for ${challenge.id}`);
      await writeFile(fixture, `${result.ron.trim()}\n`);
      await writeFile(
        path.join(outputDir, `${challenge.id}.metadata.json`),
        `${JSON.stringify(
          {
            videoUrl: result.videoUrl,
            capturedAt: new Date().toISOString(),
            range: {
              semantics: 'inclusive [startMs, endMs]',
              startMs: result.startMs,
              endMs: result.endMs,
            },
            mediaPipe: {
              package: '@mediapipe/tasks-vision 0.10.21',
              wasm: '@mediapipe/tasks-vision 0.10.18 CDN',
              model: 'pose_landmarker_lite.task',
            },
            frameCount: result.frameCount,
          },
          null,
          2
        )}\n`
      );
      generated += 1;
      console.log(
        `wrote ${path.relative(repoDir, fixture)} (${result.frameCount} frames)`
      );
    }
    if (generated === 0)
      console.log('all selected fixtures already exist; use --all to refresh');
  } finally {
    await browser?.close();
  }
}

main().catch((error) => {
  console.error(error instanceof Error ? error.message : error);
  process.exitCode = 1;
});
