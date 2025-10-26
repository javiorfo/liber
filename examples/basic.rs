use std::path::Path;

use liber::epub::{
    ContentBuilder, ContentReference, EpubBuilder, ImageType, MetadataBuilder, ReferenceType,
    Resource,
};

fn main() {
    match create() {
        Err(e) => eprintln!("{e}"),
        Ok(_) => println!("ok"),
    }
}

fn create() -> Result<(), Box<dyn std::error::Error>> {
    let style = std::fs::read("./files/style.css")?;
    let chapter1 = std::fs::read("./files/chapter1.xhtml")?;

    let mut file = std::fs::File::create("book.epub")?;
    let title = "My Book";

    let contents = vec![
        ContentBuilder::new(
            r#"<body><h1>Chapter 2</h1></body>"#.as_bytes(),
            ReferenceType::Text("Chapter 2".to_string()),
        )
        .filename("chapter2.xhtml")
        .build(),
        ContentBuilder::new(
            r#"<body><h1>Chapter 3</h1></body>"#.as_bytes(),
            ReferenceType::Text("Chapter 3".to_string()),
        )
        .filename("chapter3.xhtml")
        .add_child(
            ContentBuilder::new(
                r#"<body><h1>Chapter 4</h1></body>"#.as_bytes(),
                ReferenceType::TitlePage("Chapter 4".to_string()),
            )
            .filename("chapter4.xhtml")
            .build(),
        )
        .build(),
    ];

    let epub_builder = EpubBuilder::new(MetadataBuilder::title(title).creator("author").build())
        .stylesheet(&style)
        .cover_image(Path::new("/path/to/img.jpg"), ImageType::Jpg)
        .add_resource(Resource::Font(Path::new("/path/to/some_font.otf")))
        .add_content(
            ContentBuilder::new(&chapter1, ReferenceType::Text("Chapter 1".to_string()))
                .filename("chapter1.xhtml")
                .add_content_reference(
                    ContentReference::new("Section 1.1")
                        .id("s1-1")
                        .add_child(ContentReference::new("Section 1.1.1").id("s1-1-1")),
                )
                .add_content_reference(ContentReference::new("Section 1.2").id("s1-2"))
                .add_children(contents)
                .build(),
        );

    epub_builder.create(&mut file)?;

    Ok(())
}
