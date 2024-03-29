import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await"
import { defineConfig } from 'vite';
import solidPlugin from 'vite-plugin-solid';
// import devtools from 'solid-devtools/vite';

export default defineConfig({
  plugins: [
    /*
    Uncomment the following line to enable solid-devtools.
    For more info see https://github.com/thetarnav/solid-devtools/tree/main/packages/extension#readme
    */
    // devtools(),
    wasm(),
    topLevelAwait(),
    solidPlugin(),
  ],
  server: {
    host: "0.0.0.0",
    port: 3000,
    watch: {
      usePolling: true,
    },
  },
  build: {
    target: 'esnext',
  },
});
