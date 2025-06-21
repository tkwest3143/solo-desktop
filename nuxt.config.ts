export default defineNuxtConfig({
  // (optional) Enable the Nuxt devtools
  devtools: { enabled: true },

  //フロント側ルートディレクトリ
  srcDir: "src/",

  //SPAレンダリングモードで実装
  ssr: false,

  // Enables the development server to be discoverable by other devices when running on iOS physical devices
  devServer: { host: process.env.TAURI_DEV_HOST || "localhost" },

  vite: {
    // Better support for Tauri CLI output
    clearScreen: false,
    // Enable environment variables
    // Additional environment variables can be found at
    // https://v2.tauri.app/reference/environment-variables/
    envPrefix: ["VITE_", "TAURI_"],
    server: {
      // Tauri requires a consistent port
      strictPort: true,
    },
  },

  compatibilityDate: "2024-12-27",
  modules: ["@nuxt/icon", "@nuxtjs/tailwindcss", "@nuxtjs/color-mode", "@nuxt/test-utils/module"],
  
  colorMode: {
    preference: 'light', // default theme
    fallback: 'light', // fallback theme
    classSuffix: '',
  },
});
