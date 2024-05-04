use genpdf::elements::Paragraph;
use genpdf::Alignment;
use std::path::PathBuf;

use crate::ast::{Block, Document};

pub fn render_pdf(file_path: PathBuf, document: Document) {
    // Load a font from the file system
    let font_family =
        genpdf::fonts::from_files("./assets/fonts/computer-modern/", "ComputerModern", None)
            .expect("Failed to load font family");

    // Create a document and set the default font family
    let mut pdf_document = genpdf::Document::new(font_family);

    pdf_document.set_title(&document.info.title.clone().unwrap_or("".to_string()));

    // Customize the pages
    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(30);
    pdf_document.set_page_decorator(decorator);

    let info = document.info;

    let mut section_count = 0;
    let mut subsection_count = 0;

    for block in document.body.iter() {
        match block {
            Block::MakeTitle => {
                // Title
                let mut title = Paragraph::default();
                title.set_alignment(Alignment::Center);
    
                let mut title_style = genpdf::style::Style::new();
                title_style.set_font_size(18);
                //title_style.set_bold();
    
                title.push_styled(info.title.clone().unwrap_or("".to_string()), title_style);
                pdf_document.push(title);
    
                pdf_document.push(genpdf::elements::Break::new(1));
    
                // Author
                let mut author = Paragraph::default();
                author.set_alignment(Alignment::Center);
    
                let mut author_style = genpdf::style::Style::new();
                author_style.set_font_size(13);
    
                author.push_styled(info.author.clone().unwrap_or("".to_string()), author_style);
                pdf_document.push(author);
    
                pdf_document.push(genpdf::elements::Break::new(0.5));
    
                // Date
                let mut date = Paragraph::default();
                date.set_alignment(Alignment::Center);
    
                let mut date_style = genpdf::style::Style::new();
                date_style.set_font_size(13);
    
                date.push_styled(info.date.clone().unwrap_or("".to_string()), date_style);
                pdf_document.push(date);
    
                pdf_document.push(genpdf::elements::Break::new(2));
            },
            Block::Paragraph(strings) => {
                pdf_document.push(Paragraph::new(strings[0].string.clone()));
            },
            Block::NewPage => pdf_document.push(genpdf::elements::PageBreak::new()),
            Block::SectionHeader(strings) => {
                section_count += 1;
                subsection_count = 0;

                let mut section_header = Paragraph::default();
    
                let mut section_style = genpdf::style::Style::new();
                section_style.set_font_size(16);
                section_style.set_bold();
    
                let text = format!("{section_count}  {}", strings[0].string.clone());
    
                section_header.push_styled(text, section_style);
                pdf_document.push(section_header);
    
                pdf_document.push(genpdf::elements::Break::new(0.6));
            },
            Block::SubsectionHeader(strings) => {
                subsection_count += 1;

                let mut subsection_header = Paragraph::default();
    
                let mut subsection_style = genpdf::style::Style::new();
                subsection_style.set_font_size(14);
                subsection_style.set_bold();
    
                let text = format!("{section_count}.{subsection_count}  {}", strings[0].string.clone());
    
                subsection_header.push_styled(text, subsection_style);
                pdf_document.push(subsection_header);
    
                pdf_document.push(genpdf::elements::Break::new(0.3));
            }
        }
    }

    // Render the document and write it to a file
    let mut output_path = file_path.clone();
    output_path.set_extension("pdf");
    pdf_document.render_to_file(output_path.as_path())
        .expect("Failed to write PDF file");
}
