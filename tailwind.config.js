/** @type {import('tailwindcss').Config} */

// 色は style.css で定義した CSS 変数（RGB の 3 値）を参照し、ライト/ダークで自動的に切り替わる。
const token = (name) => `rgb(var(--${name}) / <alpha-value>)`;

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
  future: {
    // `hover:` は本当にホバーできる端末でだけ効かせ、タップ後にホバー状態が残らないようにする。
    hoverOnlyWhenSupported: true,
  },
  theme: {
    extend: {
      colors: {
        base: token("bg"),
        surface: {
          DEFAULT: token("surface"),
          2: token("surface-2"),
        },
        line: token("line"),
        fg: token("fg"),
        muted: token("muted"),
        accent: {
          DEFAULT: token("accent"),
          text: token("accent-text"),
          on: token("on-accent"),
        },
        stack: {
          frontend: token("stack-frontend"),
          backend: token("stack-backend"),
          app: token("stack-app"),
          infra: token("stack-infra"),
        },
        game: gameColors,
      },
      fontFamily: {
        sans: [
          '"Space Grotesk"',
          '"Hiragino Kaku Gothic ProN"',
          '"Hiragino Sans"',
          '"Noto Sans JP"',
          '"Yu Gothic"',
          "Meiryo",
          "sans-serif",
        ],
        mono: ['"JetBrains Mono"', "ui-monospace", "SFMono-Regular", "Menlo", "monospace"],
      },
      keyframes: {
        "fade-up": {
          from: { opacity: "0", transform: "translateY(16px)" },
          to: { opacity: "1", transform: "translateY(0)" },
        },
        "fade-in": {
          from: { opacity: "0" },
          to: { opacity: "1" },
        },
        "letter-rise": {
          from: { opacity: "0", transform: "translateY(0.45em) rotate(8deg)" },
          to: { opacity: "1", transform: "none" },
        },
        blink: {
          "0%, 49%": { opacity: "1" },
          "50%, 100%": { opacity: "0" },
        },
        // ループするアニメーションは静止状態で始まり終わるので、動きを減らす設定では止まって見える。
        float: {
          "0%, 100%": { transform: "translateY(0)" },
          "50%": { transform: "translateY(-10px)" },
        },
        sway: {
          "0%, 100%": { transform: "rotate(6deg)" },
          "50%": { transform: "rotate(9deg)" },
        },
        drift: {
          "0%, 100%": { transform: "translate(0, 0) scale(1)" },
          "50%": { transform: "translate(-48px, 32px) scale(1.12)" },
        },
      },
      animation: {
        "fade-up": "fade-up 0.7s cubic-bezier(0.2, 0.7, 0.2, 1) both",
        "fade-in": "fade-in 1.2s ease-out both",
        "letter-rise": "letter-rise 0.8s cubic-bezier(0.2, 0.7, 0.2, 1) both",
        blink: "blink 1.1s step-end infinite",
        float: "float 6s ease-in-out infinite",
        sway: "sway 8s ease-in-out infinite",
        drift: "drift 16s ease-in-out infinite",
      },
    },
  },
  plugins: [],
};
