use crate::xml::nodes::{node::XMLNode, node_type::NodeType, node_value::NodeElement};

impl XMLNode {
    pub fn rec(&self) -> String {
        let mut result = String::new();
        match self.node_type {
            NodeType::Character => format!("{}", self.get_value()),
            NodeType::Element => {
                if let Some(children) = self.get_children() {
                    for child in children {
                        result = format!("{}{}", result, child.rec());
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
                        result = format!("{}{}", result, child.rec());
                    }
                    if self.get_value().chars().nth(0).unwrap() == '?' {
                        format!("<{}?>{}", self.get_node_value().to_string(), result,)
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
impl Into<String> for XMLNode {
    fn into(self) -> String {
        self.rec()
    }
}
#[cfg(test)]
mod xml_into_str_test {
    use crate::xml::nodes::node::XMLNode;

    #[test]
    fn into_test_1() {
        let data =
            r#"<div id="1180" name="kai"><div>div-first<p>p-data</p><data/>div-data</div></div>"#;
        let root_node = XMLNode::from(data);
        let expect: String = root_node.into();
        assert_eq!(expect.len(), data.len());
    }
    #[test]

    fn into_test_2() {
        let data = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><div id="1180" name="kai"><div>div-first<p>p-data</p><data/>div-data</div></div>"#;
        let root_node = XMLNode::from(data);
        let expect: String = root_node.into();
        println!("{:?}", expect);
        assert_eq!(expect.len(), data.len());
    }
}
