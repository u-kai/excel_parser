use crate::xml::nodes::node::XMLNode;

use super::cell::Cell;

#[derive(Debug, PartialEq, Eq)]
pub struct XMLSheet {
    name: String,
    shared_values: Vec<Cell>,
    refarence_values: Vec<Cell>,
}
impl XMLSheet {
    pub fn new(sheet_name: impl Into<String>) -> Self {
        XMLSheet {
            name: sheet_name.into(),
            shared_values: Vec::new(),
            refarence_values: Vec::new(),
        }
    }
    pub fn new_with_source(sheet_name: impl Into<String>, source: &str) -> Self {
        let xml_node = XMLNode::from(source);
        let work_sheet = xml_node.search_node("worksheet").unwrap();
        let sheet_data = work_sheet.search_node("sheetData").unwrap();
        let rows = sheet_data.search_nodes("row").unwrap();
        let shared_values = rows
            .iter()
            .filter_map(|node| node.search_nodes("c"))
            .map(|c_nodes| XMLSheet::get_shared_value(c_nodes))
            .flatten()
            .collect::<Vec<_>>();
        XMLSheet {
            name: sheet_name.into(),
            shared_values,
            refarence_values: Vec::new(),
        }
    }
    fn get_shared_value<'a>(nodes: Vec<&XMLNode>) -> Vec<Cell> {
        let d = nodes
            .iter()
            .filter(|node| {
                node.is_containe_key_value("t", "str") || !(node.is_containe_key_value("t", "s"))
            })
            .filter_map(|node| {
                let cell_index = node.search_element("r").unwrap();
                let v_node = node.search_node("v");
                if v_node.is_some() {
                    Some(Cell::new(
                        v_node.unwrap().get_child_charcter(0).unwrap(),
                        cell_index,
                    ))
                } else {
                    None
                }
            })
            .collect();
        d
    }
    pub fn add_refarence_value(&mut self, cell: Cell) {
        self.refarence_values.push(cell)
    }
    pub fn add_shared_value(&mut self, cell: Cell) {
        self.shared_values.push(cell)
    }
}

#[cfg(test)]
mod xml_sheet_test {
    use crate::excel::cell::Cell;

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
        let expect = XMLSheet::new_with_source("sheet1", source);
        let mut to_be = XMLSheet::new("sheet1");
        to_be.add_shared_value(Cell::new("50", "F6"));
        to_be.add_refarence_value(Cell::new("43", "B2"));
        to_be.add_refarence_value(Cell::new("44", "J2"));
        to_be.add_refarence_value(Cell::new("0", "C3"));
        to_be.add_refarence_value(Cell::new("68", "E3"));
        to_be.add_shared_value(Cell::new("shared_value", "H4"));
        assert_eq!(expect, to_be);
    }
}
