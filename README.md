# liber
*Rust library for creating (sync/async) EPUB files*

## Description
- This crate provides a high-level, ergonomic API for creating EPUB files (2.0.1). 
- It offers both asynchronous and blocking (synchronous) implementations, with flexible builders and output options. 
- Covers all [epubcheck](https://github.com/w3c/epubcheck) validations

## Usage
Add this crate to your `Cargo.toml`:

```toml
[dependencies]
liber = "0.1.1"
```

#### Enable async feature if needed

```toml
[dependencies]
liber = { version = "0.1.1", features = ["async"] }
```

## Example

```rust
use liber::epub::{
    ContentBuilder, ContentReference, EpubBuilder, ImageType, MetadataBuilder, ReferenceType,
    Resource,
};
use std::path::Path;

fn main() {
    match create() {
        Err(e) => eprintln!("{e}"),
        Ok(_) => println!("ok"),
    }
}

fn create() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = std::fs::File::create("book.epub")?;
    let title = "My Book";

    let contents = vec![
        ContentBuilder::new(
            r#"<body><h1>Chapter 2</h1></body>"#.as_bytes(),
            ReferenceType::Text("Chapter 2".to_string()),
        )
        .build(),
        ContentBuilder::new(
            r#"<body><h1>Chapter 3</h1></body>"#.as_bytes(),
            ReferenceType::Text("Chapter 3".to_string()),
        )
        .add_child(
            ContentBuilder::new(
                r#"<body><h1>Chapter 4</h1></body>"#.as_bytes(),
                ReferenceType::TitlePage("Chapter 4".to_string()),
            )
            .build(),
        )
        .build(),
    ];

    let epub_builder = EpubBuilder::new(MetadataBuilder::title(title).creator("author").build())
        .stylesheet("body {}".as_bytes())
        .cover_image(Path::new("/path/to/img.jpg"), ImageType::Jpg)
        .add_resource(Resource::Font(Path::new("/path/to/some_font.otf")))
        .add_content(
            ContentBuilder::new(
                r#"<body><h1>Chapter 1</h1><h2 id="id01">Section 1.1</h2><h2 id="id02">Section 1.1.1</h2><h2 id="id03">Section 1.2</h2></body>"#.as_bytes(),
                ReferenceType::Text("Chapter 1".to_string()),
            )
            .add_content_reference(ContentReference::new("Section 1.1").add_child(ContentReference::new("Section 1.1.1")))
            .add_content_reference(ContentReference::new("Section 1.2"))
            .add_children(contents)
            .build(),
        );

    epub_builder.create(&mut file)?;

    Ok(())
}
```

## Details
- Every content is a xhtml. The entire xhml text or only the body could be added as content (the latter is more practical and secure because follows the standard). See [examples](https://github.com/javiorfo/liber/tree/master/examples)
- Content (Ex: Chapter) and ContentReference (Ex: Chapter#ref1) could be named with filename and id methods respectively. If none is set, Content will be sequencial cNN.xhtml (c01.xhtml, c02.xhtml...) and ContentReferences will be idNN (id01, id02...) corresponding to the Content.

## Features
- Default blocking creation. Async available too (using tokio and async_zip crates)
- Multi section creation (contents, subcontents, references and subreferences)
- Supporting file content and raw content (bytes) creation

## Docs
Find all the configuration options in the full [documentation](https://docs.rs/liber/0.1.1/liber/).

---

### Donate
- **Bitcoin** [(QR)](https://raw.githubusercontent.com/javiorfo/img/master/crypto/bitcoin.png)  `1GqdJ63RDPE4eJKujHi166FAyigvHu5R7v`
- [Paypal](https://www.paypal.com/donate/?hosted_button_id=FA7SGLSCT2H8G)
