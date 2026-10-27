# Theming

## Following the book theme out of the box

`pele.css` maps Pele's CSS variables onto mdBook's own theme variables, so diagrams follow whatever theme the reader picked — Light, Rust, Coal, Navy, or Ayu — with no extra setup:

```css
pre.mermaid,
.pele {
  --pele-bg: var(--bg);
  --pele-fg: var(--fg);
  --pele-line: var(--fg);
  --pele-muted: var(--fg);
  --pele-surface: var(--quote-bg);
  --pele-surface-alt: var(--quote-border);
  --pele-border: var(--theme-popup-border);
  --pele-accent: var(--links);
}
```

It also resets mdBook's default `<pre>` styling (background, border, monospace font) inside diagrams so Pele measures labels with the book's font.

The `install` command prepends `pele.css` to `additional-css`, so any stylesheet you add to `additional-css` after it (or any override in `theme/css/chrome.css`) takes precedence over its mappings.

## Overriding individual colors

To change how diagrams look in every theme, override a variable in your book CSS:

```css
pre.mermaid {
  --pele-accent: #7aa2f7;
}
```

To scope an override to one theme, hang it off the theme class mdBook puts on `<html>`:

```css
.coal pre.mermaid,
.navy pre.mermaid {
  --pele-surface: #1d2129;
}
```

If a variable is not overridden it falls back to the mdBook variable above, so you only need to set what you want to change.

## Full variable reference

See [pele.run/theming](https://pele.run/theming) for the complete list of Pele theme variables (`--pele-node`, `--pele-edge`, `--pele-cN` palette slots, and so on) and what each controls.

## Notes on backgrounds

`--pele-bg` must be an actual page color (not `white` or `transparent`): flowchart edge labels draw a knockout rectangle with it so Yes/No labels sit cleanly on top of the arrows. The default mapping uses `var(--bg)`, which is correct per theme; if you set a custom page background for a theme, mirror it in `--pele-bg`.
