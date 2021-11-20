use std::{fmt::format, future::Ready};

use regex::Regex;

use super::tag::tag::Tag;

#[derive(Debug)]
pub struct DOParser {
    pub content:String
}
impl DOParser {
    pub fn new(content:String) -> Self {
        DOParser{content}
    }

    pub fn search_element(&self,element:String)-> Option<Tag> {
        let re_str = format!(r"(<{} .*>)",element);
        let re = Regex::new(&re_str).unwrap();

        if !re.is_match(&self.content){
            return None
        }
        let capts = re.captures(&self.content).unwrap().get(1).unwrap().as_str();
        Some(Tag::new(capts.to_string(),None,None))
    }

    pub fn fmt_content(&mut self) -> &Self {
        self.to_lf();
        self.insert_new_line()
    }

    fn insert_new_line(&mut self) -> &Self {
        let re = Regex::new(r">([^\n<]*)<").unwrap();
        loop {
            if !re.is_match(&self.content){
                break;
            }
            let capts = re.captures(&self.content).unwrap().get(1).unwrap().as_str();
            if capts == "" {
                self.content = re.replace(&self.content, ">\n<").to_string();

            } else {
                self.content = re.replace(&self.content, format!(">\n{}\n<",capts)).to_string();
            }
        }
        self
    }

    fn to_lf(&mut self) -> &Self {
       let re = Regex::new(r"\r\n").unwrap();
       re.replace_all(&self.content, "\n");
       self
    }

}
#[cfg(test)]
mod tests{
    use crate::mods::{do_parser::DOParser, tag::tag::Tag};

#[test]

fn fmt_content_test(){
    let content = "<div><div>a</div><div>data</div></div>";
    let mut dp = DOParser::new(content.to_string());
    dp.fmt_content(); 
    assert_eq!(dp.content,"<div>\n<div>\na\n</div>\n<div>\ndata\n</div>\n</div>")
} 
fn search_element_test() {
    let content = "<element><div>a</div><div>data</div></element>";
    let mut dp = DOParser::new(content.to_string());
    dp.fmt_content();
    let tag = dp.search_element("element".to_string());
    assert_eq!(tag.unwrap(),Tag::new("<element>".to_string(),None,None))

}


}