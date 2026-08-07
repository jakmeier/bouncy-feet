/* global globalThis */

/** The accepted printed QR-code format; preserve case and every character. */
export const QR_CODE_PATTERN = /^[A-Za-z0-9_-]{1,1024}$/;

/** @typedef {'invalid-code'|'web-crypto-unavailable'} QrHashErrorKind */

export class QrHashError extends Error {
  /** @param {QrHashErrorKind} kind */
  constructor(kind) {
    super('Unable to hash QR reference');
    this.kind = kind;
  }
}

/**
 * @param {unknown} code
 * @returns {code is string}
 */
export function isValidQrCode(code) {
  // JavaScript's `$` can also match immediately before a final newline. The
  // explicit last-character check keeps this exactly equivalent to the
  // backend's ASCII-byte validation.
  return (
    typeof code === 'string' &&
    QR_CODE_PATTERN.test(code) &&
    code.at(-1) !== '\n'
  );
}

/**
 * Hash an already validated QR code in the browser. The plaintext code is
 * deliberately never passed to the public API.
 *
 * @param {string} code
 * @returns {Promise<string>} a lowercase, 64-character SHA-256 digest
 */
export async function hashQrCode(code) {
  if (!isValidQrCode(code)) {
    throw new QrHashError('invalid-code');
  }

  if (!globalThis.crypto?.subtle) {
    throw new QrHashError('web-crypto-unavailable');
  }

  // Valid codes are ASCII, so TextEncoder produces precisely the bytes used
  // by the backend SHA-256 helper.
  const bytes = new TextEncoder().encode(code);
  const digest = await globalThis.crypto.subtle.digest('SHA-256', bytes);
  return Array.from(new Uint8Array(digest), (byte) =>
    byte.toString(16).padStart(2, '0')
  ).join('');
}
