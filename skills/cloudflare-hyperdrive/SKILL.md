---
description: Cloudflare Hyperdrive connection usage rules
globs: src/**/*.ts
alwaysApply: false
---

## Cloudflare Hyperdrive

### Rules
1. Create a new database client/connection inside each Worker invocation (`fetch`, `queue`, `scheduled`, or Durable Object method); never create global clients or driver pools (`new Pool()`, `createPool()`). Hyperdrive manages the shared origin pool.
2. Use the Hyperdrive binding directly: Postgres uses `env.HYPERDRIVE.connectionString`
3. Let Hyperdrive manage connection lifecycle: keep transactions short, avoid long-lived pinned connections, and do not call `client.end()` / `connection.end()` for routine per-request cleanup.

### Example 1: PostgreSQL (Hyperdrive handles pooling)
```ts
import { Client } from "pg";

export default {
  async fetch(_request, env): Promise<Response> {
    const client = new Client({
      connectionString: env.HYPERDRIVE.connectionString,
    });

    await client.connect();
    const result = await client.query("SELECT NOW() AS now");
    return Response.json(result.rows[0]);
  },
} satisfies ExportedHandler<{ HYPERDRIVE: Hyperdrive }>;
```
