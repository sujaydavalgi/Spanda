/**
 * vite.config module (vite.config.ts).
 * @module
 */

import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

export default defineConfig({
  plugins: [react()],
  server: { port: 5173 },
  optimizeDeps: {
    exclude: ["spanda-wasm"],
  },
});
