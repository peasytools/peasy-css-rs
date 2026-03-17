# peasy-css

[![crates.io](https://img.shields.io/crates/v/peasy-css)](https://crates.io/crates/peasy-css)
[![docs.rs](https://docs.rs/peasy-css/badge.svg)](https://docs.rs/peasy-css)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

Async Rust client for the [PeasyCSS](https://peasycss.com) API — minify, format, and analyze CSS with gradient, shadow, flexbox, and grid generators. Built with reqwest, serde, and tokio.

Built from [PeasyCSS](https://peasycss.com), a free online CSS toolkit with tools for minifying, formatting, analyzing, and generating CSS code including gradients, shadows, and layouts.

> **Try the interactive tools at [peasycss.com](https://peasycss.com)** — [CSS Tools](https://peasycss.com/), [CSS Glossary](https://peasycss.com/glossary/), [CSS Guides](https://peasycss.com/guides/)

## Install

```toml
[dependencies]
peasy-css = "0.2.0"
tokio = { version = "1", features = ["full"] }
```

Or via cargo:

```bash
cargo add peasy-css
```

## Quick Start

```rust
use peasy_css::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    // List available CSS tools
    let tools = client.list_tools(&Default::default()).await?;
    for tool in &tools.results {
        println!("{}: {}", tool.name, tool.description);
    }

    Ok(())
}
```

## API Client

The client wraps the [PeasyCSS REST API](https://peasycss.com/developers/) with strongly-typed Rust structs using serde deserialization.

```rust
use peasy_css::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    // Or with a custom base URL:
    // let client = Client::with_base_url("https://custom.example.com");

    // List tools with filters
    let opts = peasy_css::ListOptions {
        page: Some(1),
        limit: Some(10),
        search: Some("minify".into()),
        ..Default::default()
    };
    let tools = client.list_tools(&opts).await?;

    // Get a specific tool
    let tool = client.get_tool("css-minify").await?;
    println!("{}: {}", tool.name, tool.description);

    // Search across all content
    let results = client.search("minify", Some(20)).await?;
    println!("Found {} tools", results.results.tools.len());

    // Browse the glossary
    let glossary = client.list_glossary(&peasy_css::ListOptions {
        search: Some("flexbox".into()),
        ..Default::default()
    }).await?;
    for term in &glossary.results {
        println!("{}: {}", term.term, term.definition);
    }

    // Discover guides
    let guides = client.list_guides(&peasy_css::ListGuidesOptions {
        category: Some("css".into()),
        ..Default::default()
    }).await?;
    for guide in &guides.results {
        println!("{} ({})", guide.title, guide.audience_level);
    }

    // List format conversions
    let conversions = client.list_conversions(&peasy_css::ListConversionsOptions {
        source: Some("css".into()),
        ..Default::default()
    }).await?;

    Ok(())
}
```

### Available Methods

| Method | Description |
|--------|-------------|
| `list_tools(&opts)` | List tools (paginated, filterable) |
| `get_tool(slug)` | Get tool by slug |
| `list_categories(&opts)` | List tool categories |
| `list_formats(&opts)` | List file formats |
| `get_format(slug)` | Get format by slug |
| `list_conversions(&opts)` | List format conversions |
| `list_glossary(&opts)` | List glossary terms |
| `get_glossary_term(slug)` | Get glossary term |
| `list_guides(&opts)` | List guides |
| `get_guide(slug)` | Get guide by slug |
| `list_use_cases(&opts)` | List use cases |
| `search(query, limit)` | Search across all content |
| `list_sites()` | List Peasy sites |
| `openapi_spec()` | Get OpenAPI specification |

Full API documentation at [peasycss.com/developers/](https://peasycss.com/developers/).
OpenAPI 3.1.0 spec: [peasycss.com/api/openapi.json](https://peasycss.com/api/openapi.json).

## Learn More

- **Tools**: [CSS Minify](https://peasycss.com/css/css-minify/) · [CSS Beautify](https://peasycss.com/css/css-beautify/) · [CSS Gradient Generator](https://peasycss.com/css/css-gradient/) · [All Tools](https://peasycss.com/)
- **Guides**: [CSS Units Explained](https://peasycss.com/guides/css-units-explained/) · [CSS Grid vs Flexbox](https://peasycss.com/guides/css-grid-vs-flexbox-when-to-use-each/) · [All Guides](https://peasycss.com/guides/)
- **Glossary**: [BEM](https://peasycss.com/glossary/bem/) · [Box Model](https://peasycss.com/glossary/box-model/) · [Cascade](https://peasycss.com/glossary/cascade/) · [All Terms](https://peasycss.com/glossary/)
- **Formats**: [CSS](https://peasycss.com/formats/css/) · [SVG](https://peasycss.com/formats/svg/) · [All Formats](https://peasycss.com/formats/)
- **API**: [REST API Docs](https://peasycss.com/developers/) · [OpenAPI Spec](https://peasycss.com/api/openapi.json)

## Also Available

| Language | Package | Install |
|----------|---------|---------|
| **Python** | [peasy-css](https://pypi.org/project/peasy-css/) | `pip install "peasy-css[all]"` |
| **TypeScript** | [peasy-css](https://www.npmjs.com/package/peasy-css) | `npm install peasy-css` |
| **Go** | [peasy-css-go](https://pkg.go.dev/github.com/peasytools/peasy-css-go) | `go get github.com/peasytools/peasy-css-go` |
| **Ruby** | [peasy-css](https://rubygems.org/gems/peasy-css) | `gem install peasy-css` |

## Peasy Developer Tools

Part of the [Peasy Tools](https://peasytools.com) open-source developer ecosystem.

| Package | PyPI | npm | crates.io | Description |
|---------|------|-----|-----------|-------------|
| peasy-pdf | [PyPI](https://pypi.org/project/peasy-pdf/) | [npm](https://www.npmjs.com/package/peasy-pdf) | [crate](https://crates.io/crates/peasy-pdf) | PDF merge, split, rotate, compress — [peasypdf.com](https://peasypdf.com) |
| peasy-image | [PyPI](https://pypi.org/project/peasy-image/) | [npm](https://www.npmjs.com/package/peasy-image) | [crate](https://crates.io/crates/peasy-image) | Image resize, crop, convert, compress — [peasyimage.com](https://peasyimage.com) |
| peasy-audio | [PyPI](https://pypi.org/project/peasy-audio/) | [npm](https://www.npmjs.com/package/peasy-audio) | [crate](https://crates.io/crates/peasy-audio) | Audio trim, merge, convert, normalize — [peasyaudio.com](https://peasyaudio.com) |
| peasy-video | [PyPI](https://pypi.org/project/peasy-video/) | [npm](https://www.npmjs.com/package/peasy-video) | [crate](https://crates.io/crates/peasy-video) | Video trim, resize, thumbnails, GIF — [peasyvideo.com](https://peasyvideo.com) |
| **peasy-css** | [PyPI](https://pypi.org/project/peasy-css/) | [npm](https://www.npmjs.com/package/peasy-css) | [crate](https://crates.io/crates/peasy-css) | CSS minify, format, analyze — [peasycss.com](https://peasycss.com) |
| peasy-compress | [PyPI](https://pypi.org/project/peasy-compress/) | [npm](https://www.npmjs.com/package/peasy-compress) | [crate](https://crates.io/crates/peasy-compress) | ZIP, TAR, gzip compression — [peasytools.com](https://peasytools.com) |
| peasy-document | [PyPI](https://pypi.org/project/peasy-document/) | [npm](https://www.npmjs.com/package/peasy-document) | [crate](https://crates.io/crates/peasy-document) | Markdown, HTML, CSV, JSON conversion — [peasyformats.com](https://peasyformats.com) |
| peasytext | [PyPI](https://pypi.org/project/peasytext/) | [npm](https://www.npmjs.com/package/peasytext) | [crate](https://crates.io/crates/peasytext) | Text case conversion, slugify, word count — [peasytext.com](https://peasytext.com) |

## License

MIT
