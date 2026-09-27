import { expect, test } from "@playwright/test";

test("Wasm版ゲームが描画可能な状態まで初期化される", async ({ page }) => {
  const errors: string[] = [];
  page.on("pageerror", (error) => errors.push(error.message));

  const wasmResponse = page.waitForResponse((response) => response.url().endsWith(".wasm"));
  const response = await page.goto("/");
  expect(response?.ok()).toBeTruthy();

  const wasm = await wasmResponse;
  expect(wasm.ok()).toBeTruthy();
  expect(wasm.headers()["content-type"]).toContain("application/wasm");

  await expect(page.locator("body")).toHaveAttribute("data-bevy-ready", "true", { timeout: 45_000 });
  await expect(page.locator("#boot-status")).toHaveText("作戦システム: ONLINE");
  const canvas = page.locator("#game-canvas");
  await expect(canvas).toBeVisible();
  expect((await canvas.boundingBox())?.width).toBeGreaterThan(100);
  expect(errors).toEqual([]);
});

