import { Config } from "tailwindcss";

export default <Config>{
  content: ["./index.html", "./src/**/*.{vue,js,ts,jsx,tsx}"],
  theme: {
    extend: {
      colors: {
        primary: {
          "50": "#fef1f6",
          "100": "#fee5ef",
          "200": "#ffcbe1",
          "300": "#ffa1c6",
          "400": "#ff6ea3",
          "500": "#fa3a7b",
          "600": "#ea1854",
          "700": "#cc0a3c",
          "800": "#a80c32",
          "900": "#8c0f2d",
          "950": "#560116",
          DEFAULT: "#ff6ea3",
        },
      },
      fontSize: {
        "2xs": [
          "0.625rem",
          {
            lineHeight: "0.625rem",
          },
        ],
      },
    },
  },
  plugins: [],
};
