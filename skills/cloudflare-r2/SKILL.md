---
description: Cloudflare R2 bucket binding usage rules
globs: src/**/*.ts
alwaysApply: false
---

## Cloudflare R2

### Rules
1. Declare bucket bindings in `wrangler.jsonc` under `r2_buckets` with both `binding` and `bucket_name`; keep this file as the source of truth.
2. Access buckets via `env.<BINDING>` inside each Worker handler, and keep the binding name identical in both config and code.
3. For object operations, handle `get()` misses (`null`) explicitly, set `httpMetadata` (for example `contentType`) on `put()`, and return clear HTTP status codes.

### Example 1: `wrangler.jsonc`
```jsonc
{
  "$schema": "node_modules/wrangler/config-schema.json",
  "name": "my-worker",
  "main": "src/index.ts",
  "compatibility_date": "2026-02-24",
  "r2_buckets": [
    {
      "binding": "FILES",
      "bucket_name": "my-files"
    }
  ]
}
```

### Example 2: Minimal `put()`
```ts
await env.FILES.put("hello.txt", "hello from r2", {
  httpMetadata: { contentType: "text/plain; charset=utf-8" },
});
```

### Example 3: Minimal `get()`
```ts
const object = await env.FILES.get("hello.txt");
if (!object) return new Response("Not found", { status: 404 });

const headers = new Headers();
object.writeHttpMetadata(headers);
headers.set("etag", object.httpEtag);

return new Response(object.body, { headers });
```

Note: if you mean "htmlEtags", the R2 field is actually `httpEtag`; use `httpEtag` for response `etag` headers because it is already correctly quoted.
