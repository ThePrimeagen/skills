## Cloudflare Project (Hono + TypeScript)

### Rules
1. Bootstrap every new project with Cloudflare C3 via npx, using TypeScript and Hono: `npx create-cloudflare@latest <project-name> --framework=hono --lang=ts`.
2. Use `wrangler.jsonc` as the Wrangler config file (not TOML), and keep bindings plus compatibility settings as the source of truth there.
3. Keep TypeScript at strictest settings: extend `@tsconfig/strictest/tsconfig.json` (or equivalent strictest flags) and do not relax strict options.
4. Generate runtime and binding types with `npx wrangler types`; include `./worker-configuration.d.ts` in `tsconfig.json` and rerun after any `wrangler.jsonc` change.
5. Do not define routes inline in the worker entrypoint; each route must be a named function in route modules grouped by domain (for example `src/routes/auth.ts`) and then mounted from `src/index.ts`.

### Example 1: Bootstrap + strict typing + Wrangler types
```bash
npx create-cloudflare@latest my-api --framework=hono --lang=ts
cd my-api
npm i -D @tsconfig/strictest
npx wrangler types
```

```jsonc
// tsconfig.json
{
  "extends": "@tsconfig/strictest/tsconfig.json",
  "compilerOptions": {
    "types": ["./worker-configuration.d.ts"]
  }
}
```

### Example 2: Grouped route functions (`routes/auth.ts`)
```ts
// src/routes/auth.ts
import { Hono } from "hono";
import type { Context } from "hono";

type AppEnv = { Bindings: Env };

const auth = new Hono<AppEnv>();

async function login(c: Context<AppEnv>) {
  return c.json({ ok: true });
}

async function logout(c: Context<AppEnv>) {
  return c.json({ ok: true });
}

auth.post("/login", login);
auth.post("/logout", logout);

export function registerAuthRoutes(app: Hono<AppEnv>) {
  app.route("/auth", auth);
}
```

```ts
// src/index.ts
import { Hono } from "hono";
import { registerAuthRoutes } from "./routes/auth";

const app = new Hono<{ Bindings: Env }>();

registerAuthRoutes(app);

export default app;
```
