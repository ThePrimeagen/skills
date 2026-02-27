# OAuth Implementation Checklist

Better Auth + Drizzle + Hono + Cloudflare Worker.
Every step is a checkbox. Do not skip any. Do them in order.

---

## 1. Rules

- [ ] Never write auth SQL by hand
- [ ] Always generate auth schema with Better Auth CLI first
- [ ] Always generate SQL migrations with Drizzle Kit second
- [ ] Always apply migrations with Drizzle Kit third
- [ ] Keep local DB URL in `.dev.local` as `LOCAL_DATABASE_URL` (single source of truth)
- [ ] Keep `dev`, `drizzle.config.ts`, `drizzle.local.config.ts`, and `src/lib/auth.ts` aligned to `.dev.local`

---

## 2. Install Dependencies

- [ ] Install runtime dependencies:

```bash
bun add better-auth drizzle-orm hono pg dotenv
```

- [ ] Install dev dependencies:

```bash
bun add -d drizzle-kit wrangler @types/pg
```

- [ ] Confirm `package.json` has these in `dependencies`:
  - [ ] `better-auth`
  - [ ] `drizzle-orm`
  - [ ] `hono`
  - [ ] `pg`
  - [ ] `dotenv`
- [ ] Confirm `package.json` has these in `devDependencies`:
  - [ ] `drizzle-kit`
  - [ ] `wrangler`

---

## 3. Create Local Environment Files

- [ ] Create `.dev.local` with exactly:

```
LOCAL_DATABASE_URL="postgresql://postgres:postgres@127.0.0.1:5432/<ProjectName>"
```

- [ ] Create `.dev.vars` with:

```
BASE_URL=http://localhost:<SitePort>
BETTER_AUTH_SECRET=<generate-a-long-random-secret>
TWITTER_CLIENT_ID=<your-twitter-client-id>
TWITTER_CLIENT_SECRET=<your-twitter-client-secret>
ALLOWED_ORIGINS=http://localhost:<SitePort>
```

- [ ] Confirm `.dev.local` is in `.gitignore`
- [ ] Confirm `.dev.vars` is in `.gitignore`

---

## 4. Create The Worker Env Type

- [ ] Create `src/types/env.ts`:

```ts
export type AppEnv = {
  Bindings: {
    ASSETS: {
      fetch(request: Request): Promise<Response>;
    };
    HYPERDRIVE: {
      connectionString: string;
    };
    ALLOWED_ORIGINS?: string;
    BASE_URL: string;
    BETTER_AUTH_SECRET?: string;
    SESSION_SIGNING_KEY?: string;
    TWITTER_CLIENT_ID: string;
    TWITTER_CLIENT_SECRET: string;
  };
};
```

- [ ] Confirm every auth-related binding is present:
  - [ ] `HYPERDRIVE` (DB connection via Cloudflare Hyperdrive)
  - [ ] `BASE_URL` (origin for callback URL construction)
  - [ ] `BETTER_AUTH_SECRET` (session signing)
  - [ ] `SESSION_SIGNING_KEY` (fallback session signing)
  - [ ] `TWITTER_CLIENT_ID`
  - [ ] `TWITTER_CLIENT_SECRET`
  - [ ] `ALLOWED_ORIGINS` (optional, comma-separated trusted origins)

---

## 5. Create The Better Auth CLI Config

This file is ONLY used by the Better Auth CLI for schema generation. It is NOT the runtime config.

- [ ] Create `src/lib/auth.ts`:

```ts
import { betterAuth } from "better-auth";
import { drizzleAdapter } from "better-auth/adapters/drizzle";
import { config } from "dotenv";
import { drizzle } from "drizzle-orm/node-postgres";
import { Pool } from "pg";
import * as schema from "../db/auth-schema";

config({ path: ".dev.local" });

const connectionString =
  process.env["LOCAL_DATABASE_URL"] ??
  "postgresql://postgres:postgres@127.0.0.1:5432/<ProjectName>";

const dbClient = new Pool({ connectionString });
const db = drizzle(dbClient, { schema });

export const auth = betterAuth({
  baseURL: process.env["BASE_URL"] ?? "http://localhost:<SitePort>",
  basePath: "/api/auth",
  secret:
    process.env["BETTER_AUTH_SECRET"] ??
    process.env["SESSION_SIGNING_KEY"] ??
    "replace-me-before-production",
  database: drizzleAdapter(db, {
    provider: "pg",
    schema,
  }),
  socialProviders: {
    twitter: {
      clientId: process.env["TWITTER_CLIENT_ID"] ?? "",
      clientSecret: process.env["TWITTER_CLIENT_SECRET"] ?? "",
    },
  },
  trustedOrigins: [
    process.env["BASE_URL"] ?? "http://localhost:<SitePort>",
    "http://localhost:<SitePort>",
  ],
  advanced: {
    useSecureCookies: true,
  },
});
```

- [ ] Confirm it reads `LOCAL_DATABASE_URL` from `.dev.local`
- [ ] Confirm `basePath` is `/api/auth`
- [ ] Confirm `socialProviders.twitter` is configured
- [ ] Confirm it imports `* as schema from "../db/auth-schema"`

---

## 6. Create App User Table

Your app needs a table to map the provider's account id to your own user identity.

- [ ] Create `src/db/schema.ts` with at minimum an app user table:

```ts
import { pgTable, text } from "drizzle-orm/pg-core";

export const appUser = pgTable("app_user", {
  xId: text("x_id").primaryKey(),
  mashId: text("mash_id"),
});
```

- [ ] Confirm `app_user` has `x_id` as primary key (this will hold the Twitter `account_id` from the Better Auth `account` table)

---

## 7. Add package.json Scripts

- [ ] Add these auth-related scripts to `package.json`:

```json
{
  "scripts": {
    "auth:schema": "bunx @better-auth/cli@latest generate --config ./src/lib/auth.ts --output ./src/db/auth-schema.ts --yes",
    "db:generate": "drizzle-kit generate",
    "db:migrate": "drizzle-kit migrate",
    "db:migrate:local": "drizzle-kit migrate --config drizzle.local.config.ts",
    "db:migrate:remote": "drizzle-kit migrate --config drizzle.remote.config.ts"
  }
}
```

- [ ] Confirm `auth:schema` points `--config` at `./src/lib/auth.ts`
- [ ] Confirm `auth:schema` points `--output` at `./src/db/auth-schema.ts`

---

## 8. Configure Drizzle Kit

- [ ] Create `drizzle.config.ts`:

```ts
import { defineConfig } from "drizzle-kit";
import { config } from "dotenv";

config({ path: ".dev.local" });

const connectionString =
  process.env.LOCAL_DATABASE_URL ??
  "postgresql://postgres:postgres@127.0.0.1:5432/<ProjectName>";

if (!connectionString) {
  throw new Error("Set LOCAL_DATABASE_URL before running Drizzle commands.");
}

export default defineConfig({
  out: "./drizzle/migrations",
  schema: ["./src/db/schema.ts", "./src/db/auth-schema.ts"],
  dialect: "postgresql",
  dbCredentials: {
    url: connectionString,
  },
});
```

- [ ] Create `drizzle.local.config.ts` (identical but without the throw guard):

```ts
import { defineConfig } from "drizzle-kit";
import { config } from "dotenv";

config({ path: ".dev.local" });

const connectionString =
  process.env.LOCAL_DATABASE_URL ??
  "postgresql://postgres:postgres@127.0.0.1:5432/<ProjectName>";

export default defineConfig({
  out: "./drizzle/migrations",
  schema: ["./src/db/schema.ts", "./src/db/auth-schema.ts"],
  dialect: "postgresql",
  dbCredentials: {
    url: connectionString,
  },
});
```

- [ ] Confirm both configs include BOTH schema files in the `schema` array:
  - [ ] `./src/db/schema.ts`
  - [ ] `./src/db/auth-schema.ts`

---

## 9. Generate Auth Schema And Migrate

- [ ] Start local postgres:

```bash
bun run db:local
```

- [ ] Generate Better Auth table definitions into `src/db/auth-schema.ts`:

```bash
bun run auth:schema
```

- [ ] Confirm `src/db/auth-schema.ts` was generated and contains these tables:
  - [ ] `user` (id, name, email, emailVerified, image, createdAt, updatedAt)
  - [ ] `session` (id, expiresAt, token, createdAt, updatedAt, ipAddress, userAgent, userId)
  - [ ] `account` (id, accountId, providerId, userId, accessToken, refreshToken, idToken, accessTokenExpiresAt, refreshTokenExpiresAt, scope, password, createdAt, updatedAt)
  - [ ] `verification` (id, identifier, value, expiresAt, createdAt, updatedAt)
- [ ] Confirm relations were generated:
  - [ ] `userRelations` (has many sessions, has many accounts)
  - [ ] `sessionRelations` (belongs to user)
  - [ ] `accountRelations` (belongs to user)
- [ ] Generate SQL migration:

```bash
bun run db:generate
```

- [ ] Confirm migration files appeared in `drizzle/migrations/`
- [ ] Apply migration locally:

```bash
bun run db:migrate:local
```

- [ ] Confirm migration applied without errors
- [ ] If you change Better Auth config or plugins later, re-run `auth:schema` THEN `db:generate` THEN `db:migrate:local`

---

## 10. Implement Runtime Auth Factory

This is the runtime config used by the actual Worker. Different from the CLI config in Step 5.

- [ ] Create `src/auth/runtime.ts`:

```ts
import { betterAuth } from "better-auth";
import { drizzleAdapter } from "better-auth/adapters/drizzle";
import { drizzle } from "drizzle-orm/node-postgres";
import { Pool } from "pg";
import * as schema from "../db/auth-schema";
import type { AppEnv } from "../types/env";

type AuthBindings = AppEnv["Bindings"];

const DEFAULT_BASE_URL = "http://localhost:<SitePort>";
const LOCAL_HOSTNAMES = new Set(["localhost", "127.0.0.1", "::1"]);

function parseCSV(value?: string): string[] {
  if (!value) {
    return [];
  }
  return value
    .split(",")
    .map((entry) => entry.trim())
    .filter((entry) => entry.length > 0);
}

function normalizeBaseURL(input: string): string | null {
  try {
    const parsed = new URL(input);
    if (LOCAL_HOSTNAMES.has(parsed.hostname) && parsed.protocol === "https:") {
      parsed.protocol = "http:";
    }
    return parsed.origin;
  } catch {
    return null;
  }
}

function resolveBaseURL(bindings: AuthBindings, requestURL?: string): string {
  const configuredBaseURL = bindings.BASE_URL?.trim();
  if (configuredBaseURL) {
    const normalizedConfiguredBaseURL = normalizeBaseURL(configuredBaseURL);
    if (normalizedConfiguredBaseURL) {
      return normalizedConfiguredBaseURL;
    }
  }
  if (requestURL) {
    const normalizedRequestBaseURL = normalizeBaseURL(requestURL);
    if (normalizedRequestBaseURL) {
      return normalizedRequestBaseURL;
    }
  }
  return DEFAULT_BASE_URL;
}

function resolveSecret(bindings: AuthBindings): string {
  return (
    bindings.BETTER_AUTH_SECRET?.trim() ||
    bindings.SESSION_SIGNING_KEY?.trim() ||
    "replace-me-before-production"
  );
}

function resolveTrustedOrigins(
  bindings: AuthBindings,
  baseURL: string,
): string[] {
  return Array.from(
    new Set([
      ...parseCSV(bindings.ALLOWED_ORIGINS),
      bindings.BASE_URL?.trim() ?? "",
      baseURL,
      "<ProductionURL>",
      DEFAULT_BASE_URL,
    ]),
  ).filter((origin) => origin.length > 0);
}

export function createRequestAuth(
  bindings: AuthBindings,
  requestURL?: string,
) {
  const baseURL = resolveBaseURL(bindings, requestURL);
  const useSecureCookies = baseURL.startsWith("https://");

  const dbClient = new Pool({
    connectionString: bindings.HYPERDRIVE.connectionString,
  });

  const db = drizzle(dbClient, { schema });

  return betterAuth({
    baseURL,
    basePath: "/api/auth",
    secret: resolveSecret(bindings),
    database: drizzleAdapter(db, {
      provider: "pg",
      schema,
    }),
    socialProviders: {
      twitter: {
        clientId: bindings.TWITTER_CLIENT_ID,
        clientSecret: bindings.TWITTER_CLIENT_SECRET,
        disableDefaultScope: true,
        scope: ["users.read", "tweet.read", "offline.access"],
      },
    },
    trustedOrigins: resolveTrustedOrigins(bindings, baseURL),
    advanced: {
      useSecureCookies,
    },
  });
}
```

- [ ] Confirm key differences from CLI config (Step 5):
  - [ ] Reads DB from `bindings.HYPERDRIVE.connectionString` (not `LOCAL_DATABASE_URL`)
  - [ ] Sets `useSecureCookies` dynamically based on HTTPS (not hardcoded `true`)
  - [ ] Sets `disableDefaultScope: true` and explicit scopes on twitter provider
  - [ ] Builds trusted origins from `ALLOWED_ORIGINS` env var + computed values
  - [ ] Secret resolves from bindings, not `process.env`

---

## 11. Implement Auth Routes

- [ ] Create `src/routes/auth.ts`:

```ts
import { Hono, type Context } from "hono";
import { and, eq } from "drizzle-orm";
import { drizzle } from "drizzle-orm/node-postgres";
import { Client } from "pg";
import { createRequestAuth } from "../auth/runtime";
import { account } from "../db/auth-schema";
import { appUser } from "../db/schema";
import type { AppEnv } from "../types/env";

type MeResponse = {
  name: string;
  profile: string | null;
  x_id: string;
  mash_id: string | null;
};

function appendSetCookieHeaders(source: Headers, target: Headers): void {
  for (const [name, value] of source.entries()) {
    if (name.toLowerCase() === "set-cookie") {
      target.append("set-cookie", value);
    }
  }
}

export async function loginTwitter(c: Context<AppEnv>): Promise<Response> {
  const callbackURL = "/";
  const auth = createRequestAuth(c.env, c.req.url);
  const authResponse = await auth.api.signInSocial({
    body: {
      provider: "twitter",
      callbackURL,
    },
    headers: c.req.raw.headers,
    asResponse: true,
  });

  const locationHeader = authResponse.headers.get("location");

  let redirectURL = locationHeader;
  if (!redirectURL) {
    const payload = (await authResponse.clone().json().catch(() => null)) as {
      url?: string;
    } | null;
    redirectURL = payload?.url ?? null;
  }

  if (!redirectURL) {
    return authResponse;
  }

  const headers = new Headers({ location: redirectURL });
  appendSetCookieHeaders(authResponse.headers, headers);
  return new Response(null, {
    status: 302,
    headers,
  });
}

export async function getMe(c: Context<AppEnv>): Promise<Response> {
  const auth = createRequestAuth(c.env, c.req.url);
  const session = await auth.api.getSession({
    headers: c.req.raw.headers,
  });

  if (!session?.user) {
    return c.json({ error: "Unauthorized" }, 401);
  }

  const client = new Client({
    connectionString: c.env.HYPERDRIVE.connectionString,
  });
  await client.connect();

  try {
    const db = drizzle(client, { schema: { account, appUser } });

    const twitterAccount = await db.query.account.findFirst({
      where: (fields) =>
        and(
          eq(fields.userId, session.user.id),
          eq(fields.providerId, "twitter"),
        ),
      columns: {
        accountId: true,
      },
    });

    if (!twitterAccount) {
      return c.json({ error: "Twitter account not linked" }, 403);
    }

    const existingAppUser = await db.query.appUser.findFirst({
      where: (fields) => eq(fields.xId, twitterAccount.accountId),
    });

    const resolvedAppUser =
      existingAppUser ??
      (
        await db
          .insert(appUser)
          .values({
            xId: twitterAccount.accountId,
          })
          .returning({
            xId: appUser.xId,
            mashId: appUser.mashId,
          })
      )[0];

    if (!resolvedAppUser) {
      throw new Error("Failed to resolve app user.");
    }

    const user: MeResponse = {
      name: session.user.name,
      profile: session.user.image ?? null,
      x_id: resolvedAppUser.xId,
      mash_id: resolvedAppUser.mashId,
    };

    return c.json(user);
  } finally {
    await client.end();
  }
}

export async function logout(c: Context<AppEnv>): Promise<Response> {
  const auth = createRequestAuth(c.env, c.req.url);
  return auth.api.signOut({
    headers: c.req.raw.headers,
    asResponse: true,
  });
}

async function forwardBetterAuth(c: Context<AppEnv>): Promise<Response> {
  return createRequestAuth(c.env, c.req.url).handler(c.req.raw);
}

export const authRoutes = new Hono<AppEnv>()
  .get("/login/twitter", loginTwitter)
  .get("/callback/:provider", forwardBetterAuth)
  .get("/me", getMe)
  .post("/logout", logout)
  .on(["GET", "POST"], "/*", forwardBetterAuth);

export function registerAuthRoutes(app: Hono<AppEnv>) {
  return app.route("/api/auth", authRoutes);
}
```

- [ ] Confirm these 5 routes exist and are in this exact order:
  - [ ] `GET /login/twitter` -> `loginTwitter`
  - [ ] `GET /callback/:provider` -> `forwardBetterAuth`
  - [ ] `GET /me` -> `getMe`
  - [ ] `POST /logout` -> `logout`
  - [ ] `GET|POST /*` -> `forwardBetterAuth` (catch-all, must be last)
- [ ] Confirm `loginTwitter` does all three of these:
  - [ ] Calls `auth.api.signInSocial` with `provider: "twitter"` and `callbackURL: "/"`
  - [ ] Extracts redirect URL from `location` header OR JSON body `url` field
  - [ ] Copies all `set-cookie` headers from Better Auth response into the final 302
- [ ] Confirm `getMe` does all of these in order:
  - [ ] Calls `auth.api.getSession({ headers })`
  - [ ] Returns 401 if no session
  - [ ] Queries `account` where `userId = session.user.id` AND `providerId = "twitter"`
  - [ ] Returns 403 if no twitter account linked
  - [ ] Finds or creates `app_user` row by `xId = account.accountId`
  - [ ] Returns `{ name, profile, x_id, mash_id }`
- [ ] Confirm `logout` calls `auth.api.signOut({ headers, asResponse: true })`
- [ ] Confirm catch-all forwards to `createRequestAuth(...).handler(c.req.raw)`

---

## 12. Mount Auth Routes In App Entrypoint

- [ ] In your `src/index.ts`, register auth routes:

```ts
import { Hono } from "hono";
import { registerAuthRoutes } from "./routes/auth";
import type { AppEnv } from "./types/env";

const app = registerAuthRoutes(new Hono<AppEnv>());

export default app;
```

- [ ] Confirm `registerAuthRoutes` mounts auth routes at `/api/auth`

---

## 13. Configure Wrangler

- [ ] Create/update `wrangler.jsonc`:

```jsonc
{
  "name": "<ProjectName>",
  "main": "src/index.ts",
  "compatibility_flags": ["nodejs_compat"],
  "assets": {
    "binding": "ASSETS",
    "directory": "./dist"
  },
  "hyperdrive": [
    {
      "binding": "HYPERDRIVE",
      "id": "<your-hyperdrive-id>",
      "localConnectionString": "postgresql://postgres:postgres@127.0.0.1:5432/<ProjectName>"
    }
  ]
}
```

- [ ] Confirm `nodejs_compat` is in `compatibility_flags` (required for `pg` module)
- [ ] Confirm `ASSETS` binding points to `./dist`
- [ ] Confirm `HYPERDRIVE` binding has a valid `localConnectionString`

---

## 14. Register Twitter OAuth Callback URLs

- [ ] In Twitter/X developer portal, configure OAuth 2.0:
  - [ ] App type: Web App / Confidential Client
  - [ ] Add local callback URL exactly: `http://localhost:<SitePort>/api/auth/callback/twitter`
  - [ ] Add production callback URL exactly: `<ProductionURL>/api/auth/callback/twitter`
  - [ ] Add www callback URL (if needed): `https://www.tweetmash.com/api/auth/callback/twitter`
  - [ ] Scopes: `users.read tweet.read offline.access`
- [ ] Copy `TWITTER_CLIENT_ID` and `TWITTER_CLIENT_SECRET` into `.dev.vars`

---

## 15. Generate, Migrate, And Start

- [ ] Start local postgres
- [ ] Generate auth schema and migrate:

```bash
bun run auth:schema
bun run db:generate
bun run db:migrate:local
```

- [ ] Start dev server:

```bash
wrangler dev
```

- [ ] Stop if any command fails

---

## 16. Verify Login Flow (local)

### 16.1 Smoke test (curl)

- [ ] Run: `curl -i http://localhost:<SitePort>/api/auth/me`
  - [ ] Expect: `401` with `{"error":"Unauthorized"}`
- [ ] Run: `curl -i http://localhost:<SitePort>/api/auth/login/twitter`
  - [ ] Expect: `302` with `Location` header pointing to Twitter
  - [ ] Expect: `Set-Cookie` headers present in response

### 16.2 Browser test

- [ ] Open `http://localhost:<SitePort>`
- [ ] Click `Sign in with X`
- [ ] Confirm redirect chain:
  - [ ] `GET /api/auth/login/twitter` -> 302 to Twitter
  - [ ] Twitter consent page loads
  - [ ] Twitter redirects to `/api/auth/callback/twitter?code=...&state=...`
  - [ ] Better Auth redirects to `/` (because `callbackURL` is `/`)
- [ ] Confirm UI shows signed-in state (name, profile, x_id, mash_id)
- [ ] Click `Log out`
- [ ] Confirm UI shows signed-out state
- [ ] Confirm `GET /api/auth/me` now returns `401`

### 16.3 Database verification

- [ ] Check `user` table has a row for the logged-in user
- [ ] Check `account` table has a row with `provider_id = 'twitter'` and valid `account_id`
- [ ] Check `session` table has a row with valid `token` and `expires_at`
- [ ] Check `app_user` table has a row with `x_id` matching `account.account_id`

---

## 17. Route Contract Reference

| Route | Method | Happy Path | Failure |
|---|---|---|---|
| `/api/auth/login/twitter` | GET | `302` + `Location` + `Set-Cookie` | Better Auth passthrough if no redirect URL |
| `/api/auth/callback/twitter` | GET | Better Auth redirect to `/` | Better Auth error (state/PKCE mismatch) |
| `/api/auth/me` | GET | `200` `{name, profile, x_id, mash_id}` | `401` (no session) or `403` (no twitter link) |
| `/api/auth/logout` | POST | Better Auth `2xx` | Better Auth managed |
| `/api/auth/*` | GET/POST | Better Auth managed | Better Auth managed |

---

## 18. Session And Security Checklist

- [ ] Confirm these Better Auth tables exist in `src/db/auth-schema.ts`:
  - [ ] `user`
  - [ ] `account`
  - [ ] `session`
  - [ ] `verification`
- [ ] Confirm session check in `getMe` uses `auth.api.getSession({ headers: c.req.raw.headers })`
- [ ] Confirm OAuth state/PKCE is handled by Better Auth callback (no manual state management)
- [ ] Confirm `loginTwitter` preserves `Set-Cookie` headers via `appendSetCookieHeaders`
- [ ] Confirm `useSecureCookies` is dynamic in runtime (true only when baseURL is HTTPS)
- [ ] Confirm secret resolution order: `BETTER_AUTH_SECRET` -> `SESSION_SIGNING_KEY` -> fallback
- [ ] Confirm trusted origins are built from: `ALLOWED_ORIGINS` + `BASE_URL` + resolved baseURL + hardcoded defaults
- [ ] For production:
  - [ ] Set `BETTER_AUTH_SECRET` to a strong random value (never use fallback)
  - [ ] Set `BASE_URL` to exact production origin (e.g., `<ProductionURL>`)
  - [ ] Ensure HTTPS so secure cookies are enabled
  - [ ] Keep `ALLOWED_ORIGINS` restricted to real origins only
  - [ ] Keep callback URLs exact-match in Twitter app settings

---

## 19. Token Access (for calling Twitter API)

The `account` table stores provider tokens. Current `getMe` only reads `accountId`.

- [ ] To access tokens, extend the account query columns:

```ts
const twitterAccountWithTokens = await db.query.account.findFirst({
  where: (fields) =>
    and(eq(fields.userId, session.user.id), eq(fields.providerId, "twitter")),
  columns: {
    accountId: true,
    accessToken: true,
    refreshToken: true,
    accessTokenExpiresAt: true,
    refreshTokenExpiresAt: true,
    scope: true,
  },
});
```

- [ ] Available token columns in `account` table:
  - [ ] `accessToken` (text, nullable)
  - [ ] `refreshToken` (text, nullable)
  - [ ] `idToken` (text, nullable)
  - [ ] `accessTokenExpiresAt` (timestamp, nullable)
  - [ ] `refreshTokenExpiresAt` (timestamp, nullable)
  - [ ] `scope` (text, nullable)
- [ ] Note: refresh-token rotation is NOT implemented in current app code

---

## 20. Production Deploy Checklist (GO_LIVE.md)

- [ ] Set Wrangler secrets:
  - [ ] `wrangler secret put TWITTER_CLIENT_ID`
  - [ ] `wrangler secret put TWITTER_CLIENT_SECRET`
  - [ ] `wrangler secret put BETTER_AUTH_SECRET`
  - [ ] `wrangler secret put SESSION_SIGNING_KEY` (optional, fallback)
- [ ] Set production vars:
  - [ ] `BASE_URL=<ProductionURL>`
  - [ ] `ALLOWED_ORIGINS=<ProductionURL>,https://www.tweetmash.com`
- [ ] Register production callback URLs in Twitter app:
 - [ ] `<ProductionURL>/api/auth/callback/twitter`
  - [ ] `https://www.tweetmash.com/api/auth/callback/twitter` (if using www)
- [ ] Run remote migration:

```bash
bun run auth:schema
bun run db:generate
bun run db:migrate:remote
```

- [ ] Deploy:

```bash
wrangler deploy
```

- [ ] Post-deploy verification:
  - [ ] Open production URL, click Sign in with X
  - [ ] Confirm redirect to Twitter, back to callback, then to `/`
  - [ ] Confirm `/api/auth/me` returns signed-in identity
  - [ ] Confirm logout works

---

## 21. Failure Triage

- [ ] `GET /api/auth/login/twitter` is not `302`:
  - [ ] Check `TWITTER_CLIENT_ID` and `TWITTER_CLIENT_SECRET` are set
  - [ ] Check `BASE_URL` is set and matches browser origin
  - [ ] Check trusted origins include the request origin
- [ ] Callback fails (state/PKCE error):
  - [ ] Verify login response includes `Set-Cookie` headers (OAuth state + PKCE verifier cookies)
  - [ ] Verify browser preserves cookies through redirect chain
  - [ ] Verify callback URL in Twitter app settings matches exactly (scheme, host, port, path)
- [ ] `/api/auth/me` returns `401` after successful login:
  - [ ] Check session cookie is present in browser
  - [ ] Check cookie domain and protocol match (secure cookies require HTTPS)
  - [ ] Check `BETTER_AUTH_SECRET` is the same value that signed the session
- [ ] `/api/auth/me` returns `403`:
  - [ ] Session exists but no row in `account` with `provider_id = 'twitter'` for that user
- [ ] Origin rejection errors:
  - [ ] Verify `ALLOWED_ORIGINS` and `BASE_URL` match the actual request origin exactly
