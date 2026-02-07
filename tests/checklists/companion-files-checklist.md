# Companion Files Checklist

Run this checklist after the skill offers/generates companion files. Every item is yes/no.

## Detection and Offering

- [ ] Skill scanned for existing companion files before offering
- [ ] Skill did NOT offer to create files that already exist
- [ ] Skill suggested the most relevant files first (not a dump of all options)
- [ ] Skill asked before generating (did not auto-create without consent)

## CONTRIBUTING.md

- [ ] References the project's actual build tool (npm/pip/cargo/etc.)
- [ ] References the project's actual test command
- [ ] References the project's actual linting/formatting tools
- [ ] Not generic boilerplate — tailored to this specific project
- [ ] Includes branching and PR workflow guidance

## CODE_OF_CONDUCT.md

- [ ] Uses Contributor Covenant v2.1 (or explicitly states which version)
- [ ] Includes contact method for reporting
- [ ] Complete text, not a summary or link-only

## LICENSE

- [ ] Skill asked which license to use (did not assume)
- [ ] Full license text provided (not a summary)
- [ ] Year and copyright holder filled in correctly
- [ ] NOT offered if a LICENSE file already exists

## CHANGELOG.md

- [ ] Uses Keep a Changelog format
- [ ] Seeded with [Unreleased] section
- [ ] Categories match KaC standard (Added, Changed, Deprecated, Removed, Fixed, Security)

## SECURITY.md

- [ ] Only offered for libraries or API projects (not CLIs or simple tools)
- [ ] Includes responsible disclosure guidance
- [ ] Includes supported versions table
- [ ] Includes contact method

## Issue/PR Templates

- [ ] Bug report template includes reproduction steps field
- [ ] Feature request template is structured
- [ ] PR template checklist is relevant to the project type
- [ ] Templates use GitHub YAML format (in .github/ISSUE_TEMPLATE/)

## Overall

- [ ] Every generated file is tailored to the project (not generic)
- [ ] No file contradicts information in the README
- [ ] Files reference correct toolchain, commands, and project structure
