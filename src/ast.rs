use std::fmt::Debug;

#[derive(Default, Debug)]
pub struct DocumentInformation {
    pub(crate) author: Option<String>,
    pub(crate) title: Option<String>,
    pub(crate) date: Option<String>,
}

#[derive(Debug)]
pub struct Document {
    pub(crate) info: DocumentInformation,
    pub(crate) body: Vec<Block>,
}

#[derive(Debug)]
pub struct RichString {
    string: String,
    bf: bool,
    it: bool,
    tt: bool,
}

#[derive(Debug)]
pub enum Block {
    Paragraph(Vec<RichString>),
    SectionHeader(Vec<RichString>),
    SubsectionHeader(Vec<RichString>),
    NewPage,
    MakeTitle,
}
