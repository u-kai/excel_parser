use super::xl::XL;
use crate::xml::nodes::{node::XMLNode, node_type::NodeType};

#[derive(Debug, PartialEq, Eq)]
pub struct SharedStrings<'a> {
    node: XMLNode<'a>,
    values: Vec<String>,
}
pub trait SharedStringsInterface<'a> {
    fn get_values(&self) -> &Vec<String>;
    fn get_shared_string(&self, index: usize) -> &str;
    fn add_shared_string(&mut self, value: &'a str) -> ();
    fn to_xml(&self) -> String;
}
impl<'a> SharedStrings<'a> {
    pub fn new(source: &'a str) -> Self {
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

impl<'a> XL<'a> for SharedStrings<'a> {
    fn get_xml_node(&'a self) -> &'a XMLNode<'a> {
        &self.node
    }
}
impl<'a> SharedStringsInterface<'a> for SharedStrings<'a> {
    fn to_xml(&self) -> String {
        self.node.to_string()
    }
    fn get_values(&self) -> &Vec<String> {
        &self.values
    }
    fn get_shared_string(&self, index: usize) -> &str {
        &self.values[index]
    }
    fn add_shared_string(&mut self, value: &'a str) -> () {
        self.values.push(value.into());
        let phonetic_pr = self
            .node
            .search_node("sst")
            .unwrap()
            .search_node("si")
            .unwrap()
            .search_node("phoneticPr")
            .unwrap();
        let mut sst_node = self.node.search_node("sst").unwrap().clone();
        let mut t_node = XMLNode::new("t", NodeType::Element);
        let mut add_node = XMLNode::new("si", NodeType::Element);
        t_node.add_text(value);
        add_node.add_node(t_node);
        add_node.add_node(phonetic_pr.clone());
        sst_node.add_node(add_node);
        // tobe not use change_child_node because clone sst
        self.node.change_child_node(sst_node);
    }
}
#[cfg(test)]
mod shared_strings_test {
    use crate::{
        excel::xmls::{shared_strings::SharedStringsInterface, xl::XL},
        xml::nodes::node::XMLNode,
    };

    use super::SharedStrings;
    #[test]
    fn get_xml_node_test() {
        let source = r#"
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<sst xmlns="http:schemas.openxmlformats.orgspreadsheetml2006main" count="938" uniqueCount="265">
    <si>
        <t>???????????????????????????</t>
        <rPh sb="0" eb="2">
            <t>???????????????</t>
        </rPh>
        <rPh sb="2" eb="4">
            <t>?????????</t>
        </rPh>
        <phoneticPr fontId="2"/>
    </si>
    <si>
        <t>?????????</t>
        <rPh sb="0" eb="3">
            <t>??????????????????</t>
        </rPh>
        <phoneticPr fontId="2"/>
    </si>
    <si>
        <t>????????????</t>
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
        <t>???????????????????????????</t>
        <rPh sb="0" eb="2">
            <t>???????????????</t>
        </rPh>
        <rPh sb="2" eb="4">
            <t>?????????</t>
        </rPh>
        <phoneticPr fontId="2"/>
    </si>
    <si>
        <t>?????????</t>
        <rPh sb="0" eb="3">
            <t>??????????????????</t>
        </rPh>
        <phoneticPr fontId="2"/>
    </si>
    <si>
        <t>????????????</t>
        <phoneticPr fontId="2"/>
    </si>
</sst>
"#,
        );
        assert_eq!(ss.get_shared_string(0), "???????????????????????????");
        assert_eq!(ss.get_shared_string(1), "?????????");
        assert_eq!(ss.get_shared_string(2), "????????????");
    }
    #[test]
    fn add_shared_string_test() {
        let mut ss = SharedStrings::new(
            r#"
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<sst xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" count="938" uniqueCount="265">
    <si>
        <t>???????????????????????????</t>
        <rPh sb="0" eb="2">
            <t>???????????????</t>
        </rPh>
        <rPh sb="2" eb="4">
            <t>?????????</t>
        </rPh>
        <phoneticPr fontId="2"/>
    </si>
    <si>
        <t>?????????</t>
        <rPh sb="0" eb="3">
            <t>??????????????????</t>
        </rPh>
        <phoneticPr fontId="2"/>
    </si>
    <si>
        <t>????????????</t>
        <phoneticPr fontId="2"/>
    </si>
</sst>
"#,
        );
        ss.add_shared_string("???????????????");
        ss.add_shared_string("???????????????");
        let tobe_node = r#"
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<sst xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" count="938" uniqueCount="265">
    <si>
        <t>???????????????????????????</t>
        <rPh sb="0" eb="2">
            <t>???????????????</t>
        </rPh>
        <rPh sb="2" eb="4">
            <t>?????????</t>
        </rPh>
        <phoneticPr fontId="2"/>
    </si>
    <si>
        <t>?????????</t>
        <rPh sb="0" eb="3">
            <t>??????????????????</t>
        </rPh>
        <phoneticPr fontId="2"/>
    </si>
    <si>
        <t>????????????</t>
        <phoneticPr fontId="2"/>
    </si>
    <si>
        <t>???????????????</t>
        <phoneticPr fontId="2"/>
    </si>
    <si>
        <t>???????????????</t>
        <phoneticPr fontId="2"/>
    </si>
</sst>
"#;
        let tobe_node = XMLNode::from(tobe_node);
        assert_eq!(ss.get_xml_node(), &tobe_node);
    }
}
