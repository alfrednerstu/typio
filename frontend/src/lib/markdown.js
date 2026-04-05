import { Marked } from 'marked';
import { gfmHeadingId } from 'marked-gfm-heading-id';

// Custom extension for wikilinks
const wikilinkExtension = {
  name: 'wikilink',
  level: 'inline',
  start(src) {
    return src.indexOf('[[');
  },
  tokenizer(src) {
    const match = src.match(/^\[\[([^\]]+)\]\]/);
    if (match) {
      const inner = match[1];
      const pipeIdx = inner.indexOf('|');
      const target = pipeIdx >= 0 ? inner.slice(0, pipeIdx).trim() : inner.trim();
      const display = pipeIdx >= 0 ? inner.slice(pipeIdx + 1).trim() : inner.trim();
      return {
        type: 'wikilink',
        raw: match[0],
        target,
        display,
      };
    }
  },
  renderer(token) {
    return `<a class="wikilink" data-target="${encodeURIComponent(token.target)}" href="#">${escapeHtml(token.display)}</a>`;
  },
};

function escapeHtml(str) {
  return str
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;');
}

const marked = new Marked();
marked.use(gfmHeadingId());
marked.use({ extensions: [wikilinkExtension] });

export function renderMarkdown(content) {
  return marked.parse(content);
}
