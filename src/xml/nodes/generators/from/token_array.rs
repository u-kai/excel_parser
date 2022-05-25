use crate::xml::{
    nodes::{node::XMLNode, node_type::NodeType},
    tokens::{states::TokenType, token_array::TokenArray},
};

impl<'a> From<&'a str> for XMLNode<'a> {
    fn from(s: &'a str) -> Self {
        let token_array = TokenArray::new(s);
        XMLNode::from(token_array)
    }
}
impl<'a> From<TokenArray<'a>> for XMLNode<'a> {
    fn from(token_array: TokenArray<'a>) -> Self {
        let token_array = token_array.token_array();
        let mut parent_stack = Vec::new();
        for token in token_array {
            match token.get_token_type() {
                TokenType::StartToken => parent_stack.push(XMLNode::from(token)),
                TokenType::Character => {
                    parent_stack.last_mut().unwrap().add_text(token.get_value())
                }
                TokenType::SingleToken => {
                    let node = XMLNode::from(token);
                    parent_stack.last_mut().unwrap().add_node(node);
                }
                TokenType::EndToken => {
                    let child = parent_stack.pop();
                    match child {
                        Some(node) => {
                            if parent_stack.len() == 0 {
                                return node;
                            }
                            parent_stack.last_mut().unwrap().add_node(node)
                        }
                        None => panic!("error: this case is not parse"),
                    }
                }
            }
        }
        // case exist declear line
        if parent_stack.len() == 1 {
            let mut single_parent = parent_stack.pop().unwrap();
            single_parent.set_node_type(NodeType::SingleElement);
            return single_parent;
        }
        panic!("not had end tag this stack : {:?}", parent_stack)
    }
}
#[cfg(test)]
mod token_array_test {

    use crate::xml::{
        nodes::{node::XMLNode, node_type::NodeType},
        tokens::token_array::TokenArray,
    };

    #[test]
    fn from_token_array_test() {
        let data = "<div>
                            <div>div-first
                                <p>p-data</p>
                                div-data
                            </div>
                        </div>";
        let token_array = TokenArray::new(data);
        let expect = XMLNode::from(token_array);
        let mut p = XMLNode::new("p", NodeType::Element);
        p.add_text("p-data");
        let mut div = XMLNode::new("div", NodeType::Element);
        let mut child_div = XMLNode::new("div", NodeType::Element);
        child_div.add_text("div-first");
        child_div.add_node(p);
        child_div.add_text("div-data");
        div.add_node(child_div);
        assert_eq!(expect, div);
        let data = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                                <div>
                                    <div>
                                        div-first
                                        <p>p-data</p>
                                        <data/>
                                        div-data
                                    </div>
                                </div>"#;
        let expect = XMLNode::from(data);
        let mut root = XMLNode::new("?xml", NodeType::SingleElement);
        root.add_element("version", vec!["1.0"]);
        root.add_element("encoding", vec!["UTF-8"]);
        root.add_element("standalone", vec!["yes"]);
        root.add_element("?", vec![]);
        let mut p = XMLNode::new("p", NodeType::Element);
        p.add_text("p-data");
        let single_data = XMLNode::new("data", NodeType::SingleElement);
        let mut div = XMLNode::new("div", NodeType::Element);
        let mut child_div = XMLNode::new("div", NodeType::Element);
        child_div.add_text("div-first");
        child_div.add_node(p);
        child_div.add_node(single_data);
        child_div.add_text("div-data");
        div.add_node(child_div);
        root.add_node(div);
        assert_eq!(expect, root)
    }
}

//#[cfg(test)]
//mod create_node {

//use crate::xml::{
//nodes::{node::XMLNode, node_type::NodeType},
//tokens::token_array::TokenArray,
//};
//#[test]
//fn from_token_array_test() {
//let data = "<div>
//<p>p-data</p>
//div-data
//</div>";
//let token_array = TokenArray::new(data);
//let expect = XMLNode::from(token_array);
//let mut p = XMLNode::new("p", NodeType::Element);
//p.add_text("p-data");
//let mut div = XMLNode::new("div", NodeType::Element);
//div.add_node(p);
//div.add_text("div-data");
//assert_eq!(expect, div);
//let data = "<div><div>div-first
//<p>p-data</p>
//div-data</div>
//</div>";
//let token_array = TokenArray::new(data);
//let expect = XMLNode::from(token_array);
//let mut p = XMLNode::new("p", NodeType::Element);
//p.add_text("p-data");
//let mut div = XMLNode::new("div", NodeType::Element);
//let mut child_div = XMLNode::new("div", NodeType::Element);
//child_div.add_text("div-first");
//child_div.add_node(p);
//child_div.add_text("div-data");
//div.add_node(child_div);
//assert_eq!(expect, div);
//let data = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
//<div><div>div-first
//<p>p-data</p>
//<data/>
//div-data</div>
//</div>"#;
//let expect = XMLNode::from(data);
//let mut root = XMLNode::new("?xml", NodeType::SingleElement);
//root.add_element("standalone", vec![r#"yes"#]);
//root.add_element("encoding", vec![r#"UTF-8"#]);
//root.add_element("version", vec![r#"1.0"#]);
//let mut p = XMLNode::new("p", NodeType::Element);
//p.add_text("p-data");
//let single_data = XMLNode::new("data", NodeType::SingleElement);
//let mut div = XMLNode::new("div", NodeType::Element);
//let mut child_div = XMLNode::new("div", NodeType::Element);
//child_div.add_text("div-first");
//child_div.add_node(p);
//child_div.add_node(single_data);
//child_div.add_text("div-data");
//div.add_node(child_div);
//root.add_node(div);
//assert_eq!(expect, root)
//}
//#[test]
//fn element_test() {
//let data = r#"<div id="1180" name="kai"><div>div-first
//<p>p-data</p>
//<data/>
//div-data</div>
//</div>"#;

//let token_array = TokenArray::new(data);
//let expect = XMLNode::from(token_array);
//let mut p = XMLNode::new("p", NodeType::Element);
//p.add_text("p-data");
//let single_data = XMLNode::new("data", NodeType::SingleElement);
//let mut div = XMLNode::new("div", NodeType::Element);
//div.add_element("name", vec!["kai"]);
//div.add_element("id", vec!["1180"]);
//let mut child_div = XMLNode::new("div", NodeType::Element);
//child_div.add_text("div-first");
//child_div.add_node(p);
//child_div.add_node(single_data);
//child_div.add_text("div-data");
//div.add_node(child_div);
//assert_eq!(expect, div);
//let data = r#"<div id="1180" name="kai" class="style1 style2"><div>div-first
//<p>p-data</p>
//<data/>
//div-data</div>
//</div>"#;
//let token_array = TokenArray::new(data);
//let expect = XMLNode::from(token_array);
//let mut p = XMLNode::new("p", NodeType::Element);
//p.add_text("p-data");
//let single_data = XMLNode::new("data", NodeType::SingleElement);
//let mut div = XMLNode::new("div", NodeType::Element);
//div.add_element("name", vec!["kai"]);
//div.add_element("id", vec!["1180"]);
//div.add_element("class", vec!["style1 style2"]);
//let mut child_div = XMLNode::new("div", NodeType::Element);
//child_div.add_text("div-first");
//child_div.add_node(p);
//child_div.add_node(single_data);
//child_div.add_text("div-data");
//div.add_node(child_div);
//assert_eq!(expect, div)
//}
//}
