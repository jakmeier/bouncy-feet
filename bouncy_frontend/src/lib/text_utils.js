/**
 * Splits an input string by line breaks and trims whitespace.
 * @param {string} inputString
 * @returns {string[]}
 */
export function createParagraphs(inputString) {
    const paragraphs = inputString.split(/\r?\n/);
    return paragraphs.map((line) => line.trim());
}

/**
 * @param {string} filename
 * @param {string} text
 */
export function downloadTextFile(filename, text) {
    // Create a temporary <a> to trigger the download
    const a = document.createElement('a');
    a.href = 'data:text/plain;charset=utf-8,' + encodeURIComponent(text);
    a.download = filename;

    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
}

/**
 * @param {string} data
 * @returns {Promise<string>}
 */
export async function compressAndBase64Encode(data) {
    const encoder = new TextEncoder();
    const buffer = encoder.encode(data);
    const gzip = new CompressionStream('gzip');
    const writer = gzip.writable.getWriter();
    const responsePromise = new Response(gzip.readable).arrayBuffer();

    await writer.write(buffer);
    await writer.close();

    const compressed = await responsePromise;
    const bytes = new Uint8Array(compressed);

    // convert to base64
    if (bytes.toBase64) {
        // Should be available on all modern browsers since September 2025
        return bytes.toBase64();
    } else {
        let binary = '';
        for (let i = 0; i < bytes.length; i++) {
            binary += String.fromCharCode(bytes[i]);
        }
        return btoa(binary);
    }
}

/**
 * @param {string} data
 * @returns {Promise<string>}
 */
export async function decodeBase64AndDecompress(data) {
    let bytes;
    if (Uint8Array.fromBase64) {
        // Should be available on all modern browsers since September 2025
        bytes = Uint8Array.fromBase64(data);
    } else {
        const binary = atob(data);
        bytes = new Uint8Array(binary.length);
        for (let i = 0; i < binary.length; i++) {
            bytes[i] = binary.charCodeAt(i);
        }
    }

    const gzip = new DecompressionStream('gzip');
    const writer = gzip.writable.getWriter();
    const responsePromise = new Response(gzip.readable).arrayBuffer();

    await writer.write(bytes);
    await writer.close();

    const decompressed = await responsePromise;
    const decoder = new TextDecoder();
    return decoder.decode(decompressed);
}
