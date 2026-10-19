//! mdBook preprocessor that turns ````mermaid` code blocks into Pele-rendered diagrams.
//!
//! Pele (https://pele.run) is a fast, small, CSS-variable-themed Mermaid renderer.
//! This preprocessor replaces fenced `mermaid` blocks with `<pre class="mermaid">`
//! elements and expects the Pele JS assets to be included in the book.

use mdbook_preprocessor::book::{Book, BookItem, Chapter};
use mdbook_preprocessor::errors::Result;
use mdbook_preprocessor::{Preprocessor, PreprocessorContext};
use pulldown_cmark::{CodeBlockKind::*, Event, Options, Parser, Tag, TagEnd};

/// The Pele preprocessor.
pub struct Pele;

impl Preprocessor for Pele {
    fn name(&self) -> &str {
        "pele"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book> {
        let mut res = None;
        book.for_each_mut(|item: &mut BookItem| {
            if let Some(Err(_)) = res {
                return;
            }

            if let BookItem::Chapter(ref mut chapter) = *item {
                res = Some(add_pele(chapter).map(|md| {
                    chapter.content = md;
                }));
            }
        });

        res.unwrap_or(Ok(())).map(|_| book)
    }

    fn supports_renderer(&self, renderer: &str) -> Result<bool> {
        Ok(renderer == "html")
    }
}

fn escape_html(s: &str) -> String {
    let mut output = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '<' => output.push_str("&lt;"),
            '>' => output.push_str("&gt;"),
            '"' => output.push_str("&quot;"),
            '&' => output.push_str("&amp;"),
            _ => output.push(c),
        }
    }
    output
}

/// Replace ````mermaid` (and ````mermaid`) fenced code blocks with
/// `<pre class="mermaid">...</pre>` so Pele can mount them.
fn process_content(content: &str) -> Result<String> {
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_FOOTNOTES);
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    opts.insert(Options::ENABLE_TASKLISTS);

    let mut in_mermaid_block = false;
    let mut code_span = 0..0;
    let mut start_new_code_span = true;
    let mut mermaid_blocks = Vec::new();

    let events = Parser::new_ext(content, opts);
    for (e, span) in events.into_offset_iter() {
        if let Event::Start(Tag::CodeBlock(Fenced(code))) = e.clone() {
            if code.as_ref() == "mermaid" {
                in_mermaid_block = true;
            }
            continue;
        }

        if !in_mermaid_block {
            continue;
        }

        // Code blocks can span multiple Text events.
        if let Event::Text(_) = e {
            if start_new_code_span {
                code_span = span;
                start_new_code_span = false;
            } else {
                code_span = code_span.start..span.end;
            }
            continue;
        }

        if let Event::End(TagEnd::CodeBlock) = e {
            in_mermaid_block = false;

            let mermaid_content = &content[code_span.clone()];
            let mermaid_content = escape_html(mermaid_content).replace("\r\n", "\n");
            let mermaid_code = format!("<pre class=\"mermaid\">{}</pre>\n\n", mermaid_content);
            mermaid_blocks.push((span, mermaid_code));
            start_new_code_span = true;
        }
    }

    let mut content = content.to_string();
    // Replace from the end so earlier offsets stay valid.
    for (span, block) in mermaid_blocks.iter().rev() {
        let pre = &content[0..span.start];
        let post = &content[span.end..];
        content = format!("{}\n{}{}", pre, block, post);
    }
    Ok(content)
}

fn add_pele(chapter: &mut Chapter) -> Result<String> {
    process_content(&chapter.content)
}

#[cfg(test)]
mod tests {
    use super::process_content;
    use pretty_assertions::assert_eq;

    #[test]
    fn adds_mermaid() {
        let content = r#"# Chapter

```mermaid
graph TD
A --> B
```

Text
"#;

        let expected = r#"# Chapter


<pre class="mermaid">graph TD
A --&gt; B
</pre>



Text
"#;

        assert_eq!(expected, process_content(content).unwrap());
    }

    #[test]
    fn leaves_tables_untouched() {
        let content = r#"# Heading

| Head 1 | Head 2 |
|--------|--------|
| Row 1  | Row 2  |
"#;

        let expected = r#"# Heading

| Head 1 | Head 2 |
|--------|--------|
| Row 1  | Row 2  |
"#;

        assert_eq!(expected, process_content(content).unwrap());
    }

    #[test]
    fn escape_in_mermaid_block() {
        let content = r#"
```mermaid
classDiagram
    class PingUploader {
        <<interface>>
        +Upload() UploadResult
    }
```

hello
"#;

        let expected = r#"

<pre class="mermaid">classDiagram
    class PingUploader {
        &lt;&lt;interface&gt;&gt;
        +Upload() UploadResult
    }
</pre>



hello
"#;

        assert_eq!(expected, process_content(content).unwrap());
    }

    #[test]
    fn more_backticks() {
        let content = r#"# Chapter

````mermaid
graph TD
A --> B
````

Text
"#;

        let expected = r#"# Chapter


<pre class="mermaid">graph TD
A --&gt; B
</pre>



Text
"#;

        assert_eq!(expected, process_content(content).unwrap());
    }
}
