import { test, expect } from '@playwright/test';

test('club member can play private combo', async ({ page, baseURL }) => {
  // Requires a user that is member (but not admin) of a club with private combos.
  if (baseURL?.includes("dev.bouncy-feet.ch")) {
    await page.goto('/club/8/');
  }
  else if (baseURL?.includes("localhost")) {
    await page.goto('/club/7/');
  }
  // No fake user on prod!
  else {
    console.warn("skipping tests - it is only set up for dev env")
    return;
  }

  await page.getByRole('button', { name: 'Login' }).click();
  await page.getByText('bouncyfeet-dev Sign in to').click();
  await page.getByRole('textbox', { name: 'Username' }).click();
  await page.getByRole('textbox', { name: 'Username' }).fill('public-tester');
  await page.getByText('bouncyfeet-dev Sign in to').click();
  await page.getByRole('textbox', { name: 'Password' }).click();
  await page.getByRole('textbox', { name: 'Password' }).fill('not-so-secret-pw');
  await page.getByRole('button', { name: "Sign In" }).click();

  // clear user interactions
  await page.reload();

  // Set up a flag the postMessage listener will flip
  // TODO: This isn't working
  // await page.addInitScript(() => {
  //   window.__videoPlaying = false;
  //   window.addEventListener('message', (e) => {
  //     const data = typeof e.data === 'string' ? JSON.parse(e.data) : e.data;
  //     if (data?.event === 'playbackStatusChange' && data?.info?.playbackState === 'playing') {
  //       window.__videoPlaying = true;
  //     }
  //   });
  // });

  // find direct sibling of "Private Combo" title, then drill down
  const privateComboElements = page.locator('h2', { hasText: 'Private Combos' })
    .locator('~ .juggler-container')
    .locator(".elements");

  // Assert at least one child element exists and has UI elements visible
  const firstElement = privateComboElements.locator('> .box').first();
  await expect(firstElement.locator('.speed-control')).toBeVisible();

  // Try to play the video by clicking the overlay (but play button should be visible)
  await expect(firstElement.locator('.play-button')).toBeVisible();
  await firstElement.locator('.play-button').click();
  // TODO: (...continued) this one hangs forever
  // await page.waitForFunction(() => window.__videoPlaying === true);
});