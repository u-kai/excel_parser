use crate::xml::nodes::node::XMLNode;

use super::cell::{Cell, CellIndex};

#[derive(Debug, PartialEq, Eq)]
pub struct XMLSheet {
    shared_values: SharedValues,
    refarence_values: RefarenceValues,
}
impl XMLSheet {
    pub fn new() -> Self {
        XMLSheet {
            shared_values: SharedValues::new(),
            refarence_values: RefarenceValues::new(),
        }
    }
    pub fn new_with_source(source: &str) -> Self {
        let xml_node = XMLNode::from(source);
        let work_sheet = xml_node.search_node("worksheet").unwrap();
        let sheet_data = work_sheet.search_node("sheetData").unwrap();
        let rows = sheet_data.search_all_nodes("row").unwrap();
        let c_nodes = rows
            .iter()
            .filter_map(|node| node.search_all_nodes("c"))
            .flatten()
            .collect::<Vec<_>>();

        XMLSheet {
            shared_values: SharedValues::from_c_nodes(&c_nodes),
            refarence_values: RefarenceValues::from_c_nodes(&c_nodes),
        }
    }
}
#[derive(Debug, PartialEq, Eq)]
struct SharedValues {
    values: Vec<Cell<String>>,
}
impl SharedValues {
    pub fn new() -> Self {
        SharedValues { values: Vec::new() }
    }
    pub fn from_c_nodes(c_nodes: &Vec<&XMLNode>) -> Self {
        let values = c_nodes
            .iter()
            .filter(|c_node| {
                c_node.is_containe_key_value("t", "str")
                    || !(c_node.is_containe_key_value("t", "s"))
            })
            .filter_map(|c_node| {
                let cell_index = c_node.search_element("r").unwrap();
                let v_node = c_node.search_node("v");
                if v_node.is_some() {
                    Some(Cell::new(
                        v_node.unwrap().get_child_text(0).unwrap().to_string(),
                        cell_index,
                    ))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        SharedValues { values }
    }
    pub fn add_value(&mut self, cell: Cell<String>) {
        self.values.push(cell)
    }
    fn get_cell(&self, cell_index: &CellIndex) -> Option<&str> {
        if let Some(s) = self.values.iter().find(|cell| cell.is_index(cell_index)) {
            Some(s.get_value().as_str())
        } else {
            None
        }
    }
}
#[derive(Debug, PartialEq, Eq)]
struct RefarenceValues {
    values: Vec<Cell<usize>>,
}
impl RefarenceValues {
    pub fn new() -> Self {
        RefarenceValues { values: Vec::new() }
    }
    pub fn from_c_nodes(c_nodes: &Vec<&XMLNode>) -> Self {
        let values = c_nodes
            .iter()
            .filter(|c_node| c_node.is_containe_key_value("t", "s"))
            .filter_map(|c_node| {
                let cell_index = c_node.search_element("r").unwrap();
                let v_node = c_node.search_node("v");
                if v_node.is_some() {
                    Some(Cell::new(
                        v_node
                            .unwrap()
                            .get_child_text(0)
                            .unwrap()
                            .parse::<usize>()
                            .unwrap(),
                        cell_index,
                    ))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        RefarenceValues { values }
    }
    pub fn add_value(&mut self, cell: Cell<usize>) {
        self.values.push(cell)
    }
    fn get_cell(&self, cell_index: &CellIndex) -> Option<usize> {
        if let Some(s) = self.values.iter().find(|cell| cell.is_index(cell_index)) {
            Some(*s.get_value())
        } else {
            None
        }
    }
}
pub trait Refarences {
    fn get_refarence_cell(&self, cell_index: &CellIndex) -> Option<usize>;
}
impl Refarences for XMLSheet {
    fn get_refarence_cell(&self, cell_index: &CellIndex) -> Option<usize> {
        self.refarence_values.get_cell(&cell_index)
    }
}
pub trait Shareds {
    fn get_shared_cell(&self, cell_index: &CellIndex) -> Option<&str>;
}
impl Shareds for XMLSheet {
    fn get_shared_cell(&self, cell_index: &CellIndex) -> Option<&str> {
        self.shared_values.get_cell(&cell_index)
    }
}
#[cfg(test)]
mod xml_sheet_test {
    use crate::excel::{
        cell::{Cell, CellIndex},
        xml_sheet::*,
    };

    use super::XMLSheet;

    #[test]
    fn new_test() {
        let source = r#"
            <?xml version="1.0" encoding="UTF-8" standalone="yes"?>
            <worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006" mc:Ignorable="x14ac xr xr2 xr3" xmlns:x14ac="http://schemas.microsoft.com/office/spreadsheetml/2009/9/ac" xmlns:xr="http://schemas.microsoft.com/office/spreadsheetml/2014/revision" xmlns:xr2="http://schemas.microsoft.com/office/spreadsheetml/2015/revision2" xmlns:xr3="http://schemas.microsoft.com/office/spreadsheetml/2016/revision3" xr:uid="{44FEEDED-D128-4496-B199-BCD526D1EB2C}">
                <sheetData>
                    <row r="2" spans="2:19" x14ac:dyDescent="0.4">
                        <c r="B2" s="15" t="s">
                            <v>43</v>
                        </c>
                        <c r="C2" s="12"/>
                        <c r="D2" s="16"/>
                        <c r="E2" s="13"/>
                        <c r="J2" s="15" t="s">
                            <v>44</v>
                        </c>
                        <c r="K2" s="13"/>
                        <c r="P2" s="15" t="s">
                            <v>59</v>
                        </c>
                        <c r="Q2" s="13"/>
                    </row>
                    <row r="3" spans="2:19" x14ac:dyDescent="0.4">
                        <c r="B3" s="4"/>
                        <c r="C3" s="15" t="s">
                            <v>0</v>
                        </c>
                        <c r="F6">
                            <v>50</v>
                        </c>
                        <c r="D3" s="16"/>
                        <c r="E3" s="3" t="s">
                            <v>68</v>
                        </c>
                        <c r="H4" t="str">
                            <f>$E$3&amp;G4</f>
                            <v>shared_value</v>
                        </c>
                    </row>
                </sheetData>
            </worksheet>
        "#;
        let expect = XMLSheet::new_with_source(source);
        let shared = SharedValues {
            values: vec![
                Cell::new("50".to_string(), "F6"),
                Cell::new("shared_value".to_string(), "H4"),
            ],
        };
        let refarence = RefarenceValues {
            values: vec![
                Cell::new(43, "B2"),
                Cell::new(44, "J2"),
                Cell::new(59, "P2"),
                Cell::new(0, "C3"),
                Cell::new(68, "E3"),
            ],
        };
        let to_be = XMLSheet {
            shared_values: shared,
            refarence_values: refarence,
        };
        assert_eq!(expect, to_be);
    }
    #[test]
    fn get_value_test() {
        let source = r#"
            <?xml version="1.0" encoding="UTF-8" standalone="yes"?>
            <worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006" mc:Ignorable="x14ac xr xr2 xr3" xmlns:x14ac="http://schemas.microsoft.com/office/spreadsheetml/2009/9/ac" xmlns:xr="http://schemas.microsoft.com/office/spreadsheetml/2014/revision" xmlns:xr2="http://schemas.microsoft.com/office/spreadsheetml/2015/revision2" xmlns:xr3="http://schemas.microsoft.com/office/spreadsheetml/2016/revision3" xr:uid="{44FEEDED-D128-4496-B199-BCD526D1EB2C}">
                <sheetData>
                    <row r="2" spans="2:19" x14ac:dyDescent="0.4">
                        <c r="B2" s="15" t="s">
                            <v>43</v>
                        </c>
                        <c r="C2" s="12"/>
                        <c r="D2" s="16"/>
                        <c r="E2" s="13"/>
                        <c r="J2" s="15" t="s">
                            <v>44</v>
                        </c>
                        <c r="K2" s="13"/>
                        <c r="P2" s="15" t="s">
                            <v>59</v>
                        </c>
                        <c r="Q2" s="13"/>
                    </row>
                    <row r="3" spans="2:19" x14ac:dyDescent="0.4">
                        <c r="B3" s="4"/>
                        <c r="C3" s="15" t="s">
                            <v>0</v>
                        </c>
                        <c r="F6">
                            <v>50</v>
                        </c>
                        <c r="D3" s="16"/>
                        <c r="E3" s="3" t="s">
                            <v>68</v>
                        </c>
                        <c r="H4" t="str">
                            <f>$E$3&amp;G4</f>
                            <v>shared_value</v>
                        </c>
                    </row>
                </sheetData>
            </worksheet>
        "#;
        let sheet = XMLSheet::new_with_source(source);
        assert_eq!(sheet.get_shared_cell(&CellIndex::new("F6")), Some("50"));
        assert_eq!(sheet.get_refarence_cell(&CellIndex::new("P2")), Some(59));
        assert_eq!(sheet.get_shared_cell(&CellIndex::new("P2")), None);
        assert_eq!(sheet.get_refarence_cell(&CellIndex::new("F6")), None);
        assert_eq!(sheet.get_refarence_cell(&CellIndex::new("P100")), None);
    }
}
