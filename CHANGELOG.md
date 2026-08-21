# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/), and this project adheres to [Semantic Versioning](https://semver.org/).

## [Unreleased]

### Added

- Add --version flag support, reporting installed version and a best-effort GitHub update check
- README `## Changelog` section linking to `CHANGELOG.md`
- Help-mechanism (`--help`/`:help` + `help.md`) generation as a companion file, with a README pointer to it added automatically
- `/readme` skill with create and improve modes
- Project type detection (Node.js, Python, Rust, Go, Java, Ruby, PHP, monorepos)
- Companion file generation (CONTRIBUTING, CODE_OF_CONDUCT, LICENSE, CHANGELOG, SECURITY, issue/PR templates)
- Optional arguments: `audience`, `type`, `tone`, `dry-run`
- Gap analysis for improve mode with voice preservation
- Badge support via shields.io
- Test fixtures for 5 project types
- Acceptance checklists for create mode, improve mode, and companion files
- Reference snapshots for fixture outputs
