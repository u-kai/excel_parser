use std::{
    fs::File,
    io::{BufReader, Read},
};

use crate::xml::nodes::node::XMLNode;

type HTMLNode = XMLNode;
#[derive(Debug)]
pub struct Dom {
    node: HTMLNode,
}

impl Dom {
    #[allow(dead_code)]
    pub fn new(s: &str) -> Self {
        Dom {
            node: HTMLNode::from(s),
        }
    }
    pub fn get_node(&self) -> &HTMLNode {
        &self.node
    }
    pub fn get_element_by_id(&self, id: &str) -> Option<&HTMLNode> {
        let node = self.get_node();
        if node.get_value() == id {
            return Some(node);
        }
        node.search_child_by_id("id", id)
    }
    pub fn get_elements_by_class_name(&self, class_name: &str) -> Option<Vec<&HTMLNode>> {
        let nodes = self.node.search_all_child("class", class_name);
        if nodes.len() == 0 {
            None
        } else {
            Some(nodes)
        }
    }
    //let node = self.get_node();
    //if node.get_child().is_some() {
    //node.get_child().unwrap().iter().filter(|child|{child.is_containe_key_value("class", )})
    //}
    //None
    //}
    //fn searche_rec(&mut self,key:&str,value:&str)->&Option<Vec<&HTMLNode>> {
    //let node =self.get_node();
    //if node.get_child().is_some() {
    //for
    //}
    //&None

    //}
}
impl From<File> for Dom {
    fn from(f: File) -> Self {
        let mut buf = String::new();
        let mut buf_reader = BufReader::new(f);
        let _ = buf_reader.read_to_string(&mut buf);
        let node = HTMLNode::from(buf.as_str());
        Dom::from(node)
    }
}
impl From<&str> for Dom {
    fn from(s: &str) -> Self {
        let node = XMLNode::from(s);
        Dom::from(node)
    }
}
impl From<XMLNode> for Dom {
    fn from(xml: XMLNode) -> Self {
        let html = xml
            .search_node("html")
            .expect(&format!("{:?} is not has html node", xml))
            .clone();
        Dom { node: html }
    }
}

#[cfg(test)]
mod dom_test {
    use crate::{html::dom::Dom, xml::nodes::node::XMLNode};
    #[test]
    fn get_elements_by_class_name_test() {
        let data = r#"
    <!DOCUMENT TYPE>
    <html>
        <div class="test">
             <p>hello</p>
            <div>div-hello</div>
            <data/>
            <div>
                <div class="test data">
                data
                </div>
            </div>
        </div>
    </html>
    "#;
        let dom = Dom::from(data);
        let elements = dom.get_elements_by_class_name("test");
        let node1 = XMLNode::from(
            r#"
    <div class="test">
        <p>hello</p>
        <div>
            div-hello
        </div>
        <data/>
        <div>
            <div class="test data">
                data
            </div>
        </div>
    </div>
    "#,
        );
        let node2 = XMLNode::from(
            r#"
    <div class="test data">
        data
    </div>
    "#,
        );
        assert_eq!(elements, Some(vec![&node1, &node2]))
    }
    #[test]

    fn get_element_by_id_test() {
        let data = r#"<test><html><div id="1180" name="kai"><div>div-first
            <p>p-data</p>
            <data/>
            div-data</div>
        </div></html>"#;
        let dom = Dom::from(data);
        let element = dom.get_element_by_id("1180");
        assert_eq!(
            element,
            Some(&XMLNode::from(
                r#"<div id="1180" name="kai"><div>div-first
            <p>p-data</p>
            <data/>
            div-data</div>
        </div>"#
            ))
        );
        let data = r#"
                <!DOCTYPE html>
                <html lang="en">
                <head>
                    <meta charset="UTF-8" />
                    <meta http-equiv="X-UA-Compatible" content="IE=edge" />
                    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                    <title>Document</title>
                </head>
                <body>
                    <div id="message" class="not-class data ddsfa" div="d">
                    これはrustで XMLを解析するプロジェクトです
                    </div>
                </body>
                </html>
        "#;
        let dom = Dom::from(data);
        let element = dom.get_element_by_id("message");
        assert_eq!(
            element,
            Some(&XMLNode::from(
                r#"
                    <div id="message" class="not-class data ddsfa" div="d">
                    これはrustで XMLを解析するプロジェクトです
                    </div>
                "#
            ))
        );
    }
}
