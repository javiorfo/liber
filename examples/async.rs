use std::path::Path;

use liber::{
    ZipCompression,
    epub::{
        ContentBuilder, ContentReference, EpubBuilder, ImageType, MetadataBuilder, ReferenceType,
    },
};

#[tokio::main]
async fn main() {
    match create().await {
        Err(e) => eprintln!("{e}"),
        Ok(_) => println!("ok"),
    }
}

async fn create() -> Result<(), Box<dyn std::error::Error>> {
    let style = tokio::fs::read("./files/style.css").await?;

    let mut file = tokio::fs::File::create("book.epub").await?;
    let title = "My Book";

    let contents = vec![
        ContentBuilder::new(
            r#"<h1>Chapter 2</h1>"#.as_bytes(),
            ReferenceType::Text("Chapter 2".to_string()),
        )
        .build(),
        ContentBuilder::new(
            r#"<h1>Chapter 3</h1>"#.as_bytes(),
            ReferenceType::Text("Chapter 3".to_string()),
        )
        .add_child(
            ContentBuilder::new(
                r#"<h1>Chapter 4</h1>"#.as_bytes(),
                ReferenceType::TitlePage("Chapter 4".to_string()),
            )
            .build(),
        )
        .build(),
    ];

    let epub_builder = EpubBuilder::new(MetadataBuilder::title(title)
        .creator("author")
        .build())
        .stylesheet(&style)
        .cover_image(Path::new("/path/to/image.jpg"), ImageType::Jpg)
        .add_content(
            ContentBuilder::new(
                r#"<h1>Chapter 1</h1><h2 id="id01">Section 1.1</h2><h2 id="id02">Section 1.1.1</h2><h2 id="id03">Section 1.2</h2>"#.as_bytes(),
                ReferenceType::Text("Chapter 1".to_string()),
            )
            .add_content_reference(ContentReference::new("Section 1.1").add_child(ContentReference::new("Section 1.1.1")))
            .add_content_reference(ContentReference::new("Section 1.2"))
            .add_children(contents)
            .build(),
        );

    epub_builder
        .async_create_with_compression(&mut file, ZipCompression::Stored)
        .await?;

    Ok(())
}
