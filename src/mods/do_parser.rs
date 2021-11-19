use regex::Regex;

use super::tag::tag::Tag;

#[derive(Debug)]
pub struct DOParser {
    content:String
}
impl DOParser {
    pub fn new(content:String) -> Self {
        DOParser{content}
    }
    pub fn search_element(&self,element:String)-> Tag {
        Tag::new("sheetData".to_string(),None,None)
    }
    pub fn fmt_content(&mut self) -> &Self {
        &self.to_lf();
        &self.fmt_do()
    }
    fn fmt_do(&mut self)-> &Self {
        let re = Regex::new(r">([^\n]).+").unwrap();
        println!("{}",re.is_match(&self.content));
        let capts = re.captures(&self.content).unwrap();
        self.content = re.replace_all(&self.content, format!(">\n{}",&capts.get(0).unwrap().as_str()[1..])).to_string();
        let re = Regex::new(r"([^\s]+)<").unwrap();
        let capts = re.captures(&self.content).unwrap();
        println!("{}",capts.get(0).unwrap().as_str());
        self.content = re.replace_all(&self.content, format!("{}\n<",&capts.get(0).unwrap().as_str()[..&capts.get(0).unwrap().as_str().len()-1])).to_string();
        self
    }
    fn to_lf(&mut self) -> &Self {
       let re = Regex::new(r"\r\n").unwrap();
        println!("{}",re.is_match(&self.content));
        re.replace_all(&self.content, "\n");
        self
    }
}