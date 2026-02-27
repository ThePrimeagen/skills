# prime-agent

`prime-agent` is an interactive CLI to copy skills from this repo into a project using both modern layouts:

- `.agents/skills/<skill-name>`
- `.cursor/rules/<skill-name>`

## Install

From the repository root:

```bash
./install
```

This will run tests, build the binary, and install `prime-agent` to:

```text
~/.local/bin/prime-agent
```

## Usage

Run in the project you want to copy skills into:

```bash
prime-agent
```

Or choose an explicit target directory:

```bash
prime-agent --target-dir /path/to/project
```

Initialize a TypeScript project template:

```bash
prime-agent init ts
```

Clear all prime-agent-managed skills from the current project:

```bash
prime-agent clear
```

Or clear from an explicit target directory:

```bash
prime-agent clear --target-dir /path/to/project
```

Show which prime-agent-managed skills are out of date:

```bash
prime-agent status
```

Or show status for an explicit target directory:

```bash
prime-agent status --target-dir /path/to/project
```

Recopy all already-managed skills (to pull in updates):

```bash
prime-agent sync
```

Or sync an explicit target directory:

```bash
prime-agent sync --target-dir /path/to/project
```

In the picker:

- `Up` / `Down` to move
- `Space` to select or unselect
- Type to fuzzy-find skills
- `Backspace` to edit your filter
- `Enter` to apply

Behavior:

- Skills are loaded from `skills/` in this repo.
- Already-linked skills in the target are preselected.
- `prime-agent` copies only newly selected or missing skills.
- Existing selected skills are not overwritten by `prime-agent`.
- Unselected skills are removed from both locations.
- After selection, `prime-agent` reports which managed skills are out of date.
- Missing `.agents/skills` and `.cursor/rules` directories are created automatically.
- `.agents` copies strip Cursor frontmatter headers (`--- ... ---`) from markdown files.
- `.cursor/rules` copies preserve existing frontmatter headers and add a default header if one is missing.

`init` behavior:

- `prime-agent init ts` writes a project `AGENTS.md` template in the current directory.
- Any other template value fails with: `Unsupported init`.

`clear` behavior:

- Removes only skills that exist in this repo's `skills/` set from `.agents/skills` and `.cursor/rules`.
- Leaves unrelated/custom directories untouched.

`status` behavior:

- Reports only skills already managed by prime-agent in the target project.
- Marks a skill out of date when its `.agents` or `.cursor/rules` copy differs from current source skill content.

`sync` behavior:

- Re-copies all already-managed skills from this repo into both `.agents/skills` and `.cursor/rules`.
- Does not add brand-new skills that are not already managed in the target project.
