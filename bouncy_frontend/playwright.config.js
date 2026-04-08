
/** @type {import('@playwright/test').PlaywrightTestConfig} */
const config = {
    testDir: 'tests',
    testMatch: /(.+\.)?(test|spec)\.[jt]s/,
    use: {
        baseURL: process.env.TEST_ENV || 'http://localhost:5173',
    }
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
