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

// Top-level Env type for the main worker
type Env = {
  R2: R2Bucket;
  // ... other bindings
};

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

### Added cloudflare binding
If a binding requires an id from cloudflare include the comment for how
to create the cloudflare item via wrangler and how to get the id.

always include all commands that are required.

### Functions That Use Cloudflare
Every function that you create that uses cloudflare should have a first
parameter called `env` of interface `<Object>Env` (usually named after file, or object type)

```typescript
// in file matchmaking.ts
export async function get_game(env: MatchMakingEnv): Promise<...> {
  ...
}
```

This pattern of env applies to all cloudflare bindings such as D1, R2, Containers, Workers, etc etc

### Example using Containers and Durable Objects
Creating a container requires getContainer and an Env object that is defined
with the container name

```typescript
interface Env {
  <container_class_name_all_caps>: DurableObjectNamespace<<ContainerClassName>>;
  // ... other env requirements
}
```

```jsonc
	"containers": [
		{
			"class_name": "<ContainerClassName>",
            // ... other properites here
		}
	],
```

Example:
```jsonc
"class_name": "GameContainer",
```

```typescript
interface Env {
  GAME: DurableObjectNamespace<GameContainer>,
}
```

### Env object
* always search for the main `Env` (usually defined in the main worker file). Use the same interface field names/values when creating sub env
* When creating env interfaces, never import within it `import("...")`, always
import above the interface if you are creating an interface, such as the following:

```typescript
import { DOContainer } from "./path/to/container";
interface MatchMakingEnv = {
    DO_NAME: DurableObjectNamespace<DOContainer>
}
```

