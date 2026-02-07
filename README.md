# ReadMeSkill

A Claude Code skill that generates or improves README.md files for any GitHub project. Analyzes the codebase — package manifests, directory structure, CI config, existing docs — and produces a well-structured, GitHub-flavored markdown README tailored to the detected project type.

Works in two modes: **create mode** (no README exists) generates a complete file from scratch; **improve mode** (README exists) performs a gap analysis and enhances the existing file in-place, preserving the author's voice. After the README is done, offers to create companion files like CONTRIBUTING.md, CODE_OF_CONDUCT.md, LICENSE, and issue/PR templates.

## Installation

```bash
mkdir -p ~/.claude/skills/readme
cp skill/SKILL.md ~/.claude/skills/readme/SKILL.md
```

Or, if you cloned this repo elsewhere:

```bash
mkdir -p ~/.claude/skills/readme
cp /path/to/ReadMeSkill/skill/SKILL.md ~/.claude/skills/readme/SKILL.md
```

## Usage

In any project directory, open a Claude Code session and run:

```
/readme
```

The skill will:

1. Analyze the codebase (package manifests, directory structure, CI config, existing docs)
2. Detect the project type (library, CLI, webapp, API, monorepo)
3. Ask whether you want badges
4. Generate a complete README (create mode) or improve the existing one (improve mode)
5. Offer to create companion files (CONTRIBUTING.md, CODE_OF_CONDUCT.md, LICENSE, etc.)

### Optional Arguments

Pass arguments after `/readme` to override auto-detection:

```
/readme audience=end-users tone=casual
/readme type=library tone=formal
```

| Argument | Values | Default |
|----------|--------|---------|
| `audience` | `developers`, `end-users`, `data-scientists`, `mixed` | Auto-detect |
| `type` | `library`, `cli`, `webapp`, `api`, `monorepo` | Auto-detect |
| `tone` | `formal`, `casual`, `minimal`, `playful` | `professional` (create) / `match-existing` (improve) |

## Supported Project Types

- **Node.js** — package.json (npm, yarn, pnpm)
- **Python** — pyproject.toml, setup.py, setup.cfg
- **Rust** — Cargo.toml
- **Go** — go.mod
- **Java** — pom.xml, build.gradle
- **Ruby** — Gemfile, .gemspec
- **PHP** — composer.json
- **Monorepos** — Turborepo, Lerna, pnpm workspaces, npm workspaces

## License

MIT
