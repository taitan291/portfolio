# taitan291 Portfolio

Rust / Leptos（CSR・WebAssembly）で作ったポートフォリオサイトです。
自己紹介（About）、スキル一覧（Skills）、作品一覧（Works）、404ページを備えています。

公開URL: <https://taitan291.github.io/portfolio/index.php>

| パス | 内容 |
| --- | --- |
| `/` | Webサーバーの初期ページ風のトップ（ネタページ。本編へのリンクあり） |
| `/index.php` | 本編（About / Skills / Works / Contact）。PHPではなくRustです |
| それ以外 | 404ページ。Canvasの射的ゲームを遊べます |

## 技術構成

| 用途 | 使用技術 |
| --- | --- |
| UI | Rust / Leptos 0.8（CSR）、leptos_router |
| ビルド | Trunk（WebAssembly） |
| スタイル | Tailwind CSS v3 |
| 作品の本文 | Markdown を `build.rs`（pulldown-cmark）でHTMLに変換 |
| 開発環境 | Nix flakes（Rust・rustfmt・Clippy・Trunk・Tailwindを固定） |
| 整形 | treefmt（Nixはalejandra、Rustはrustfmt）+ pre-commit |
| 配信 | GitHub Pages（GitHub Actions） |

## 開発

Nix flakesを有効にした環境で実行します。

```sh
nix develop      # direnv なら direnv allow
trunk serve      # http://localhost:8080/portfolio/
```

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
cargo test --locked   # スキルの並べ替えと404ゲームのロジック
trunk build --release --locked --public-url "/portfolio/"
```

出力先は `dist/` です。PRとmainへのpushで検証（`.github/workflows/check.yml`）が走り、
mainへのpushではさらにGitHub Pagesへデプロイします（`.github/workflows/gh-pages.yml`）。
デプロイ時は `dist/index.html` を `404.html` にコピーし、存在しないURLでもアプリの404ページを表示します。

### サブパス配信

サイトは `/portfolio/` 配下で配信されるため、次のベースパスを揃えてください。

- `Trunk.toml` の `public_url`
- `src/app.rs` の `Router base`
- `src/data/site.rs` の `INDEX_URL`
- ワークフローの `--public-url`

アセットは `assets/...` のような相対パスで指定します。

## ディレクトリ構成

```text
src/
├── main.rs, app.rs      エントリポイントとルーティング
├── pages/               root（/）・index（本編）・not_found（404）
├── layouts/             共通レイアウト
├── components/
│   ├── element/         ヘッダー、フッター（Contact）、スキル/作品カード、モーダル、
│   │                    スキル一覧（並べ替え）、セレクト、スライドショーなど
│   ├── layout/          Section
│   ├── reveal.rs        スクロールでのフェードイン
│   └── game/            404の射的ゲーム（engine: 純Rustのロジック / platform: Canvas・音・入力）
├── data/
│   ├── site.rs          セクションID、ナビゲーション、本編のURL、GitHub URL
│   ├── types.rs         共有の型（Stack・Lang・Image・Link など）
│   ├── skills.rs        スキル一覧（SKILLS）
│   ├── skill_sort.rs    スキルの並べ替え（表示モード）
│   ├── works.rs         作品一覧（WORKS）
│   └── content/<作品名>/ 作品の説明（explanation.md）と画像
└── assets/              アイコンなどの静的ファイル
```

`src/assets` は `assets/`、`src/data/content` は `content/` として `dist/` にコピーされます（`index.html` の `copy-dir`）。

## コンテンツの変更

| 変更内容 | 場所 |
| --- | --- |
| スキル | `src/data/skills.rs` の `SKILLS` |
| 作品 | `src/data/works.rs` の `WORKS` |
| ナビゲーション | `src/data/site.rs` の `PAGE_LINKS` |
| 色（ライト/ダーク） | `style.css` の `:root` の CSS 変数（`tailwind.config.js` から参照） |
| 404ゲームの色 | `tailwind.config.js` の `gameColors`（Canvasは `src/components/game/platform/palette.rs` と値を揃える） |
| 画像・アイコン | `src/assets/` |

### 作品を追加する

1. `src/data/content/<作品名>/explanation.md` に説明をMarkdownで書きます。スクリーンショットも同じフォルダに置けます（`content/example/` がひな形です）。
2. `src/data/works.rs` の `WORKS` に `Work` を追加し、`id` に `<作品名>`、`description_html` に次のように指定します。

   ```rust,ignore
   description_html: include_str!(concat!(env!("OUT_DIR"), "/works/<作品名>.html")),
   ```

3. 画像のURLは `content/<作品名>/...` の形式で書きます。`images` はモーダル内のスライドショーに表示されます。

`build.rs` が `src/data/content/<作品名>/explanation.md`（または `src/data/content/*.md`）を
`$OUT_DIR/works/<作品名>.html` に変換します。MarkdownはHTMLとして描画されるため、
リポジトリで管理する信頼済みの本文だけを置いてください。
`is_public` は公開/非公開の表示、`links` はモーダルの題名の下に `題名:URL` の形で表示する外部リンクです。

### スキルを追加する

`SKILLS` に `Skill` を追加します。ロゴはSimple IconsのURLを指定し、`stacks` にスタックごとの習熟度（`StackLevel::new(スタック, 習熟度, 0〜100)`）、`credit` に商標表記を入れます。

## 表示の仕様

- スキル・作品の詳細表示にはHTMLの `<dialog>` と `showModal()` を使います。
- Skillsは表示モード（Default / スタック別 / Custom）で並べ替えられます。Customではスタックを選び、習熟度・名前の昇順/降順で並べます。
- スキルカードは幅9.5rem以上を目安に自動で列数が決まり（スマホでは2列）、1024px以上では4列です。
- 作品の画像はスワイプ・前後ボタン・ドット・矢印キーで切り替えられ、クリックで拡大します。
- 本文は最大幅72rem（`max-w-6xl`）で中央に配置し、ヘッダーは画面上部に固定されます。
- 配色はOSのライト/ダーク設定に追従します。アニメーションは `prefers-reduced-motion` で無効になります。
- フッターは少なくとも画面下端に配置されます。
- ノッチなどのセーフエリアを余白に反映し、詳細ダイアログは動的な画面の高さに収めて内部をスクロールできます。
- ナビゲーションと閉じるボタンの高さは44px以上です。

## 表示確認のチェックリスト

- ヘッダーの About / Skills / Works / Contact から対象セクションへ移動できる。
- スキルカードをTabで選択し、Enter / Spaceで開ける。
- 詳細表示中はTabの移動がダイアログ内に留まり、Escape・閉じるボタン・背景クリックで閉じられる。
- 閉じた後は元のカードにフォーカスが戻る。
- 320px幅でも一覧・詳細・404ページに横スクロールが出ない。
- `/` と存在しないURLから、本編へ戻れる。

確認する画面幅は端末名ではなく、利用できる幅を基準にします。

- スマホ: 320〜430px、横向き 568×320 / 844×390
- タブレット: 768×1024 / 820×1180 / 1024×1366 とその横向き
- 画面分割: 320px / 507px
- PC: 1440px / 1920px

ダイアログを開いたままの回転、長い本文、文字サイズ200%でも横にはみ出さず、閉じるボタンを操作できることを確認してください。
実機のSafariでは、アドレスバーの開閉とセーフエリアも確認します。

## ライセンス

- **ソースコード**: [GPL-3.0-or-later](LICENSE)
- **アイコン画像**（`src/assets/my_icon.avif`）: [CC BY-NC 4.0](https://creativecommons.org/licenses/by-nc/4.0/deed.ja)（クレジット: taitan291）
- **以下はGPLの対象外です。著作権はtaitan291に帰属し、すべての権利を留保します（All rights reserved）。**
  - サイトに表示される文章（`src/data/` 内の文字列や `src/data/content/**/*.md` を含む）
  - `src/data/content/` 内の画像（チーム作品のスクリーンショットは、各制作者・チームにも権利があります）
- スキル欄のロゴは各権利者の商標です。リポジトリには含めず、Simple Iconsから読み込んでいます。
- 依存ライブラリは各ライブラリのライセンスに従います。
