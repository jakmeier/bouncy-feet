import { describe, expect, it } from 'vitest';
import { hashQrCode, isValidQrCode } from './qr.js';
import { isValidQrTarget, resolveQrTarget } from './qrResolver.js';

describe('QR browser resolver', () => {
  it('validates and hashes exact ASCII code bytes as lowercase hex', async () => {
    expect(isValidQrCode('000AbC-_')).toBe(true);
    expect(isValidQrCode('a\n')).toBe(false);
    expect(isValidQrCode('a/b')).toBe(false);
    expect(isValidQrCode('ä')).toBe(false);
    expect(isValidQrCode('x'.repeat(1025))).toBe(false);
    expect(await hashQrCode('abc')).toBe(
      'ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad'
    );
  });

  it('uses a digest-only, cookie-free, uncached GET request', async () => {
    let requestedUrl = '';
    /** @type {RequestInit | undefined} */
    let requestedOptions;
    const target = 'https://discount.example/redeem/unique-code';

    await expect(
      resolveQrTarget('abc', {
        fetchImpl: async (url, options) => {
          requestedUrl = String(url);
          requestedOptions = options;
          return new Response(JSON.stringify({ target_url: target }), {
            status: 200,
            headers: { 'Content-Type': 'application/json' },
          });
        },
      })
    ).resolves.toBe(target);

    expect(requestedUrl).toMatch(
      /\/qr\/ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad\/resolve$/
    );
    expect(requestedUrl).not.toContain('abc');
    expect(requestedOptions).toMatchObject({
      method: 'GET',
      credentials: 'omit',
      cache: 'no-store',
    });
  });

  it('rejects unsafe targets and turns bad responses into safe failures', async () => {
    expect(isValidQrTarget('https://app.bouncy-feet.ch/challenges/')).toBe(
      true
    );
    expect(isValidQrTarget('http://localhost:5173/challenges/')).toBe(true);
    expect(isValidQrTarget('http://localhost:3000/challenges/')).toBe(false);
    expect(isValidQrTarget('http://app.bouncy-feet.ch/')).toBe(false);
    expect(isValidQrTarget('https://app.bouncy-feet.ch/qr/again/')).toBe(false);
    expect(isValidQrTarget('https://user:password@app.bouncy-feet.ch/')).toBe(
      false
    );

    await expect(
      resolveQrTarget('abc', {
        fetchImpl: async () => new Response('not json', { status: 200 }),
      })
    ).rejects.toMatchObject({ kind: 'invalid-response' });

    await expect(
      resolveQrTarget('abc', {
        fetchImpl: async () =>
          new Response(JSON.stringify({ target_url: 'http://bad.example/' }), {
            status: 200,
          }),
      })
    ).rejects.toMatchObject({ kind: 'invalid-target' });

    await expect(
      resolveQrTarget('abc', {
        timeoutMs: 1,
        fetchImpl: (_url, options) =>
          new Promise((_resolve, reject) => {
            options?.signal?.addEventListener('abort', () => {
              reject(new DOMException('Aborted', 'AbortError'));
            });
          }),
      })
    ).rejects.toMatchObject({ kind: 'timeout' });
  });
});
