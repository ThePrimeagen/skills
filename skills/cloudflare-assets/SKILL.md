---
description: Cloudflare Assets binding usage rules
globs: src/**/*.ts
alwaysApply: false
---

## Cloudflare Assets

### Rules
1. Configure static assets in `wrangler.jsonc` with an `assets` block that sets both `binding` and `directory`.
2. Serve static files from the Worker by returning `env.ASSETS.fetch(request)` inside `fetch`.
3. Keep the binding name consistent across config and code (for example `ASSETS` in both places).

### Example 1: `wrangler.jsonc`
```jsonc
{
  "$schema": "node_modules/wrangler/config-schema.json",
  "name": "my-worker",
  "main": "src/index.ts",
  "compatibility_date": "2026-02-24",
  "assets": {
    "binding": "ASSETS",
    "directory": "./public"
  }
}
```

### Example 2: Minimal Worker
```ts
export default {
  async fetch(request, env, ctx) {
    return env.ASSETS.fetch(request);
  },
};
```
