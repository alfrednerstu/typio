import { unified } from 'unified';
import remarkParse from 'remark-parse';
import remarkGfm from 'remark-gfm';
import remarkWikiLink from 'remark-wiki-link';
import remarkRehype from 'remark-rehype';
import rehypeStringify from 'rehype-stringify';

const processor = unified()
  .use(remarkParse)
  .use(remarkWikiLink, {
    wikiLinkClassName: 'wikilink',
    hrefTemplate: (permalink: string) => `#wikilink/${encodeURIComponent(permalink)}`,
    pageResolver: (name: string) => [name],
  })
  .use(remarkGfm)
  .use(remarkRehype)
  .use(rehypeStringify);

export function renderMarkdown(content: string): string {
  const result = processor.processSync(content);
  return String(result);
}
