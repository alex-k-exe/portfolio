// @ts-check
import { defineConfig } from "astro/config";
import mdx from "@astrojs/mdx";
import svelte from "@astrojs/svelte";

import tailwindcss from "@tailwindcss/vite";

// https://astro.build/config
export default defineConfig({
  site: "https://alexkammin.com",
  integrations: [mdx(), svelte()],

  vite: {
    plugins: [tailwindcss()],
  },
});