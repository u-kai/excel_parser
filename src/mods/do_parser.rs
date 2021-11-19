use super::tag::tag::Tag;

#[derive(Debug)]
pub struct DOParser {
    content:String
}
impl DOParser {
    pub fn new(content:String) -> Self {
        DOParser{content}
    }
    pub fn search_element(element:String)-> Tag {
        Tag::new("sheetData".to_string(),None,None)
    }
}