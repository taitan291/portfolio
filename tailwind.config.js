/** @type {import('tailwindcss').Config} */
// bar は熟練度 100 の色。実際のバー色は熟練度に応じて Rust 側で OKLCH から生成する。
const stackColors = {
  frontend: { bg: "#EEF6FE", text: "#255079", bar: "#579DE2", border: "#C4DAF2" },
  backend: { bg: "#EBF8F8", text: "#005959", bar: "#00AEAE", border: "#BAE0E0" },
  infra: { bg: "#FDF3EC", text: "#703F19", bar: "#D38246", border: "#EDD1BF" },
  app: { bg: "#FEF1F4", text: "#733747", bar: "#D8758E", border: "#F0CDD4" },
};

module.exports = {
  content: ["*.html", "./src/**/*.rs"],
  theme: {
    extend: {
      colors: {
        cyber: {
          page: "#FFFFFF",
          container: "#FAFAFA",
          layout: "#FFFFFF",
          surface: "#FFFFFF",
          text: "#262626",
          layouttext: "#525252",
          muted: "#737373",
          accent: "#171717",
          border: "#E5E5E5",
        },
        stack: stackColors,
      },
    },
  },
  plugins: [],
};
