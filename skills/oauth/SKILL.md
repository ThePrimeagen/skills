# Twitch OAuth 2.0 (Working Setup for This Project)

This document shows a complete Twitch OAuth Authorization Code flow for this app, including the callback at:

- `api/auth/callback/twitch`

It also highlights the most common failure in local dev:

- `BASE_URL` does not match your actual running origin.

## 1) Flow Overview

1. User clicks Login in the frontend.
2. Backend redirects to Twitch authorize endpoint with:
   - `client_id`
   - `redirect_uri`
   - `response_type=code`
   - `scope`
   - `state`
3. Twitch redirects back to `/api/auth/callback/twitch?code=...&state=...`.
4. Backend validates `state` against the state cookie.
5. Backend exchanges `code` for access token.
6. Backend fetches Twitch user profile.
7. Backend signs JWT and sets `session_token` cookie.
8. Frontend calls `/api/auth/me` and renders `id` + `name`.

## 2) Required URLs (Exact Match Rules)

OAuth redirect URIs must match exactly by:

- scheme (`http` vs `https`)
- host
- port
- path

For local dev in this project, if you run `bun run dev` on port `46920`, use:

- `http://localhost:46920/api/auth/callback/twitch`

For production, use your production callback URL.

## 3) Environment Variables

Set these secrets/bindings for the worker:

- `TWITCH_CLIENT_ID`
- `TWITCH_CLIENT_SECRET`
- `SESSION_SIGNING_KEY`
- `BASE_URL`

Optional:

- `TWITCH_REDIRECT_ORIGIN` (only if you intentionally override derived origin)

### Important: BASE_URL Pitfall

`BASE_URL` must equal the browser origin that starts login.

Examples:

- Local: `BASE_URL=http://localhost:46920`
- Prod: `BASE_URL=https://your-domain.com`

If `BASE_URL` is set to the wrong scheme/host/port, Twitch login can fail even when the rest of the OAuth URL looks correct.

## 4) Backend Route Shape

Mount auth routes under:

- `/api/auth`

Resulting endpoints:

- `GET /api/auth/login/twitch`
- `GET /api/auth/callback/twitch`
- `GET /api/auth/me`
- `POST /api/auth/logout`

## 5) Login Endpoint Example

```ts
auth.get("/login/twitch", async (c) => {
  const state = crypto.randomUUID();
  setCookie(c, "oauth_state", state, {
    httpOnly: true,
    sameSite: "Lax",
    secure: c.req.url.startsWith("https://"),
    path: "/api/auth",
    maxAge: 60 * 10,
  });

  const redirectUri = `${c.env.BASE_URL}/api/auth/callback/twitch`;
  const authorizationUrl = new URL("https://id.twitch.tv/oauth2/authorize");
  authorizationUrl.searchParams.set("client_id", c.env.TWITCH_CLIENT_ID);
  authorizationUrl.searchParams.set("redirect_uri", redirectUri);
  authorizationUrl.searchParams.set("response_type", "code");
  authorizationUrl.searchParams.set("scope", "user:read:email");
  authorizationUrl.searchParams.set("state", state);

  return c.redirect(authorizationUrl.toString(), 302);
});
```

## 6) Callback Endpoint Example

```ts
auth.get("/callback/twitch", async (c) => {
  const callbackState = c.req.query("state");
  const code = c.req.query("code");
  const stateCookie = getCookie(c, "oauth_state");
  deleteCookie(c, "oauth_state", { path: "/api/auth" });

  if (
    !callbackState ||
    !code ||
    !stateCookie ||
    callbackState !== stateCookie
  ) {
    deleteCookie(c, "session_token", { path: "/" });
    return c.redirect("/", 302);
  }

  const redirectUri = `${c.env.BASE_URL}/api/auth/callback/twitch`;
  const tokenRes = await fetch("https://id.twitch.tv/oauth2/token", {
    method: "POST",
    headers: { "Content-Type": "application/x-www-form-urlencoded" },
    body: new URLSearchParams({
      client_id: c.env.TWITCH_CLIENT_ID,
      client_secret: c.env.TWITCH_CLIENT_SECRET,
      code,
      grant_type: "authorization_code",
      redirect_uri: redirectUri,
    }),
  });

  if (!tokenRes.ok) return c.redirect("/", 302);
  const tokenJson = (await tokenRes.json()) as { access_token?: string };
  if (!tokenJson.access_token) return c.redirect("/", 302);

  const userRes = await fetch("https://api.twitch.tv/helix/users", {
    headers: {
      Authorization: `Bearer ${tokenJson.access_token}`,
      "Client-Id": c.env.TWITCH_CLIENT_ID,
    },
  });

  if (!userRes.ok) return c.redirect("/", 302);
  const userJson = (await userRes.json()) as {
    data?: Array<{ id: string; login: string; display_name: string }>;
  };
  const twitchUser = userJson.data?.[0];
  if (!twitchUser) return c.redirect("/", 302);

  const now = Math.floor(Date.now() / 1000);
  const jwt = await sign(
    {
      sub: twitchUser.id,
      name: twitchUser.display_name || twitchUser.login,
      iat: now,
      exp: now + 60 * 60 * 24 * 7,
    },
    c.env.SESSION_SIGNING_KEY,
  );

  setCookie(c, "session_token", jwt, {
    httpOnly: true,
    sameSite: "Lax",
    secure: c.req.url.startsWith("https://"),
    path: "/",
    maxAge: 60 * 60 * 24 * 7,
  });

  return c.redirect("/", 302);
});
```

## 7) Session Validation Endpoint Example

```ts
auth.get("/me", async (c) => {
  const token = getCookie(c, "session_token");
  if (!token) return c.json({ ok: false }, 401);

  try {
    const claims = (await verify(
      token,
      c.env.SESSION_SIGNING_KEY,
      "HS256",
    )) as {
      sub: string;
      name: string;
    };
    return c.json({ id: claims.sub, name: claims.name });
  } catch {
    deleteCookie(c, "session_token", { path: "/" });
    return c.json({ ok: false }, 401);
  }
});
```

## 8) Frontend Integration

- Login button: navigate to `/api/auth/login/twitch`
- App load: call `/api/auth/me`
- If `401`: show login prompt
- If `200`: render `id` and `name`
- Logout: call `POST /api/auth/logout`, then refetch `/api/auth/me`

## 9) Local Debug Checklist

If login fails:

1. Confirm app is running on expected origin (`http://localhost:46920`).
2. Confirm Twitch app redirect URL exactly matches:
   - `http://localhost:46920/api/auth/callback/twitch`
3. Confirm `BASE_URL` equals the same origin.
4. Confirm callback path is exactly `/api/auth/callback/twitch`.
5. Confirm `TWITCH_CLIENT_ID` and `TWITCH_CLIENT_SECRET` are valid.
6. Confirm `oauth_state` cookie is set on login and validated on callback.

## 10) Security Notes

- Keep `SESSION_SIGNING_KEY` secret and rotate when needed.
- Keep session cookie `httpOnly`.
- Keep `sameSite: "Lax"` for standard OAuth redirect behavior.
- Use `secure: true` in HTTPS environments.
