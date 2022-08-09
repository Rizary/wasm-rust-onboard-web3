module.exports = {
    content: [
        "./devhtml/**/*.{html,js,ts,jsx,tsx}",
        "./src/**/*.rs"
    ],
    theme: {
      extend: {},
    },
    plugins: [
      require('@tailwindcss/forms'),
    ],
  }