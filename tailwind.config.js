module.exports = {
  content: ["./templates/**/*.html", "./src/frontend/*.html"],
  theme: {
    extend: {},
  },
  plugins: [
    require('@tailwindcss/forms')
  ],
}
