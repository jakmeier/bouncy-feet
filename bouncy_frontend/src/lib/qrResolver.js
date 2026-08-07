import { PUBLIC_API_BASE, PUBLIC_APP_URL } from '$env/static/public';
import { hashQrCode } from '$lib/qr.js';

export const QR_RESOLVE_TIMEOUT_MS = 10000;

/** @typedef {'timeout'|'network'|'http'|'invalid-response'|'invalid-target'|'aborted'} QrResolveErrorKind */

export class QrResolveError extends Error {
  /** @param {QrResolveErrorKind} kind */
  constructor(kind) {
    super('Unable to resolve QR reference');
    this.kind = kind;
  }
}

/**
 * @param {unknown} target
 * @returns {target is string}
 */
export function isValidQrTarget(target) {
  if (
    typeof target !== 'string' ||
    target.length === 0 ||
    target.length > 2048
  ) {
    return false;
  }

  try {
    const url = new URL(target);
    const appUrl = new URL(PUBLIC_APP_URL);
    const isQrPath = url.pathname === '/qr' || url.pathname.startsWith('/qr/');
    // Allow all HTTPS and mirror the backend permits for HTTP (only allowed for a local app target).
    const isLocalHttpAppTarget =
      appUrl.protocol === 'http:' &&
      ['localhost', '127.0.0.1', '[::1]'].includes(appUrl.hostname) &&
      url.protocol === 'http:' &&
      url.origin === appUrl.origin;
    return (
      (url.protocol === 'https:' || isLocalHttpAppTarget) &&
      url.username === '' &&
      url.password === '' &&
      !isQrPath
    );
  } catch {
    return false;
  }
}

/**
 * Resolve a browser-hashed QR reference without using authenticated API-client
 * code, cookies, a request body, or headers containing the plaintext code.
 *
 * @param {string} code
 * @param {{
 *   fetchImpl?: typeof fetch,
 *   signal?: AbortSignal,
 *   timeoutMs?: number,
 * }} [options]
 * @returns {Promise<string>} the exact validated target URL supplied by the API
 */
export async function resolveQrTarget(code, options = {}) {
  const hash = await hashQrCode(code);
  const endpoint = new URL(`/qr/${hash}/resolve`, PUBLIC_API_BASE).toString();
  const controller = new AbortController();
  const timeoutMs = options.timeoutMs ?? QR_RESOLVE_TIMEOUT_MS;
  const timeout = setTimeout(() => controller.abort(), timeoutMs);
  const abortFromCaller = () => controller.abort();

  if (options.signal) {
    if (options.signal.aborted) {
      controller.abort();
    } else {
      options.signal.addEventListener('abort', abortFromCaller, { once: true });
    }
  }

  try {
    const fetchImpl = options.fetchImpl ?? fetch;
    const response = await fetchImpl(endpoint, {
      method: 'GET',
      credentials: 'omit',
      cache: 'no-store',
      signal: controller.signal,
    });

    if (!response.ok) {
      throw new QrResolveError('http');
    }

    let body;
    try {
      body = await response.json();
    } catch {
      throw new QrResolveError('invalid-response');
    }

    if (
      !body ||
      typeof body !== 'object' ||
      !isValidQrTarget(body.target_url)
    ) {
      throw new QrResolveError('invalid-target');
    }

    return body.target_url;
  } catch (error) {
    if (error instanceof QrResolveError) {
      throw error;
    }
    if (controller.signal.aborted) {
      throw new QrResolveError(options.signal?.aborted ? 'aborted' : 'timeout');
    }
    throw new QrResolveError('network');
  } finally {
    clearTimeout(timeout);
    options.signal?.removeEventListener('abort', abortFromCaller);
  }
}
