use std::{collections::HashMap, fmt::format, fs::OpenOptions, future::Ready, hash::Hash};

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
        let re_str = format!(r"(<{})( *[^>]*)>",element);
        let re = Regex::new(&re_str).unwrap();
        if !re.is_match(&self.content){
            return None
        }
        let capts = re.captures(&self.content).unwrap();
        let element = capts.get(1).unwrap().as_str();
        let properties:Option<HashMap<String,String>> = match capts.get(2) {
            Some(m)=>{
                let re = Regex::new(r" = ").unwrap();
                let new  = re.replace_all(&m.as_str()[1..],"=" ).to_string();
                let mut keys:Vec<String> = vec![];
                let mut values:Vec<String> = vec![];
                for s in new.split(" ") {
                    let strs = s.split("=").collect::<Vec<_>>();
                    for (i,s) in strs.iter().enumerate() {
                        if i % 2 == 0 {
                            keys.push(s.to_string())
                        } else {
                            values.push(s.to_string())
                        }
                    }
                }
                let map:HashMap<String,String> = keys.iter().zip(values.iter()).map(|d|{
                    (d.0.to_string(),d.1.to_string())
                }).collect();
                Some(map)
            },
            None=>None
        };
        Some(Tag::new(format!("{}>",element),None,properties))
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
    use std::collections::HashMap;

    use crate::mods::{do_parser::DOParser, tag::tag::Tag};

#[test]

fn fmt_content_test(){
    let content = "<div><div>a</div><div>data</div></div>";
    let mut dp = DOParser::new(content.to_string());
    dp.fmt_content(); 
    assert_eq!(dp.content,"<div>\n<div>\na\n</div>\n<div>\ndata\n</div>\n</div>")
} 
#[test]
fn search_element_test() {
    let content = "<element p=12 d=13><div>a</div><div>data</div></element>";
    let mut dp = DOParser::new(content.to_string());
    dp.fmt_content();
    let tag = dp.search_element("element".to_string());
    let mut properties = HashMap::new();
    properties.insert("p".to_string(), "12".to_string());
    properties.insert("d".to_string(), "13".to_string());
    assert_eq!(tag.unwrap(),Tag::new("element".to_string(),None,Some(properties)))
}
}