## hyperdrive-drizzle

### Rules
1. Create the Postgres/Drizzle client inside each Worker invocation (`fetch`, `queue`, `scheduled`, or Durable Object method) using `env.HYPERDRIVE.connectionString`; never create global pools or long-lived shared clients.
2. Migrations must ALWAYS run through Drizzle (`drizzle-kit generate`, `drizzle-kit migrate`, or project `bun run db:*` scripts); never hand-roll migration SQL or apply schema changes manually.
3. Keep schema and migrations in lockstep: change `schema.ts`, generate a migration, and run the migration before deploying code that depends on it.

### Example 1: Drizzle in a Worker (Hyperdrive connection)
```ts
import { Client } from "pg";
import { drizzle } from "drizzle-orm/node-postgres";
import { users } from "./db/schema";

export default {
  async fetch(_request, env): Promise<Response> {
    const client = new Client({
      connectionString: env.HYPERDRIVE.connectionString,
    });

    await client.connect();
    const db = drizzle(client);

    const result = await db.select().from(users).limit(1);
    return Response.json(result[0] ?? null);
  },
} satisfies ExportedHandler<{ HYPERDRIVE: Hyperdrive }>;
```

### Example 2: Migration scripts (Drizzle-only workflow)
```json
{
  "scripts": {
    "db:generate": "drizzle-kit generate",
    "db:migrate:local": "drizzle-kit migrate --config drizzle.local.config.ts",
    "db:migrate:remote": "drizzle-kit migrate --config drizzle.remote.config.ts"
  }
}
```
