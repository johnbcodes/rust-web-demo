import { defineConfig } from "vite-plus";
import tailwindcss from "@tailwindcss/vite";
import { resolve } from "node:path";

export default defineConfig({
  root: "ui/src",
  base: "/dist/",
  publicDir: false,
  plugins: [tailwindcss()],
  build: {
    outDir: "../target/public",
    emptyOutDir: true,
    manifest: "manifest.json",
    rollupOptions: {
      input: resolve("ui/src/base.html"),
    },
  },
});
