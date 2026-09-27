import { readdirSync, statSync } from "node:fs";
import { join } from "node:path";

const limit = 25 * 1024 * 1024;
const files = readdirSync("dist", { recursive: true }).filter((name) => statSync(join("dist", name)).isFile());
let failed = false;
for (const name of files) {
  const bytes = statSync(join("dist", name)).size;
  console.log(`${name}: ${(bytes / 1024 / 1024).toFixed(2)} MiB`);
  if (bytes > limit) {
    console.error(`ERROR: ${name} は Cloudflare の 25 MiB 制限を超えています`);
    failed = true;
  }
}
if (failed) process.exit(1);

