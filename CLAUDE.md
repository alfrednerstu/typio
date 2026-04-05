# CLAUDE.md

## Project

Typio is a fast, local markdown knowledge base browser and editor. Built with Tauri + Svelte 5. Domain: typio.app. Open source, MIT license.

## Stack

- Tauri (Rust backend for file I/O, file watching, indexing)
- Svelte 5 (UI, markdown rendering)
- CodeMirror 6 (markdown editing, v0.2)
- TypeScript
- Markdown parsing: mdast/unified ecosystem (remark-parse, remark-rehype, rehype-stringify)
- Wikilink parsing: support `[[wikilinks]]` syntax

## What it does

User opens a folder of `.md` files. Typio reads the file tree, parses all markdown files including YAML frontmatter and wikilinks, and presents a navigable, interlinked knowledge base. The folder structure provides navigation. Backlinks and cross-references provide the graph.

## Sidebar

The left sidebar shows the contents of the opened folder.

### Sort order

1. `index.md` pinned at top (displayed as "Index")
1. Folders A–Z
1. Files A–Z

### Display rules

- File names are displayed without the `.md` extension
- Names are title-cased from the filename (e.g. `log.md` → "Log", `CLAUDE.md` → "Claude")
- Folders show as expandable sections
- The sidebar reflects the actual file system — no renaming, no imposed structure

### Convention detection

The app does not hardcode menu sections. It discovers structure from what exists in the folder:

- If `index.md` exists, pin it to the top of the sidebar
- All other files and folders are shown in the sort order above
- No files are hidden unless they start with `.` (dotfiles)

### User sort (later)

Users can drag to reorder sidebar items. Persist custom order in a `.typio` config file in the folder root. This is not needed for v0.1.

## Main content area

### Reader view (default)

- Render markdown as beautifully typeset HTML
- Parse and render YAML frontmatter as a metadata header
- Wikilinks (`[[page name]]`) are clickable and navigate to the linked file
- Syntax highlighting for code blocks
- Support images with relative paths

### Backlinks panel

- Below the rendered content (or in a right sidebar), show a list of all pages that link to the current page
- Each backlink shows the linking page name and the surrounding context

### Editor view (v0.2)

- Toggle between read and edit mode
- Use CodeMirror 6 for the editing surface
- Save on Cmd+S, write back to disk

## Search

- Cmd+K opens a command palette for quick file switching
- Full-text search across all `.md` files in the folder
- Search results show filename and matching context

## File watching

- Watch the opened folder for changes using Tauri's fs watch API
- When a file is added, removed, or modified on disk, update the sidebar and content immediately
- This is critical — LLM agents (Claude Code, Codex) modify files externally and the app must reflect changes in real time

## Design principles

- Read-first. The primary use case is browsing, not writing.
- Respect existing structure. Don't impose folder hierarchy or naming conventions.
- Fast. Instant file switching, no loading states for local files.
- Minimal. No plugin system, no sync, no publishing.
- Beautiful typography. This is a reading app — text rendering matters. Use good defaults: system font stack, generous line height, comfortable measure (60–80 characters).

## v0.1 scope

1. Open a folder
1. Sidebar with file tree (sort order as specified above)
1. Markdown reader with rendered view
1. Wikilink parsing and navigation
1. Backlinks panel
1. Frontmatter display
1. Full-text search
1. File watching for live reload

## v0.2 scope

- Editor mode with CodeMirror 6
- Graph view of page connections
- Tag filtering from frontmatter
- Command palette (Cmd+K)
- Drag-to-reorder sidebar with `.typio` persistence
