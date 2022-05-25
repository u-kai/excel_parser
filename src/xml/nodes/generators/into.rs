use crate::xml::nodes::{node::XMLNode, node_type::NodeType};

impl<'a> XMLNode<'a> {
    pub fn to_string(&self) -> String {
        let mut result = String::new();
        match self.get_node_type() {
            NodeType::Character => format!("{}", self.get_value()),
            NodeType::Element => {
                if let Some(children) = self.get_children() {
                    for child in children {
                        result = format!("{}{}", result, child.to_string());
                    }
                    format!(
                        "<{}>{}</{}>",
                        self.get_node_value().to_string(),
                        result,
                        self.get_value()
                    )
                } else {
                    format!(
                        "<{}></{}>",
                        self.get_node_value().to_string(),
                        self.get_node_value().get_value()
                    )
                }
            }
            NodeType::SingleElement => {
                if let Some(children) = self.get_children() {
                    for child in children {
                        result = format!("{}{}", result, child.to_string());
                    }
                    if self.get_value().chars().nth(0).unwrap() == '?' {
                        let str = self.get_node_value().to_string();
                        let remove_last_question = str.get(..(str.len() - 2)).unwrap();
                        format!("<{}?>\n{}", remove_last_question, result,)
                    } else {
                        format!("<{}>{}", self.get_node_value().to_string(), result,)
                    }
                } else {
                    format!("<{}/>", self.get_node_value().to_string())
                }
            }
        }
    }
}
impl<'a> Into<String> for XMLNode<'a> {
    fn into(self) -> String {
        self.to_string()
    }
}
#[cfg(test)]
mod xml_into_str_test {

    use crate::xml::nodes::node::XMLNode;

    #[test]
    fn to_string_test_1() {
        let data =
            r#"<div id="1180" name="kai"><div>div-first<p>p-data</p><data/>div-data</div></div>"#;
        let root_node = XMLNode::from(data);
        let expect: String = root_node.to_string();
        assert_eq!(expect, data);
    }

    #[test]
    fn into_test_1() {
        let data =
            r#"<div id="1180" name="kai"><div>div-first<p>p-data</p><data/>div-data</div></div>"#;
        let root_node = XMLNode::from(data);
        let expect: String = root_node.into();
        assert_eq!(expect, data);
    }
    #[test]

    fn into_test_2() {
        let data = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<div id="1180" name="kai"><div>div-first<p>p-data</p><data/>div-data</div></div>"#;
        let root_node = XMLNode::from(data);
        println!("{:?}", root_node);
        let expect: String = root_node.into();
        assert_eq!(expect, data);
    }
    //#[test]
    //fn same_file_test() {
    //let mut buf = String::new();
    //let mut file = BufReader::new(File::open("test/xl/worksheets/sheet1.xml").unwrap());
    //let _ = file.read_to_string(&mut buf);

    //let node = XMLNode::from(buf.as_str());
    //assert_eq!(node.to_string(), buf);
    //}
}
