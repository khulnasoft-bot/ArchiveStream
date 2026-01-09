// eslint.config.js
import nextPlugin from "@next/eslint-plugin-next";

export default [
  {
    ignores: [
      "node_modules/",
      ".next/",
      "out/",
      "dist/",
      "*.log",
      "npm-debug.log*",
      "yarn-debug.log*",
      "yarn-error.log*",
      ".pnpm-debug.log*",
      ".env",
      ".env.local",
      ".env.production",
      ".vscode/",
      ".idea/",
      "*.swp",
      "*.swo",
      "*~",
      ".DS_Store",
      "*.db",
      "*.sqlite",
      "*.wasm",
      "*.so",
      "*.dylib",
      "*.dll",
      "coverage/",
      ".nyc_output/",
      ".cache/",
      "tmp/",
      "temp/",
    ],
    plugins: {
      "@next/next": nextPlugin,
    },
    rules: {
      ...nextPlugin.configs["core-web-vitals"].rules,
      // Your custom rules here
    },
  },
];
