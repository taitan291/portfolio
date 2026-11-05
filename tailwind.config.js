/** @type {import('tailwindcss').Config} */

// 技術スタック別の配色。キーは `data::types::Stack` のバリアント名に対応する。
//   surface: 背景 / fg: 文字 / border: 枠線
// 熟練度バーの色は熟練度に応じて Rust 側 (components/cards/stack_style.rs) で OKLCH から生成する。
const stackColors = {
  frontend: { surface: "#EEF6FE", fg: "#255079", border: "#C4DAF2" },
  backend: { surface: "#EBF8F8", fg: "#005959", border: "#BAE0E0" },
  infrastructure: { surface: "#FDF3EC", fg: "#703F19", border: "#EDD1BF" },
  application: { surface: "#FEF1F4", fg: "#733747", border: "#F0CDD4" },
};

// サイト全体のニュートラル配色 (bg-site-page, text-site-muted, ...)。
const siteColors = {
  page: "#FFFFFF", // ページ背景
  surface: "#FFFFFF", // ヘッダー・カード・ダイアログなどの面
  subtle: "#FAFAFA", // ホバー・コード・バー背景などの淡い面
  text: "#262626", // 本文
  subtext: "#525252", // ナビゲーションなどの副次テキスト
  muted: "#737373", // 補足テキスト
  accent: "#171717", // 見出し・リンク強調
  link: "#2563EB", // 外部リンク
  border: "#E5E5E5", // 区切り線・枠線
  "border-strong": "#A3A3A3", // ホバー時の枠線
  focus: "#525252", // フォーカスリング
};

module.exports = {
  content: ["*.html", "./src/**/*.rs"],
  theme: {
    extend: {
      colors: {
        site: siteColors,
        stack: stackColors,
      },
    },
  },
  plugins: [],
};
