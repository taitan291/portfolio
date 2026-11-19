/** @type {import('tailwindcss').Config} */

// Colors are CSS variables (RGB triplets) defined in style.css so that they
// switch automatically between light and dark mode.
const token = (name) => `rgb(var(--${name}) / <alpha-value>)`;

module.exports = {
  content: ["*.html", "./src/**/*.rs"],
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
        blink: {
          "0%, 49%": { opacity: "1" },
          "50%, 100%": { opacity: "0" },
        },
      },
      animation: {
        "fade-up": "fade-up 0.7s cubic-bezier(0.2, 0.7, 0.2, 1) both",
        blink: "blink 1.1s step-end infinite",
      },
    },
  },
  plugins: [],
};
