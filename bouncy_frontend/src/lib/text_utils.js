/**
 * Splits an input string by line breaks and trims whitespace.
 * @param {string} inputString
 * @returns {string[]}
 */
export function createParagraphs(inputString) {
    const paragraphs = inputString.split(/\r?\n/);
    return paragraphs.map((line) => line.trim());
}