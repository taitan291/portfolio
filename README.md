# taitan291 Portfolio

Rust / LeptosのCSR（WebAssembly）で実装したポートフォリオです。
Trunkでビルドし、Tailwind CSS v3でスタイルを生成します。
自己紹介、スキル一覧と詳細ダイアログ、404ページを実装しています。

## 開発

Nixのflakesを有効にした環境で、次を実行します。

```sh
nix develop
trunk serve
```

`http://localhost:8080` で確認できます。direnvを利用する場合は
`direnv allow` でも同じ開発環境に入れます。
`cargo run` はブラウザ用アプリを起動しません。
開発シェルは `RUSTUP_TOOLCHAIN` も設定し、既存のrustupがある環境でも
Nixで固定したRust・rustfmt・Clippyを使用します。

`nix fmt` でtreefmtを実行し、Nixをalejandra、Rustをrustfmtで整形します。
開発環境内では `treefmt` でも実行できます。

`nix develop`（direnv利用時は `direnv reload`）でpre-commitフックが自動で有効になります。
コミット時にステージ済みのNix・Rustファイルをtreefmtで整形します。
修正が発生するとコミットを中断するので、差分を確認して再度 `git add` してください。
全ファイルを手動で確認するには `pre-commit run --all-files` を実行します。
生成される `.pre-commit-config.yaml` はGit管理の対象外です。

## 確認とビルド

開発環境内で実行します。

```sh
nix fmt -- --ci
cargo clippy --locked --target wasm32-unknown-unknown -- -D warnings
trunk build --release --locked --public-url "/portfolio/"
```

出力先は `dist/` です。PRとmainへのpush時に同じ検証を実行します。
mainへのpush時はGitHub Pagesへのデプロイも実行します。
プロジェクトサイト（`https://taitan291.github.io/portfolio/`）のサブパス配信を前提としています。
ベースパスは `Trunk.toml` の `public_url`、`src/app.rs` の `Router base`、
ワークフローの `--public-url` で揃えています。アセットは `assets/...` の相対パスで指定します。

ブラウザでは以下を確認してください。

- ヘッダー・目次のAbout / Skillsから対象セクションへ移動できる。
- スキルカードをTabで選択し、Enter / Spaceで開ける。
- 詳細表示中はTabの移動がダイアログ内に留まり、Escape・閉じるボタン・背景クリックで閉じられる。
- 閉じた後は元のカードにフォーカスが戻る。
- 320px幅の画面でも一覧・詳細・404ページに横スクロールが発生しない。
- 存在しないURLの表示からトップへ戻れる。

レスポンシブ表示は端末名ではなく利用できる画面幅に合わせます。
スキルカードは画面幅に応じて1〜3列、640〜1023pxでは3列です。
1024px以上ではページの最大幅制限を外し、目次を左、本文を右に配置します。
PCのスキルカードは本文の幅に応じて列数が増え、フッターは少なくとも画面下端に配置されます。
ノッチなどのセーフエリアを余白に反映し、詳細ダイアログは動的な画面の高さに
収めて内部をスクロールできます。ナビゲーションと閉じるボタンは高さ44px以上です。

表示確認ではスマホ（320〜430px）、スマホ横向き（568×320 / 844×390）、
iPad相当の縦横（768×1024 / 820×1180 / 1024×1366とその横向き）、
画面分割（320px / 507px）、PC（1440px / 1920px）を含めてください。
ダイアログを開いたままの回転、長い本文、文字サイズ200%でも横にはみ出さず、
閉じるボタンを操作できることを確認します。
実機のSafariでは、アドレスバーの開閉とセーフエリアも確認してください。

詳細表示にはHTMLの `<dialog>` と `showModal()` を使用しています。

## コンテンツの変更

- スキル一覧：`src/pages/index.rs` の `SKILLS`
- ナビゲーション：`src/components/element/header.rs` の `PAGE_LINKS`
- 色：`tailwind.config.js`
- 画像：`src/assets/`

作品の情報は未登録のため、Worksへのリンクは表示していません。
`WorksCard` はコンパイル対象に含まれています。作品を追加する場合は、
`Section` とナビゲーションを追加し、次のようにMarkdown本文を渡します。

```rust,ignore
<WorksCard
    title="作品名"
    image_url="assets/work.avif"
    stack=vec!["Rust".to_string()]
    markdown=include_str!("../../content/work.md")
/>
```

`include_str!` のパスは呼び出し元Rustファイルからの相対パスです。
MarkdownはHTMLとして描画するため、リポジトリで管理する信頼済みの本文を渡してください。
