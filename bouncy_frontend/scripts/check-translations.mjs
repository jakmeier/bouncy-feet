import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const sourceRoot = path.join(root, 'src');
const translationsRoot = path.join(sourceRoot, 'lib/i18n');
const sourceFilePattern = /\.(?:svelte|js|ts)$/;
const stringLiteralPattern = /(['"])([^'"\n]+)\1/g;
const translationCallPattern = /\$t\(\s*(['"`])([^'"`\n$]+)\1\s*\)/g;
const dynamicTranslationCallPattern =
  /\$t\(\s*`([^`$]*)\$\{[^}]+\}([^`]*)`\s*\)/g;

function flattenTranslations(value, prefix = '', keys = new Set()) {
  for (const [key, child] of Object.entries(value)) {
    const fullKey = prefix ? `${prefix}.${key}` : key;
    if (child !== null && typeof child === 'object' && !Array.isArray(child)) {
      flattenTranslations(child, fullKey, keys);
    } else {
      keys.add(fullKey);
    }
  }
  return keys;
}

function findSourceFiles(directory) {
  return fs.readdirSync(directory, { withFileTypes: true }).flatMap((entry) => {
    const file = path.join(directory, entry.name);
    if (entry.isDirectory()) return findSourceFiles(file);
    return sourceFilePattern.test(entry.name) ? [file] : [];
  });
}

function formatKeys(keys) {
  return [...keys]
    .sort()
    .map((key) => `  - ${key}`)
    .join('\n');
}

const localeFiles = fs
  .readdirSync(translationsRoot)
  .filter((file) => file.endsWith('.json'))
  .sort((a, b) => {
    if (a === 'en-GB.json') return -1;
    if (b === 'en-GB.json') return 1;
    return a.localeCompare(b);
  });

const localeKeys = new Map(
  localeFiles.map((file) => {
    const translations = JSON.parse(
      fs.readFileSync(path.join(translationsRoot, file), 'utf8')
    );
    return [file, flattenTranslations(translations)];
  })
);

const allDefinedKeys = new Set(
  [...localeKeys.values()].flatMap((keys) => [...keys])
);
const usedKeys = new Set();
const staticReferences = new Map();
const dynamicReferences = [];

for (const file of findSourceFiles(sourceRoot)) {
  const source = fs
    .readFileSync(file, 'utf8')
    .replace(/<!--[\s\S]*?-->|\/\*[\s\S]*?\*\/|\/\/.*$/gm, '');
  const relativeFile = path.relative(root, file);

  // Some components receive a translation key as a prop and call $t(key)
  // internally. Count only literals that are known translation keys so normal
  // string literals cannot become translation references by accident.
  for (const match of source.matchAll(stringLiteralPattern)) {
    if (allDefinedKeys.has(match[2])) usedKeys.add(match[2]);
  }

  // Keep undefined literal calls as references so they produce a useful error.
  for (const match of source.matchAll(translationCallPattern)) {
    usedKeys.add(match[2]);
    const files = staticReferences.get(match[2]) ?? [];
    files.push(relativeFile);
    staticReferences.set(match[2], files);
  }

  // A dynamic key cannot be validated exhaustively. Treat every defined key
  // matching its static prefix/suffix as used; its possible values must be
  // constrained by the code that supplies the interpolated value.
  for (const match of source.matchAll(dynamicTranslationCallPattern)) {
    dynamicReferences.push({
      prefix: match[1],
      suffix: match[2],
      file: relativeFile,
    });
  }
}

for (const { prefix, suffix } of dynamicReferences) {
  for (const key of allDefinedKeys) {
    if (key.startsWith(prefix) && key.endsWith(suffix)) usedKeys.add(key);
  }
}

const errors = [];
const [defaultLocale, ...otherLocales] = localeFiles;
const defaultKeys = localeKeys.get(defaultLocale);

for (const locale of otherLocales) {
  const keys = localeKeys.get(locale);
  const missing = new Set([...defaultKeys].filter((key) => !keys.has(key)));
  const additional = new Set([...keys].filter((key) => !defaultKeys.has(key)));
  if (missing.size || additional.size) {
    errors.push(
      `${locale} does not match ${defaultLocale}:` +
        (missing.size ? `\nmissing:\n${formatKeys(missing)}` : '') +
        (additional.size ? `\nadditional:\n${formatKeys(additional)}` : '')
    );
  }
}

for (const [locale, keys] of localeKeys) {
  const undefinedKeys = new Set([...usedKeys].filter((key) => !keys.has(key)));
  if (undefinedKeys.size) {
    const details = [...undefinedKeys].sort().map((key) => {
      const references = staticReferences.get(key);
      return references
        ? `  - ${key} (${references.join(', ')})`
        : `  - ${key}`;
    });
    errors.push(`${locale} is missing referenced keys:\n${details.join('\n')}`);
  }
}

const unusedKeys = new Set(
  [...allDefinedKeys].filter((key) => !usedKeys.has(key))
);
if (unusedKeys.size)
  errors.push(`Unused translation keys:\n${formatKeys(unusedKeys)}`);

if (errors.length) {
  console.error(`Translation check failed:\n\n${errors.join('\n\n')}`);
  process.exitCode = 1;
} else {
  console.log(
    `Translation check passed (${allDefinedKeys.size} keys in ${localeFiles.join(', ')}).`
  );
}
