# Deception RTS

R.U.S.E.のような戦略ズーム、索敵、欺瞞工作を将来実装する、PCブラウザ向け3Dリアルタイムストラテジーゲームの開発基盤です。現段階では平面地形、簡易戦車、俯瞰カメラ、ホイールズーム、WASD/矢印キー移動を備えた最小プロトタイプを提供します。

## システム構成

- `crates/rts-core`: BevyやブラウザAPIに依存しない決定論的ゲームシミュレーション。`step(&GameState, &Command)` は入力状態を変更せず、新しい状態を返します。
- `crates/rts-app`: Bevyによる3D描画、入力、カメラ、ブラウザ起動フックを担当するImperative Shell。
- `index.html` / `Trunk.toml`: WebAssemblyビルドとブラウザ画面。
- `wrangler.jsonc`: Cloudflare Workers Static Assets設定。WorkerのJavaScriptエントリーポイントやSPAフォールバックは使用しません。
- `tests/e2e`: Wranglerで配信した本番成果物をChromiumで検査するPlaywrightテスト。

## 必要な開発ツール

- Rust 1.95.0（`rust-toolchain.toml` が自動選択）
- Node.js 22以上（Wrangler 4.141.0の要件）
- npm 11系
- Trunk 0.21.14
- macOSではXcode Command Line Tools、Linuxでは一般的なC/C++ビルドツール

RustとTrunkを準備します。

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup show
cargo install trunk --version 0.21.14 --locked
```

`rustup show` によってRust 1.95.0と `wasm32-unknown-unknown` が自動導入されます。手動で追加する場合は次を実行します。

```bash
rustup target add wasm32-unknown-unknown --toolchain 1.95.0
```

macOSで「Xcode license agreements」と表示された場合は、利用規約を確認したうえで次を実行します。

```bash
sudo xcodebuild -license
```

## 初期セットアップ

リポジトリ直下でnpm依存をロックファイルどおりに導入し、Playwright用Chromiumを取得します。

```bash
npm ci
npx playwright install chromium
```

Linux CI相当のOS依存も導入する場合は、管理者権限のある環境で次を使用します。

```bash
npx playwright install --with-deps chromium
```

## ローカル開発

Web版の開発サーバーを起動し、`http://127.0.0.1:8080` を開きます。

```bash
npm run dev
```

ネイティブ版は次で起動できます。

```bash
cargo run -p rts-app
```

画面操作はWASDまたは矢印キーで平行移動、マウスホイールでズームです。

## Rustの検査とテスト

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

`rts-core` だけを高速に検証する場合は `cargo test -p rts-core` を使えます。通常の単体テストに加え、`proptest` によるゲーム内時間の単調性検査が実行されます。ゲームロジックのテストにGPUやウィンドウは不要です。

## リリースビルドとブラウザテスト

```bash
npm run build
npm run preview
```

`npm run build` はTrunkのリリースビルド、Binaryenの `wasm-opt -Oz`、`dist/` の全ファイルに対する25 MiB上限検査を順に実行します。プレビューは `http://127.0.0.1:8787` で、Cloudflare Workers Static Assetsと同じWrangler経由です。

別のターミナルからE2Eを実行できます。Playwright自身がプレビューを起動できるため、通常は先に `npm run preview` を実行する必要はありません。

```bash
npm run test:e2e
```

E2EはHTMLのHTTP成功だけでなく、Wasm取得、`application/wasm`、JavaScript実行時例外なし、Bevyの初期化フック、Canvasの表示領域を検査します。

## Cloudflare Workersへのデプロイ

初回だけWranglerへ認証します。

```bash
npx wrangler login
```

検証済みの `dist/` をデプロイします。

```bash
npm run build
npm run deploy
```

この構成はCloudflare PagesではなくWorkers Static Assetsを使います。API Workerはなく、存在しないWasm/JavaScriptを `index.html` へフォールバックさせません。

## GitHub ActionsとSecrets

`.github/workflows/ci.yml` はPull Requestと `main` pushでfmt、clippy、Rustテスト、Wasmビルド、サイズ検査、Chromium E2Eを実行します。`.github/workflows/deploy.yml` は `main` のCI成功後、または手動実行時にデプロイします。

GitHubリポジトリの **Settings → Secrets and variables → Actions** に次のRepository secretsを登録してください。

- `CLOUDFLARE_API_TOKEN`: 対象Workersへのデプロイに必要な最小権限のAPIトークン
- `CLOUDFLARE_ACCOUNT_ID`: 対象CloudflareアカウントID

認証情報はリポジトリへ保存しません。

## トラブルシューティング

- `trunk: command not found`: `cargo install trunk --version 0.21.14 --locked` を実行し、`~/.cargo/bin` をPATHへ追加します。
- Wasmターゲット不足: `rustup target add wasm32-unknown-unknown --toolchain 1.95.0` を実行します。
- Xcodeライセンスエラー: `sudo xcodebuild -license` で内容を確認し同意します。
- Playwrightのbrowser executable不足: `npx playwright install chromium` を実行します。
- `npm run preview` 前に404になる: 先に `npm run build` を実行し、`dist/` を生成します。
- 25 MiB検査失敗: アセット分割より先にBevyの不要featureを削減します。圧縮後ではなく実Wasmファイルが検査対象です。
- 初回ビルドが長い: BevyとLTOの初回コンパイルによるものです。`target/` を保持すれば2回目以降はキャッシュされます。

## 今後の拡張方針

移動・戦闘・索敵・情報戦は `rts-core` のCommandと純粋な状態遷移として追加し、乱数が必要になった時点でシードまたはRNG状態を入力へ明示します。描画用Transformを唯一のゲーム状態にはせず、初期状態と命令ログから戦闘を再現できる構造を維持します。
