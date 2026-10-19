//! mdBook preprocessor binary for Pele (Mermaid → SVG).

use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};
use mdbook_pele::Pele;
use mdbook_preprocessor::{Preprocessor, parse_input};
use semver::{Version, VersionReq};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process;

const PELE_JS: &str = include_str!("bin/assets/pele.global.js");
const PELE_INIT_JS: &str = include_str!("bin/assets/pele-init.js");

#[derive(Parser)]
#[command(
    name = "mdbook-pele",
    about = "mdBook preprocessor for Pele Mermaid diagrams",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Install Pele assets and configure book.toml for a book
    Install {
        /// Path to the book directory (default: current directory)
        #[arg(default_value = ".")]
        dir: PathBuf,
    },
    /// Check whether this preprocessor supports a renderer (used by mdBook)
    Supports { renderer: String },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Install { dir }) => {
            if let Err(e) = install(&dir) {
                eprintln!("error: {e:#}");
                process::exit(1);
            }
        }
        Some(Commands::Supports { renderer }) => {
            let ok = Pele.supports_renderer(&renderer).unwrap_or(false);
            if ok {
                process::exit(0);
            } else {
                process::exit(1);
            }
        }
        None => {
            // Default: act as preprocessor (mdBook pipes JSON on stdin)
            if let Err(e) = handle_preprocessing() {
                eprintln!("{e}");
                process::exit(1);
            }
        }
    }
}

fn handle_preprocessing() -> Result<()> {
    let (ctx, book) = parse_input(io::stdin())?;

    let book_version = Version::parse(&ctx.mdbook_version)?;
    let version_req = VersionReq::parse(mdbook_preprocessor::MDBOOK_VERSION)?;

    if !version_req.matches(&book_version) {
        eprintln!(
            "Warning: The {} plugin was built against version {} of mdbook, \
             but we're being called from version {}",
            Pele.name(),
            mdbook_preprocessor::MDBOOK_VERSION,
            ctx.mdbook_version
        );
    }

    let processed = Pele.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed)?;
    Ok(())
}

fn install(book_dir: &Path) -> Result<()> {
    let book_toml = book_dir.join("book.toml");
    if !book_toml.exists() {
        bail!(
            "No book.toml found in {}. Run this from an mdBook project directory.",
            book_dir.display()
        );
    }

    // Copy assets next to book.toml (same place mdbook-mermaid uses)
    let assets = [("pele.global.js", PELE_JS), ("pele-init.js", PELE_INIT_JS)];
    for (name, content) in assets {
        let dest = book_dir.join(name);
        fs::write(&dest, content).with_context(|| format!("failed to write {}", dest.display()))?;
        eprintln!("Wrote {}", dest.display());
    }

    // Update book.toml
    let toml_str = fs::read_to_string(&book_toml)
        .with_context(|| format!("failed to read {}", book_toml.display()))?;
    let mut doc = toml_str
        .parse::<toml_edit::DocumentMut>()
        .context("failed to parse book.toml")?;

    // [preprocessor.pele]
    if doc
        .get("preprocessor")
        .and_then(|t| t.get("pele"))
        .is_none()
    {
        let preprocessor = doc
            .entry("preprocessor")
            .or_insert(toml_edit::Item::Table(toml_edit::Table::new()));
        let table = preprocessor.as_table_mut().unwrap();
        let mut pele = toml_edit::Table::new();
        pele["command"] = toml_edit::value("mdbook-pele");
        table["pele"] = toml_edit::Item::Table(pele);
        eprintln!("Added [preprocessor.pele] to book.toml");
    } else {
        eprintln!("[preprocessor.pele] already present");
    }

    // [output.html] additional-js
    let output = doc
        .entry("output")
        .or_insert(toml_edit::Item::Table(toml_edit::Table::new()));
    let output_table = output.as_table_mut().unwrap();
    let html = output_table
        .entry("html")
        .or_insert(toml_edit::Item::Table(toml_edit::Table::new()));
    let html_table = html.as_table_mut().unwrap();

    let needed = ["pele.global.js", "pele-init.js"];
    match html_table.get_mut("additional-js") {
        Some(item) => {
            if let Some(arr) = item.as_array_mut() {
                for js in needed {
                    let already = arr.iter().any(|v| v.as_str() == Some(js));
                    if !already {
                        arr.push(js);
                        eprintln!("Added {js} to additional-js");
                    }
                }
            } else {
                // Replace non-array value
                let mut arr = toml_edit::Array::new();
                for js in needed {
                    arr.push(js);
                }
                *item = toml_edit::value(arr);
                eprintln!("Set additional-js = [\"pele.global.js\", \"pele-init.js\"]");
            }
        }
        None => {
            let mut arr = toml_edit::Array::new();
            for js in needed {
                arr.push(js);
            }
            html_table["additional-js"] = toml_edit::value(arr);
            eprintln!("Added additional-js = [\"pele.global.js\", \"pele-init.js\"]");
        }
    }

    fs::write(&book_toml, doc.to_string())
        .with_context(|| format!("failed to write {}", book_toml.display()))?;
    eprintln!("Updated {}", book_toml.display());
    eprintln!("Done. Run `mdbook build` (or `mdbook serve`) to see Pele diagrams.");
    Ok(())
}
