
/** @type {import('@playwright/test').PlaywrightTestConfig} */
const config = {
    webServer: {
        command: 'npm run dev',
        port: 5173,
        reuseExistingServer: true
    },
    testDir: 'tests',
    testMatch: /(.+\.)?(test|spec)\.[jt]s/,
    baseURL: process.env.TEST_ENV || 'http://localhost:5173',
};

export default config;
