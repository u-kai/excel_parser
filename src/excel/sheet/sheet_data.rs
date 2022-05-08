use crate::{
    excel::{cell::CellIndex, xmls::shared_strings::SharedStringsInterface},
    xml::nodes::{node::XMLNode, node_type::NodeType},
};

use super::cell_node::CellNode;

#[derive(Debug, PartialEq, Eq)]
struct SheetDataNode<'a>(&'a mut XMLNode);
impl<'a> SheetDataNode<'a> {
    pub fn new(node: &'a mut XMLNode) -> Self {
        SheetDataNode(node)
    }
    pub fn get_node(&mut self) -> &mut XMLNode {
        self.0
    }
}

pub struct Rows<'a>(Vec<&'a mut XMLNode>);
impl<'a> Rows<'a> {
    pub fn new(node: Vec<&'a mut XMLNode>) -> Self {
        Rows(node)
    }
    pub fn get_index_row(&mut self, index: CellIndex) -> Option<&XMLNode> {
        let row_index = index.get_row_index().to_string();
        for row in self.get_rows() {
            if row.is_containe_key_value("r", row_index.as_str()) {
                return Some(row);
            }
        }
        None
    }
    pub fn get_index_row_mut(&mut self, index: CellIndex) -> Option<&&mut XMLNode> {
        let row_index = index.get_row_index().to_string();
        for row in self.get_rows() {
            if row.is_containe_key_value("r", row_index.as_str()) {
                return Some(row);
            }
        }
        None
    }
    pub fn get_index_cell(&mut self, index: CellIndex) -> Option<&str> {
        None
        //let row_node = self.get_index_row_mut(index);
        //if let Some(row) = row_node.map(|node| *node) {
        //let mut c_nodes = row.search_all_nodes_mut("c");
        //if let Some(c_nodes) = c_nodes {
        ////let cells = c_nodes
        ////.iter_mut()
        ////.map(|node|CellNode::new(node))
        //None
        //} else {
        //None
        //}
        //} else {
        //None
        //}
    }
    //pub fn get_index_cell_mut(&mut self, index: CellIndex) -> Option<&str> {
    //let row_node = self.get_index_row_mut(index);
    //if let Some(row) = row_node {
    //row.search_all_nodes_mut("c");
    //None
    //} else {
    //None
    //}
    //}
    pub fn get_rows(&self) -> &Vec<&mut XMLNode> {
        &self.0
    }
}

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
    fn get_cell_v(&'a self, index: CellIndex) -> &str {
        //let c_node = self
        //.node
        //.search_child_by_id_mut("r", index.get_value())
        //.unwrap();
        //let c_node = CellNode::new(c_node, self.shared_strings);
        //c_node.get_v_text()
        ""
    }
    //fn get_index_row(&'a mut self, index: CellIndex) -> Option<&'a mut XMLNode> {
    ////let rows = self.node.search_all_nodes_mut("row");
    ////let row_index = index.get_row_index().to_string();
    ////if let Some(mut rows) = rows {
    ////for row in rows.iter_mut() {
    ////if row.is_containe_key_value("c", index.get_value()) {
    ////return Some(*row);
    ////}
    ////}
    ////return None;
    //////let row = rows
    //////.iter()
    //////.filter(|row| row.is_containe_key_value("r", row_index.as_str()))
    //////.map(|node| *node)
    //////.take(0);
    //////let r = row.next();

    //////for row in rows.iter_mut() {
    //////return  Some(row)  ;
    //////}
    ////}
    ////None
    //}
}

impl<'a, T: SharedStringsInterface> SheetOperator for SheetData<'a, T> {
    fn get_cell(&mut self, index: CellIndex) -> &str {
        let rows = self.node.search_all_nodes_mut("row");
        let row_index = index.get_row_index().to_string();
        if let Some(mut rows) = rows {
            let row = rows
                .iter_mut()
                .filter(|row| row.is_containe_key_value("r", row_index.as_str()))
                .next();
            if let Some(row) = row {
                let mut r = &mut *row;
                let mut cell_nodes = row.search_all_nodes_mut("c");
                if let Some(mut cell_nodes) = cell_nodes {
                    for node in cell_nodes.iter_mut() {
                        let cell_node = CellNode::new(node, self.shared_strings);
                    }
                    ""
                } else {
                    ""
                }
            } else {
                ""
            }
        } else {
            ""
        }
    }
}
pub trait SheetOperator {
    fn get_cell(&mut self, index: CellIndex) -> &str;
}
//impl SheetData {
//pub fn new(sheet: &str) -> Self {
//let sheet_node = XMLNode::from(sheet);
//let work_sheet = sheet_node.search_node("worksheet").unwrap();
//let sheet_data = work_sheet.search_node("sheetData").unwrap();
//let node = sheet_data.drain();
//SheetData { node }
//}
//}
//impl SheetValue {
//pub fn new(sheet_node: &XMLNode) -> Self {
//let work_sheet = sheet_node.search_node("worksheet").unwrap();
//let sheet_data = work_sheet.search_node("sheetData").unwrap();
//let rows = sheet_data.search_all_nodes("row").unwrap();
//let c_nodes = rows
//.iter()
//.filter_map(|node| node.search_all_nodes("c"))
//.flatten()
//.collect::<Vec<_>>();
//SheetValue {
//shared_values: SharedValues::from_c_nodes(&c_nodes),
//refarence_values: RefarenceValues::from_c_nodes(&c_nodes),
//}
//}
//}
//#[derive(Debug, PartialEq, Eq)]
//struct SharedValues {
//values: Vec<Cell<String>>,
//}
//impl SharedValues {
//pub fn new() -> Self {
//SharedValues { values: Vec::new() }
//}
//pub fn from_c_nodes(c_nodes: &Vec<&XMLNode>) -> Self {
//let values = c_nodes
//.iter()
//.filter(|c_node| {
//c_node.is_containe_key_value("t", "str")
//|| !(c_node.is_containe_key_value("t", "s"))
//})
//.filter_map(|c_node| {
//let cell_index = c_node.search_element("r").unwrap();
//let v_node = c_node.search_node("v");
//if v_node.is_some() {
//Some(Cell::new(
//v_node.unwrap().get_child_text(0).unwrap().to_string(),
//cell_index,
//))
//} else {
//None
//}
//})
//.collect::<Vec<_>>();
//SharedValues { values }
//}
//pub fn add_value(&mut self, cell: Cell<String>) {
//self.values.push(cell)
//}
//fn get_cell(&self, cell_index: &CellIndex) -> Option<&str> {
//if let Some(s) = self.values.iter().find(|cell| cell.is_index(cell_index)) {
//Some(s.get_value().as_str())
//} else {
//None
//}
//}
//}
//#[derive(Debug, PartialEq, Eq)]
//struct RefarenceValues {
//values: Vec<Cell<usize>>,
//}
//impl RefarenceValues {
//pub fn new() -> Self {
//RefarenceValues { values: Vec::new() }
//}
//pub fn from_c_nodes(c_nodes: &Vec<&XMLNode>) -> Self {
//let values = c_nodes
//.iter()
//.filter(|c_node| c_node.is_containe_key_value("t", "s"))
//.filter_map(|c_node| {
//let cell_index = c_node.search_element("r").unwrap();
//let v_node = c_node.search_node("v");
//if v_node.is_some() {
//Some(Cell::new(
//v_node
//.unwrap()
//.get_child_text(0)
//.unwrap()
//.parse::<usize>()
//.unwrap(),
//cell_index,
//))
//} else {
//None
//}
//})
//.collect::<Vec<_>>();
//RefarenceValues { values }
//}
//pub fn add_value(&mut self, cell: Cell<usize>) {
//self.values.push(cell)
//}
//fn get_cell(&self, cell_index: &CellIndex) -> Option<usize> {
//if let Some(s) = self.values.iter().find(|cell| cell.is_index(cell_index)) {
//Some(*s.get_value())
//} else {
//None
//}
//}
//}
//pub trait Refarences {
//fn get_refarence_cell(&self, cell_index: &CellIndex) -> Option<usize>;
//fn get_all_cell(&self) -> &Vec<Cell<usize>>;
//}
//impl Refarences for SheetValue {
//fn get_refarence_cell(&self, cell_index: &CellIndex) -> Option<usize> {
//self.refarence_values.get_cell(&cell_index)
//}
//fn get_all_cell(&self) -> &Vec<Cell<usize>> {
//&self.refarence_values.values
//}
//}
//pub trait Shareds {
//fn get_shared_cell(&self, cell_index: &CellIndex) -> Option<&str>;
//fn get_all_cell(&self) -> &Vec<Cell<String>>;
//}
//impl Shareds for SheetValue {
//fn get_shared_cell(&self, cell_index: &CellIndex) -> Option<&str> {
//self.shared_values.get_cell(&cell_index)
//}
//fn get_all_cell(&self) -> &Vec<Cell<String>> {
//&self.shared_values.values
//}
//}
#[cfg(test)]
mod xml_sheet_test {
    use crate::{
        excel::{
            cell::{Cell, CellIndex},
            sheet::sheet_data::{SheetData, SheetOperator},
            sheet_data::*,
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
