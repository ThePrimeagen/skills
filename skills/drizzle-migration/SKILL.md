## drizzle-migration

### Rules
1. Set up Drizzle scripts up front and use them as the only migration entrypoints: `bun run db:generate`, `bun run db:migrate:local`, and `bun run db:migrate:remote`.
2. Migration generation and schema syncing must always be Drizzle-driven: update schema files, generate with Drizzle, and apply with Drizzle; never hand-roll SQL migration files and never run out-of-band schema changes.
3. Follow a safe rollout flow: apply `bun run db:migrate:local` first, validate locally, commit schema + generated migrations together, then apply `bun run db:migrate:remote` for Cloudflare-connected environments.
