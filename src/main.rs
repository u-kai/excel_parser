use std::io::Read;
use std::{fs::File, io::BufReader};

use html::dom::Dom;
use xml::nodes::node::XMLNode;

mod html;
mod xml;

fn main() {
    let mut buf = String::new();
    let file = File::open("test.html").unwrap();
    let mut source = BufReader::new(file);
    let _ = source.read_to_string(&mut buf);
    let xml_node = XMLNode::from(buf.as_str());
    let file = File::open("test.html").unwrap();
    println!("{:?}", file);
    let html_node = Dom::from(file);
    println!("{:?}", xml_node.search_node("html").unwrap());
    println!("{:?}", html_node.get_element_by_id("js-review-widget"))
}
