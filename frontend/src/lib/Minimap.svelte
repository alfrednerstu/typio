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
    top: 48px;
    max-height: calc(100vh - 96px);
    overflow-y: auto;
    width: 180px;
    min-width: 180px;
    padding: 0 16px;
    flex-shrink: 0;
  }

  .minimap::-webkit-scrollbar {
    width: 0;
  }

  .minimap-item {
    display: block;
    width: 100%;
    text-align: left;
    padding: 3px 0;
    font-size: 11px;
    line-height: 1.4;
    color: var(--text-muted);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    transition: color 0.15s;
    border-left: 2px solid transparent;
    padding-left: 8px;
  }

  .minimap-item:hover {
    color: var(--accent);
    border-left-color: var(--accent);
  }

  .depth-1 {
    font-weight: 600;
    font-size: 12px;
    color: var(--text-secondary);
    margin-top: 4px;
  }

  .depth-2 {
    font-weight: 500;
    color: var(--text-secondary);
  }

  .depth-3 {
    padding-left: 16px;
  }

  .depth-4,
  .depth-5,
  .depth-6 {
    padding-left: 24px;
    font-size: 10px;
  }
</style>
