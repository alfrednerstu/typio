import { unified } from 'unified';
import remarkParse from 'remark-parse';
import remarkGfm from 'remark-gfm';
import remarkRehype from 'remark-rehype';
import rehypeRaw from 'rehype-raw';
import rehypeStringify from 'rehype-stringify';

function escapeHtml(str: string): string {
  return str
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;');
}

/** Replace [[wikilinks]] with HTML anchor tags before parsing. */
function expandWikilinks(markdown: string): string {
  return markdown.replace(/\[\[([^\]]+)\]\]/g, (_match, inner: string) => {
    const pipeIdx = inner.indexOf('|');
    const target = pipeIdx >= 0 ? inner.slice(0, pipeIdx).trim() : inner.trim();
    const display = pipeIdx >= 0 ? inner.slice(pipeIdx + 1).trim() : inner.trim();
    return `<a class="wikilink" data-target="${encodeURIComponent(target)}" href="#">${escapeHtml(display)}</a>`;
  });
}

const processor = unified()
  .use(remarkParse)
  .use(remarkGfm)
  .use(remarkRehype, { allowDangerousHtml: true })
  .use(rehypeRaw)
  .use(rehypeStringify);

export function renderMarkdown(content: string): string {
  const expanded = expandWikilinks(content);
  const result = processor.processSync(expanded);
  return String(result);
}
