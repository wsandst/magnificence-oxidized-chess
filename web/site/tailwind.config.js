/** @type {import('tailwindcss').Config} */
export default {
    content: [
        "./index.html",
        "./src/**/*.{vue,js,ts,jsx,tsx}",
    ],
    theme: {
        extend: {
            colors: {
                "light-square": {
                    "DEFAULT": "#F0D9B5",
                    "hightlight": "#CDD26A"
                },
                "dark-square": {
                    "DEFAULT": "#B58863",
                    "hightlight": "#AAA23A"
                }
            }
        },
    },
    plugins: [],
  }