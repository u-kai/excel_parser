use std::{cell::RefCell, fmt::Debug};

use super::shared_strings::SharedStringsInterface;
use crate::{
    excel::cells::{
        cell::{CellIndex, ColumnAlphabet, ECell},
        cell_node::CellNode,
    },
    xml::nodes::{node::XMLNode, node_type::NodeType},
};

#[derive(PartialEq, Eq, Debug)]
pub struct Sheet<'a, S: SharedStringsInterface<'a>> {
    sheet_name: String,
    node: XMLNode<'a>,
    shared_strings: RefCell<&'a S>,
}
impl<'a, S: SharedStringsInterface<'a>> Sheet<'a, S> {
    pub fn new(sheet_name: &str, source: &'a str, shared_strings: &'a S) -> Self {
        let node = XMLNode::from(source);
        Sheet {
            sheet_name: sheet_name.to_string(),
            node,
            shared_strings: RefCell::new(shared_strings),
        }
    }
    pub fn get_sheet_name(&self) -> &str {
        &self.sheet_name
    }
    fn get_sheet_data_node(&self) -> &XMLNode {
        self.node
            .search_node("worksheet")
            .expect(format!("not found worksheet\n{:?}", &self.node).as_str())
            .search_node("sheetData")
            .expect(format!("not found sheetData\n{:?}", &self.node).as_str())
    }
    fn get_sheet_data_node_mut(&mut self) -> &mut XMLNode<'a> {
        self.node
            .search_node_mut("worksheet")
            .unwrap()
            .search_node_mut("sheetData")
            .unwrap()
    }
    pub fn get_all_row_index(&self) -> Vec<usize> {
        let rows = self.get_sheet_data_node().search_all_nodes("row");
        if let Some(rows) = rows {
            return rows
                .iter()
                .map(|row| row.search_element("r").unwrap())
                .map(|index| index.parse::<usize>().unwrap())
                .collect();
        }
        Vec::new()
    }
    fn get_cell_v(&self, index: &str) -> Option<String> {
        let index = CellIndex::new(index);
        let c_node = self
            .get_sheet_data_node()
            .search_child_by_id("r", index.get_value());
        if let Some(c_node) = c_node {
            let shared_strings = self.shared_strings.borrow().get_values();
            let c_node = CellNode::new(c_node, shared_strings);
            c_node.get_v_text()
        } else {
            None
        }
    }
    fn get_max_column_index(&self) -> usize {
        self.get_all_cell()
            .iter()
            .map(|row| row.len())
            .max()
            .unwrap()
    }
    pub fn to_xml(&self) -> String {
        self.node.to_string()
    }
}
impl<'a, S: SharedStringsInterface<'a>> WorkSheet<'a> for Sheet<'a, S> {
    fn get_cell(&self, cell_index: &str) -> Option<String> {
        self.get_cell_v(cell_index)
    }
    fn get_column_range(&self, start: &str, end: &str) -> Vec<Vec<Option<String>>> {
        let start = ColumnAlphabet::new(start);
        let end = ColumnAlphabet::new(end);
        let max = self.get_max_column_index();
        if end.to_number() > max {
            return self
                .get_all_cell()
                .iter_mut()
                .map(|row| {
                    let mut row = row.drain((start.to_number() - 1)..max).collect::<Vec<_>>();
                    row.append(&mut [0..(end.to_number() - max)].map(|_| None).to_vec());
                    row
                })
                .collect();
        }
        self.get_all_cell()
            .iter_mut()
            .map(|row| {
                row.drain((start.to_number() - 1)..end.to_number())
                    .collect::<Vec<_>>()
            })
            .collect()
    }
    fn get_row(&self, row_index: usize) -> Vec<Option<String>> {
        let row_node = self
            .get_sheet_data_node()
            .search_child_by_id("r", row_index.to_string().as_str());
        if let Some(row_node) = row_node {
            let c_nodes = row_node.search_all_nodes("c").unwrap();
            let mut result = vec![];
            for c_node in c_nodes.iter() {
                let cell_index = CellIndex::new(c_node.search_element("r").unwrap());
                let column_index = cell_index.get_column_index();
                let nones_range = (result.len() + 1)..column_index;
                nones_range.for_each(|_| result.push(None));
                let shared_strings = self.shared_strings.borrow().get_values();
                let c_node = CellNode::new(c_node, shared_strings);
                result.push(c_node.get_v_text());
            }
            result
        } else {
            vec![None]
        }
    }
    fn get_column(&self, s: &str) -> Vec<Option<String>> {
        let rows = self.get_sheet_data_node().search_all_nodes("row");
        if rows.is_none() {
            return vec![None];
        }
        let s = ColumnAlphabet::new(s);
        let mut result = Vec::new();
        for row in rows.unwrap().iter() {
            let mut is_exist = false;
            for c in row.search_all_nodes("c").unwrap().iter() {
                let element = c.search_element("r");
                if element.is_none() {
                    continue;
                }
                let cell_index = CellIndex::new(c.search_element("r").unwrap());
                let column_index = cell_index.get_column_index();
                if column_index == s.to_number() {
                    result.push(self.get_cell(cell_index.get_value()));
                    is_exist = true;
                    break;
                }
            }
            if is_exist == false {
                result.push(None)
            }
        }

        result
    }
    fn get_all_cell(&self) -> Vec<Vec<Option<String>>> {
        let indexs = self.get_all_row_index();
        let before_t = indexs.iter().map(|i| self.get_row(*i)).collect::<Vec<_>>();
        let max_len = before_t.iter().map(|vec| vec.len()).max().unwrap();
        let mut result = Vec::new();
        for (_, row) in before_t.iter().enumerate() {
            let mut buf = Vec::new();
            for j in 0..max_len {
                let cell = row.get(j);
                if let Some(cell) = cell {
                    buf.push(cell.clone());
                } else {
                    buf.push(None);
                }
            }
            result.push(buf)
        }
        result
    }
    fn set_cell(&mut self, cell: &'a ECell<'a>) -> () {
        let index = cell.get_index();
        let value = cell.get_value();
        let maybe_child = self
            .get_sheet_data_node_mut()
            .search_child_by_id_mut("r", index.get_value());
        if let Some(cell) = maybe_child {
            if let Some(v_node) = cell.search_node_mut("v") {
                v_node.change_text(value);
            } else {
                let mut v_child = XMLNode::new("v", NodeType::Element);
                v_child.add_text(value);
                cell.add_node(v_child);
                cell.add_element("t", vec!["str"]);
                cell.add_element("r", vec![index.get_value()]);
                cell.set_node_type(NodeType::Element);
            }
            return;
        }
    }
}
pub trait WorkSheet<'a> {
    fn get_cell(&self, cell_index: &str) -> Option<String>;
    fn get_row(&self, u: usize) -> Vec<Option<String>>;
    fn get_column(&self, s: &str) -> Vec<Option<String>>;
    fn get_all_cell(&self) -> Vec<Vec<Option<String>>>;
    fn get_column_range(&self, start: &str, end: &str) -> Vec<Vec<Option<String>>>;
    fn set_cell(&mut self, cell: &'a ECell<'a>) -> ();
}

#[cfg(test)]
mod xml_sheet_test {
    use crate::excel::{
        cells::cell::ECell,
        xmls::{
            shared_strings::SharedStringsInterface,
            sheet::{Sheet, WorkSheet},
        },
    };

    use super::mock_shared_strings::SharedStringsMock;

    const SOURCE1: &str = r#"
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
    const SOURCE2: &str = r#"
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006" mc:Ignorable="x14ac xr xr2 xr3" xmlns:x14ac="http://schemas.microsoft.com/office/spreadsheetml/2009/9/ac" xmlns:xr="http://schemas.microsoft.com/office/spreadsheetml/2014/revision" xmlns:xr2="http://schemas.microsoft.com/office/spreadsheetml/2015/revision2" xmlns:xr3="http://schemas.microsoft.com/office/spreadsheetml/2016/revision3" xr:uid="{44FEEDED-D128-4496-B199-BCD526D1EB2C}">
<sheetData>
<row r="1" spans="2:19" x14ac:dyDescent="0.4">
<c r="A1">
<v>a</v>
</c>
</row>
<row r="2" spans="2:19" x14ac:dyDescent="0.4">
<c r="A2"/>
<c r="B2">
<v>b</v>
</c>
<c r="C2" s="12"/>
<c r="D2" s="16"/>
<c r="E2" s="13"/>
</row>
<row r="3" spans="2:19" x14ac:dyDescent="0.4">
<c r="C3">
<v>c</v>
</c>
</row>
<row r="4" spans="2:19" x14ac:dyDescent="0.4">
<c r="D4">
<v>d</v>
</c>
</row>
<row r="5" spans="2:19" x14ac:dyDescent="0.4">
<c r="A5" s="15" t="s">
<v>0</v>
</c>
<c r="E5">
<v>e</v>
</c>
<c r="G5">
<v>0</v>
</c>
</row>
</sheetData>
</worksheet>
"#;
    #[test]
    fn get_cell_test() {
        let mut mock = SharedStringsMock::new();
        mock.add_shared_string("zero");
        mock.add_shared_string("one");
        mock.add_shared_string("two");
        mock.add_shared_string("three");
        let expect = Sheet::new("test", SOURCE1, &mut mock);
        assert_eq!(expect.get_cell("B2"), Some("zero".to_string()));
        assert_eq!(expect.get_cell("J2"), Some("one".to_string()));
        assert_eq!(expect.get_cell("XX3"), None);
    }
    #[test]
    fn get_row_test() {
        let mut mock = SharedStringsMock::new();
        mock.add_shared_string("zero");
        mock.add_shared_string("one");
        mock.add_shared_string("two");
        mock.add_shared_string("three");
        mock.add_shared_string("four");
        let sheet = Sheet::new("test", SOURCE1, &mut mock);
        assert_eq!(
            sheet.get_row(2),
            vec![
                None,
                Some("zero".to_string()),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some("one".to_string()),
                None,
                None,
                None,
                None,
                None,
                Some("two".to_string()),
                None
            ]
        );
        assert_eq!(
            sheet.get_row(3),
            vec![
                None,
                None,
                Some("three".to_string()),
                None,
                Some("four".to_string()),
                Some("50".to_string()),
                None,
                Some("shared_value".to_string()),
            ]
        );
    }
    #[test]
    /// expect ECell
    /// | |A|B|C|D|E|F|G|
    /// |-|-|-|-|-|-|-|-|
    /// |1|a| | | | | | |
    /// |2| |b| | | | | |
    /// |3| | |c| | | | |
    /// |4| | | |d| | | |
    /// |5|???| | | |e| |0|
    fn get_column_test() {
        let mut shareds = SharedStringsMock::new();
        shareds.add_shared_string("???");
        let sheet = Sheet::new("test", SOURCE2, &mut shareds);
        assert_eq!(
            sheet.get_column("A"),
            vec![
                Some("a".to_string()),
                None,
                None,
                None,
                Some("???".to_string())
            ]
        );
        assert_eq!(
            sheet.get_column("G"),
            vec![None, None, None, None, Some("0".to_string())]
        );
        assert_eq!(
            sheet.get_column("D"),
            vec![None, None, None, Some("d".to_string()), None]
        );
    }
    #[test]
    /// expect ECell
    /// | |A|B|C|D|E|F|G|
    /// |-|-|-|-|-|-|-|-|
    /// |1|a| | | | | | |
    /// |2| |b| | | | | |
    /// |3| | |c| | | | |
    /// |4| | | |d| | | |
    /// |5|???| | | |e| |0|
    fn get_all_cell_test() {
        let mut shareds = SharedStringsMock::new();
        shareds.add_shared_string("???");
        let sheet = Sheet::new("test", SOURCE2, &mut shareds);
        assert_eq!(
            sheet.get_all_cell(),
            vec![
                vec![Some("a".to_string()), None, None, None, None, None, None],
                vec![None, Some("b".to_string()), None, None, None, None, None],
                vec![None, None, Some("c".to_string()), None, None, None, None],
                vec![None, None, None, Some("d".to_string()), None, None, None],
                vec![
                    Some("???".to_string()),
                    None,
                    None,
                    None,
                    Some("e".to_string()),
                    None,
                    Some("0".to_string())
                ],
            ]
        );
    }
    #[test]
    /// expect ECell
    /// | |A|B|C|D|E|F|G|
    /// |-|-|-|-|-|-|-|-|
    /// |1|a| | | | | | |
    /// |2| |b| | | | | |
    /// |3| | |c| | | | |
    /// |4| | | |d| | | |
    /// |5|???| | | |e| |0|
    fn get_column_range_test() {
        let mut shareds = SharedStringsMock::new();
        shareds.add_shared_string("???");
        let sheet = Sheet::new("test", SOURCE2, &mut shareds);
        assert_eq!(
            sheet.get_column_range("B", "E"),
            vec![
                vec![None, None, None, None],
                vec![Some("b".to_string()), None, None, None],
                vec![None, Some("c".to_string()), None, None],
                vec![None, None, Some("d".to_string()), None],
                vec![None, None, None, Some("e".to_string()),],
            ]
        );
    }
    #[test]
    fn get_max_column_index_test() {
        let mut shareds = SharedStringsMock::new();
        shareds.add_shared_string("???");
        let sheet = Sheet::new("test", SOURCE2, &mut shareds);
        assert_eq!(sheet.get_max_column_index(), 7);
    }
    #[test]
    fn set_cell_test() {
        let mut shareds = SharedStringsMock::new();
        shareds.add_shared_string("???");
        let mut sheet = Sheet::new("test", SOURCE2, &mut shareds);
        let new_cell = ECell::new("new-data", "A2");
        sheet.set_cell(&new_cell);
        assert_eq!(
            sheet.get_column("A"),
            vec![
                Some("a".to_string()),
                Some("new-data".to_string()),
                None,
                None,
                Some("???".to_string())
            ]
        )
    }
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
    impl<'a> SharedStringsInterface<'a> for SharedStringsMock {
        fn to_xml(&self) -> String {
            "".to_string()
        }
        fn get_values(&self) -> &Vec<String> {
            &self.values
        }
        fn get_shared_string(&self, index: usize) -> &str {
            self.values[index].as_str()
        }
        fn add_shared_string(&mut self, value: &'a str) -> () {
            self.values.push(value.to_string())
        }
    }
}
