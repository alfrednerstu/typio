<script lang="ts">
  import { renderMarkdown } from './markdown.ts';
  import type { FileContent } from './tauri.ts';

  let { file, onNavigateWikilink }: { file: FileContent; onNavigateWikilink: (target: string) => void } = $props();

  let renderedHtml = $derived(file ? renderMarkdown(file.raw) : '');

  // Tags from frontmatter
  let tags = $derived(() => {
    if (!file?.frontmatter?.tags) return [];
    const t = file.frontmatter.tags;
    if (Array.isArray(t)) return t;
    if (typeof t === 'string') return t.split(',').map(s => s.trim());
    return [];
  });

  function handleClick(e) {
    const link = e.target.closest('a.wikilink');
    if (link) {
      e.preventDefault();
      const target = decodeURIComponent(link.dataset.target);
      onNavigateWikilink(target);
    }
  }
</script>

<article class="reader">
  {#if file.frontmatter && Object.keys(file.frontmatter).length > 0}
    <div class="frontmatter">
      {#if file.frontmatter.title}
        <h1 class="page-title">{file.frontmatter.title}</h1>
      {/if}
      <div class="meta-row">
        {#each Object.entries(file.frontmatter) as [key, value]}
          {#if key !== 'title' && key !== 'tags'}
            <span class="meta-item">
              <span class="meta-key">{key}</span>
              <span class="meta-value">{typeof value === 'object' ? JSON.stringify(value) : value}</span>
            </span>
          {/if}
        {/each}
      </div>
      {#if tags().length > 0}
        <div class="tags">
          {#each tags() as tag}
            <span class="tag">{tag}</span>
          {/each}
        </div>
      {/if}
    </div>
  {/if}

  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <div class="prose" onclick={handleClick}>
    {@html renderedHtml}
  </div>
</article>

<style>
  .reader {
    max-width: 720px;
    margin: 0 auto;
    padding: 48px 32px 96px;
  }

  .frontmatter {
    margin-bottom: 32px;
    padding-bottom: 24px;
    border-bottom: 1px solid var(--border);
  }

  .page-title {
    font-family: var(--font-serif);
    font-size: 36px;
    font-weight: 700;
    line-height: 1.2;
    letter-spacing: -0.02em;
    color: var(--text);
    margin-bottom: 12px;
  }

  .meta-row {
    display: flex;
    flex-wrap: wrap;
    gap: 16px;
    margin-bottom: 8px;
  }

  .meta-item {
    font-size: 13px;
  }

  .meta-key {
    color: var(--text-muted);
    text-transform: uppercase;
    font-size: 11px;
    font-weight: 600;
    letter-spacing: 0.05em;
    margin-right: 6px;
  }

  .meta-value {
    color: var(--text-secondary);
  }

  .tags {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
    margin-top: 8px;
  }

  .tag {
    display: inline-block;
    padding: 2px 10px;
    border-radius: 12px;
    background: var(--accent-light);
    color: var(--accent);
    font-size: 12px;
    font-weight: 500;
  }

  /* Prose styles for rendered markdown */
  .prose {
    font-family: var(--font-serif);
    font-size: 17px;
    line-height: 1.7;
    color: var(--text);
  }

  .prose :global(h1) {
    font-family: var(--font-serif);
    font-size: 32px;
    font-weight: 700;
    line-height: 1.2;
    margin: 48px 0 16px;
    letter-spacing: -0.02em;
  }

  .prose :global(h2) {
    font-family: var(--font-serif);
    font-size: 24px;
    font-weight: 600;
    line-height: 1.3;
    margin: 40px 0 12px;
    letter-spacing: -0.01em;
  }

  .prose :global(h3) {
    font-family: var(--font-sans);
    font-size: 18px;
    font-weight: 600;
    line-height: 1.4;
    margin: 32px 0 8px;
  }

  .prose :global(h4),
  .prose :global(h5),
  .prose :global(h6) {
    font-family: var(--font-sans);
    font-size: 15px;
    font-weight: 600;
    line-height: 1.4;
    margin: 24px 0 8px;
    color: var(--text-secondary);
  }

  .prose :global(p) {
    margin: 0 0 16px;
  }

  .prose :global(a) {
    color: var(--link);
    text-decoration: underline;
    text-decoration-color: rgba(109, 40, 217, 0.3);
    text-underline-offset: 2px;
    transition: text-decoration-color 0.15s;
  }

  .prose :global(a:hover) {
    text-decoration-color: var(--link);
  }

  .prose :global(a.wikilink) {
    color: var(--accent);
    text-decoration: none;
    background: var(--accent-light);
    padding: 1px 6px;
    border-radius: 4px;
    font-family: var(--font-sans);
    font-size: 15px;
    transition: background 0.15s;
  }

  .prose :global(a.wikilink:hover) {
    background: #ddd6fe;
  }

  .prose :global(ul),
  .prose :global(ol) {
    margin: 0 0 16px;
    padding-left: 24px;
  }

  .prose :global(li) {
    margin-bottom: 4px;
  }

  .prose :global(blockquote) {
    border-left: 3px solid var(--accent);
    margin: 0 0 16px;
    padding: 8px 16px;
    color: var(--text-secondary);
    background: var(--accent-light);
    border-radius: 0 6px 6px 0;
  }

  .prose :global(code) {
    font-family: var(--font-mono);
    font-size: 14px;
    background: #f0eeeb;
    padding: 2px 6px;
    border-radius: 4px;
    color: var(--text);
  }

  .prose :global(pre) {
    background: #1e1e2e;
    color: #cdd6f4;
    border-radius: 8px;
    padding: 16px;
    margin: 0 0 16px;
    overflow-x: auto;
    font-size: 14px;
    line-height: 1.5;
  }

  .prose :global(pre code) {
    background: none;
    padding: 0;
    border-radius: 0;
    color: inherit;
    font-size: inherit;
  }

  .prose :global(hr) {
    border: none;
    height: 1px;
    background: var(--border);
    margin: 32px 0;
  }

  .prose :global(table) {
    width: 100%;
    border-collapse: collapse;
    margin: 0 0 16px;
    font-family: var(--font-sans);
    font-size: 14px;
  }

  .prose :global(th),
  .prose :global(td) {
    border: 1px solid var(--border);
    padding: 8px 12px;
    text-align: left;
  }

  .prose :global(th) {
    background: var(--bg-sidebar);
    font-weight: 600;
  }

  .prose :global(img) {
    max-width: 100%;
    border-radius: 8px;
    margin: 8px 0;
  }
</style>
