import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import tailwindcss from "@tailwindcss/vite";

export default defineConfig({
  plugins: [react(), tailwindcss()],
  server: {
    port: 3003,
    proxy: {
      "/graphql": "http://localhost:8083",
      "/graphiql": "http://localhost:8083",
      "/api": "http://localhost:8083",
    },
  },
});
