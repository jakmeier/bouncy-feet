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