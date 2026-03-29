import tailwindcss from "@tailwindcss/vite";
import { defineConfig } from "vite";

export default defineConfig({
  root: ".",
  publicDir: "public",
  plugins: [tailwindcss()],
});
