import { expect, test } from '@playwright/test';

const longCode = `000AbC-_` + 'x'.repeat(392);

test('a QR deep link uses the lightweight fallback shell', async ({ page }) => {
  const requests: string[] = [];
  page.on('request', (request) => requests.push(request.url()));

  await page.goto(`/qr/${longCode}/`);

  await expect(page.getByRole('heading')).toHaveText('Hi!\nAre you looking for something?');
  const appLink = page.getByRole('link', { name: 'Open the Bouncy Feet app' });
  await expect(appLink).toHaveAttribute('href', 'http://localhost:5173');
  await expect(page.getByText(`QR reference: ${longCode}`)).toBeVisible();
  await page.keyboard.press('Tab');
  await expect(appLink).toBeFocused();

  expect(requests.some((url) => url.includes('localhost:3000'))).toBe(false);
  expect(requests.some((url) => url.endsWith('.wasm'))).toBe(false);
});

test('preserves a maximum-length QR code exactly', async ({ page }) => {
  const code = `0A-_` + 'Z'.repeat(1020);

  await page.goto(`/qr/${code}/`);

  await expect(page.getByText(`QR reference: ${code}`)).toBeVisible();
});
