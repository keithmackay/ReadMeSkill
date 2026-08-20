# Companion Files — Full Procedure (Step 6)

Consulted after the README is finalized (create or improve mode), to scan for, offer, and generate companion project files.

## 6a: Scan for Existing Files

Check which of these already exist at the project root (or in `.github/`):
- `CONTRIBUTING.md`
- `CODE_OF_CONDUCT.md`
- `LICENSE` (or `LICENSE.md`, `LICENSE.txt`)
- `CHANGELOG.md`
- `SECURITY.md`
- `.github/ISSUE_TEMPLATE/bug_report.yml`
- `.github/ISSUE_TEMPLATE/feature_request.yml`
- `.github/pull_request_template.md`

Also check whether this project is itself an agent skill or plugin: look for a `SKILL.md` file (at the project root or in a `skills/*/` subdirectory) or a `.codex-plugin/plugin.json` manifest. If so, note it as a **skill** (has `SKILL.md`) or **plugin** (has a plugin manifest without its own `SKILL.md`, or both), and check whether each manifest copy already has a working help mechanism: a `--help` flag (skills) or `:help` command (plugins) documented in the manifest, backed by a `help.md` file alongside it.

## 6b: Offer Missing Files

Present the user with a list of missing files, ordered by relevance:

1. **CONTRIBUTING.md** — almost always useful if the project accepts contributions
2. **LICENSE** — critical if not present
3. **Help mechanism** (`--help`/`:help` + `help.md`) — only offer if 6a found this project is a skill or plugin and at least one manifest copy is missing it. Lets users run `/<skill-name> --help` or `<plugin-name>:help` to see usage without triggering the workflow.
4. **CODE_OF_CONDUCT.md** — important for open-source projects
5. **CHANGELOG.md** — useful for projects with releases
6. **.github/ISSUE_TEMPLATE/** — useful for projects accepting issues
7. **.github/pull_request_template.md** — useful for projects accepting PRs
8. **SECURITY.md** — only suggest for libraries and APIs, not CLIs or simple tools

Do NOT offer files that already exist, and do NOT offer the help mechanism for non-skill/plugin projects. Ask the user which they'd like to generate. Do not generate anything without consent.

## 6c: Generate Requested Files

**CONTRIBUTING.md**:
- Section on how to report bugs (link to issue template if it exists)
- Section on how to suggest features
- Section on development setup — use the ACTUAL commands from the manifest:
  - Clone, install, run tests, run linter
  - Reference the correct package manager and test framework
- Section on PR process (branch naming, commit messages, review process)
- Section on code style (reference linter config if it exists)

**CODE_OF_CONDUCT.md**:
- Do NOT generate the full Contributor Covenant text inline — it contains language about harassment and discrimination that triggers Anthropic's content filtering policy and will cause a 400 error.
- Instead, create a short CODE_OF_CONDUCT.md that links to the Contributor Covenant v2.1 externally: `https://www.contributor-covenant.org/version/2/1/code_of_conduct/`
- Include a brief statement that the project adopts the Contributor Covenant and a link to the full text.
- Fill in the contact method (ask the user if not obvious from the manifest)

**LICENSE**:
- Ask: "Would you like to use the MIT license (yes, default) or a different license?" — if the user says yes, nothing, or anything non-specific, use MIT
- Other options if requested: Apache 2.0, GPL 3.0, BSD 2-Clause, BSD 3-Clause
- Provide the complete license text
- Fill in year (current year) and copyright holder (from manifest author or ask user)

**Help mechanism (`help.md` + Flags/Commands section)**:
- Kind: a project with a `SKILL.md` is a **skill** — the invocation is `--help` (e.g. `/some-skill --help`). A project with only a `.codex-plugin/plugin.json` (no `SKILL.md` at that level) is a **plugin** — the invocation is `:help` (e.g. `/plugin-name:help`).
- Find every copy of the core manifest file — some skills ship it in more than one location for cross-platform packaging (e.g. a root `SKILL.md` and a nested `skills/<name>/SKILL.md`). Apply the same edit to each copy.
- For each manifest copy, check whether it already has inline help content — an existing section (commonly named `## Help`) that explains usage directly in the file rather than delegating to a separate file.
  - **If inline help content exists:** extract that text into a new `help.md` file alongside the manifest, written in a plain, human-readable style (see git-release's `help.md` for the reference format: `WHAT IT DOES` / `WHAT IT NEEDS` / `USAGE` / `FLAGS` or `COMMANDS`, all-caps section headers, no markdown). Then replace the inline content in the manifest with a short `## Flags` (skills) or `## Commands` (plugins) section instructing the agent: on `--help`/`:help`, read and display `help.md` verbatim and stop, without running the rest of the workflow. Don't leave the same explanation duplicated in both places.
  - **If no help content exists at all:** generate `help.md` from scratch using the same codebase analysis already performed for the README (what it does, what it needs, usage examples, arguments/flags), then add the same short `## Flags`/`## Commands` section to the manifest.
- If the manifest is duplicated across multiple locations, duplicate `help.md` alongside each copy too — this mirrors how `git-release` itself is packaged.
- Present the generated `help.md` and the manifest edit to the user before writing.

**CHANGELOG.md**:
- Use [Keep a Changelog](https://keepachangelog.com/) format
- Seed with `## [Unreleased]` section
- Include category headers: Added, Changed, Deprecated, Removed, Fixed, Security
- If there's a current version in the manifest, add a section for it

**SECURITY.md**:
- Supported versions table (current major version)
- Reporting vulnerabilities section with responsible disclosure guidance
- Response timeline expectations
- Ask user for security contact email

**Issue Templates** (`.github/ISSUE_TEMPLATE/`):
- `bug_report.yml`: title, description, reproduction steps, expected behavior, actual behavior, environment
- `feature_request.yml`: title, problem statement, proposed solution, alternatives considered
- Both in GitHub YAML form format

**PR Template** (`.github/pull_request_template.md`):
- Checklist relevant to the project:
  - Tests added/updated
  - Documentation updated
  - Linter passes
  - Breaking changes noted (if applicable)

Generate each requested file, present it, and write after approval.
