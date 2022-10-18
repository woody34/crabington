const colors = require('tailwindcss/colors')

module.exports = {
  purge: {
      mode: "all",
      content: [
          "./src/**/*.rs",
          "./index.html",
          "./src/**/*.html",
          "./src/**/*.css",
      ],
  },
  theme: {
    extend: {
      colors: {
        primary: {
          '50': '#f3f5f0',
          '100': '#e5e9de',
          '200': '#ccd4bf',
          '300': '#aebb9b',
          '400': '#91a17a',
          '500': '#74855d',
          '600': '#5a6947',
          '700': '#465239',
          '800': '#3a4331',
          '900': '#343a2d',
        },
        secondary: {
          '50': '#fbf7f1',
          '100': '#f6ecde',
          '200': '#e7cba9',
          '300': '#e0ba91',
          '400': '#d39764',
          '500': '#c97e46',
          '600': '#bb693b',
          '700': '#9c5232',
          '800': '#7d432f',
          '900': '#663928',
        },
        tertiary: {
          '50': '#f5f3e7',
          '100': '#eeead7',
          '200': '#dbd5ac',
          '300': '#c9bc80',
          '400': '#bca863',
          '500': '#b09150',
          '600': '#9b7844',
          '700': '#825e3b',
          '800': '#6b4c35',
          '900': '#59402e',
        },
      },
      keyframes: {
        'fade-in': {
          '0%': { opacity: '0', transform: 'translateY(-12px)' },
          '100%': { opacity: '1', transform: 'translateY(0)' }
        }
      },
      animation: {
        'fade-in': 'fade-in .5s ease-in forwards',
      }
    }
  },
  variants: {},
  plugins: [
    require('@tailwindcss/forms'), 
  ],
};