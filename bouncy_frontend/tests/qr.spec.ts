import { expect, test, type Page, type Request } from '@playwright/test';

const longCode = `000AbC-_` + 'x'.repeat(392);
const longCodeHash =
  '2b92a6c19e404a9e84efe9d161364ad16385ece729f7d027b82b6b690a9acde8';

async function mockNotFoundResolution(page: Page, delayMs = 0) {
  await page.route('**/qr/*/resolve', async (route) => {
    if (delayMs) {
      await new Promise((resolve) => setTimeout(resolve, delayMs));
    }
    await route.fulfill({
      status: 404,
      headers: {
        'Access-Control-Allow-Origin': '*',
        'Cache-Control': 'no-store',
      },
    });
  });
}

test('a QR deep link uses the lightweight fallback shell and sends only its digest', async ({
  page,
}) => {
  const requests: string[] = [];
  page.on('request', (request) => requests.push(request.url()));
  await mockNotFoundResolution(page, 1000);

  await page.goto(`/qr/${longCode}/`);

  await expect(page.getByRole('heading')).toHaveText('Hi!');
  await expect(page.locator('.status')).toHaveText('Just a second...');
  await expect(page.locator('.spinner')).toBeVisible();
  await expect(page.locator('.reference')).toHaveCount(0);

  const appLink = page.getByRole('link', { name: 'Open the Bouncy Feet app' });
  await expect(appLink).toHaveAttribute('href', 'http://localhost:5173');
  await expect(page.locator('.status')).toHaveText(
    "Are you looking for something?\nI'm afraid this QR link doesn't go anywhere."
  );
  await expect(page.getByText(`QR reference: ${longCode}`)).toBeVisible();
  await page.keyboard.press('Tab');
  await expect(appLink).toBeFocused();

  const resolverRequests = requests.filter(
    (url) => url.includes('/qr/') && url.endsWith('/resolve')
  );
  expect(resolverRequests).toEqual([
    `http://localhost:3000/qr/${longCodeHash}/resolve`,
  ]);
  expect(resolverRequests[0]).not.toContain(longCode);
  expect(requests.some((url) => url.endsWith('.wasm'))).toBe(false);
  expect(requests.some((url) => url.includes('/clubs'))).toBe(false);
});

test('an active QR mapping replaces the fallback page with its exact target', async ({
  context,
  page,
}) => {
  const code = 'abc';
  const hash =
    'ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad';
  // Local development mappings deliberately use the HTTP app origin. They
  // must pass the same target validation as the backend's localhost exception.
  const target = 'http://localhost:5173/challenges/';
  let resolverRequest: Request | undefined;

  await context.addCookies([
    {
      name: 'api-session',
      value: 'must-not-be-sent',
      url: 'http://localhost:3000/',
    },
  ]);
  await page.route(
    `http://localhost:3000/qr/${hash}/resolve`,
    async (route) => {
      resolverRequest = route.request();
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        headers: { 'Access-Control-Allow-Origin': '*' },
        body: JSON.stringify({ target_url: target }),
      });
    }
  );
  await page.route(target, async (route) => {
    await route.fulfill({
      status: 200,
      contentType: 'text/html',
      body: '<title>Discount</title>',
    });
  });

  await page.goto(`/qr/${code}/`);

  await expect(page).toHaveURL(target);
  expect(resolverRequest).toBeDefined();
  expect(resolverRequest?.url()).toBe(
    `http://localhost:3000/qr/${hash}/resolve`
  );
  expect(resolverRequest?.method()).toBe('GET');
  expect(resolverRequest?.postData()).toBeNull();
  expect(resolverRequest?.headers().cookie).toBeUndefined();
});

test('shows a maximum-length QR code only after a failed resolution', async ({
  page,
}) => {
  const code = `0A-_` + 'Z'.repeat(1020);
  await mockNotFoundResolution(page);

  await page.goto(`/qr/${code}/`);

  await expect(page.locator('.status')).toHaveText(
    "Are you looking for something?\nI'm afraid this QR link doesn't go anywhere."
  );
  await expect(page.getByText(`QR reference: ${code}`)).toBeVisible();
});
