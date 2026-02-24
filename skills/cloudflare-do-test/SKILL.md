## Cloudflare Durable Object Testing

Use this skill when writing or refactoring tests for Cloudflare Durable Objects and their workflows.

This guidance is pulled from `durable-user.test.ts`, `durable-post.test.ts`, and `test/helpers/waitForWorkflow.ts`.

### Test Design Principles

- Test externally observable behavior, not storage internals.
- Make async workflow completion explicit; never assert distributed side effects before workflow status is terminal.
- Treat workflow orchestration as eventually consistent and retry-prone.
- Keep tests deterministic by running in a single worker when DO and workflow state can interact.
- Prefer idempotency assertions for all commands that can be retried in production.

### Harness Configuration

- Use `@cloudflare/vitest-pool-workers` and `cloudflare:test` bindings.
- Keep `fileParallelism: false`, `maxWorkers: 1`, and `minWorkers: 1` for stateful DO suites unless isolation is guaranteed.
- Use a dedicated wrangler test config (`wrangler.test.jsonc`) with test-only vars and local bindings.
- Set `ENVIRONMENT: "test"` so production-only side effects can be disabled in runtime code.

### Reusable Helpers

- Create fixture factories (`createTestUser`) that return typed stubs and hide repeated setup.
- Centralize workflow polling in one helper (`waitForWorkflow`) with:
  - timeout budgets lower than Vitest global timeout,
  - transient error retry handling,
  - cleanup/dispose retry handling,
  - fatal status detection (`errored`, `terminated`, `unknown`).
- Add workflow-specific wrappers (`waitForFollowWorkflow`, `waitForUnfollowWorkflow`) for readability.

### What to Assert for DOs

- Creation: object exists and required fields are persisted.
- Idempotency: repeated like/follow/unfollow calls converge without duplicate state.
- Invariants: cooldowns, bounds, and validation errors return stable outputs.
- Fanout effects: author indexes, follower feeds, and absence/presence rules are all verified.
- Compensation paths: delete/rollback flows remove all references across related objects.

### Style Rules for This Repo Pattern

- Use behavior-driven test names (`should ...`).
- Use helper functions for repetitive setup and workflow waiting.
- Prefer one behavior per test; avoid broad scenario tests unless they validate orchestration boundaries.
- Keep comments minimal and only where sequencing would be unclear.

### Durable Object Test Checklist

1. Arrange fixtures with helper builders (`createTestUser`, optional `create_post`).
2. Act via public DO methods or static constructors.
3. Await workflow completion through helper before cross-object assertions.
4. Assert both positive and idempotent repeat-call behavior.
5. Verify terminal state across all affected objects (author, follower, target post).
6. Keep helper timeouts and global Vitest timeouts coherent.

### Anti-Patterns To Avoid

- Asserting distributed side effects before awaiting workflow terminal status.
- Duplicating polling/retry logic across tests instead of using a shared helper.
- Depending on test execution order or leaked state from prior tests.
- Running stateful DO suites with parallel workers without strict isolation.
- Verifying private storage details when public behavior assertions are sufficient.
- Using generous timeouts as a substitute for deterministic setup and proper retries.
