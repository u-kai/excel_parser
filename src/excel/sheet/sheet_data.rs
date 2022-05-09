use crate::{
    excel::{
        cell::{Cell, CellIndex},
        xmls::shared_strings::SharedStringsInterface,
    },
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
    fn get_cell_v(&self, index: CellIndex) -> String {
        let c_node = self.node.search_child_by_id("r", index.get_value());
        if let Some(c_node) = c_node {
            let c_node = CellNode::new(c_node, self.shared_strings);
            c_node.get_v_text()
        } else {
            "".to_string()
        }
    }
}

impl<'a, T: SharedStringsInterface> SheetOperator for SheetData<'a, T> {
    fn get_cell(&self, index: CellIndex) -> String {
        self.get_cell_v(index)
    }
    fn get_row(&self, row_index: usize) -> Vec<String> {
        let row_node = self
            .node
            .search_child_by_id("r", row_index.to_string().as_str());
        if let Some(row_node) = row_node {
            let c_nodes = row_node.search_all_nodes("c").unwrap();
            let mut result = vec![];
            for c_node in c_nodes.iter() {
                let cell_index = CellIndex::new(c_node.search_element("r").unwrap());
                let column_index = cell_index.get_column_index();
                let nones_range = (result.len() + 1)..column_index;
                nones_range.for_each(|_| result.push("".to_string()));
                let c_node = CellNode::new(c_node, self.shared_strings);
                result.push(c_node.get_v_text());
            }
            result
        } else {
            vec!["".to_string()]
        }
    }
}
pub trait SheetOperator {
    fn get_cell(&self, index: CellIndex) -> String;
    fn get_row(&self, row_index: usize) -> Vec<String>;
    //fn add_value(&mut self, cell: Cell<&str>) -> ();
}
#[cfg(test)]
mod xml_sheet_test {
    use crate::{
        excel::{
            cell::{Cell, CellIndex},
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
                <c r="D3" s="16"/>
                <c r="E3" s="3" t="s">
                    <v>4</v>
                </c>
                <c r="F6">
                    <v>50</v>
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
        let expect = SheetData::new(&mut xml, &mut mock);
        assert_eq!(expect.get_cell(CellIndex::new("B2")), "zero");
        assert_eq!(expect.get_cell(CellIndex::new("J2")), "one");
        assert_eq!(expect.get_cell(CellIndex::new("XX3")), "");
    }
    #[test]
    fn get_row_test() {
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
                <c r="D3" s="16"/>
                <c r="E3" s="3" t="s">
                    <v>4</v>
                </c>
                <c r="F6">
                    <v>50</v>
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
        mock.add_shared_string("four");
        let sheet = SheetData::new(&mut xml, &mut mock);
        assert_eq!(
            sheet.get_row(2),
            vec![
                "".to_string(),
                "zero".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "one".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "two".to_string(),
                "".to_string()
            ]
        );
        assert_eq!(
            sheet.get_row(3),
            vec![
                "".to_string(),
                "".to_string(),
                "three".to_string(),
                "".to_string(),
                "four".to_string(),
                "50".to_string(),
                "".to_string(),
                "shared_value".to_string(),
            ]
        );
    }
    //#[test]
    //fn change_value_test() {
    //let source = r#"
    //<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
    //<worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006" mc:Ignorable="x14ac xr xr2 xr3" xmlns:x14ac="http://schemas.microsoft.com/office/spreadsheetml/2009/9/ac" xmlns:xr="http://schemas.microsoft.com/office/spreadsheetml/2014/revision" xmlns:xr2="http://schemas.microsoft.com/office/spreadsheetml/2015/revision2" xmlns:xr3="http://schemas.microsoft.com/office/spreadsheetml/2016/revision3" xr:uid="{44FEEDED-D128-4496-B199-BCD526D1EB2C}">
    //<sheetData>
    //<row r="2" spans="2:19" x14ac:dyDescent="0.4">
    //<c r="B2" s="15" t="s">
    //<v>0</v>
    //</c>
    //<c r="C2" s="12"/>
    //<c r="D2" s="16"/>
    //<c r="E2" s="13"/>
    //<c r="J2" s="15" t="s">
    //<v>1</v>
    //</c>
    //<c r="K2" s="13"/>
    //<c r="P2" s="15" t="s">
    //<v>2</v>
    //</c>
    //<c r="Q2" s="13"/>
    //</row>
    //<row r="3" spans="2:19" x14ac:dyDescent="0.4">
    //<c r="B3" s="4"/>
    //<c r="C3" s="15" t="s">
    //<v>3</v>
    //</c>
    //<c r="F6">
    //<v>50</v>
    //</c>
    //<c r="D3" s="16"/>
    //<c r="E3" s="3" t="s">
    //<v>4</v>
    //</c>
    //<c r="H4" t="str">
    //<f>$E$3&amp;G4</f>
    //<v>shared_value</v>
    //</c>
    //</row>
    //</sheetData>
    //</worksheet>
    //"#;
    //let mut mock = SharedStringsMock::new();
    //let mut xml = XMLNode::from(source);
    //mock.add_shared_string("zero");
    //mock.add_shared_string("one");
    //mock.add_shared_string("two");
    //mock.add_shared_string("three");
    //let mut sheet = SheetData::new(&mut xml, &mut mock);
    //sheet.add_value(Cell::new("new-data", "A1"));
    //let new_source = r#"
    //<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
    //<worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006" mc:Ignorable="x14ac xr xr2 xr3" xmlns:x14ac="http://schemas.microsoft.com/office/spreadsheetml/2009/9/ac" xmlns:xr="http://schemas.microsoft.com/office/spreadsheetml/2014/revision" xmlns:xr2="http://schemas.microsoft.com/office/spreadsheetml/2015/revision2" xmlns:xr3="http://schemas.microsoft.com/office/spreadsheetml/2016/revision3" xr:uid="{44FEEDED-D128-4496-B199-BCD526D1EB2C}">
    //<sheetData>
    //<row r="1" spans="2:19" x14ac:dyDescent="0.4">
    //<c r="A1" s="15" t="s">
    //<v>new-data</v>
    //</c>
    //</row>
    //<row r="2" spans="2:19" x14ac:dyDescent="0.4">
    //<c r="B2" s="15" t="s">
    //<v>0</v>
    //</c>
    //<c r="C2" s="12"/>
    //<c r="D2" s="16"/>
    //<c r="E2" s="13"/>
    //<c r="J2" s="15" t="s">
    //<v>1</v>
    //</c>
    //<c r="K2" s="13"/>
    //<c r="P2" s="15" t="s">
    //<v>2</v>
    //</c>
    //<c r="Q2" s="13"/>
    //</row>
    //<row r="3" spans="2:19" x14ac:dyDescent="0.4">
    //<c r="B3" s="4"/>
    //<c r="C3" s="15" t="s">
    //<v>3</v>
    //</c>
    //<c r="F6">
    //<v>50</v>
    //</c>
    //<c r="D3" s="16"/>
    //<c r="E3" s="3" t="s">
    //<v>4</v>
    //</c>
    //<c r="H4" t="str">
    //<f>$E$3&amp;G4</f>
    //<v>shared_value</v>
    //</c>
    //</row>
    //</sheetData>
    //</worksheet>
    //"#;
    //assert_eq!(&xml, &XMLNode::from(new_source))
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
