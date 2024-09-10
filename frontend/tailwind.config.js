/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{svelte,html,ts}'],
  theme: {
    extend: {
      colors: {
        gray: {
          100: '#a9a9ca',
          200: '#a9a9ca99'
        },
        red: {
          100: '#ff5a5c'
        },
        dark: {
          100: '#1f1c1c',
          200: '#232121',
          300: '#2c2727',
          400: '#322d2d',
          500: '#171515'
        }
      }
    }
  },
  plugins: [require('tailwind-scrollbar-hide')]
};
