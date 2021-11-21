use std::{collections::HashMap, fmt::format, fs::{OpenOptions, set_permissions}, future::Ready, hash::Hash};

use regex::Regex;

use crate::mods::tag::tag::Tag;

#[derive(Debug)]
pub struct DOParser {
    pub content: String,
}
impl DOParser {
    pub fn new(content: String) -> Self {
        DOParser { content }
    }

    pub fn search_element(&self, element: String) -> Option<Tag> {
        let re_str = format!(r"<({})( *[^>]*)>", element);
        let re = Regex::new(&re_str).unwrap();
        if !re.is_match(&self.content) {
            return None;
        }
        let capts = re.captures(&self.content).unwrap();
        let element = capts.get(1).unwrap().as_str();
        let properties: Option<HashMap<String, String>> = match capts.get(2) {
            Some(m) => {
                let re = Regex::new(r" = ").unwrap();
                let new = re.replace_all(&m.as_str()[1..], "=").to_string();
                let mut keys: Vec<String> = vec![];
                let mut values: Vec<String> = vec![];
                for s in new.split(" ") {
                    let strs = s.split("=").collect::<Vec<_>>();
                    for (i, s) in strs.iter().enumerate() {
                        if i % 2 == 0 {
                            keys.push(s.to_string())
                        } else {
                            values.push(s.to_string())
                        }
                    }
                }
                let map: HashMap<String, String> = keys
                    .iter()
                    .zip(values.iter())
                    .map(|d| (d.0.to_string(), d.1.to_string()))
                    .collect();
                Some(map)
            }
            None => None,
        };
        Some(Tag::new(format!("{}", element), None, properties))
    }
    pub fn is_require_element(&self, content: &str, element_name: &str) -> bool {
        let re_str = format!(r"<({})( *[^>]*)>", element_name);
        let re = Regex::new(&re_str).unwrap();
        re.is_match(content)
    }
    pub fn is_require_end_element(&self, content: &str, element_name: &str) -> bool {
        let re_str = format!(r"</({})>", element_name);
        let re = Regex::new(&re_str).unwrap();
        re.is_match(content)
    }
    pub fn fmt_content(&mut self) -> &Self {
        self.to_lf();
        self.insert_new_line()
    }

    fn insert_new_line(&mut self) -> &Self {
        let re = Regex::new(r">([^\n<]*)<").unwrap();
        loop {
            if !re.is_match(&self.content) {
                break;
            }
            let capts = re.captures(&self.content).unwrap().get(1).unwrap().as_str();
            if capts == "" {
                self.content = re.replace(&self.content, ">\n<").to_string();
            } else {
                self.content = re
                    .replace(&self.content, format!(">\n{}\n<", capts))
                    .to_string();
            }
        }
        self
    }

    pub fn get_tag(&self,element_name:&str)->String{
        //let close_tag_re = format!("(<{}>.*</{}>)*",element_name,element_name,element_name);
        let re_str_close = format!(r".*< *{} *([^>]*)/ *>",element_name);
        let re = Regex::new(&re_str_close).unwrap();
        if re.is_match(&self.content) {
            let capts = re.captures(&self.content).unwrap();
            return capts.get(1).unwrap().as_str().to_string()
        }
        let re_str = format!(r"< *{} *([^>]*)>((hello+|< *{} *[^>]*>.*</{}>|<[^{}][^>]*>.*</[^{}][^>]*>)*)</{}>",element_name,element_name,element_name,element_name,element_name,element_name);//element_name);

        //let re_str = format!(r"< *{} *([^>]*)>(.*)</{}>",element_name,element_name);//,element_name,element_name);
        println!("{}",re_str);
        println!("{}",&self.content);
        let re = Regex::new(&re_str).unwrap();
        if !re.is_match(&self.content) {
            println!("not match")
        }
        let capts = re.captures(&self.content).unwrap();
        println!("{:?}",capts);
        capts.get(2).unwrap().as_str().to_string()
    } 
    fn to_lf(&mut self) -> &Self {
        let re = Regex::new(r"\r\n").unwrap();
        if !re.is_match(&self.content) {
            return self
        }
        let data = re.replace_all(&self.content, "\n");
        self.content = data.to_string();
        self
    }
}
#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::mods::document_obj::do_parser::DOParser;
    use crate::mods::tag::tag::Tag;

    #[test]

    fn fmt_content_test() {
        let content = "<div><div>a</div><div>data</div></div>";
        let mut dp = DOParser::new(content.to_string());
        dp.fmt_content();
        assert_eq!(
            dp.content,
            "<div>\n<div>\na\n</div>\n<div>\ndata\n</div>\n</div>"
        )
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
        assert_eq!(
            tag.unwrap(),
            Tag::new("element".to_string(), None, Some(properties))
        )
    }
    #[test]
    fn get_tag_test() {
        let content = "<div>fa</div><element p=12 d=13>hello<element>dfa</element><div>a<element>dfa2</element></div><div>data</div></element><element>element2</element><element>element3</element>";
        let mut dp = DOParser::new(content.to_string());
        let properites = dp.get_tag("element");
        assert_eq!(properites,"hello<element>dfa</element><div>a<element>dfa2</element></div><div>data</div>");

        let content = "<div>fa</div><element p=12 d=13/><element>dfa</element><div>a<element>dfa2</element></div><div>data</div><element>element2</element>";
        let mut dp = DOParser::new(content.to_string());
        let element = dp.get_tag("element");
        
        assert_eq!(element,"p=12 d=13")
    }
}
