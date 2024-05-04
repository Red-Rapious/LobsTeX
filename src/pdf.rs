use genpdf::elements::Paragraph;
use genpdf::Alignment;
use std::path::PathBuf;

pub fn render_pdf(file_path: PathBuf) {
    // Load a font from the file system
    let font_family =
        genpdf::fonts::from_files("./assets/fonts/computer-modern/", "ComputerModern", None)
            .expect("Failed to load font family");

    // Create a document and set the default font family
    let mut doc = genpdf::Document::new(font_family);

    // Change the default settings
    doc.set_title("Demo document");

    // Customize the pages
    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(10);
    doc.set_page_decorator(decorator);

    // Add one or more elements
    let mut title = Paragraph::default();
    title.set_alignment(Alignment::Center);

    let mut title_style = genpdf::style::Style::new();
    title_style.set_font_size(32);
    title_style.set_bold();

    title.push_styled("Demo document title", title_style);
    doc.push(title);

    doc.push(Paragraph::new("This is a demo document."));

    // Render the document and write it to a file
    let mut output_path = file_path.clone();
    output_path.set_extension("pdf");
    doc.render_to_file(output_path.as_path())
        .expect("Failed to write PDF file");
}
