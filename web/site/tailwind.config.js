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
                    "highlight": "#D68875",
                    "legal-highlight": "#D6D375"
                },
                "dark-square": {
                    "DEFAULT": "#B58863",
                    "highlight": "#AA593A",
                    "legal-highlight": "#BFB44F"
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
    safelist: [
        'text-light-square',
        'text-dark-square',
        'bg-light-square',
        'bg-dark-square',
        'bg-light-square-highlight',
        'bg-dark-square-highlight',
        'bg-light-square-legal-highlight',
        'bg-dark-square-legal-highlight',
    ]
  }