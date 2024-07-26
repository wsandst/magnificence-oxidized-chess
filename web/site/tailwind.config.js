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
                    "highlight": "#CDD26A"
                },
                "dark-square": {
                    "DEFAULT": "#B58863",
                    "highlight": "#AAA23A"
                },
                "container": {
                    "DEFAULT": "rgb(53, 53, 53)",
                    "lighter": "rgb(74, 74, 74)"
                },
                "primary": {
                    "DEFAULT": "hsla(0, 0%, 100%, 0.92)",
                    "darker": "hsla(0, 0%, 100%, 0.72)"
                }
            },
            fontFamily: {
                'sans': ['ProximaNova', 'Arial', 'sans-serif'],
                'mono': ['DroidSansMono'],
            },
            screens: {
                'sm': '640px',          
                'md': '768px',
                'lg': '1024px',
                'xl': '1280px',
                '2xl': '1536px',
                '3xl': '1792px',
              }
        },
    },
    plugins: [],
  }