# Create Mode Checklist

Run this checklist after the skill generates a README from scratch. Every item is yes/no.

## Project Detection

- [ ] Title matches the project name from the package manifest
- [ ] Detected project type is correct (library/CLI/webapp/API/monorepo)
- [ ] Detected language and framework are correct

## Description

- [ ] Description explains what the project does (not "This project is a...")
- [ ] Description covers what, why, and how in 2-4 sentences
- [ ] Description does not repeat the title verbatim

## Getting Started

- [ ] Prerequisites lists actual dependencies (correct runtime, database, etc.)
- [ ] Installation commands are copy-paste-ready
- [ ] Installation commands match the actual package manager (npm/pip/cargo/etc.)
- [ ] Commands use the correct package name from the manifest

## Usage

- [ ] Code blocks have language hints (```js, ```python, etc.)
- [ ] Usage examples are relevant to the project type (CLI shows invocation, library shows import)
- [ ] Examples reference actual exports, commands, or endpoints from the codebase

## Development

- [ ] Lists actual dev commands from the manifest (test, lint, build)
- [ ] Commands are correct (not generic placeholders)

## Sections

- [ ] Only relevant sections are included (no API Reference for a CLI tool, etc.)
- [ ] Configuration section present if .env or config files exist
- [ ] Configuration uses table format (Variable | Description | Default | Required)
- [ ] No empty or placeholder sections

## Formatting

- [ ] No emoji in section headers
- [ ] Code blocks all have language hints
- [ ] Relative links used for in-repo files (not absolute URLs)
- [ ] Blank line between every section
- [ ] TOC included if more than 5 sections

## Writing Quality

- [ ] No corporate speak or buzzwords ("blazing fast", "cutting-edge", "seamless")
- [ ] No placeholder text ("[describe X here]", "TODO", "coming soon")
- [ ] No over-promising or exaggerated claims
- [ ] Professional but approachable tone
- [ ] Consistent voice throughout

## Badges (if user opted in)

- [ ] 3-6 badges, not more
- [ ] Badges are relevant (build status, version, license — not decorative)
- [ ] shields.io format used

## Overall

- [ ] README is self-contained — a developer could get started without other docs
- [ ] Nothing factually wrong (correct commands, correct package names, correct structure)
- [ ] Length is proportional to project complexity (small project = shorter README)
