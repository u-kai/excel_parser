use crate::xml::nodes::node::XMLNode;

pub trait XL<'a> {
    fn get_xml_node(&'a self) -> &'a XMLNode;
}
