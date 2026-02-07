# markdown-it-footnotes

A footnote plugin for markdown-it. Adds `[^1]` footnote syntax with configurable anchor and backref CSS classes. Renders footnotes as a numbered list at the end of the document.

## Highlights

- **Standard syntax** — Uses the widely-recognized `[^label]` footnote format
- **Configurable** — Custom CSS classes for anchors and backrefs
- **TypeScript** — Full type definitions included

## Getting Started

### Installation

```bash
npm install markdown-it-footnotes
```

### Quick Start

```typescript
import MarkdownIt from 'markdown-it';
import footnotePlugin from 'markdown-it-footnotes';

const md = new MarkdownIt();
md.use(footnotePlugin, { anchorClass: 'footnote-anchor' });

const result = md.render('Text with a footnote[^1]\n\n[^1]: Footnote content');
```

## Usage

```typescript
import MarkdownIt from 'markdown-it';
import footnotePlugin from 'markdown-it-footnotes';

const md = new MarkdownIt();

// Default options
md.use(footnotePlugin);

// With custom options
md.use(footnotePlugin, {
  anchorClass: 'my-anchor',
  backrefClass: 'my-backref',
});
```

## API Reference

### `footnotePlugin(md: MarkdownIt, options?: FootnoteOptions): void`

Register the footnote plugin with a markdown-it instance.

#### `FootnoteOptions`

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `anchorClass` | `string` | `undefined` | CSS class for footnote anchors |
| `backrefClass` | `string` | `undefined` | CSS class for backref links |

## Development

```bash
git clone https://github.com/example/markdown-it-footnotes.git
cd markdown-it-footnotes
npm install
npm test
```

| Command | Description |
|---------|-------------|
| `npm test` | Run tests with Vitest |
| `npm run build` | Build with tsup |
| `npm run lint` | Lint source files |

## Contributing

Contributions are welcome. Fork the repo, create a feature branch, and open a pull request.

## License

[MIT](LICENSE)
