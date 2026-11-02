import { defineConfig } from '@playwright/test';
export default defineConfig({
  testDir: './tests',
  fullyParallel: false,
  workers: 1,
  use: { baseURL: 'http://127.0.0.1:5173' },
  webServer: [
    { command: 'cargo loco start --no-banner', cwd: '../backend', url: 'http://127.0.0.1:5150/api/questions', env: { DATABASE_URL: 'sqlite::memory:' }, timeout: 120000, reuseExistingServer: false },
    { command: 'npm run dev', url: 'http://127.0.0.1:5173', reuseExistingServer: false },
  ],
});
