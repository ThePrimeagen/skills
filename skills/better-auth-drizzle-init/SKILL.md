## better-auth-drizzle-init

### Goal
Set up Better Auth with Drizzle ORM and drizzle-kit using a migration-safe, team-friendly workflow.

### Rules
1. Use the Drizzle adapter in Better Auth and set `provider` to one of `"pg"`, `"mysql"`, or `"sqlite"`.
2. Keep the Better Auth config in a CLI-discoverable path (`./auth.ts`, `./lib/auth.ts`, `./utils/auth.ts`, or equivalents under `src/`) or always pass `--config` to Better Auth CLI.
3. For Drizzle projects, do not use `@better-auth/cli migrate`; use `@better-auth/cli generate` for schema generation, then `drizzle-kit generate` and `drizzle-kit migrate` for SQL migrations.
4. Always pass `--output` to Better Auth CLI generate so auth schema lands in a dedicated file (avoid root-level `schema.ts` collisions).
5. Keep `drizzle.config.ts` as source of truth for migrations. Include both your app schema files and Better Auth generated schema in `schema` paths.
6. Use this execution order after auth/plugin model changes:
   - regenerate Better Auth schema,
   - generate Drizzle SQL migration,
   - review SQL,
   - apply migration.
7. Commit schema and migration artifacts together (`auth schema file`, app schema files, and `drizzle/*` migration files/snapshots).
8. If you enable Better Auth experimental joins, ensure Drizzle relations exist and are passed through adapter schema where required.
9. If table or field names are customized, map them intentionally with adapter schema/model mapping (`schema`, `modelName`, `fields`) so Better Auth and Drizzle stay aligned.
10. If Better Auth CLI cannot resolve import aliases, temporarily switch auth config imports to relative paths when running CLI.
11. Do not rely on `@better-auth/cli init` for Drizzle bootstrapping; it is limited to specific scaffolding paths and does not replace the Drizzle migration flow.

### Recommended File Layout
```text
project/
  src/
    db/
      schema.ts
      auth-schema.ts
    lib/
      auth.ts
  drizzle/
  drizzle.config.ts
  package.json
```

### Canonical Setup

#### 1) Better Auth server config
```ts
import { betterAuth } from "better-auth";
import { drizzleAdapter } from "better-auth/adapters/drizzle";
import { db, schema } from "../db";

export const auth = betterAuth({
  database: drizzleAdapter(db, {
    provider: "pg",
    schema,
  }),
  emailAndPassword: {
    enabled: true,
  },
});
```

#### 2) drizzle-kit config
```ts
import { defineConfig } from "drizzle-kit";

export default defineConfig({
  dialect: "postgresql",
  schema: ["./src/db/schema.ts", "./src/db/auth-schema.ts"],
  out: "./drizzle",
  dbCredentials: {
    url: process.env.DATABASE_URL!,
  },
});
```

#### 3) package scripts
```json
{
  "scripts": {
    "auth:schema": "npx @better-auth/cli@latest generate --config ./src/lib/auth.ts --output ./src/db/auth-schema.ts --yes",
    "db:generate": "drizzle-kit generate",
    "db:migrate": "drizzle-kit migrate",
    "db:check": "drizzle-kit check",
    "db:sync": "npm run auth:schema && npm run db:generate",
    "db:sync:migrate": "npm run db:sync && npm run db:migrate"
  }
}
```

### Daily Workflow
1. Update Better Auth config and/or plugin config.
2. Run `auth:schema`.
3. Run `db:generate`.
4. Inspect generated SQL in `drizzle/*/migration.sql`.
5. Run `db:migrate`.
6. Commit all related schema and migration files together.

### Common Pitfalls
- Using `@better-auth/cli migrate` with Drizzle.
  - Fix: use Drizzle migration flow (`drizzle-kit generate` + `drizzle-kit migrate`).
- Forgetting `--output` on Better Auth CLI generate.
  - Fix: always target a dedicated file such as `src/db/auth-schema.ts`.
- Better Auth CLI cannot find auth config.
  - Fix: use `--config` explicitly.
- Better Auth CLI fails due to path aliases.
  - Fix: use relative imports in auth config for CLI execution.
- Experimental joins enabled but no relations.
  - Fix: define Drizzle relations and include relation schema in adapter mapping.

### Notes For Cloudflare Workers
- If runtime env access blocks CLI usage, prefer one of:
  - `import { env } from "cloudflare:workers"` in auth config (newer Workers setup), or
  - `nodejs_compat_populate_process_env` compatibility flag and `process.env` access.

### Reference Docs
- Better Auth installation: https://www.better-auth.com/docs/installation
- Better Auth CLI: https://www.better-auth.com/docs/concepts/cli
- Better Auth database concepts: https://www.better-auth.com/docs/concepts/database
- Better Auth Drizzle adapter: https://www.better-auth.com/docs/adapters/drizzle
- Drizzle Kit overview: https://orm.drizzle.team/docs/kit-overview
- drizzle-kit generate: https://orm.drizzle.team/docs/drizzle-kit-generate
- drizzle-kit migrate: https://orm.drizzle.team/docs/drizzle-kit-migrate
- drizzle config file: https://orm.drizzle.team/docs/drizzle-config-file
