import { readdirSync, renameSync } from "node:fs";
import { spawnSync } from "node:child_process";
import { join } from "node:path";

const wasmFiles = readdirSync("dist").filter((name) => name.endsWith(".wasm"));
if (wasmFiles.length === 0) throw new Error("dist に Wasm ファイルがありません");

for (const name of wasmFiles) {
  const source = join("dist", name);
  const output = `${source}.optimized`;
  const binary = process.platform === "win32" ? "node_modules/.bin/wasm-opt.cmd" : "node_modules/.bin/wasm-opt";
  const result = spawnSync(binary, ["-Oz", "--enable-bulk-memory", source, "-o", output], { stdio: "inherit" });
  if (result.status !== 0) process.exit(result.status ?? 1);
  renameSync(output, source);
}

