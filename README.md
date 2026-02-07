# ReadMeSkill

A Claude Code skill that generates or improves README.md files for any GitHub project. Analyzes the codebase — package manifests, directory structure, CI config, existing docs — and produces a well-structured, GitHub-flavored markdown README tailored to the detected project type.

Works in two modes: **create mode** (no README exists) generates a complete file from scratch; **improve mode** (README exists) performs a gap analysis and enhances the existing file in-place, preserving the author's voice. After the README is done, offers to create companion files like CONTRIBUTING.md, CODE_OF_CONDUCT.md, LICENSE, and issue/PR templates.

## Highlights

- **Two modes** — Create from scratch or improve an existing README with gap analysis
- **Project-aware** — Detects project type (library, CLI, webapp, API, monorepo) and tailors sections accordingly
- **Voice preservation** — Improve mode matches the tone and style of existing content
- **Companion files** — Offers to generate CONTRIBUTING.md, CODE_OF_CONDUCT.md, LICENSE, CHANGELOG.md, issue/PR templates
- **No dependencies** — Pure prompt; install by copying a single file

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
/readme dry-run
```

| Argument | Values | Default |
|----------|--------|---------|
| `audience` | `developers`, `end-users`, `data-scientists`, `mixed` | Auto-detect |
| `type` | `library`, `cli`, `webapp`, `api`, `monorepo` | Auto-detect |
| `tone` | `formal`, `casual`, `minimal`, `playful` | `professional` (create) / `match-existing` (improve) |
| `dry-run` | (flag, no value) | Off |

`dry-run` shows the detected project type, planned section list, and gap analysis (for improve mode) without generating any content.

## Supported Project Types

- **Node.js** — package.json (npm, yarn, pnpm)
- **Python** — pyproject.toml, setup.py, setup.cfg
- **Rust** — Cargo.toml
- **Go** — go.mod
- **Java** — pom.xml, build.gradle
- **Ruby** — Gemfile, .gemspec
- **PHP** — composer.json
- **Monorepos** — Turborepo, Lerna, pnpm workspaces, npm workspaces

## Project Structure

```
skill/SKILL.md              The skill — all prompt logic lives here
tests/fixtures/             5 minimal fake projects for testing
tests/checklists/           Acceptance criteria (create, improve, companion files)
tests/snapshots/            Reference outputs from fixture runs
docs/plans/                 Implementation plan and phase summary
docs/TESTING_GUIDELINES.md  Prompt-testing strategy
```

## Development

There's no build step or automated test suite — the deliverable is a single prompt file. Testing is manual and fixture-based.

**Run the skill against a fixture:**

```bash
cd tests/fixtures/node-express-api
# In a Claude Code session:
/readme
```

**Validate output against checklists:**

Review the generated README against the appropriate checklist in `tests/checklists/`:

| Checklist | When to use |
|-----------|-------------|
| `create-mode-checklist.md` | After running on a fixture with no README |
| `improve-mode-checklist.md` | After running on `existing-readme-with-gaps` |
| `companion-files-checklist.md` | After generating companion files |

**Compare against snapshots:**

Reference outputs for each fixture are saved in `tests/snapshots/`. After changing `skill/SKILL.md`, re-run against at least 2 fixtures and compare with the snapshots.

## Contributing

Contributions are welcome. See [CONTRIBUTING.md](CONTRIBUTING.md) for development setup and guidelines.

## License

[MIT](LICENSE)
