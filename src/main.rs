use std::{collections::HashMap, str::Chars, sync::BarrierWaitResult};

#[derive(Debug, PartialEq, Eq)]
pub struct Dom {
    root: String,
    brothers: Vec<Box<Dom>>,
    children: String,
}
impl Dom {
    fn new(data: &str) -> Dom {
        let mut root = "".to_string();
        let mut tmp_root = root.clone();
        let mut is_root = true;
        let mut children = "".to_string();
        let mut tmp_children = children.clone();
        let mut tag_num = 0;
        let mut is_child_range = false;
        let mut brothers = vec![];
        let mut is_start_check_brothers = false;
        for c in data.chars() {
            match c {
                '<' => {
                    if tag_num == 0 {
                        is_root = true
                    }
                    if is_root {
                        root = format!("{}{}", root, c);
                    }
                    if is_child_range {
                        children = format!("{children}{c}");
                    }
                    tag_num += 1;
                }
                '>' => {
                    if is_root {
                        is_root = false;
                        root = format!("{}{}", root, c);
                    }
                    if is_child_range {
                        children = format!("{children}{c}");
                    }
                    if tag_num == 1 {
                        is_child_range = true;
                    }
                }
                '/' => {
                    tag_num -= 2;
                    if tag_num == 0 {
                        if !is_start_check_brothers {
                            is_start_check_brothers = true;
                            tmp_children = format!("{}", children);
                            tmp_root = format!("{}", root);
                            root = "".to_string();
                            children = "".to_string();
                            continue;
                        }
                        if is_start_check_brothers {
                            brothers.push(Box::new(Dom {
                                root,
                                brothers: vec![],
                                children,
                            }));
                            root = "".to_string();
                            children = "".to_string();
                        }
                    }
                    children = format!("{children}{c}");
                }
                _ => {
                    if is_root {
                        root = format!("{}{}", root, c);
                    }
                    if is_child_range {
                        children = format!("{children}{c}");
                    }
                }
            }
        }
        let len = tmp_children.len() - 1;
        Dom {
            root: tmp_root,
            children: tmp_children[0..len].to_string(),
            brothers,
        }
    }
    fn get_root(&self) -> &str {
        &self.root
    }
    fn get_children(&self) -> &str {
        &self.children
    }
    fn get_brothers(&self) -> &Vec<Box<Dom>> {
        &self.brothers
    }
}
impl From<&str> for Dom {
    fn from(data: &str) -> Self {
        Dom::new(data)
    }
}
//#[cfg(test)]
//mod tests {
//use super::*;
//#[test]
//fn get_brothers_test() {
//let data = r#"<div class="class"><div><p>hello world</p></div></div><div class="class"><div><p>hello world</p></div></div>"#;
//let dom = Dom::from(data);
//assert_eq!(
//dom.get_brothers(),
//&vec![Box::new(Dom::from(
//r#"<div class="class"><div><p>hello world</p></div></div>"#
//))]
//);
//let data = r#"<div class="class"><div><p>hello world</p></div></div><div class="class"></div><div class="class"></div><div class="class"></div>"#;
//let dom = Dom::from(data);
//}
//#[test]
//fn get_root_test() {
//let data = r#"<div class="class"><div><p>hello world</p></div></div>"#;
//let dom = Dom::from(data);
//assert_eq!(dom.get_root(), r#"<div class="class">"#.to_string());
//}
//#[test]
//fn get_children_test() {
//let data = r#"<div><div><p>hello world</p></div></div>"#;
//let dom = Dom::from(data);
//assert_eq!(
//dom.get_children(),
//"<div><p>hello world</p></div>".to_string()
//);
//}
//}
fn main() {
    println!("hello")
}
