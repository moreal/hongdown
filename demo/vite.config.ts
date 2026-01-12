import { defineConfig } from "vite";
import solid from "vite-plugin-solid";
import UnoCSS from "unocss/vite";

export default defineConfig({
  plugins: [
    UnoCSS(),
    solid(),
  ],
  base: process.env.GITHUB_ACTIONS ? "/hongdown/" : "/",
  build: {
    target: "esnext",
  },
});
