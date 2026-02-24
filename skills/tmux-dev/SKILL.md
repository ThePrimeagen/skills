---
description: Tmux-based dev.sh script conventions
globs: dev.sh
alwaysApply: false
---

## Tmux Dev Scripts

### Rules

1. Assign each long-running process (database, dev server, etc.) a dedicated high-numbered window (e.g. 8, 9) and kill any stale instance with `tmux kill-window -t "$SESSION:$WIN" 2>/dev/null || true` before creating a new one.
 - the highest number should be 9, so if there are 3 processes, 7, 8, and 9 should be the windows
2. When a process must be healthy before the next step (e.g. Postgres before migrations), poll in a bounded loop (`for i in $(seq 1 N)`) with a `sleep` between attempts and `exit 1` on timeout — never assume instant readiness.
3. End every `tmux new-window` command string with `; exec bash` so the window stays open for inspection if the process exits.

### Example 1: Full dev.sh with database + worker

```bash
#!/usr/bin/env bash
set -euo pipefail

DIR="$(cd "$(dirname "$0")" && pwd)"
SESSION="${TMUX_SESSION:-$(tmux display-message -p '#S')}"
PG_WIN="8"
DEV_WIN="9"
LOCAL_DB="postgresql://postgres:postgres@localhost:54321/myapp"

for win in "$PG_WIN" "$DEV_WIN"; do
  tmux kill-window -t "$SESSION:$win" 2>/dev/null || true
done

tmux new-window -t "$SESSION:$PG_WIN" -n "postgres" -c "$DIR" "bun run db:local; exec bash"

echo "Waiting for postgres on port 54321..."
for i in $(seq 1 30); do
  if DATABASE_URL="$LOCAL_DB" bun run db:migrate 2>/dev/null; then break; fi
  if [ "$i" -eq 30 ]; then echo "Timed out waiting for postgres" >&2; exit 1; fi
  sleep 1
done
echo "Migrations applied."

tmux new-window -t "$SESSION:$DEV_WIN" -n "dev" -c "$DIR" "bun run dev; exec bash"
```

### Example 2: Adding a third window (e.g. frontend dev server)

```bash
UI_WIN="7"
tmux kill-window -t "$SESSION:$UI_WIN" 2>/dev/null || true
tmux new-window -t "$SESSION:$UI_WIN" -n "ui" -c "$DIR" "bun run dev:ui; exec bash"
```

