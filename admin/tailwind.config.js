import daisyui from "daisyui"

module.exports = {
  content: [
    './src/pages/**/*.tsx',
    './src/components/**/*.tsx',
    './src/layouts/**/*.tsx',
  ],
  plugins: [
    daisyui,
    require('@tailwindcss/forms'),
  ],
}
