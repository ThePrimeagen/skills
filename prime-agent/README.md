# prime-agent

`prime-agent` is an interactive CLI to symlink skills from this repo into a project using both modern layouts:

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

Run in the project you want to link skills into:

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

In the picker:

- `Up` / `Down` to move
- `Space` to select or unselect
- Type to fuzzy-find skills
- `Backspace` to edit your filter
- `Enter` to apply

Behavior:

- Skills are loaded from `skills/` in this repo.
- Already-linked skills in the target are preselected.
- Selected skills are symlinked into both `.agents/skills` and `.cursor/rules`.
- Unselected skills are removed from both locations.
- Missing `.agents/skills` and `.cursor/rules` directories are created automatically.

`init` behavior:

- `prime-agent init ts` writes a project `AGENTS.md` template in the current directory.
- Any other template value fails with: `Unsupported init`.
