import { expect, test } from '@playwright/test';

test.beforeEach(async ({ page, baseURL }) => {
  const url = new URL('/tools/tap-and-dance-demo', baseURL);
  await page.goto(url.href);
});

test('defaults to a 124 BPM beat and lets sounds be toggled', async ({
  page,
}) => {
  await expect(
    page.getByRole('slider', { name: 'Beats per minute' })
  ).toHaveValue('124');
  const kick = page.getByRole('button', { name: 'Kick', exact: true });
  await expect(kick).toHaveAttribute('aria-pressed', 'true');
  await kick.click();
  await expect(kick).toHaveAttribute('aria-pressed', 'false');
});

test('offers inline speed ramp controls with only 4 or 8 beats', async ({
  page,
}) => {
  await page
    .getByRole('switch', { name: 'Increase speed during exercise' })
    .click();
  const interval = page.getByRole('combobox', { name: 'Increase every' });
  await expect(interval.locator('option')).toHaveText(['4 beats', '8 beats']);
  await expect(interval).toHaveValue('8');
  await interval.selectOption('4');
  await expect(interval).toHaveValue('4');
  await expect(page.getByLabel('8 beat loop').locator('b')).toHaveCount(8);

  const start = page.getByRole('spinbutton', { name: 'Start BPM' });
  const end = page.getByRole('spinbutton', { name: 'End BPM' });
  await start.fill('150');
  await start.blur();
  await expect(end).toHaveValue('150');
  await end.fill('100');
  await end.blur();
  await expect(end).toHaveValue('150');
  const duration = page.getByRole('spinbutton', {
    name: 'Duration (s)',
    exact: true,
  });
  await duration.fill('0');
  await duration.blur();
  await expect(duration).toHaveValue('1');
});

test('plays, locks ramp settings and stops automatically', async ({ page }) => {
  await page
    .getByRole('switch', { name: 'Increase speed during exercise' })
    .click();
  await page
    .getByRole('spinbutton', { name: 'Duration (s)', exact: true })
    .fill('2');
  await page.getByRole('switch', { name: 'Stop after duration' }).click();
  await page.getByRole('button', { name: 'Play beat', exact: true }).click();
  await expect(
    page.getByRole('button', { name: 'Stop beat', exact: true })
  ).toBeVisible();
  await expect(
    page.getByRole('combobox', { name: 'Increase every' })
  ).toBeDisabled();
  await expect(
    page.getByRole('switch', { name: 'Increase speed during exercise' })
  ).toHaveAttribute('aria-disabled', 'true');
  await expect(
    page.getByRole('button', { name: 'Play beat', exact: true })
  ).toBeVisible({ timeout: 5000 });
  await expect(
    page.getByRole('combobox', { name: 'Increase every' })
  ).toBeEnabled();
});

test('reports loading failures and allows retrying', async ({ page }) => {
  await page.route('**/audio/house_909_kick.mp3', (route) =>
    route.fulfill({ status: 503 })
  );
  await page.getByRole('button', { name: 'Play beat', exact: true }).click();
  await expect(page.getByRole('alert')).toHaveText(
    'Could not start audio. Please try again.'
  );
  await page.unroute('**/audio/house_909_kick.mp3');
  await page.getByRole('button', { name: 'Play beat', exact: true }).click();
  await expect(
    page.getByRole('button', { name: 'Stop beat', exact: true })
  ).toBeVisible();
  await expect(page.getByRole('alert')).toHaveCount(0);
  await page.getByRole('button', { name: 'Stop beat', exact: true }).click();
  await expect(
    page.getByRole('button', { name: 'Play beat', exact: true })
  ).toBeVisible();
});
