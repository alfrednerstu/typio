import { unified } from 'unified';
import remarkParse from 'remark-parse';
import remarkFrontmatter from 'remark-frontmatter';
import remarkGfm from 'remark-gfm';
import remarkWikiLink from 'remark-wiki-link';
import remarkRehype from 'remark-rehype';
import rehypeHighlight from 'rehype-highlight';
import rehypeStringify from 'rehype-stringify';

export interface Heading {
  depth: number;
  text: string;
  id: string;
}

const processor = unified()
  .use(remarkParse)
  .use(remarkFrontmatter, ['yaml'])
  .use(remarkWikiLink, {
    wikiLinkClassName: 'wikilink',
    hrefTemplate: (permalink: string) => `#wikilink/${encodeURIComponent(permalink)}`,
    pageResolver: (name: string) => [name],
  })
  .use(remarkGfm)
  .use(remarkRehype)
  .use(rehypeHighlight, { detect: true, ignoreMissing: true })
  .use(rehypeStringify);

export function renderMarkdown(content: string): string {
  const result = processor.processSync(content);
  return String(result);
}

/** Extract headings from raw markdown for the minimap. */
export function extractHeadings(content: string): Heading[] {
  const headings: Heading[] = [];
  // Skip frontmatter
  let body = content;
  if (body.trimStart().startsWith('---')) {
    const end = body.indexOf('\n---', 3);
    if (end !== -1) {
      body = body.slice(end + 4);
    }
  }

  for (const line of body.split('\n')) {
    const match = line.match(/^(#{1,6})\s+(.+)$/);
    if (match) {
      const text = match[2].trim();
      const id = text
        .toLowerCase()
        .replace(/[^\w\s-]/g, '')
        .replace(/\s+/g, '-');
      headings.push({
        depth: match[1].length,
        text,
        id,
      });
    }
  }
  return headings;
}
