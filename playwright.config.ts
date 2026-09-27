import { defineConfig } from "@playwright/test";

export default defineConfig({
  testDir: "tests/e2e",
  timeout: 60_000,
  retries: 1,
  use: { baseURL: "http://127.0.0.1:8787", headless: true },
  webServer: {
    command: "npm run preview",
    url: "http://127.0.0.1:8787",
    reuseExistingServer: !process.env.CI,
    timeout: 60_000,
  },
});

