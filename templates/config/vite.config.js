import fs from "fs";

import { defineConfig, loadEnv } from "vite";

export default defineConfig(({ mode }) => {
  const env = loadEnv(mode, process.cwd(), "");
  const isDevHTTPSEnabled =
    typeof env.HTTPS_CERT === "string" &&
    env.HTTPS_CERT.length > 0 &&
    typeof env.HTTPS_KEY === "string" &&
    env.HTTPS_KEY.length > 0;
  return {
    server: isDevHTTPSEnabled
      ? {
          https: {
            key: fs.readFileSync(env.HTTPS_KEY),
            cert: fs.readFileSync(env.HTTPS_CERT),
          },
        }
      : undefined,
    plugins: [
      // Minimal CSP for client-side rendered (SPA)
      // Resources to learn more:
      // - https://developer.mozilla.org/en-US/docs/Web/HTTP/Guides/CSP
      // - https://cheatsheetseries.owasp.org/cheatsheets/Content_Security_Policy_Cheat_Sheet.html
      {
        name: "csp",
        transformIndexHtml(html) {
          const csp = `default-src 'none'; script-src 'self'; connect-src 'self'; img-src 'self'; style-src 'self'; form-action 'self'; object-src 'none'; media-src 'self'; base-uri 'none'; upgrade-insecure-requests;`;
          html = html.replace(
            /<head>/,
            `<head>\n<meta http-equiv="Content-Security-Policy" content="${csp}">`,
          );
          return html;
        },
      },
    ],
    build: {
      outDir: "dist/browser",
      rollupOptions: {
        output: {
          entryFileNames: "[name].[hash].js",
          chunkFileNames: "[name].[hash].js",
          assetFileNames: "[name].[hash].[ext]",
        },
      },
      sourcemap: false,
      minify: "terser",
      terserOptions: {
        format: {
          comments: false,
        },
        parse: {
          html5_comments: false,
        },
        sourceMap: false,
        compress: {
          drop_console: true,
          drop_debugger: true,
        },
      },
    },
  };
});
