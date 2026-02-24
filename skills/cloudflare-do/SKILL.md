## Cloudflare Durable Objects

Use this skill when implementing Durable Objects in a workflow-heavy app.

The guidance below is principle-first (why), with concrete patterns (how), derived from `durable-user.ts` and `durable-post.ts`.

### Principles

- Single writer per aggregate: one DO instance owns one aggregate (`user`, `post`) and is the only place that mutates it.
- Fast restart safety: all in-memory state is reconstructable from persisted storage.
- Idempotency over retries: duplicate calls should converge to one final state.
- Local commit before distributed side effects: persist local DO state before triggering cross-object workflows.
- Explicit invariants: return typed failure codes for business-rule failures; throw only for impossible/invalid runtime state.

### State Model

- Keep a small in-memory cache (`this.user`, `this.post`) plus durable backing (`ctx.storage.kv`).
- Use Durable SQLite for relation/index style data (followers, feed refs, mutuals, post refs).
- Use KV for singleton snapshots and small maps (`user`, `post`, `theme`, top-friends).
- In constructor, use `ctx.blockConcurrencyWhile(...)` to run boot-time migration and repair logic.

### Mutation Rules

- Every successful mutation writes durable state and updates memory in the same method.
- Make writes idempotent by default (`onConflictDoNothing`, early return if already liked/following).
- Return narrow result DTOs rather than exposing internal storage shape.
- Keep low-level mutators separate from workflow-triggering methods.

### Repair and Compatibility

- On load, normalize persisted data from older shapes (missing IDs, malformed arrays, duplicates).
- Do repairs at startup, store the repaired snapshot back, and continue with a clean canonical shape.

### Workflow Boundary

- DO methods own local correctness.
- Workflows own multi-DO orchestration and compensation.
- Provide explicit rollback primitives on DOs (`restorePost`, `removePost`, `deletePost`, `removeFromFeed`).

### API Design Heuristics

- `getByName(id)` as canonical identity mapping.
- Static helper methods for lookup/bootstrap (`getUser`, `createPost`) keep handlers thin.
- Cursor pagination must use stable ordering (`timestamp` then ID) and enforce max limits.

### Binding and Typing Discipline

- Keep binding names aligned across wrangler config, `Env` types, and runtime usage.
- Prefer ambient `Env`; avoid drifting local env aliases unless intentionally narrowed.

### Build Checklist

1. Export DO from worker entry.
2. Add wrangler durable binding plus migration tag.
3. Ensure `Env` typing includes new bindings.
4. Implement constructor boot gate (`blockConcurrencyWhile`) with migration/repair.
5. Split high-level workflow triggers from low-level mutators.
6. Add rollback/dev-reset methods needed by workflows and tests.

### Anti-Patterns To Avoid

- Triggering workflows before local DO state is durably written.
- Encoding orchestration logic directly in HTTP handlers instead of DO/workflow boundaries.
- Returning raw storage payloads from command methods instead of stable result DTOs.
- Mixing relationship/index data into KV snapshots when it should live in Durable SQLite tables.
- Skipping startup repair/normalization for legacy persisted shapes.
- Relying on non-idempotent mutations for operations that may be retried.
