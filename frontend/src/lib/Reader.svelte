<script lang="ts">
  import { renderMarkdown, extractHeadings } from './markdown.ts';
  import type { FileContent } from './tauri.ts';
  import Minimap from './Minimap.svelte';

  let { file, onNavigateWikilink }: { file: FileContent; onNavigateWikilink: (target: string) => void } = $props();

  let renderedHtml = $derived(file ? renderMarkdown(file.raw) : '');
  let headings = $derived(file ? extractHeadings(file.raw) : []);

  // Tags from frontmatter
  let tags = $derived(() => {
    if (!file?.frontmatter?.tags) return [];
    const t = file.frontmatter.tags;
    if (Array.isArray(t)) return t;
    if (typeof t === 'string') return t.split(',').map(s => s.trim());
    return [];
  });

  function handleClick(e: MouseEvent) {
    const link = (e.target as HTMLElement).closest('a.wikilink') as HTMLAnchorElement | null;
    if (link) {
      e.preventDefault();
      const href = link.getAttribute('href') || '';
      const prefix = '#wikilink/';
      if (href.startsWith(prefix)) {
        const target = decodeURIComponent(href.slice(prefix.length));
        onNavigateWikilink(target);
      }
    }
  }
</script>

<div class="reader-layout">
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

{#if headings.length > 1}
  <Minimap {headings} />
{/if}
</div>

<style>
  .reader-layout {
    display: flex;
    max-width: 60rem;
    margin: 0 auto;
  }

  .reader {
    max-width: 45rem;
    margin: 0 auto;
    padding: 3rem 2rem 6rem;
  }

  .frontmatter {
    margin-bottom: 2rem;
    padding-bottom: 1.5rem;
    border-bottom: 1px solid var(--border);
  }

  .page-title {
    font-family: var(--font-serif);
    font-size: var(--text-5xl);
    font-weight: 700;
    line-height: 1.2;
    letter-spacing: -0.02em;
    color: var(--text);
    margin-bottom: 0.75rem;
  }

  .meta-row {
    display: flex;
    flex-wrap: wrap;
    gap: 1rem;
    margin-bottom: 0.5rem;
  }

  .meta-item {
    font-size: var(--text-base);
  }

  .meta-key {
    color: var(--text-muted);
    text-transform: uppercase;
    font-size: var(--text-xs);
    font-weight: 600;
    letter-spacing: 0.05em;
    margin-right: 0.375rem;
  }

  .meta-value {
    color: var(--text-secondary);
  }

  .tags {
    display: flex;
    flex-wrap: wrap;
    gap: 0.375rem;
    margin-top: 0.5rem;
  }

  .tag {
    display: inline-block;
    padding: 0.125rem 0.625rem;
    border-radius: 0.75rem;
    background: var(--accent-light);
    color: var(--accent);
    font-size: var(--text-sm);
    font-weight: 500;
  }
</style>
