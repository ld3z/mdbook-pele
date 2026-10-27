# Troubleshooting

## Diagrams show as plain code blocks

**Symptom:** the fenced block renders as a normal code block in the HTML.

Checklist:

- Is `[preprocessor.pele]` present in `book.toml`? (Re-run `mdbook-pele install`.)
- Is the info string exactly `mermaid` (lowercase)? The preprocessor matches the fenced block info string literally.
- Did you rebuild after installing? A stale `book/` output directory can hide the change — `mdbook build` regenerates it.

## "Pele is not loaded" in the console

`pele-init.js` ran but `pele.global.js` did not load. Check that:

- `additional-js = ["pele.global.js", "pele-init.js"]` is in `[output.html]`.
- Both files exist next to `book.toml`.
- Your book uses a custom template that includes mdBook's global JS injection; the assets are registered through mdBook's standard mechanism, so custom `index.hbs` overrides may need updating.

## "Pele failed to render diagram" in the console

The diagram syntax is rejected by Pele's parser. The error message includes the line, column, and a snippet of the offending text.

- Compare the first line with the keywords in [Supported Diagram Types](./diagram-types.md); detection is done on the first line's first token.
- Beta diagram types require their explicit keyword (`venn-beta`, not `venn`).
- Check YAML front matter inside diagrams — invalid YAML produces a syntax error before parsing starts.

## Diagram colors look wrong in one theme

- Verify `pele.css` is in `additional-css` — it is what maps Pele variables onto the active mdBook theme.
- If you overrode variables, remember the fallback chain: your override wins only where you set it; otherwise it falls back to `var(--bg)`, `var(--fg)`, and friends.
- If you use a dark theme with a custom page background, also set `--pele-bg` to the same color — see [Theming](./theming.md#notes-on-backgrounds).

## Diagram is larger than its container

Large diagrams scroll and support pan/zoom automatically. If you want different constraint behavior, target `pre.mermaid` in your own CSS (`max-width`, centering, and so on) — Pele adapts the layout to the width it is given.

## Version warning at build time

```
Warning: The pele plugin was built against version X of mdbook, but we're being called from version Y
```

Re-install `mdbook-pele` (or update mdBook) so versions line up. This is a warning only; the preprocessor still runs.

## Anything else

- Search the [GitHub issues](https://github.com/ld3z/mdbook-pele/issues) or open one.
- General Mermaid syntax questions are best asked against the [Mermaid docs](https://mermaid.js.org/intro/) — Pele aims for Mermaid 12.1.0 compatibility.
