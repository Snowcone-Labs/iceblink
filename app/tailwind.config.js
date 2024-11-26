/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/app/**/*.tsx", "./src/components/**/*.tsx"],
  presets: [require("nativewind/preset")],
  theme: {
    extend: {
      colors: {
        "iceblink-fg-dark": "#f7f7fa",
        "iceblink-bg-dark": "#5865F2",

        "iceblink-fg-dim": "#c8cbea",
        "iceblink-bg-dim": "#2f314b",

        "iceblink-fg-light": "#1e202f",
        "iceblink-bg-light": "#f7f7fa",

        "iceblink-bg-info": "#0c346b",
        "iceblink-fg-info": "#207efe",
        "iceblink-fg-success": "#20fea1",
        "iceblink-bg-success": "#0c6b43",
        "iceblink-bg-warning": "#6b450c",
        "iceblink-fg-warning": "#ffbf42",
        "iceblink-bg-danger": "#521111",
        "iceblink-fg-danger": "#ff4f4f",

        "iceblink-gray-1": "#898dae",
        "iceblink-gray-2": "#2d2f43",
        "iceblink-gray-3": "#2e314c",
        "iceblink-gray-4": "#363953",
      },
    },
  },
  plugins: [],
};
