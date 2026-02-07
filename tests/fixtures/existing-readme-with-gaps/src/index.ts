import MarkdownIt from 'markdown-it';

export interface FootnoteOptions {
  anchorClass?: string;
  backrefClass?: string;
}

export default function footnotePlugin(md: MarkdownIt, options?: FootnoteOptions): void {
  // Plugin implementation
}
