/** @type {import('tailwindcss').Config} */

// Colors are CSS variables (RGB triplets) defined in style.css so that they
// switch automatically between light and dark mode.
const token = (name) => `rgb(var(--${name}) / <alpha-value>)`;

module.exports = {
  content: ["*.html", "./src/**/*.rs"],
  future: {
    // `hover:` only applies on devices that can really hover, so cards and
    // buttons don't get stuck in their hover state after a tap on touch screens.
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
        // Looping keyframes start and end at rest, so reduced motion leaves them still.
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
