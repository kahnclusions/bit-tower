/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: ["./**/src/**/*.{rs,html,css}"],
  theme: {
    fontFamily: {
      sans: ["InterVariable", "sans-serif"],
      serif: ["Playfair", "Georgia"],
      display: ["Silkscreen"],
      cubic: ["Cubic"],
      pressstart: ["PressStart"],
      pressstartk: ["PressStartK"],
      dos: ["PerfectDOS"],
      openpx: ["OpenSansPX"],
      roca: ["Roca"],
    },
    extend: {
      boxShadow: {
        border:
          "inset 0 -1px 0 0 hsl(var(--grey)), inset -1px 0 0 0 hsl(var(--grey))",
      },
      colors: {
        border: "hsl(var(--border))",
        input: "hsl(var(--input))",
        ring: "hsl(var(--ring))",
        background: "hsl(var(--background))",
        background_highlight: "hsl(var(--background-highlight))",
        background_dark: "hsl(var(--background-dark))",
        foreground: "hsl(var(--foreground))",
        primary: {
          DEFAULT: "hsl(var(--primary))",
          foreground: "hsl(var(--primary-foreground))",
        },
        secondary: {
          DEFAULT: "hsl(var(--secondary))",
          foreground: "hsl(var(--secondary-foreground))",
        },
        destructive: {
          DEFAULT: "hsl(var(--destructive))",
          foreground: "hsl(var(--destructive-foreground))",
        },
        muted: {
          DEFAULT: "hsl(var(--muted))",
          foreground: "hsl(var(--muted-foreground))",
        },
        accent: {
          DEFAULT: "hsl(var(--accent))",
          foreground: "hsl(var(--accent-foreground))",
        },
        popover: {
          DEFAULT: "hsl(var(--popover))",
          foreground: "hsl(var(--popover-foreground))",
        },
        card: {
          DEFAULT: "hsl(var(--card))",
          foreground: "hsl(var(--card-foreground))",
        },
        green1: "hsl(var(--green-1))",
        green2: "hsl(var(--green-2))",
        green3: "hsl(var(--green-3))",
        grey: "hsl(var(--grey))",
      },
    },
  },
  plugins: [],
};
