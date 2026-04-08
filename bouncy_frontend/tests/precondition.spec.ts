import { test, expect } from '@playwright/test';
import 'dotenv/config';

test('api server running', async ({ page }) => {
  const apiBase = process.env.PUBLIC_API_BASE;
  if (!apiBase) throw new Error('PUBLIC_API_BASE not set');
  await page.goto(apiBase);
  await expect(page.locator('pre')).toContainText('Bouncy Feet stats API server running.');
});
