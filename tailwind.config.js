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

// 404 射的ゲームの DOM オーバーレイの配色 (bg-game-black, text-game-yellow, ...)。
// Canvas 側のパレット (components/game/platform/palette.rs) と同じ値にそろえる。
const gameColors = {
  black: "#0b0d1a", // 背景・ボタン文字 (palette::BLACK)
  red: "#d8344c", // 主ボタン・見出しの影 (palette::RED)
  "red-hover": "#e8506a", // 主ボタンのホバー
  "red-light": "#ff7a8a", // 「0 は赤い的」の説明・リセットのホバー
  penalty: "#ff5a6e", // HUD の誤射ペナルティ表示
  "blue-light": "#8fb4ff", // 「O は青い的」の説明
  yellow: "#f5d45e", // 見出し・進捗・強調 (palette::YELLOW)
  cream: "#fbf1da", // 副ボタン (palette::CREAM)
};

module.exports = {
  content: ["*.html", "./src/**/*.rs"],
  theme: {
    extend: {
      colors: {
        site: siteColors,
        stack: stackColors,
        game: gameColors,
      },
    },
  },
  plugins: [],
};
