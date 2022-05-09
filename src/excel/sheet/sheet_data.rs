use crate::{
    excel::{cell::CellIndex, xmls::shared_strings::SharedStringsInterface},
    xml::nodes::node::XMLNode,
};

use super::cell_node::CellNode;

#[derive(Debug, PartialEq, Eq)]
pub struct SheetData<'a, T: SharedStringsInterface> {
    node: &'a mut XMLNode,
    shared_strings: &'a mut T,
}
impl<'a, T: SharedStringsInterface> SheetData<'a, T> {
    pub fn new(root_node: &'a mut XMLNode, shared_strings: &'a mut T) -> Self {
        let node = root_node.search_node_mut("worksheet").unwrap();
        let node = node.search_node_mut("sheetData").unwrap();
        SheetData {
            node,
            shared_strings,
        }
    }
    fn get_cell_v(&mut self, index: CellIndex) -> String {
        let c_node = self.node.search_child_by_id_mut("r", index.get_value());
        if let Some(c_node) = c_node {
            let c_node = CellNode::new(c_node, self.shared_strings);
            c_node.get_v_text()
        } else {
            "".to_string()
        }
    }
}

impl<'a, T: SharedStringsInterface> SheetOperator for SheetData<'a, T> {
    fn get_cell(&mut self, index: CellIndex) -> String {
        self.get_cell_v(index)
    }
}
pub trait SheetOperator {
    fn get_cell(&mut self, index: CellIndex) -> String;
}
#[cfg(test)]
mod xml_sheet_test {
    use crate::{
        excel::{
            cell::CellIndex,
            sheet::sheet_data::{SheetData, SheetOperator},
            xmls::shared_strings::SharedStringsInterface,
        },
        xml::nodes::node::XMLNode,
    };

    use super::mock_shared_strings::SharedStringsMock;

    //use super::SheetData;

    #[test]
    fn get_cell_test() {
        let source = r#"
    <?xml version="1.0" encoding="UTF-8" standalone="yes"?>
    <worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006" mc:Ignorable="x14ac xr xr2 xr3" xmlns:x14ac="http://schemas.microsoft.com/office/spreadsheetml/2009/9/ac" xmlns:xr="http://schemas.microsoft.com/office/spreadsheetml/2014/revision" xmlns:xr2="http://schemas.microsoft.com/office/spreadsheetml/2015/revision2" xmlns:xr3="http://schemas.microsoft.com/office/spreadsheetml/2016/revision3" xr:uid="{44FEEDED-D128-4496-B199-BCD526D1EB2C}">
        <sheetData>
            <row r="2" spans="2:19" x14ac:dyDescent="0.4">
                <c r="B2" s="15" t="s">
                    <v>0</v>
                </c>
                <c r="C2" s="12"/>
                <c r="D2" s="16"/>
                <c r="E2" s="13"/>
                <c r="J2" s="15" t="s">
                    <v>1</v>
                </c>
                <c r="K2" s="13"/>
                <c r="P2" s="15" t="s">
                    <v>2</v>
                </c>
                <c r="Q2" s="13"/>
            </row>
            <row r="3" spans="2:19" x14ac:dyDescent="0.4">
                <c r="B3" s="4"/>
                <c r="C3" s="15" t="s">
                    <v>3</v>
                </c>
                <c r="F6">
                    <v>50</v>
                </c>
                <c r="D3" s="16"/>
                <c r="E3" s="3" t="s">
                    <v>4</v>
                </c>
                <c r="H4" t="str">
                <f>$E$3&amp;G4</f>
                <v>shared_value</v>
                </c>
            </row>
        </sheetData>
    </worksheet>
    "#;
        let mut mock = SharedStringsMock::new();
        let mut xml = XMLNode::from(source);
        mock.add_shared_string("zero");
        mock.add_shared_string("one");
        mock.add_shared_string("two");
        mock.add_shared_string("three");
        let mut expect = SheetData::new(&mut xml, &mut mock);
        assert_eq!(expect.get_cell(CellIndex::new("B2")), "zero");
        assert_eq!(expect.get_cell(CellIndex::new("J2")), "one");
        //let expect = SheetData::new(source);
        //let tobe = XMLNode::from(
        //r#"
        //<sheetData>
        //<row r="2" spans="2:19" x14ac:dyDescent="0.4">
        //<c r="B2" s="15" t="s">
        //<v>43</v>
        //</c>
        //<c r="C2" s="12"/>
        //<c r="D2" s="16"/>
        //<c r="E2" s="13"/>
        //<c r="J2" s="15" t="s">
        //<v>44</v>
        //</c>
        //<c r="K2" s="13"/>
        //<c r="P2" s="15" t="s">
        //<v>59</v>
        //</c>
        //<c r="Q2" s="13"/>
        //</row>
        //<row r="3" spans="2:19" x14ac:dyDescent="0.4">
        //<c r="B3" s="4"/>
        //<c r="C3" s="15" t="s">
        //<v>0</v>
        //</c>
        //<c r="F6">
        //<v>50</v>
        //</c>
        //<c r="D3" s="16"/>
        //<c r="E3" s="3" t="s">
        //<v>68</v>
        //</c>
        //<c r="H4" t="str">
        //<f>$E$3&amp;G4</f>
        //<v>shared_value</v>
        //</c>
        //</row>
        //</sheetData>
        //"#;
        //,
        //       );
        //assert_eq!(expect, tobe);
    }
    //#[test]
    //fn get_value_test() {
    //let source = r#"
    //<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
    //<worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006" mc:Ignorable="x14ac xr xr2 xr3" xmlns:x14ac="http://schemas.microsoft.com/office/spreadsheetml/2009/9/ac" xmlns:xr="http://schemas.microsoft.com/office/spreadsheetml/2014/revision" xmlns:xr2="http://schemas.microsoft.com/office/spreadsheetml/2015/revision2" xmlns:xr3="http://schemas.microsoft.com/office/spreadsheetml/2016/revision3" xr:uid="{44FEEDED-D128-4496-B199-BCD526D1EB2C}">
    //<sheetData>
    //<row r="2" spans="2:19" x14ac:dyDescent="0.4">
    //<c r="B2" s="15" t="s">
    //<v>43</v>
    //</c>
    //<c r="C2" s="12"/>
    //<c r="D2" s="16"/>
    //<c r="E2" s="13"/>
    //<c r="J2" s="15" t="s">
    //<v>44</v>
    //</c>
    //<c r="K2" s="13"/>
    //<c r="P2" s="15" t="s">
    //<v>59</v>
    //</c>
    //<c r="Q2" s="13"/>
    //</row>
    //<row r="3" spans="2:19" x14ac:dyDescent="0.4">
    //<c r="B3" s="4"/>
    //<c r="C3" s="15" t="s">
    //<v>0</v>
    //</c>
    //<c r="F6">
    //<v>50</v>
    //</c>
    //<c r="D3" s="16"/>
    //<c r="E3" s="3" t="s">
    //<v>68</v>
    //</c>
    //<c r="H4" t="str">
    //<f>$E$3&amp;G4</f>
    //<v>shared_value</v>
    //</c>
    //</row>
    //</sheetData>
    //</worksheet>
    //"#;
    //let sheet = SheetData::new(&XMLNode::from(source));
    //assert_eq!(sheet.get_shared_cell(&CellIndex::new("F6")), Some("50"));
    //assert_eq!(sheet.get_refarence_cell(&CellIndex::new("P2")), Some(59));
    //assert_eq!(sheet.get_shared_cell(&CellIndex::new("P2")), None);
    //assert_eq!(sheet.get_refarence_cell(&CellIndex::new("F6")), None);
    //assert_eq!(sheet.get_refarence_cell(&CellIndex::new("P100")), None);
    //}
}

mod mock_shared_strings {
    use crate::excel::xmls::shared_strings::SharedStringsInterface;

    pub struct SharedStringsMock {
        values: Vec<String>,
    }
    impl SharedStringsMock {
        pub fn new() -> Self {
            SharedStringsMock { values: Vec::new() }
        }
    }
    impl SharedStringsInterface for SharedStringsMock {
        fn get_shared_string(&self, index: usize) -> &str {
            self.values[index].as_str()
        }
        fn add_shared_string(&mut self, value: &str) -> () {
            self.values.push(value.to_string())
        }
    }
}
