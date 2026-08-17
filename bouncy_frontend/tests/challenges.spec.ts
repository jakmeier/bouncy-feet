import { test, expect } from '@playwright/test';

test('jumping-jacks preview uses its recorded video and starts an attempt', async ({
  page,
}) => {
  await page.goto('/challenges/jumping-jacks/jumping-jacks');

  const video = page.getByLabel('Jumping jacks solution video');
  await expect(video.locator('source')).toHaveAttribute(
    'src',
    'https://app.bouncy-feet.ch/media/videos/challenges/jumping_jacks/jj_00.mp4'
  );
  await expect(page.locator('.timeline')).toBeVisible();

  await page.getByRole('button', { name: 'Continue as guest' }).click();
  await page.getByRole('button', { name: 'Start challenge' }).click();
  await expect(page.getByTestId('challenge-live')).toBeVisible();
});
