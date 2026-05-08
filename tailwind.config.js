/** @type {import('tailwindcss').Config} */
const neutralStack = {
  bg: "#F5F5F5",
  text: "#525252",
  bar: "#525252",
  border: "#E5E5E5",
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
        stack: {
          frontend: { ...neutralStack },
          backend: { ...neutralStack },
          app: { ...neutralStack },
          infra: { ...neutralStack },
        },
      },
    },
  },
  plugins: [],
};
