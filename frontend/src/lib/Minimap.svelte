<script lang="ts">
  import type { Heading } from './markdown.ts';

  let { headings }: { headings: Heading[] } = $props();

  function scrollTo(id: string) {
    const el = document.getElementById(id);
    if (el) {
      el.scrollIntoView({ behavior: 'smooth', block: 'start' });
    }
  }
</script>

{#if headings.length > 0}
  <nav class="minimap">
    {#each headings as heading}
      <button
        class="minimap-item depth-{heading.depth}"
        onclick={() => scrollTo(heading.id)}
        title={heading.text}
      >
        {heading.text}
      </button>
    {/each}
  </nav>
{/if}

<style>
  .minimap {
    position: sticky;
    top: 3rem;
    max-height: calc(100vh - 6rem);
    overflow-y: auto;
    width: 11.25rem;
    min-width: 11.25rem;
    padding: 0 1rem;
    flex-shrink: 0;
  }

  .minimap::-webkit-scrollbar {
    width: 0;
  }

  .minimap-item {
    display: block;
    width: 100%;
    text-align: left;
    padding: 0.1875rem 0;
    font-size: var(--text-xs);
    line-height: 1.4;
    color: var(--text-muted);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    transition: color 0.15s;
    border-left: 0.125rem solid transparent;
    padding-left: 0.5rem;
  }

  .minimap-item:hover {
    color: var(--accent);
    border-left-color: var(--accent);
  }

  .depth-1 {
    font-weight: 600;
    font-size: var(--text-sm);
    color: var(--text-secondary);
    margin-top: 0.25rem;
  }

  .depth-2 {
    font-weight: 500;
    color: var(--text-secondary);
  }

  .depth-3 {
    padding-left: 1rem;
  }

  .depth-4,
  .depth-5,
  .depth-6 {
    padding-left: 1.5rem;
    font-size: var(--text-2xs);
  }
</style>
