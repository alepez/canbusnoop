/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./crates/ui/src/**/*.{rs,html,css}",
    "./src/**/*.{rs,html,css}",
    "./dist/**/*.html",
  ],
  theme: {
    extend: {},
  },
  plugins: [
    require("daisyui"),
  ],
}

