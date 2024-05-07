use std::fmt::Debug;

#[derive(Default, Debug)]
pub struct DocumentInformation {
    pub(crate) author: Option<String>,
    pub(crate) title: Option<String>,
    pub(crate) date: Option<String>,
    pub(crate) margin: Option<u8>,
    pub(crate) font_size: Option<u8>,
}

#[derive(Debug)]
pub struct Document {
    pub(crate) info: DocumentInformation,
    pub(crate) body: Vec<Block>,
}

#[derive(Debug)]
pub struct RichString {
    pub string: String,
    bf: bool,
    it: bool,
    tt: bool,
}

impl RichString {
    pub fn standard(string: String) -> Self {
        Self {
            string,
            bf: false,
            it: false,
            tt: false,
        }
    }
}

#[derive(Debug)]
pub enum Block {
    Paragraph(Vec<RichString>),
    SectionHeader(Vec<RichString>),
    SubsectionHeader(Vec<RichString>),
    NewPage,
    MakeTitle,
}
