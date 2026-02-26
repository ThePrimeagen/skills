## better-auth-social-login

### Goal
Implement Better Auth social login (OAuth/OIDC) with Drizzle and drizzle-kit using a secure, migration-safe workflow, including cookie/JWT hardening and exact auth table expectations.

### Rules
1. Configure providers under `socialProviders` with valid OAuth credentials and provider redirect URIs that exactly match your Better Auth callback route.
2. Set a correct `baseURL` (or `BETTER_AUTH_URL`) for each environment; callback mismatch is the most common social login failure.
3. For Drizzle projects, do not use `@better-auth/cli migrate`; use `@better-auth/cli generate` then `drizzle-kit generate` and `drizzle-kit migrate`.
4. Always run Better Auth schema generation with `--output` to a dedicated auth schema file.
5. Keep Better Auth config in a CLI-discoverable file path or pass `--config` explicitly.
6. Keep auth cookies `httpOnly`; keep `sameSite` at `lax` unless you have a specific cross-site requirement.
7. Keep CSRF and origin protections enabled (`disableCSRFCheck: false`, `disableOriginCheck: false` by default).
8. Use `trustedOrigins` as an explicit allowlist for browser origins that are allowed to use auth endpoints.
9. For shared-subdomain sessions, enable `crossSubDomainCookies` only when needed and scope `domain` as narrowly as possible.
10. If using JWT plugin, treat JWT as service token support, not a replacement for session-based browser auth.
11. Rotate provider secrets and revoke compromised refresh/access tokens through provider consoles.
12. Commit config, generated auth schema, and Drizzle migration artifacts together.

### End-to-End Social Flow
1. User initiates sign-in via `authClient.signIn.social({ provider })`.
2. Better Auth redirects to provider authorize endpoint with callback URL, state, and PKCE values.
3. Provider redirects to `/api/auth/callback/{provider}`.
4. Better Auth validates OAuth state/PKCE and origin safeguards.
5. Better Auth resolves/creates `user` and `account` records.
6. Better Auth creates/updates `session`, sets signed cookie(s), and returns control to app callback path.

### Canonical Server Config (Drizzle + Social + JWT)
```ts
import { betterAuth } from "better-auth";
import { drizzleAdapter } from "better-auth/adapters/drizzle";
import { jwt } from "better-auth/plugins";
import { db, schema } from "../db";

export const auth = betterAuth({
  baseURL: process.env.BETTER_AUTH_URL,
  database: drizzleAdapter(db, {
    provider: "pg",
    schema,
  }),
  socialProviders: {
    github: {
      clientId: process.env.GITHUB_CLIENT_ID as string,
      clientSecret: process.env.GITHUB_CLIENT_SECRET as string,
    },
    google: {
      clientId: process.env.GOOGLE_CLIENT_ID as string,
      clientSecret: process.env.GOOGLE_CLIENT_SECRET as string,
      accessType: "offline",
      prompt: "select_account consent",
    },
  },
  trustedOrigins: [
    "http://localhost:3000",
    "https://app.example.com",
  ],
  advanced: {
    useSecureCookies: true,
    // Keep origin and CSRF checks enabled by default.
    // disableOriginCheck: false,
    // disableCSRFCheck: false,
  },
  plugins: [
    jwt(),
  ],
});
```

### Cookie and Security Baseline
- Cookies are signed by Better Auth secret, `httpOnly`, and `sameSite=lax` by default.
- `secure` is enabled in production mode; set `advanced.useSecureCookies: true` to force secure cookies.
- Use `trustedOrigins` to allow only known origins and block CSRF/open redirect vectors.
- Keep OAuth callback URLs exact by scheme/host/port/path.
- Do not disable `disableCSRFCheck` or `disableOriginCheck` except in controlled debugging.
- For Safari cross-domain setups, use reverse proxy or shared parent domain with careful cookie domain scoping.

### JWT Plugin Guidance
- Add `jwt()` server plugin and `jwtClient()` client plugin when downstream services need bearer-style JWT.
- Use `/api/auth/token` to retrieve JWT and `/api/auth/jwks` for verifier keys.
- Verify JWT in services using JWKS (`kid`-aware cache strategy).
- Default issuer/audience are based on `baseURL`; set explicit values when required by consumers.
- JWT plugin adds `jwks` table; include it in schema generation and migrations.

### Exact Table Formation (Core + JWT)

Core Better Auth tables:

1. `user`
   - `id` (pk)
   - `name`
   - `email` (unique)
   - `emailVerified`
   - `image` (nullable)
   - `createdAt`
   - `updatedAt`
2. `session`
   - `id` (pk)
   - `userId` (fk -> user.id)
   - `token` (unique)
   - `expiresAt`
   - `ipAddress` (nullable)
   - `userAgent` (nullable)
   - `createdAt`
   - `updatedAt`
3. `account`
   - `id` (pk)
   - `userId` (fk -> user.id)
   - `accountId` (provider account id)
   - `providerId` (google/github/etc)
   - `accessToken` (nullable)
   - `refreshToken` (nullable)
   - `idToken` (nullable)
   - `accessTokenExpiresAt` (nullable)
   - `refreshTokenExpiresAt` (nullable)
   - `scope` (nullable)
   - `password` (nullable; credential auth)
   - `createdAt`
   - `updatedAt`
4. `verification`
   - `id` (pk)
   - `identifier`
   - `value`
   - `expiresAt`
   - `createdAt`
   - `updatedAt`

JWT plugin table:

5. `jwks`
   - `id` (pk)
   - `publicKey`
   - `privateKey`
   - `createdAt`
   - `expiresAt` (nullable)

Notes:
- Better Auth also manages OAuth state/PKCE protections as part of its OAuth flow; keep generated schema current whenever auth/plugins change.
- Names can be customized (`modelName`, `fields`, plugin `schema` mapping), but mappings must remain consistent across auth config, Drizzle schema, and migrations.

### Drizzle Schema and Config Pattern
```ts
// drizzle.config.ts
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

### Required Scripts
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

### Migration Workflow (Auth + Social Changes)
1. Change social provider config, auth options, or plugins.
2. Run `npm run auth:schema`.
3. Run `npm run db:generate`.
4. Review SQL migration output in `drizzle/*`.
5. Run `npm run db:migrate`.
6. Validate sign-in callback and session persistence in browser.
7. Commit auth config + `auth-schema.ts` + `drizzle/*` artifacts together.

### Provider Setup Checklist
- GitHub redirect URI: `http://localhost:3000/api/auth/callback/github` (dev) and production equivalent.
- Google redirect URI: `http://localhost:3000/api/auth/callback/google` (dev) and production equivalent.
- Ensure `BETTER_AUTH_URL`/`baseURL` matches the deployed domain used by users.
- For GitHub Apps, grant email read permission or expect email lookup failures.
- For Google refresh tokens, prefer `accessType: "offline"` with consent prompt.

### Common Failure Patterns
- `redirect_uri_mismatch`
  - Fix: align provider console redirect URI with Better Auth callback and `baseURL`.
- Social callback succeeds but no session in browser
  - Fix: check cookie `secure`/domain/SameSite settings and frontend `credentials: "include"` usage.
- Safari only: session appears lost across domains
  - Fix: proxy auth route under frontend domain or use shared parent domain cookie strategy.
- JWT verifies locally but fails in service
  - Fix: verify issuer/audience and refresh JWKS on unknown `kid`.
- Drizzle migration missing plugin tables
  - Fix: regenerate auth schema before Drizzle generation and ensure `auth-schema.ts` is listed in `drizzle.config.ts`.

### Reference Docs
- Better Auth OAuth: https://www.better-auth.com/docs/concepts/oauth
- Better Auth cookies: https://www.better-auth.com/docs/concepts/cookies
- Better Auth security: https://www.better-auth.com/docs/reference/security
- Better Auth database concepts: https://www.better-auth.com/docs/concepts/database
- Better Auth JWT plugin: https://www.better-auth.com/docs/plugins/jwt
- Better Auth GitHub provider: https://www.better-auth.com/docs/authentication/github
- Better Auth Google provider: https://www.better-auth.com/docs/authentication/google
- Better Auth Drizzle adapter: https://www.better-auth.com/docs/adapters/drizzle
- Drizzle Kit overview: https://orm.drizzle.team/docs/kit-overview
- drizzle-kit generate: https://orm.drizzle.team/docs/drizzle-kit-generate
- drizzle-kit migrate: https://orm.drizzle.team/docs/drizzle-kit-migrate
