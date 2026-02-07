# formstack

A modular form toolkit with validation, CLI scaffolding, and React components. Built as a Turborepo monorepo with three packages that share a core validation engine.

## Highlights

- **Schema-driven** — Define forms with Zod schemas, render with React or scaffold via CLI
- **Monorepo architecture** — Shared core with independent CLI and web packages
- **Type-safe** — Full TypeScript across all packages
- **Turborepo builds** — Cached, parallel builds across the workspace

## Packages

| Package | Description |
|---------|-------------|
| [`@formstack/core`](packages/core) | Form validation and state management |
| [`@formstack/cli`](packages/cli) | CLI for scaffolding forms from schema files |
| [`@formstack/web`](packages/web) | React components for rendering forms |

<details>
<summary><strong>@formstack/core</strong></summary>

Core form validation and state management built on Zod.

```typescript
import { createForm, validate } from '@formstack/core';
```

</details>

<details>
<summary><strong>@formstack/cli</strong></summary>

CLI for scaffolding forms from schema files.

```bash
# Initialize a form schema
formstack init

# Generate form from schema
formstack generate
```

</details>

<details>
<summary><strong>@formstack/web</strong></summary>

React components for rendering forms.

```tsx
import { Form, Field } from '@formstack/web';
```

</details>

## Getting Started

### Prerequisites

- Node.js 18+
- npm or pnpm

### Installation

```bash
git clone <repo-url>
cd formstack
npm install
```

## Development

```bash
git clone <repo-url>
cd formstack
npm install
npm run build
npm run test
```

| Command | Description |
|---------|-------------|
| `npm run build` | Build all packages (via Turbo) |
| `npm run test` | Run all tests (via Turbo) |
| `npm run lint` | Lint all packages (via Turbo) |
| `npm run dev` | Start all packages in dev mode |

## Contributing

Contributions are welcome. Fork the repo, create a feature branch, and open a pull request.

## License

See individual packages for license information.
