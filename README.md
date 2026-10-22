# taitan291 Portfolio

Rust / Leptos（CSR・WebAssembly）で作ったポートフォリオサイトです。
自己紹介（About）、スキル一覧（Skills）、作品一覧（Works）、404ページを備えています。

公開URL: <https://taitan291.github.io/portfolio/>

## 技術構成

| 用途 | 使用技術 |
| --- | --- |
| UI | Rust / Leptos 0.8（CSR）、leptos_router |
| ビルド | Trunk（WebAssembly） |
| スタイル | Tailwind CSS v3 |
| 作品の本文 | Markdown を `build.rs`（pulldown-cmark）でHTMLに変換 |
| 開発環境 | Nix flakes（Rust・rustfmt・Clippy・Trunk・Tailwindを固定） |
| 整形 | treefmt（Nixはalejandra、Rustはrustfmt）+ pre-commit |
| 配信 | GitHub Pages |

## 開発

Nix flakesを有効にした環境で実行します。

```sh
nix develop
trunk serve
```

`http://localhost:8080` で確認できます。direnvを使う場合は `direnv allow` で同じ環境に入れます。
`cargo run` ではブラウザ用アプリは起動しません。

- 開発シェルは `RUSTUP_TOOLCHAIN` も設定するため、rustupがある環境でもNixで固定したツールチェーンが使われます。
- `nix fmt`（開発環境内では `treefmt`）で整形します。
- `nix develop`（direnvでは `direnv reload`）でpre-commitフックが有効になり、コミット時にステージ済みのNix・Rustファイルを整形します。修正が入るとコミットが中断されるので、差分を確認して再度 `git add` してください。
- 全ファイルを手動で確認するには `pre-commit run --all-files` を実行します。生成される `.pre-commit-config.yaml` はGit管理外です。

## 検証とビルド

開発環境内で実行します。

```sh
nix fmt -- --ci
cargo clippy --locked --target wasm32-unknown-unknown -- -D warnings
trunk build --release --locked --public-url "/portfolio/"
```

出力先は `dist/` です。PRとmainへのpushで同じ検証（`.github/workflows/check.yml`）が走り、
mainへのpushではさらにGitHub Pagesへデプロイします（`.github/workflows/gh-pages.yml`）。
デプロイ時は `dist/index.html` を `404.html` にコピーし、存在しないURLでもアプリの404ページを表示します。

### サブパス配信

サイトは `/portfolio/` 配下で配信されるため、次の3か所のベースパスを揃えてください。

- `Trunk.toml` の `public_url`
- `src/app.rs` の `Router base`
- ワークフローの `--public-url`

アセットは `assets/...` のような相対パスで指定します。

## ディレクトリ構成

```text
src/
├── main.rs, app.rs      エントリポイントとルーティング
├── pages/               index（トップ）・not_found（404）
├── layouts/             共通レイアウト
├── components/
│   ├── element/         ヘッダー、目次、スキル/作品カード、モーダルなど
│   └── layout/          Section
├── data/
│   ├── skills.rs        スキル一覧（SKILLS）
│   ├── works.rs         作品一覧（WORKS）
│   ├── types.rs         Proficiency / Stack など
│   └── content/<作品名>/ 作品の説明（explanation.md）と画像
└── assets/              アイコンなどの静的ファイル
```

`src/assets` は `assets/`、`src/data/content` は `content/` として `dist/` にコピーされます（`index.html` の `copy-dir`）。

## コンテンツの変更

| 変更内容 | 場所 |
| --- | --- |
| スキル | `src/data/skills.rs` の `SKILLS` |
| 作品 | `src/data/works.rs` の `WORKS` |
| ナビゲーション | `src/components/element/header.rs` の `PAGE_LINKS` |
| 色 | `tailwind.config.js` |
| 画像・アイコン | `src/assets/` |

### 作品を追加する

1. `src/data/content/<作品名>/explanation.md` に説明をMarkdownで書きます。スクリーンショットも同じフォルダに置けます。
2. `src/data/works.rs` の `WORKS` に `Work` を追加し、`description_html` に次のように指定します。

   ```rust,ignore
   description_html: include_str!(concat!(env!("OUT_DIR"), "/works/<作品名>.html")),
   ```

3. 画像のURLは `content/<作品名>/...` の形式で書きます。

`build.rs` が `src/data/content/<作品名>/explanation.md`（または `src/data/content/*.md`）を
`$OUT_DIR/works/<作品名>.html` に変換します。MarkdownはHTMLとして描画されるため、
リポジトリで管理する信頼済みの本文だけを置いてください。
`is_public` は公開/非公開の表示、`url` は外部リンクに使われます。

### スキルを追加する

`SKILLS` に `Skill` を追加します。ロゴを使う場合は、商標・ライセンスに合わせて `credit` に表記を入れてください。

## 表示の仕様

- スキル・作品の詳細表示にはHTMLの `<dialog>` と `showModal()` を使います。
- スキルカードは画面幅に応じて1〜3列（640〜1023pxでは3列）で、1024px以上では列数が本文の幅に応じて増えます。
- 1024px以上ではページの最大幅制限を外し、目次を左・本文を右に配置します。
- フッターは少なくとも画面下端に配置されます。
- ノッチなどのセーフエリアを余白に反映し、詳細ダイアログは動的な画面の高さに収めて内部をスクロールできます。
- ナビゲーションと閉じるボタンの高さは44px以上です。

## 表示確認のチェックリスト

- ヘッダー・目次の About / Skills / Works から対象セクションへ移動できる。
- スキルカードをTabで選択し、Enter / Spaceで開ける。
- 詳細表示中はTabの移動がダイアログ内に留まり、Escape・閉じるボタン・背景クリックで閉じられる。
- 閉じた後は元のカードにフォーカスが戻る。
- 320px幅でも一覧・詳細・404ページに横スクロールが出ない。
- 存在しないURLから、トップへ戻れる。

確認する画面幅は端末名ではなく、利用できる幅を基準にします。

- スマホ: 320〜430px、横向き 568×320 / 844×390
- タブレット: 768×1024 / 820×1180 / 1024×1366 とその横向き
- 画面分割: 320px / 507px
- PC: 1440px / 1920px

ダイアログを開いたままの回転、長い本文、文字サイズ200%でも横にはみ出さず、閉じるボタンを操作できることを確認してください。
実機のSafariでは、アドレスバーの開閉とセーフエリアも確認します。
