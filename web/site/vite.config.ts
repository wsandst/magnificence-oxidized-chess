import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import ViteRsw from 'vite-plugin-rsw'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    vue(),
    ViteRsw(),
  ],
  base: '/chess/',
  server: {
    host: "0.0.0.0",
    fs: {
        allow: ["../"]
    }
  }
})