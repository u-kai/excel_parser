use crate::{
    excel::xmls::shared_strings::SharedStringsInterface,
    xml::nodes::{node::XMLNode, node_type::NodeType},
};

use super::cell::CellIndex;
#[derive(Debug, PartialEq, Eq)]
pub enum CellType {
    Str,
    Num,
}

#[derive(Debug, PartialEq, Eq)]
pub struct CellNode<'a, T: SharedStringsInterface> {
    node: &'a XMLNode,
    shared_strings: &'a T,
}
impl<'a, T: SharedStringsInterface> CellNode<'a, T> {
    pub fn new(node: &'a XMLNode, shared_strings: &'a T) -> Self {
        CellNode {
            node,
            shared_strings,
        }
    }
    pub fn new_c_node(value: &'a str, index: CellIndex, value_type: CellType) -> XMLNode {
        let mut c_node = XMLNode::new("c", NodeType::Element);
        c_node.add_element("r", vec![index.get_value()]);
        match value_type {
            CellType::Num => (),
            CellType::Str => c_node.add_element("t", vec!["s"]),
        }
        let mut v_node = XMLNode::new("v", NodeType::Element);
        v_node.add_text(value);
        c_node.add_node(v_node);
        c_node
    }
    pub fn get_v_text(&self) -> Option<String> {
        if let Some(v_node) = self.node.search_node("v") {
            if let Some(text) = v_node.get_child_text(0) {
                if self.is_use_raw_data() {
                    return Some(text.to_string());
                }
                if self.is_use_shared_strings() {
                    let index = text.parse::<usize>().unwrap();
                    return Some(self.shared_strings.get_shared_string(index).to_string());
                }
            }
            return None;
        }
        None
    }
    fn is_use_shared_strings(&self) -> bool {
        self.node.is_containe_key_value("t", "s")
    }
    fn is_use_raw_data(&self) -> bool {
        self.node.is_containe_key_value("t", "str") || !(self.node.is_containe_key_value("t", "s"))
    }
    pub fn is_index(&self, index: CellIndex) -> bool {
        self.node.is_containe_key_value("r", index.get_value())
    }
}

pub struct MutCellNode<'a, T: SharedStringsInterface> {
    node: &'a mut XMLNode,
    shared_strings: &'a T,
}
impl<'a, T: SharedStringsInterface> MutCellNode<'a, T> {
    pub fn new(node: &'a mut XMLNode, shared_strings: &'a T) -> Self {
        MutCellNode {
            node,
            shared_strings,
        }
    }
    pub fn get_v_text(&self) -> String {
        if let Some(v_node) = self.node.search_node("v") {
            if let Some(text) = v_node.get_child_text(0) {
                if self.is_use_raw_data() {
                    return text.to_string();
                }
                if self.is_use_shared_strings() {
                    let index = text.parse::<usize>().unwrap();
                    return self.shared_strings.get_shared_string(index).to_string();
                }
            }
            return "".to_string();
        }
        "".to_string()
    }
    fn is_use_shared_strings(&self) -> bool {
        self.node.is_containe_key_value("t", "s")
    }
    fn is_use_raw_data(&self) -> bool {
        self.node.is_containe_key_value("t", "str") || !(self.node.is_containe_key_value("t", "s"))
    }
    pub fn is_index(&self, index: CellIndex) -> bool {
        self.node.is_containe_key_value("r", index.get_value())
    }
    pub fn change_text(&mut self, text: &str) {
        self.node.change_text(text)
    }
}
