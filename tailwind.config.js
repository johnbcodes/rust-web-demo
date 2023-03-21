module.exports = {
  content: ["./ui/templates/**/*.html", "./ui/src/*.html"],
  theme: {
    extend: {},
  },
  plugins: [
    require('@tailwindcss/forms')
  ],
}
