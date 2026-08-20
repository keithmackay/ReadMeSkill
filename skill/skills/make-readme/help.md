make-readme — generate or improve a project's README.md

WHAT IT DOES
  Analyzes the current project (package manifests, directory structure,
  CI config, entry points) to auto-detect project type, language, and
  framework, then either generates a complete README.md from scratch
  (Create mode) or improves an existing one in-place (Improve mode: gap
  analysis, then enhance only what's needed while preserving voice).
  Afterward offers to generate companion files (CONTRIBUTING.md,
  CODE_OF_CONDUCT.md, LICENSE, CHANGELOG.md, SECURITY.md, issue/PR
  templates).

WHAT IT NEEDS
  - Run from inside the project directory you want a README for

USAGE
  /make-readme                              Auto-detect everything
  /make-readme audience=end-users tone=casual
  /make-readme type=library tone=formal
  /make-readme dry-run                      Preview only, no changes
  /make-readme --help                       Show this message and exit

FLAGS
  audience   developers | end-users | data-scientists | mixed
             (default: auto-detect)
  type       library | cli | webapp | api | monorepo
             (default: auto-detect)
  tone       formal | casual | minimal | playful
             (default: professional for create mode, match-existing for
             improve mode)
  dry-run    Show the analysis, section plan, and gap report without
             generating any content
  --help     Show this help message without making any changes
