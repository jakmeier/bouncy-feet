import { devices } from '@playwright/test';

/** @type {import('@playwright/test').PlaywrightTestConfig} */
const config = {
    testDir: 'tests',
    testMatch: /(.+\.)?(test|spec)\.[jt]s/,
    use: {
        baseURL: process.env.TEST_ENV || 'http://localhost:5173',
    },
    workers: 4,
    projects: [
        {
            name: 'chromium',
            use: { ...devices['Desktop Chrome'] },
        },
        {
            name: 'firefox',
            use: { ...devices['Desktop Firefox'] },
        },
        // {
        //     name: 'webkit',
        //     use: { ...devices['Desktop Safari'] },
        // },
        {
            name: 'mobile-chrome',
            use: { ...devices['Pixel 5'] },
        },
    ],
};

// Only start webServer for local testing
if (!process.env.TEST_ENV || process.env.TEST_ENV === 'http://localhost:5173') {
    config.webServer = {
        command: 'npm run dev',
        port: 5173,
        reuseExistingServer: true,
    };
}

export default config;
