use crate::xml::nodes::{node::XMLNode, node_type::NodeType};

impl Into<String> for XMLNode {
    fn into(self) -> String {
        //match self.node_type {
        //NodeType::Character=>
        //}
        "".to_string()
    }
}
#[cfg(test)]
mod xml_into_str_test {
    use crate::xml::nodes::node::XMLNode;

    //#[test]
    //fn into_test_1() {
    //let data =
    //r#"<div id="1180" name="kai"><div>div-first<p>p-data</p><data/>div-data</div></div>"#;
    //let root_node = XMLNode::from(data);
    //let expect: String = root_node.into();
    //assert_eq!(expect, data.to_string());
    //}
    //#[test]

    //fn into_test_2() {
    //let data = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><div id="1180" name="kai"><div>div-first<p>p-data</p><data/>div-data</div></div>"#;
    //let root_node = XMLNode::from(data);
    //let expect: String = root_node.into();
    //assert_eq!(expect, data.to_string());
    //}
}
