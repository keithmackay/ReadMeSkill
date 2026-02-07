# ReadMeSkill — Phases Summary

Quick-reference overview of all implementation phases.

## Current Status

| Phase | Title | Status |
|-------|-------|--------|
| 0 | Project Scaffolding | ✅ Complete |
| 1 | Core Skill — Create Mode | ✅ Complete |
| 2 | Improve Mode | ✅ Complete |
| 3 | Companion Files | ✅ Complete |
| 4 | Arguments and Final Polish | ✅ Complete |

---

## Phase 0: Project Scaffolding

**Goal**: Organize the repo for development. Replace template artifacts.

| Task | Description | Deliverable |
|------|-------------|-------------|
| 0.1 | Rewrite TESTING_GUIDELINES.md | `docs/TESTING_GUIDELINES.md` with prompt-testing strategy |
| 0.2 | Create test fixtures | 5 minimal fake projects in `tests/fixtures/` |
| 0.3 | Create acceptance checklists | 3 checklists in `tests/checklists/` |
| 0.4 | Write plan and summary to repo | `docs/plans/IMPLEMENTATION_PLAN.md`, `docs/plans/PHASES_SUMMARY.md` |

## Phase 1: Core Skill — Create Mode

**Goal**: `/readme` on a project with no README produces a well-structured one.

| Task | Description | Deliverable |
|------|-------------|-------------|
| 1.1 | Skill skeleton with codebase analysis | `skill/SKILL.md` with analysis + detection steps |
| 1.2 | Create-mode section menu and generation rules | Section selection logic, writing instructions, badge logic |
| 1.3 | Test across fixtures and iterate | Refined SKILL.md, snapshots in `tests/snapshots/` |
| 1.4 | Writing quality and self-check | Quality guidelines and self-verification checklist |

## Phase 2: Improve Mode

**Goal**: When README exists, gap-analyze and improve in-place.

| Task | Description | Deliverable |
|------|-------------|-------------|
| 2.1 | Gap analysis and voice preservation | Improve-mode workflow in SKILL.md |
| 2.2 | Test and iterate improve mode | Refined improve-mode instructions, snapshot |

## Phase 3: Companion Files

**Goal**: Generate CONTRIBUTING.md, CODE_OF_CONDUCT.md, LICENSE, CHANGELOG.md, etc.

| Task | Description | Deliverable |
|------|-------------|-------------|
| 3.1 | Companion file detection and templates | Companion file workflow in SKILL.md |
| 3.2 | Test companion files across fixtures | Refined templates |

## Phase 4: Arguments and Final Polish

**Goal**: Add optional arguments, install, validate end-to-end.

| Task | Description | Deliverable |
|------|-------------|-------------|
| 4.1 | Optional arguments | `audience`, `type`, `tone` argument parsing in SKILL.md |
| 4.2 | Installation instructions and project README | Updated `README.md` with real instructions |
| 4.3 | End-to-end validation on real projects | Checklist passes on real repos |
| 4.4 | Final cleanup | All docs current, no stale references |

---

## Key Principles

- **YAGNI**: No CLI wrapper, no automated test harness, no web UI
- **Single deliverable**: Everything lives in `skill/SKILL.md`
- **TDD for prompts**: Checklists are the tests, fixtures are the test data
- **Frequent commits**: One commit per task minimum

## Technology Stack

- Claude Code skill system (SKILL.md with YAML frontmatter)
- GitHub-flavored Markdown
- No runtime dependencies
