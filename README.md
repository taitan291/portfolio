# taitan291 Portfolio

Rust / Leptos（CSR・WebAssembly）で作ったポートフォリオサイトです。

公開URL: <https://taitan291.github.io/portfolio/index.php>

| パス | 内容 |
| --- | --- |
| `/` | Webサーバーの初期ページ風のトップ（ネタページ。本編へのリンクあり） |
| `/index.php` | 本編（About / Skills / Works）。PHPではなくRustです |
| それ以外 | 404ページ。Canvasの射的ゲームを遊べます |

## 技術構成

- UI: Rust / Leptos 0.8（CSR）、leptos_router
- ビルド: Trunk（WebAssembly）、Tailwind CSS v3
- 作品の説明: Markdown を `build.rs`（pulldown-cmark）でHTMLに変換
- 開発環境: Nix flakes、treefmt（alejandra / rustfmt）+ pre-commit
- 配信: GitHub Pages（GitHub Actions）

## 開発

```sh
nix develop      # direnv なら direnv allow
trunk serve      # http://localhost:8080/portfolio/
```

`cargo run` ではブラウザ用アプリは起動しません。
`nix develop` でpre-commitフックが有効になり、コミット時に整形されます（修正が入ったら再度 `git add`）。

## 検証とビルド

```sh
nix fmt -- --ci
cargo clippy --locked --target wasm32-unknown-unknown -- -D warnings
cargo test --locked   # 404ゲームのロジック
trunk build --release --locked --public-url "/portfolio/"
```

PRとmainへのpushで検証が走り（`.github/workflows/check.yml`）、mainへのpushで `dist/` をデプロイします（`.github/workflows/gh-pages.yml`）。
デプロイ時に `index.html` を `404.html` にコピーし、未知のURLでもアプリの404ページを表示します。

ベースパス `/portfolio/` を変えるときは、`Trunk.toml` の `public_url`・`src/app.rs` の `Router base`・`src/data/site.rs` の `INDEX_URL`・ワークフローの `--public-url` を揃えてください。

## ディレクトリ構成

```text
src/
├── app.rs             ルーティング
├── pages/             root（/）・index（本編）・not_found（404）
├── components/
│   ├── layout/        ヘッダー、フッター、目次、セクション
│   ├── cards/         スキル・作品カード
│   ├── ui/            モーダル、セレクト、スライドショーなど
│   ├── skills_view.rs スキル一覧（並べ替え）
│   └── game/          404の射的ゲーム（engine: 純Rustのロジック / platform: Canvas・音・入力）
├── data/
│   ├── site.rs        セクションID、ナビゲーション、本編のURL、GitHub URL
│   ├── types.rs       共有の型（Stack・Lang・Image・Link など）
│   ├── skills.rs      スキル一覧（SKILLS）
│   ├── skill_sort.rs  スキルの並べ替え（表示モード）
│   ├── works.rs       作品一覧（WORKS）
│   └── content/<作品名>/  説明（explanation.md）と画像
└── assets/            アイコン
```

## コンテンツの変更

- スキル: `src/data/skills.rs` の `SKILLS`。ロゴはSimple IconsのURLを指定し、`stacks` にスタックごとの習熟度（`StackLevel::new(スタック, 習熟度, 0〜100)`）、`credit` に商標表記を入れます。
- 作品:
  1. `src/data/content/<作品名>/explanation.md` と画像を置く
  2. `src/data/works.rs` の `WORKS` に `Work` を追加する。画像は `content/<作品名>/...`、本文は次のように指定します。

     ```rust,ignore
     description_html: include_str!(concat!(env!("OUT_DIR"), "/works/<作品名>.html")),
     ```

  MarkdownはそのままHTMLとして描画されるため、信頼できる本文だけを置いてください。
- 配色: `tailwind.config.js`（404ゲームのCanvasは `src/components/game/platform/palette.rs` と値を揃える）

## ライセンス

- **ソースコード**: [GPL-3.0-or-later](LICENSE)
- **アイコン画像**（`src/assets/my_icon.avif`）: [CC BY-NC 4.0](https://creativecommons.org/licenses/by-nc/4.0/deed.ja)（クレジット: taitan291）
- **以下はGPLの対象外です。著作権はtaitan291に帰属し、すべての権利を留保します（All rights reserved）。**
  - サイトに表示される文章（`src/data/` 内の文字列や `src/data/content/**/*.md` を含む）
  - `src/data/content/` 内の画像（チーム作品のスクリーンショットは、各制作者・チームにも権利があります）
- スキル欄のロゴは各権利者の商標です。リポジトリには含めず、Simple Iconsから読み込んでいます。
- 依存ライブラリは各ライブラリのライセンスに従います。
