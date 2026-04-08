import { expect, test } from '@playwright/test';

test('index page has expected h1', async ({ page }) => {
    await page.goto('/');
    await expect(page.getByRole('heading', { name: 'Hey hey! :)' })).toBeVisible();
});

test('guest can see users', async ({ page }) => {
    await page.goto('/');
    await page.getByRole('button', { name: 'I\'m visiting for the first' }).click();
    await page.getByRole('button', { name: 'Continue as guest' }).click();

    await expect(page.getByText('Rave Energy')).toBeVisible();

    let scrollCount = 1;
    while (scrollCount <= 5 && !page.getByText('Pure Joy').isVisible()) {
        await page.evaluate(() => window.scrollBy(0, window.innerHeight));
        await page.waitForTimeout(500)
    }
    await expect(page.getByText('Pure Joy')).toBeVisible();

    while (scrollCount <= 10 && !page.getByRole('button', { name: 'Show all' }).isVisible()) {
        await page.evaluate(() => window.scrollBy(0, window.innerHeight));
        await page.waitForTimeout(500)
    }
    await page.getByRole('button', { name: 'Show all' }).click();
    await expect(page.getByRole('heading', { name: 'Dancers' })).toBeVisible();

    await page.getByRole('textbox', { name: 'Search' }).click();
    await page.getByRole('textbox', { name: 'Search' }).fill('Jakob');
    await page.getByRole('textbox', { name: 'Search' }).press('Enter');

    const searchResults = await page.getByRole('listitem').filter({ hasText: 'Jakob' });
    await expect(searchResults.first()).toBeVisible();
    if (await searchResults.count() === 1) {
        searchResults.click();
    } else {
        await page.getByRole('listitem').filter({ hasText: 'tester_jakob_2' }).first().click();
    }

    await expect(page.getByRole('heading', { name: 'Jakob' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Combos' })).toBeVisible();

    await page.locator('header').getByRole('button').click();
    await expect(page.getByRole('heading', { name: 'Dancers' })).toBeVisible();

    await page.locator('header').getByRole('button').click();
    await expect(page.getByText('Rave Energy')).toBeVisible();
    await expect(page.getByText('Pure Joy')).toBeVisible();
});
