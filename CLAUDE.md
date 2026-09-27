# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

Browser-first 3D RTS prototype (Bevy → WebAssembly), aiming at R.U.S.E.-style strategic zoom, reconnaissance, and deception mechanics. Currently: flat terrain, placeholder tanks, top-down camera with WASD/arrow pan and wheel zoom. README, UI strings, and test names are in Japanese; keep that convention for user-facing text.

## Commands

Toolchain is pinned: Rust 1.95.0 via `rust-toolchain.toml` (includes `wasm32-unknown-unknown`), Trunk 0.21.14, Node ≥22. Run `npm ci` once for Wrangler / Playwright / Binaryen.

```bash
# Rust checks (same as CI)
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace

# Fast loop on game logic (no GPU/window needed; includes proptest)
cargo test -p rts-core
cargo test -p rts-core advancing_time_is_monotonic   # single test by name

# Run
npm run dev                # trunk serve → http://127.0.0.1:8080 (wasm)
cargo run -p rts-app       # native window

# Release build + browser tests
npm run build              # trunk release build → wasm-opt -Oz → 25 MiB per-file check on dist/
npm run preview            # wrangler dev --local on :8787, serves dist/
npm run test:e2e           # Playwright; starts `npm run preview` itself
npx playwright test -g "Wasm版"   # single E2E test by title
```

Notes:
- The workspace `default-members` is only `rts-app`, so plain `cargo test`/`cargo clippy` skip `rts-core` — use `--workspace` or `-p rts-core`.
- E2E tests run against the built `dist/`, not the dev server. Run `npm run build` after Rust changes or E2E will test stale output (or 404).
- On Linux, native builds need `pkg-config libudev-dev libasound2-dev libwayland-dev libxkbcommon-dev` (see `.github/workflows/ci.yml`).

## Architecture

Functional core / imperative shell split across two crates:

- **`crates/rts-core`** — deterministic simulation with no Bevy or browser dependencies. All gameplay changes go through `step(&GameState, &Command) -> GameState`, which clones and returns a new state and never mutates its input. Invalid commands (unknown unit, non-finite coordinates) must return the state unchanged. Tests assert purity, determinism, and invariants (proptest).
- **`crates/rts-app`** — Bevy 0.19 shell: rendering (`renderer.rs` spawns meshes from `SimulationState`), camera zoom (`camera.rs`), keyboard pan (`input.rs`). The core `GameState` is held in the `SimulationState` resource; Bevy `Transform`s are derived views, not the source of truth.

Design rules from the README for extending the game:
- New mechanics (movement, combat, recon, information warfare) are added as `Command` variants plus pure transitions in `rts-core`.
- When randomness is needed, pass a seed/RNG state explicitly as input. Battles must stay reproducible from initial state + command log.

### Browser integration contract

`index.html` is the Trunk entry point (`data-bin="rts-app"`, Trunk's own wasm-opt disabled because `scripts/optimize-wasm.mjs` runs Binaryen afterwards). Bevy renders into `#game-canvas`. On startup, the wasm-only `mark_ready` system in `rts-app/src/main.rs` sets `#boot-status` to `作戦システム: ONLINE` and `body[data-bevy-ready="true"]`. `tests/e2e/game.spec.ts` depends on these IDs/strings plus `.wasm` served as `application/wasm` and zero page errors — keep them in sync when changing either side.

### Size / dependency constraints

- Deployed to Cloudflare Workers Static Assets (`wrangler.jsonc`, no Worker script, no SPA fallback). Every file in `dist/` must be ≤25 MiB (uncompressed), enforced by `scripts/check-dist-size.mjs`.
- Bevy is used with `default-features = false` and an explicit feature list in the root `Cargo.toml`; prefer trimming features over adding them. The release profile uses `opt-level="z"`, LTO, `codegen-units=1`, `panic="abort"` (first builds are slow).
- Dependency versions are pinned exactly (`=x.y.z`) in `[workspace.dependencies]`; crates reference them with `.workspace = true`.

## CI / Deploy

`ci.yml` runs on PRs and `main`: fmt, clippy, `cargo test --workspace`, `npm run build`, Playwright E2E. `deploy.yml` runs `npm run deploy` after CI succeeds on `main` (or manually), using `CLOUDFLARE_API_TOKEN` / `CLOUDFLARE_ACCOUNT_ID` secrets.
