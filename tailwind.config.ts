import { Config } from "tailwindcss";

export default <Config>{
  content: ["./index.html", "./src/**/*.{vue,js,ts,jsx,tsx}"],
  theme: {
    extend: {
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
