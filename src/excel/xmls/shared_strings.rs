use super::xl::XL;
use crate::xml::nodes::node::XMLNode;

#[derive(Debug, PartialEq, Eq)]
pub struct SharedStrings {
    node: XMLNode,
    values: Vec<String>,
}
pub trait SharedStringsInterface {
    fn get_shared_string(&self, index: usize) -> &str;
    fn add_shared_string(&mut self, value: &str) -> ();
}
impl<'a> XL<'a> for SharedStrings {
    fn get_xml_node(&'a self) -> &'a XMLNode {
        &self.node
    }
}
impl SharedStrings {
    pub fn new(source: &str) -> Self {
        let node = XMLNode::from(source);
        let sst = node
            .search_node("sst")
            .expect(format!("not exist <sst> for {:?}", node).as_str());
        let values = if let Some(si_vec) = sst.search_all_nodes("si") {
            si_vec
                .iter()
                .filter_map(|node| node.search_node("t"))
                .filter_map(|node| node.get_child_text(0))
                .map(|str| str.into())
                .collect::<Vec<_>>()
        } else {
            Vec::new()
        };
        SharedStrings { node, values }
    }
}

//impl<'a> From<&'a XMLNode> for SharedStrings {
//fn from(xml_node: &'a XMLNode) -> Self {
//let sst = xml_node.nth_child_node(0).unwrap();
//let si_vec = sst.search_all_nodes("si").unwrap();
//let values = si_vec
//.iter()
//.filter_map(|node| node.search_node("t"))
//.filter_map(|node| node.get_child_text(0))
//.map(|str| str.into())
//.collect::<Vec<_>>();
//SharedStrings { values }
//}
//}
//impl SharedStrings {
//pub fn new(s: &str) -> Self {
//let xml_node = XMLNode::from(s);
//let sst = xml_node.nth_child_node(0).unwrap();
//let si_vec = sst.search_all_nodes("si").unwrap();
//let values = si_vec
//.iter()
//.filter_map(|node| node.search_node("t"))
//.filter_map(|node| node.get_child_text(0))
//.map(|str| str.into())
//.collect::<Vec<_>>();
//SharedStrings { values }
//}
//pub fn get_value(&self, index: usize) -> &str {
//&self.values[index]
//}
//}
//impl SharedStore for SharedStrings {
//fn get_shared_value(&self, index: usize) -> &str {
//&self.values[index]
//}
//}

#[cfg(test)]
mod shared_strings_test {
    use crate::{
        excel::{
            shared_strings::{self, SharedStore},
            xmls::xl::XL,
        },
        xml::nodes::node::XMLNode,
    };

    use super::SharedStrings;
    #[test]
    fn get_xml_node_test() {
        let source = r#"
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<sst xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" count="938" uniqueCount="265">
    <si>
        <t>詳細画面レイアウト</t>
        <rPh sb="0" eb="2">
            <t>ショウサイ</t>
        </rPh>
        <rPh sb="2" eb="4">
            <t>ガメン</t>
        </rPh>
        <phoneticPr fontId="2"/>
    </si>
    <si>
        <t>会社名</t>
        <rPh sb="0" eb="3">
            <t>カイシャメイ</t>
        </rPh>
        <phoneticPr fontId="2"/>
    </si>
    <si>
        <t>タイトル</t>
        <phoneticPr fontId="2"/>
    </si>
</sst>
"#;
        let node = XMLNode::from(source);
        let shared_strings = SharedStrings::new(source);
        assert_eq!(shared_strings.get_xml_node(), &node);
    }
    #[test]
    fn get_shared_string_test() {
        let ss = SharedStrings::new(
            r#"
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<sst xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" count="938" uniqueCount="265">
    <si>
        <t>詳細画面レイアウト</t>
        <rPh sb="0" eb="2">
            <t>ショウサイ</t>
        </rPh>
        <rPh sb="2" eb="4">
            <t>ガメン</t>
        </rPh>
        <phoneticPr fontId="2"/>
    </si>
    <si>
        <t>会社名</t>
        <rPh sb="0" eb="3">
            <t>カイシャメイ</t>
        </rPh>
        <phoneticPr fontId="2"/>
    </si>
    <si>
        <t>タイトル</t>
        <phoneticPr fontId="2"/>
    </si>
</sst>
"#,
        );
        //assert_eq!(ss.get_shared_string(0), "詳細画面レイアウト");
        //assert_eq!(ss.get_shared_string(1), "会社名");
        //assert_eq!(ss.get_shared_string(2), "タイトル");
    }
}
