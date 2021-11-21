use regex::Regex;
use std::{collections::HashMap, future::Ready, net::ToSocketAddrs, panic::catch_unwind};

use crate::mods::tag::tag::Tag;

use super::do_parser::DOParser;
#[derive(Debug)]
pub struct DOSearcher {
    content: String,
}

impl DOSearcher {
    pub fn new(content: String) -> Self {
        DOSearcher { content }
    }
    pub fn get_content(&self)->&String {
        &self.content
    }
    //pub fn get_tag(&self,element_name:&str){
        //let re_str = format!("< *{} *(?P<properties>.*)>[<{}>.*</{}>]*</{}>",element_name,element_name,element_name,element_name);
        //let re = Regex::new(r"")
    //} 
    pub fn news(&self, element_name: &str) -> Option<Tag> {
        let mut dop = DOParser::new(self.content.clone());
        let lines = dop.content.split("\n");
        let mut same_element_counter = 0;
        let mut element = "";
        let mut children = "".to_string();
        let mut properites: Option<HashMap<String, String>> = None;
        for line in lines {
            if dop.is_require_element(line, element_name) {
                if same_element_counter == 0 {
                    same_element_counter += 1;
                    element = element_name;
                    properites = self.get_properties(line, &element_name);
                    continue;
                }
                same_element_counter += 1
            }
            if dop.is_require_end_element(line, element_name) {
                same_element_counter -= 1;
                if same_element_counter == 0 {
                    break;
                }
            }
            if same_element_counter > 0 {
                children += line
            }
        }
        if element == "" {
            None
        } else {
            Some(Tag::new(element.to_string(), self.get_children(&children), properites))
        }
    }
    fn get_children(&self, children: &str) -> Option<String> {
        let children = if children != "" {
            Some(children.to_string())
        } else {
            None
        };
        children
    }
    fn get_properties(&self, element: &str, element_name: &str) -> Option<HashMap<String, String>> {
        let re_str = format!(r"<{}( *[^>]*)>", element_name);
        let re = Regex::new(&re_str).unwrap();
        let properties = re.captures(element).unwrap().get(1).unwrap().as_str();
        let properties: Option<HashMap<String, String>> = if properties != "" {
            let re = Regex::new(r" = ").unwrap();
            let new = re.replace_all(&properties[1..], "=").to_string();
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
        } else {
            None
        };
        properties
    }
}
#[cfg(test)]
mod tests {
    use std::net::ToSocketAddrs;
    use std::{collections::HashMap, ops::Index};

    use crate::mods::document_obj::do_searcher::DOSearcher;
    use crate::mods::tag::tag::Tag;
    #[test]
    fn test() {
        let ds = DOSearcher::new(
            "<element p=12 d=13><div>a</div><element></element><div>data</div></element>"
                .to_string(),
        );
        let tag = ds.news(&"element".to_string());
        let mut map = HashMap::new();
        map.insert("p".to_string(), "12".to_string());
        map.insert("d".to_string(), "13".to_string());
        assert_eq!(
            tag.unwrap(),
            Tag::new(
                "element".to_string(),
                Some("<div>a</div><element></element><div>data</div>".to_string()),
                Some(map)
            )
        );

        let ds = DOSearcher::new("<div>a</div><div>data</div>".to_string());
        let tag = ds.news(&"div".to_string());
        //map.insert("p".to_string(), "12".to_string());
        //map.insert("d".to_string(), "13".to_string());
        assert_eq!(
            tag.unwrap(),
            Tag::new("div".to_string(), Some("a".to_string()), None)
        )
    }
}
