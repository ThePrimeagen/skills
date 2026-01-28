## Worker
* always prefer to use hono

### Assets
* always use vite to build to `dist`
* assets should always be served out of `dist`
* the catch all route should have assets as the last thing it serves

## Using Cloudflare
All non hono routes or other cloudflare specific functions in typescript should use the following pattern

/path/to/file/example-using-r2.ts
---------------------

```typescript
// This file is an example file of some r2 related functions
import { R2Bucket } from "@cloudflare/workers-types";

// There should always be an `Env` type defined in the file that
// uses cloudflare bindings
type MyR2ExampleEnv = {
    // the names here should be a 1:1 naming to what is in the main worker Env object
    R2: R2Bucket
}

// all exported function's first argument should be the env if they interact
// with r2 (or any other cloudflare binding)
export async function some_operation(env: MyR2ExampleEnv): Promise<...> {
}
```

/path/to/file/main-worker.ts
---------------------
```typescript
import { Hono } from 'hono';
import { serveStatic } from 'hono/cloudflare-workers';
import { R2Bucket } from '@cloudflare/workers-types';
import type { D1Database } from '@cloudflare/workers-types';
import type { KVNamespace } from '@cloudflare/workers-types';
import { some_operation } from "./example-using-r2"

const app = new Hono<{ Bindings: Env }>();
app.get('/api/example', async (c) => {
  const env = c.env;
  await some_operation(env)
  // other operations or business logic here
  return c.json({ message: 'Hello' });
});

// Serve static assets from dist (catch-all should be last)
app.get('*', serveStatic({ root: './dist' }));

export default app;
```

/path/to/file/wrangler.jsonc
---------------------
```jsonc
{
  // ... other wrangler configuration
  "r2_buckets": [
    {
      "binding": "R2",
      "bucket_name": "my-bucket",
      "preview_bucket_name": "my-bucket-preview"
    }
  ],
  // ... other wrangler configuration
}
```

#### Added cloudflare binding
Here is an example of creating a d1 entry for the user.  Notice i include the command AND how to get the ID

```jsonc
    // wrangler d1 create <db-name>
  "d1_databases": [
    {
      "binding": "<DB_NAME>",
      "database_name": "<db-name>",

      // when creating the db, id will be printed out
      // if you missed it: wrangler d1 list
      "database_id": "<id>"
    }
  ]
}
```

📣 Remember to rerun 'wrangler types' after you change your wrangler.jsonc file.

### Functions That Use Cloudflare
Every function that you create that uses cloudflare bindings should have a first
parameter called `env` of interface `Env` defined typically in ./worker-congifuration.d.ts

```typescript
// in file matchmaking.ts
export async function get_game(env: Env): Promise<...> {
  ...
}
```

This pattern of env applies to all cloudflare bindings such as D1, R2, Containers, Workers, etc etc

### Env object
* `Env` is always defined in worker-congifuration.d.ts
* never create one yourself, always use the ambient definition
