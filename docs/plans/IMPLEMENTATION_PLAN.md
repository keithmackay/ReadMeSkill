# ReadMeSkill Implementation Plan

## Context

ReadMeSkill is a globally-installable Claude Code skill that auto-generates or improves README.md files for any GitHub project. The skill analyzes the codebase (package manifests, directory structure, CI config, existing docs) and produces a well-structured, GitHub-flavored markdown README tailored to the detected project type. It also offers to create companion files (CONTRIBUTING.md, CODE_OF_CONDUCT.md, LICENSE, etc.).

The primary deliverable is a single file: `skill/SKILL.md` (development copy), installed to `~/.claude/skills/readme/SKILL.md`. This is a prompt engineering project — no runtime dependencies, no build step, no separate codebase.

## Key Design Decisions

- **Auto-generate**: Analyze codebase, produce complete README in one shot
- **Universal/adaptive**: Detect project type (library, CLI, webapp, API, monorepo) and tailor sections
- **Improve in-place**: When README exists, gap-analyze and enhance while preserving author's voice
- **Full doc suite**: Offer companion files after README is done
- **Badges**: Ask user once whether to include them
- **Optional arguments**: `audience`, `type`, `tone` — auto-detect if not provided

## Critical Files

| File | Role |
|------|------|
| `skill/SKILL.md` | The deliverable — all prompt logic lives here |
| `~/.claude/skills/writeli/SKILL.md` | Reference implementation for SKILL.md format |
| `tests/fixtures/` | 5 minimal fake projects for testing |
| `tests/checklists/` | Acceptance criteria (the "tests") |
| `docs/plans/IMPLEMENTATION_PLAN.md` | This file |
| `docs/plans/PHASES_SUMMARY.md` | Quick-reference summary |
| `docs/TESTING_GUIDELINES.md` | Prompt-testing strategy |

## Testing Strategy

TDD for a prompt-based skill means: define expected output characteristics, run the skill, verify against checklists. No unit tests, no automated assertions.

1. **Fixture-based testing**: Run skill against `tests/fixtures/` directories (controlled inputs)
2. **Checklist validation**: 3 checklists in `tests/checklists/` — every item is yes/no
3. **Snapshot testing**: Save good outputs to `tests/snapshots/` as reference points
4. **Real-world validation**: Final pass against real repos
5. **Regression protocol**: After any SKILL.md change, re-run against 2+ fixtures

## What NOT to Build (YAGNI)

- No CLI tool or wrapper script (installation is `cp`)
- No automated test harness (output is natural language)
- No web UI or interactive mode
- No per-language template files
- No versioning system

---

## Phase 0: Project Scaffolding

**Goal**: Organize the repo for development. Replace template artifacts.

### Task 0.1: Rewrite TESTING_GUIDELINES.md
- **File**: `docs/TESTING_GUIDELINES.md`
- **What**: Replace Flutter/Playwright/Dart references with prompt-testing strategy: fixture-based testing, checklist validation, snapshot comparison, real-world validation
- **Test**: Read the file — no mentions of Flutter, Playwright, or Dart
- **Commit**: `Phase 0.1: Replace template testing guidelines with prompt-testing strategy`

### Task 0.2: Create Test Fixtures
- **Directory**: `tests/fixtures/`
- **What**: 5 minimal fake projects (just enough files for the skill to analyze, no working code):
  1. `node-express-api/` — package.json (express, jest), src/, .env.example, CI config. No README.
  2. `python-cli-tool/` — pyproject.toml (click), src/cli/, tests/, LICENSE. No README.
  3. `rust-library/` — Cargo.toml (lib crate), src/lib.rs, examples/, benches/. No README.
  4. `existing-readme-with-gaps/` — package.json, src/, weak README (title + one-line description only)
  5. `monorepo/` — package.json (workspaces), 3 packages with minimal structure. No README.
- **Test**: `ls` each fixture — file structure is plausible
- **Commit**: `Phase 0.2: Add test fixtures for 5 project types`

### Task 0.3: Create Acceptance Checklists
- **Directory**: `tests/checklists/`
- **What**: 3 checklists with yes/no items:
  1. `create-mode-checklist.md` — Title matches project? Description uses what/why/how? Getting Started is copy-paste-ready? Code blocks have language hints? Only relevant sections included? etc.
  2. `improve-mode-checklist.md` — Preserves author's voice? Keeps good existing content? Identifies gaps? Doesn't rewrite strong sections? etc.
  3. `companion-files-checklist.md` — Asks before generating? CONTRIBUTING tailored to toolchain? CODE_OF_CONDUCT uses Contributor Covenant? LICENSE text correct? etc.
- **Test**: Every checklist item is objectively verifiable
- **Commit**: `Phase 0.3: Add acceptance checklists for skill output validation`

### Task 0.4: Write Implementation Plan and Summary to Repo
- **Files**: `docs/plans/IMPLEMENTATION_PLAN.md`, `docs/plans/PHASES_SUMMARY.md`
- **What**: Commit the detailed plan and summary into the repo so `/next` and `/checkwork` commands work
- **Test**: Files exist and match agreed design
- **Commit**: `Phase 0.4: Add implementation plan and phases summary`

---

## Phase 1: Core Skill — Create Mode

**Goal**: Build the SKILL.md so that `/readme` on a project with no README produces a well-structured one.

### Task 1.1: Skill Skeleton with Codebase Analysis
- **File**: `skill/SKILL.md`
- **What**: Create SKILL.md with:
  - YAML frontmatter (`name: readme`, `description: ...`)
  - Title: `# Generate or Improve README`
  - Overview (2-3 sentences)
  - Step 1 — Analyze the Codebase: scan package manifests, directory structure, CI config, existing docs, license files, config files, entry points
  - Step 2 — Detect Project Type: classify as library/CLI/webapp/API/monorepo/framework/other, note language/framework
  - Step 3 — Determine Mode: README exists → improve mode; no README → create mode
- **Test**: Install skill (`cp skill/SKILL.md ~/.claude/skills/readme/SKILL.md`). Invoke on `tests/fixtures/node-express-api/`. Skill should analyze and detect "Node.js Express API" before getting stuck (generate instructions don't exist yet)
- **Commit**: `Phase 1.1: Add skill skeleton with codebase analysis instructions`

### Task 1.2: Create-Mode Section Menu and Generation Rules
- **File**: `skill/SKILL.md`
- **What**: Add Step 4 (Create Mode) with:
  - **Section selection logic** — which of 14 sections to include based on detected project:
    - Always: Title, Description, Highlights, Getting Started, Usage, Development, Contributing, License
    - Conditional: Badges (ask user), TOC (>5 sections), API Reference (libraries only), Configuration (if env/config exists), Roadmap (if evidence), Acknowledgments (if warranted)
  - **Per-section writing instructions** — tone, length, content guidance:
    - Description: what/why/how framework, 2-4 sentences
    - Getting Started: copy-paste-ready commands from actual manifest
    - Usage: real code blocks with language hints
    - Configuration: table format (Variable | Description | Default | Required)
    - GitHub alert syntax (`> [!NOTE]`) and collapsible sections (`<details>`) where appropriate
  - **Badge logic** — if user opts in, suggest 3-6 from: build status, coverage, version, license, downloads, last commit. shields.io format.
- **Test**: Install updated skill. Run on `tests/fixtures/node-express-api/`. Walk through `create-mode-checklist.md`. Should include Getting Started with `npm install`, Development with `npm test`, Configuration (because .env.example exists), NOT API Reference
- **Commit**: `Phase 1.2: Add create-mode section menu and generation instructions`

### Task 1.3: Test Across Fixtures and Iterate
- **File**: `skill/SKILL.md` (refine based on results)
- **What**: Run skill against remaining fixtures, iterate on prompt:
  1. `python-cli-tool/` — Usage shows CLI invocation, not library import. Getting Started uses `pip install`.
  2. `rust-library/` — Includes API Reference. Usage shows `use` statements and Cargo.toml dependency. Detects benchmarks.
  3. `monorepo/` — Detects workspaces. Describes all packages. Uses collapsible sections for per-package details.
- **Test**: All 3 fixtures pass create-mode checklist. Save outputs to `tests/snapshots/`
- **Commit**: `Phase 1.3: Refine create-mode instructions across project types`

### Task 1.4: Writing Quality and Self-Check
- **File**: `skill/SKILL.md`
- **What**: Add "Writing Quality" section:
  - Tone: professional but approachable, no emoji in headers, no corporate speak
  - Formatting: code blocks always have language hints, relative links for in-repo files, blank lines between sections
  - Anti-patterns: no "This project is a...", no placeholders, no over-promising ("blazing fast"), no repeating title in description
  - Quality Checklist: self-verify before presenting output (like writeli skill has)
- **Test**: Re-run on `node-express-api`. No emoji in headers, code blocks have language hints, no placeholder text
- **Commit**: `Phase 1.4: Add writing quality guidelines and self-check instructions`

---

## Phase 2: Improve Mode

**Goal**: When README exists, gap-analyze and improve in-place without rewriting good content.

### Task 2.1: Gap Analysis and Voice Preservation
- **File**: `skill/SKILL.md`
- **What**: Add improve-mode workflow:
  1. Parse existing README — map headings to section menu (fuzzy: "Installation" = "Getting Started", etc.)
  2. Score each section: Strong / Adequate / Weak / Missing
  3. Present gap report to user before making changes
  4. Fill gaps: generate new content for Missing, enhance Weak, leave Strong/Adequate alone
  5. Voice preservation: match tone, vocabulary, sentence structure of existing content
- **Test**: Run on `tests/fixtures/existing-readme-with-gaps/`. Apply `improve-mode-checklist.md`. Should detect weak description, missing Usage/Development/Contributing/License. Should preserve existing title and tone.
- **Commit**: `Phase 2.1: Add improve-mode gap analysis and voice preservation`

### Task 2.2: Test and Iterate Improve Mode
- **File**: `skill/SKILL.md` (refine)
- **What**: Run improve mode multiple times on the gaps fixture. Then test on one real repo. Refine instructions.
- **Test**: Improve-mode checklist passes. Enhanced content doesn't clash tonally with preserved content. Save snapshot.
- **Commit**: `Phase 2.2: Refine improve-mode instructions from testing`

---

## Phase 3: Companion Files

**Goal**: Skill can generate CONTRIBUTING.md, CODE_OF_CONDUCT.md, LICENSE, CHANGELOG.md, SECURITY.md, issue/PR templates.

### Task 3.1: Companion File Detection and Templates
- **File**: `skill/SKILL.md`
- **What**: Add final step to skill workflow (after create/improve):
  1. Scan for existing companion files
  2. Offer to create missing ones (suggest most relevant first, don't push all)
  3. Template guidance for each:
     - CONTRIBUTING.md: tailored to actual toolchain (correct build/test commands)
     - CODE_OF_CONDUCT.md: Contributor Covenant v2.1
     - LICENSE: ask which, provide full text for MIT/Apache 2.0/GPL 3.0/BSD
     - CHANGELOG.md: Keep a Changelog format, seed with [Unreleased]
     - SECURITY.md: only for libraries/APIs, responsible disclosure guidance
     - Issue templates: bug report + feature request in GitHub YAML format
     - PR template: checklist relevant to project type
- **Test**: Run on `tests/fixtures/python-cli-tool/` (has LICENSE). Should NOT offer LICENSE. Should offer CONTRIBUTING (referencing `pip`, `pytest`). Should NOT push SECURITY.md (CLI tool). Apply `companion-files-checklist.md`.
- **Commit**: `Phase 3.1: Add companion file generation with project-tailored templates`

### Task 3.2: Test Companion Files Across Fixtures
- **File**: `skill/SKILL.md` (refine)
- **What**: Run companion generation on 2+ fixtures. Verify tailoring (not generic boilerplate).
- **Test**: Companion-files checklist passes for each fixture
- **Commit**: `Phase 3.2: Refine companion file templates from testing`

---

## Phase 4: Arguments and Final Polish

**Goal**: Add optional arguments, install skill, validate end-to-end.

### Task 4.1: Optional Arguments
- **File**: `skill/SKILL.md`
- **What**: Add argument parsing near top of skill:
  - `audience`: developers / end-users / data-scientists / mixed (default: auto-detect)
  - `type`: library / cli / webapp / api / monorepo (default: auto-detect)
  - `tone`: formal / casual / minimal / playful (default: professional-but-approachable for create, match-existing for improve)
- **Test**: Run on `node-express-api` with `audience=end-users tone=casual`. README should be noticeably more casual.
- **Commit**: `Phase 4.1: Add optional audience, type, and tone arguments`

### Task 4.2: Installation Instructions and Project README
- **Files**: `skill/SKILL.md` (minor tweaks if needed), `README.md`
- **What**: Verify install/invoke flow end-to-end. Update project README with real installation and usage instructions (replace placeholders).
- **Test**: Fresh Claude Code session. `/readme` activates. Full workflow runs. Project README is accurate.
- **Commit**: `Phase 4.2: Verify install flow and update project README`

### Task 4.3: End-to-End Validation on Real Projects
- **No file changes** (unless fixes needed)
- **What**: Run skill against 2-3 real repos (this project, a Keith-owned project, a well-known OSS project). Both create and improve modes. Walk through all 3 checklists.
- **Test**: All checklists pass. Output is something you'd actually commit.
- **Commit**: `Phase 4.3: Fix issues found during end-to-end validation` (only if needed)

### Task 4.4: Final Cleanup
- **Files**: `README.md`, `docs/TESTING_GUIDELINES.md`
- **What**: Dog-food the skill on its own repo. Clean up any temporary files. Ensure all docs are current and reference the right project.
- **Test**: No file references wrong project. README is complete. TESTING_GUIDELINES describes actual strategy.
- **Commit**: `Phase 4.4: Final README and documentation cleanup`

---

## Verification

To verify the skill works end-to-end:

1. `mkdir -p ~/.claude/skills/readme && cp skill/SKILL.md ~/.claude/skills/readme/SKILL.md`
2. Open a new Claude Code session in any project directory
3. Run `/readme`
4. Skill should: analyze the codebase, ask about badges, generate a complete README (or improve the existing one), then offer companion files
5. Validate output against the appropriate checklist(s) in `tests/checklists/`

## Next Steps (Post-Launch Enhancements)

- [Keith's idea] Support for reading GitHub Issues/PRs to inform the Roadmap section
- [Claude's idea] A `--dry-run` argument that shows the section menu and gap analysis without generating content
- [Claude's idea] Language-specific README conventions (e.g., Rust crates include `## MSRV`, Python packages include `## Compatibility`)
- [Keith's idea] Support for generating docs beyond the repo root (e.g., monorepo per-package READMEs)
- [Claude's idea] A "minimalist mode" argument that generates a 20-line README for small utilities
