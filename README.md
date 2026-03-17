# peasy-css

[![crates.io](https://img.shields.io/crates/v/peasy-css)](https://crates.io/crates/peasy-css)
[![docs.rs](https://docs.rs/peasy-css/badge.svg)](https://docs.rs/peasy-css)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![crates.io](https://agentgif.com/badge/crates/peasy-css/version.svg)](https://crates.io/crates/peasy-css)
[![GitHub stars](https://agentgif.com/badge/github/peasytools/peasy-css-rs/stars.svg)](https://github.com/peasytools/peasy-css-rs)

Async Rust client for the [PeasyCSS](https://peasycss.com) API -- minify, beautify, and convert CSS units with tools for gradient generation, shadow creation, flexbox layouts, and grid systems. Built with reqwest, serde, and tokio.

Built from [PeasyCSS](https://peasycss.com), a comprehensive CSS toolkit offering free online tools for minifying, formatting, analyzing, and generating CSS code with detailed property guides, layout references, and a glossary covering modern CSS specifications including Grid, Flexbox, Custom Properties, and Container Queries.

> **Try the interactive tools at [peasycss.com](https://peasycss.com)** -- [CSS Minify](https://peasycss.com/css/css-minify/), [CSS Beautify](https://peasycss.com/css/css-beautify/), [CSS Unit Converter](https://peasycss.com/css/css-unit-converter/), and more.

<p align="center">
  <img src="demo.gif" alt="peasy-css demo -- CSS minify, beautify, and unit conversion tools in Rust terminal" width="800">
</p>

## Table of Contents

- [Install](#install)
- [Quick Start](#quick-start)
- [What You Can Do](#what-you-can-do)
  - [CSS Optimization Tools](#css-optimization-tools)
  - [Browse CSS Reference Content](#browse-css-reference-content)
  - [Search and Discovery](#search-and-discovery)
- [API Client](#api-client)
  - [Available Methods](#available-methods)
- [Learn More About CSS Tools](#learn-more-about-css-tools)
- [Also Available](#also-available)
- [Peasy Developer Tools](#peasy-developer-tools)
- [License](#license)

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

## What You Can Do

### CSS Optimization Tools

CSS optimization is essential for web performance. Minified CSS reduces file sizes by removing whitespace, comments, and redundant declarations, while beautified CSS restores readability for debugging. The PeasyCSS API provides programmatic access to these transformations alongside unit conversion tools for responsive design workflows.

| Tool | Description | Use Case |
|------|-------------|----------|
| CSS Minify | Remove whitespace and comments from CSS | Production builds, CI/CD pipelines |
| CSS Beautify | Format and indent CSS for readability | Code review, debugging |
| CSS Unit Converter | Convert between px, rem, em, vw, vh | Responsive design systems |

```rust
use peasy_css::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    // Fetch the CSS Minify tool details
    let tool = client.get_tool("css-minify").await?;
    println!("Tool: {}", tool.name);           // CSS minification tool
    println!("Category: {}", tool.category);   // CSS optimization category

    // List all CSS file formats and their MIME types
    let formats = client.list_formats(&Default::default()).await?;
    for fmt in &formats.results {
        println!(".{} -- {} ({})", fmt.extension, fmt.name, fmt.mime_type);
    }

    Ok(())
}
```

Learn more: [CSS Minify Tool](https://peasycss.com/css/css-minify/) · [CSS Beautify Tool](https://peasycss.com/css/css-beautify/) · [How to Minify CSS for Production](https://peasycss.com/guides/how-to-minify-css-production/)

### Browse CSS Reference Content

Modern CSS has evolved far beyond simple selectors and properties. Concepts like CSS Grid, Flexbox, Custom Properties (CSS variables), specificity rules, and the cascade determine how styles are applied. The PeasyCSS glossary provides clear definitions and practical examples for these foundational concepts, while guides offer in-depth tutorials on layout techniques and optimization strategies.

| Glossary Term | Description |
|---------------|-------------|
| Flexbox | One-dimensional layout model for distributing space along a row or column |
| Grid | Two-dimensional layout system for rows and columns simultaneously |
| Specificity | Algorithm that determines which CSS rule takes precedence |
| Custom Property | CSS variables defined with `--` prefix, enabling dynamic theming |
| Minification | Process of removing unnecessary characters from CSS without changing functionality |

```rust
// Browse CSS glossary terms programmatically
let glossary = client.list_glossary(&peasy_css::ListOptions {
    search: Some("flexbox".into()),
    ..Default::default()
}).await?;
for term in &glossary.results {
    println!("{}: {}", term.term, term.definition); // Flexbox layout definition
}

// Read in-depth CSS guides on layout and optimization
let guides = client.list_guides(&peasy_css::ListGuidesOptions {
    category: Some("layout".into()),
    ..Default::default()
}).await?;
for guide in &guides.results {
    println!("{} ({})", guide.title, guide.audience_level); // Guide title and difficulty
}
```

Learn more: [Flexbox Glossary](https://peasycss.com/glossary/flexbox/) · [Grid Glossary](https://peasycss.com/glossary/grid/) · [CSS Grid vs Flexbox Guide](https://peasycss.com/guides/css-grid-vs-flexbox-when-to-use-each/)

### Search and Discovery

The unified search endpoint queries across all CSS tools, glossary terms, guides, and file formats simultaneously. This is useful for building IDE integrations, documentation search, or CLI tools that need to surface relevant CSS content based on user queries.

```rust
// Search across all CSS tools, glossary, and guides
let results = client.search("gradient generator", Some(20)).await?;
println!("Found {} tools, {} glossary terms",
    results.results.tools.len(),
    results.results.glossary.len()); // Cross-content CSS search results
```

Learn more: [Specificity Glossary](https://peasycss.com/glossary/specificity/) · [Custom Property Glossary](https://peasycss.com/glossary/custom-property/) · [All CSS Guides](https://peasycss.com/guides/)

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

## Learn More About CSS Tools

- **Tools**: [CSS Minify](https://peasycss.com/css/css-minify/) · [CSS Beautify](https://peasycss.com/css/css-beautify/) · [CSS Unit Converter](https://peasycss.com/css/css-unit-converter/) · [All Tools](https://peasycss.com/)
- **Guides**: [CSS Grid vs Flexbox](https://peasycss.com/guides/css-grid-vs-flexbox-when-to-use-each/) · [How to Minify CSS for Production](https://peasycss.com/guides/how-to-minify-css-production/) · [All Guides](https://peasycss.com/guides/)
- **Glossary**: [Flexbox](https://peasycss.com/glossary/flexbox/) · [Grid](https://peasycss.com/glossary/grid/) · [Minification](https://peasycss.com/glossary/minification/) · [Specificity](https://peasycss.com/glossary/specificity/) · [Custom Property](https://peasycss.com/glossary/custom-property/) · [All Terms](https://peasycss.com/glossary/)
- **Formats**: [All Formats](https://peasycss.com/formats/)
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
| peasy-pdf | [PyPI](https://pypi.org/project/peasy-pdf/) | [npm](https://www.npmjs.com/package/peasy-pdf) | [crate](https://crates.io/crates/peasy-pdf) | PDF merge, split, rotate, compress -- [peasypdf.com](https://peasypdf.com) |
| peasy-image | [PyPI](https://pypi.org/project/peasy-image/) | [npm](https://www.npmjs.com/package/peasy-image) | [crate](https://crates.io/crates/peasy-image) | Image resize, crop, convert, compress -- [peasyimage.com](https://peasyimage.com) |
| peasy-audio | [PyPI](https://pypi.org/project/peasy-audio/) | [npm](https://www.npmjs.com/package/peasy-audio) | [crate](https://crates.io/crates/peasy-audio) | Audio trim, merge, convert, normalize -- [peasyaudio.com](https://peasyaudio.com) |
| peasy-video | [PyPI](https://pypi.org/project/peasy-video/) | [npm](https://www.npmjs.com/package/peasy-video) | [crate](https://crates.io/crates/peasy-video) | Video trim, resize, thumbnails, GIF -- [peasyvideo.com](https://peasyvideo.com) |
| **peasy-css** | [PyPI](https://pypi.org/project/peasy-css/) | [npm](https://www.npmjs.com/package/peasy-css) | [crate](https://crates.io/crates/peasy-css) | CSS minify, format, analyze -- [peasycss.com](https://peasycss.com) |
| peasy-compress | [PyPI](https://pypi.org/project/peasy-compress/) | [npm](https://www.npmjs.com/package/peasy-compress) | [crate](https://crates.io/crates/peasy-compress) | ZIP, TAR, gzip compression -- [peasytools.com](https://peasytools.com) |
| peasy-document | [PyPI](https://pypi.org/project/peasy-document/) | [npm](https://www.npmjs.com/package/peasy-document) | [crate](https://crates.io/crates/peasy-document) | Markdown, HTML, CSV, JSON conversion -- [peasyformats.com](https://peasyformats.com) |
| peasytext | [PyPI](https://pypi.org/project/peasytext/) | [npm](https://www.npmjs.com/package/peasytext) | [crate](https://crates.io/crates/peasytext) | Text case conversion, slugify, word count -- [peasytext.com](https://peasytext.com) |

## License

MIT
