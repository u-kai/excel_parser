use crate::xml::nodes::node::XMLNode;

pub struct SharedStrings {
    values: Vec<String>,
}

impl SharedStrings {
    pub fn new(s: &str) -> Self {
        let mut xml_node = XMLNode::from(s);
        let sst = xml_node.nth_child_node(0).unwrap();

        let vec = sst.search_nodes("si").unwrap();
        println!("{:?}", vec);
        let values = vec
            .iter()
            .map(|node| {
                if let Some(child) = node.search_node("t") {
                    child.get_child_charcter(0).unwrap().to_string()
                } else {
                    "".to_string()
                }
            })
            .collect::<Vec<_>>();
        SharedStrings { values }
    }
    pub fn get_value(&self, index: usize) -> &str {
        &self.values[index]
    }
}

#[cfg(test)]
mod shared_strings_test {
    use super::SharedStrings;

    #[test]
    fn get_value_test() {
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
        assert_eq!(ss.get_value(0), "詳細画面レイアウト");
        assert_eq!(ss.get_value(1), "会社名");
        assert_eq!(ss.get_value(2), "タイトル");
    }
}
