# ReadMeSkill Testing Guidelines

This document defines the testing strategy for ReadMeSkill, a prompt-based Claude Code skill. Since the deliverable is a prompt (SKILL.md) rather than executable code, traditional unit tests don't apply. Instead, we validate output quality through fixture-based testing, checklist evaluation, and snapshot comparison.

---

## Testing Layers

```
        /\
       /  \  Real-World Validation
      /----\
     /      \  Snapshot Comparison
    /--------\
   /          \  Checklist Evaluation
  /--------------\
   Fixture-Based Testing
```

### 1. Fixture-Based Testing (Foundation)

**What**: Run the skill against controlled, minimal fake projects with known characteristics.

**Fixtures** (in `tests/fixtures/`):
- `node-express-api/` — Node.js Express API with no README
- `python-cli-tool/` — Python CLI tool with no README
- `rust-library/` — Rust library crate with no README
- `existing-readme-with-gaps/` — Project with a weak, incomplete README
- `monorepo/` — Multi-package workspace with no README

**How**: Invoke `/readme` while in the fixture directory. The controlled inputs let us verify that the skill correctly detects project type, selects appropriate sections, and generates relevant content.

**Pass criteria**: The skill produces output that reflects the fixture's characteristics (correct language, correct toolchain commands, correct section selection).

### 2. Checklist Evaluation

**What**: Walk through a yes/no checklist after each skill invocation to verify output quality.

**Checklists** (in `tests/checklists/`):
- `create-mode-checklist.md` — Validates README generation from scratch
- `improve-mode-checklist.md` — Validates gap analysis and in-place improvement
- `companion-files-checklist.md` — Validates companion file generation

**How**: After the skill produces output, open the relevant checklist and answer each item honestly. Every item is designed to be objectively verifiable (yes/no, not subjective).

**Pass criteria**: All checklist items pass. Any failure means the SKILL.md prompt needs refinement.

### 3. Snapshot Comparison

**What**: Save known-good outputs as reference points to detect regressions.

**Snapshots** (in `tests/snapshots/`):
- Named after the fixture and mode: e.g., `node-express-api-create.md`
- Saved after a successful checklist pass

**How**: After modifying SKILL.md, re-run the skill against at least 2 fixtures and compare output to the saved snapshot. Look for regressions: lost sections, degraded formatting, wrong commands, tonal drift.

**Pass criteria**: Output is at least as good as the snapshot. Differences should be improvements, not regressions. Update the snapshot if output is genuinely better.

### 4. Real-World Validation (Top)

**What**: Run the skill against real GitHub repositories to verify it works outside controlled conditions.

**How**: Pick 2-3 real repos (varying in size, language, and completeness). Run both create and improve modes where applicable. Walk through the relevant checklist.

**Pass criteria**: Output is something you'd actually commit to the repo. Content is accurate, commands are correct, tone is appropriate.

---

## Regression Protocol

After ANY change to `skill/SKILL.md`:

1. Re-run the skill against at least 2 fixtures
2. Walk through the relevant checklist(s)
3. Compare output to saved snapshots
4. If output regressed, fix the prompt before committing
5. If output improved, update the snapshot

---

## TDD Adaptation for Prompt Engineering

Traditional TDD (write failing test → make it pass) adapts to prompt work as:

1. **Define expected behavior** — Write or update the checklist item that describes what the skill should do
2. **Run the skill** — Invoke `/readme` on a fixture
3. **Evaluate** — Walk the checklist. If the item fails, the "test" fails.
4. **Refine the prompt** — Edit SKILL.md to address the failure
5. **Re-run** — Verify the checklist item now passes
6. **Check for regressions** — Verify other checklist items still pass

---

## Test Execution Quick Reference

| Action | Command |
|--------|---------|
| Run skill on a fixture | `cd tests/fixtures/<name> && /readme` |
| Evaluate output | Open `tests/checklists/<checklist>.md`, answer each item |
| Save a snapshot | Copy skill output to `tests/snapshots/<fixture>-<mode>.md` |
| Regression check | Re-run on 2+ fixtures after any SKILL.md change |

---

## Anti-Patterns

- **Don't test mocked behavior**: Always run the skill against real (or fixture) project structures, never simulated input
- **Don't skip the checklist**: "It looks fine" is not a test pass
- **Don't update snapshots without checking**: Only update a snapshot when the output is genuinely better, not just different
- **Don't test only one fixture**: Different project types exercise different skill logic
