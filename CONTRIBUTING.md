# Contributing to ReadMeSkill

Thanks for your interest in contributing. This guide covers the development workflow and how to submit changes.

## Reporting Bugs

Open a [bug report](../../issues/new?template=bug_report.yml) with:

- Steps to reproduce (which fixture or project type triggered the issue)
- The generated output (or relevant excerpt)
- What you expected instead

## Suggesting Features

Open a [feature request](../../issues/new?template=feature_request.yml) describing the problem and your proposed solution.

## Development Setup

Clone the repo and install the skill:

```bash
git clone <repo-url>
cd ReadMeSkill
mkdir -p ~/.claude/skills/readme
cp skill/SKILL.md ~/.claude/skills/readme/SKILL.md
```

There's no build step or runtime dependencies. The entire skill is a single prompt file at `skill/SKILL.md`.

## Testing Changes

After editing `skill/SKILL.md`, test against the fixtures:

```bash
cd tests/fixtures/node-express-api
# In a Claude Code session:
/readme
```

Validate the output against the checklists in `tests/checklists/`:

- `create-mode-checklist.md` — for fixtures with no README
- `improve-mode-checklist.md` — for `existing-readme-with-gaps`
- `companion-files-checklist.md` — for companion file generation

Compare results with reference outputs in `tests/snapshots/`. Re-run against at least 2 fixtures after any change.

## Submitting a Pull Request

1. Fork the repo and create a feature branch
2. Make your changes to `skill/SKILL.md`
3. Test against at least 2 fixtures
4. Verify against the relevant checklists
5. Open a pull request with a description of what changed and why

## Code of Conduct

This project follows the [Contributor Covenant](CODE_OF_CONDUCT.md). Please read it before participating.
