# mdbook-pele

[mdBook](https://rust-lang.github.io/mdBook/) preprocessor that renders [Mermaid](https://mermaid.js.org) diagrams with **[Pele](https://pele.run)**.

Pele is a fast, small (~230 kB gzipped), zero-dependency Mermaid → SVG renderer that themes via CSS variables. It supports every diagram type in Mermaid 12.1.0 and is used by Obsidian.

Compared to the classic `mdbook-mermaid` (client-side mermaid.js):

| | Pele | mermaid.js |
|---|---|---|
| Bundle size (gzip) | ~230 kB | ~1.6 MB |
| Render time | 0.05–0.3 ms | 6–82 ms |
| Dependencies | 0 | 23 |
| Theming | CSS variables | theme config |
| Responsive layout | adapts layout | scales |

## Install

```bash
cargo install --path .          # from this repo
# or later: cargo install mdbook-pele
```

Then, in your book directory:

```bash
mdbook-pele install
```

This will:

1. Write `pele.global.js` and `pele-init.js` next to `book.toml`
2. Add to `book.toml`:

```toml
[preprocessor.pele]
command = "mdbook-pele"

[output.html]
additional-js = ["pele.global.js", "pele-init.js"]
```

## Usage

Write ordinary Mermaid fenced blocks:

````markdown
```mermaid
flowchart TD
  A[Mermaid text] --> B{Flowchart?}
  B -- Yes --> C[Pele]
  B -- No --> D[Mermaid]
  C --> E([SVG])
  D --> E
```
````

Build or serve as usual:

```bash
mdbook serve
```

The preprocessor turns the block into:

```html
<pre class="mermaid">...</pre>
```

and the included `pele-init.js` calls `pele.mount()` on each of them.

## Theming

Pele uses CSS variables. Override them in your book’s theme (e.g. `theme/css/chrome.css` or `additional-css`) to match light/dark modes:

```css
/* Light (mdBook default) */
.pele,
pre.mermaid {
  --pele-bg: transparent;
  --pele-fg: var(--fg, #1a1a1a);
  --pele-muted: var(--fg, #6f6e69);
  --pele-line: var(--fg, #1a1a1a);
  --pele-surface: var(--bg, #f2f0e5);
  --pele-surface-alt: #e6e4d9;
  --pele-border: #cecdc3;
  --pele-accent: #205ea6;
  --pele-font: var(--font, sans-serif);
}

/* Dark themes (navy, coal, ayu, etc.) */
.ayu .pele, .coal .pele, .navy .pele,
.ayu pre.mermaid, .coal pre.mermaid, .navy pre.mermaid {
  --pele-bg: transparent;
  --pele-fg: var(--fg);
  --pele-muted: #a0a0a0;
  --pele-line: var(--fg);
  --pele-surface: #2a2a2a;
  --pele-surface-alt: #333;
  --pele-border: #555;
  --pele-accent: #7aa2f7;
}
```

See [pele.run/theming](https://pele.run/theming) for the full variable list.

## How it works

1. **Preprocessor (Rust)** – walks every chapter, finds fenced `mermaid` code blocks with `pulldown-cmark`, HTML-escapes the source, and replaces the block with `<pre class="mermaid">…</pre>`.
2. **Assets** – `pele.global.js` (Pele 1.6.0 UMD build) + `pele-init.js` (mounts every `pre.mermaid` on `DOMContentLoaded`).
3. **Runtime** – Pele renders each diagram to SVG synchronously, adapts layout to container width, and supports zoom/pan when the diagram is larger than its box.

## Migrating from mdbook-mermaid

1. Remove the old preprocessor and JS assets.
2. Run `mdbook-pele install`.
3. Keep your `` ```mermaid `` blocks — no syntax changes.

## Development

```bash
cargo test
cargo build --release
```

To refresh the bundled Pele build:

```bash
curl -sL "https://cdn.jsdelivr.net/npm/pele@1/dist/pele.global.js" \
  -o src/bin/assets/pele.global.js
```

## License

MIT. Pele itself is MIT (https://github.com/obsidianmd/pele).
