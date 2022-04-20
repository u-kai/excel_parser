use std::io::Read;
use std::{fs::File, io::BufReader};

mod xml;

fn main() {
    let mut buf = String::new();
    let mut source = BufReader::new(File::open("test.html").unwrap());
    let _ = source.read_to_string(&mut buf);
    let xml_node = xml::node::XMLNode::from(buf.as_str());
    println!(
        "{:?}",
        xml_node.search_node("html").unwrap().search_nodes("body")
    )
}
