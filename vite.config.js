import react from "@vitejs/plugin-react";
import { defineConfig, Plugin } from "vite";

export default defineConfig({
  plugins: [react()],
  clearScreen: false,
  server: {
    port: 3000,
    strictPort: true,
  },
  envPrefix: ["VITE_", "TAURI_"],
  build: {
    outDir: "build",
    // Tauri uses Chromium on Windows and WebKit on macOS and Linux
    target: process.env.TAURI_PLATFORM == "windows" ? "chrome105" : "safari13",
    // don't minify for debug builds
    minify: !process.env.TAURI_DEBUG ? "esbuild" : false,
    // produce sourcemaps for debug builds
    sourcemap: !!process.env.TAURI_DEBUG,
  },
});
