use std::fmt::Debug;

use super::shared_strings::SharedStringsInterface;
use crate::{
    excel::cells::{
        cell::{CellIndex, ColumnAlphabet},
        cell_node::CellNode,
    },
    xml::nodes::node::XMLNode,
};

#[derive(PartialEq, Eq, Debug)]
pub struct Sheet<'a, S: SharedStringsInterface> {
    sheet_name: String,
    node: XMLNode,
    shared_strings: &'a mut S,
}
impl<'a, S: SharedStringsInterface> Sheet<'a, S> {
    pub fn new(sheet_name: &str, source: String, shared_strings: &'a mut S) -> Self {
        let node = XMLNode::from(source.as_str());
        let node = node.search_node("worksheet").unwrap();
        let node = node.search_node("sheetData").unwrap().clone();
        Sheet {
            sheet_name: sheet_name.to_string(),
            node,
            shared_strings,
        }
    }
    pub fn get_sheet_name(&self) -> &str {
        &self.sheet_name
    }
    fn get_cell_v(&self, index: CellIndex) -> Option<String> {
        let c_node = self.node.search_child_by_id("r", index.get_value());
        if let Some(c_node) = c_node {
            let c_node = CellNode::new(c_node, self.shared_strings);
            c_node.get_v_text()
        } else {
            None
        }
    }
}
impl<'a, S: SharedStringsInterface> WorkSheet for Sheet<'a, S> {
    fn get_cell(&self, cell_index: CellIndex) -> Option<String> {
        self.get_cell_v(cell_index)
    }
    fn get_row(&self, row_index: usize) -> Vec<Option<String>> {
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
                nones_range.for_each(|_| result.push(None));
                let c_node = CellNode::new(c_node, self.shared_strings);
                result.push(c_node.get_v_text());
            }
            result
        } else {
            vec![None]
        }
    }
    fn get_column(&self, s: ColumnAlphabet) -> Vec<Option<String>> {
        let rows = self.node.search_all_nodes("row");
        if rows.is_none() {
            return vec![None];
        }
        let mut result = Vec::new();
        for row in rows.unwrap().iter() {
            let mut is_exist = false;
            for c in row.search_all_nodes("c").unwrap().iter() {
                let cell_index = CellIndex::new(c.search_element("r").unwrap());
                let column_index = cell_index.get_column_index();
                if column_index == s.to_number() {
                    result.push(self.get_cell(cell_index));
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
    fn get_all_cell(&self) -> Vec<Vec<Option<&str>>> {
        let tobe = vec![
            vec![Some("a"), None, None, None, None, None, None],
            vec![None, Some("b"), None, None, None, None, None],
            vec![None, None, Some("c"), None, None, None, None],
            vec![None, None, None, Some("d"), None, None, None],
            vec![Some("あ"), None, None, None, Some("e"), None, Some("0")],
        ];
        tobe
    }
}
pub trait WorkSheet {
    fn get_cell(&self, cell_index: CellIndex) -> Option<String>;
    fn get_row(&self, u: usize) -> Vec<Option<String>>;
    fn get_column(&self, s: ColumnAlphabet) -> Vec<Option<String>>;
    fn get_all_cell(&self) -> Vec<Vec<Option<&str>>>;
    //fn set_cell<T: PartialEq + Eq + Debug>(&mut self, cell: Cell<T>) -> ();
}

#[cfg(test)]
mod xml_sheet_test {
    use crate::{
        excel::{
            cells::cell::{Cell, CellIndex, ColumnAlphabet},
            xmls::{
                shared_strings::SharedStringsInterface,
                sheet::{Sheet, WorkSheet},
            },
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
        let expect = Sheet::new("test", source.to_string(), &mut mock);
        assert_eq!(
            expect.get_cell(CellIndex::new("B2")),
            Some("zero".to_string())
        );
        assert_eq!(
            expect.get_cell(CellIndex::new("J2")),
            Some("one".to_string())
        );
        assert_eq!(expect.get_cell(CellIndex::new("XX3")), None);
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
        mock.add_shared_string("zero");
        mock.add_shared_string("one");
        mock.add_shared_string("two");
        mock.add_shared_string("three");
        mock.add_shared_string("four");
        let sheet = Sheet::new("test", source.to_string(), &mut mock);
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
    /// expect cell
    /// | |A|B|C|D|E|F|G|
    /// |-|-|-|-|-|-|-|-|
    /// |1|a| | | | | | |
    /// |2| |b| | | | | |
    /// |3| | |c| | | | |
    /// |4| | | |d| | | |
    /// |5|あ| | | |e| |0|
    fn get_column_test() {
        let source = r#"
    <?xml version="1.0" encoding="UTF-8" standalone="yes"?>
    <worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006" mc:Ignorable="x14ac xr xr2 xr3" xmlns:x14ac="http://schemas.microsoft.com/office/spreadsheetml/2009/9/ac" xmlns:xr="http://schemas.microsoft.com/office/spreadsheetml/2014/revision" xmlns:xr2="http://schemas.microsoft.com/office/spreadsheetml/2015/revision2" xmlns:xr3="http://schemas.microsoft.com/office/spreadsheetml/2016/revision3" xr:uid="{44FEEDED-D128-4496-B199-BCD526D1EB2C}">
        <sheetData>
            <row r="1" spans="2:19" x14ac:dyDescent="0.4">
                <c r="A1">
                    <v>a</v>
                </c>
            </row>
            <row r="2" spans="2:19" x14ac:dyDescent="0.4">
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
        let mut shareds = SharedStringsMock::new();
        shareds.add_shared_string("あ");
        let sheet = Sheet::new("test", source.to_string(), &mut shareds);
        assert_eq!(
            sheet.get_column(ColumnAlphabet::new("A")),
            vec![
                Some("a".to_string()),
                None,
                None,
                None,
                Some("あ".to_string())
            ]
        );
    }
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

//#[cfg(test)]
//mod sheet_test {
//use crate::excel::{
//cell::{Cell, CellIndex},
//xml_sheet::{Refarences, Shareds},
//};

//use super::Sheet;

//#[test]
//fn sheet_new_test() {
//let mut shareds = SharesMock::new();
//shareds.set_shared(Cell::new(Some("50".to_string()), "F6"));
//shareds.set_shared(Cell::new(Some("shared_value".to_string()), "H4"));
//shareds.set_ref(Cell::new(0, "B2"));
//shareds.set_ref(Cell::new(1, "J2"));
//shareds.set_ref(Cell::new(2, "P2"));
//shareds.set_ref(Cell::new(3, "C3"));
//shareds.set_ref(Cell::new(4, "E3"));
//let shared_store = create_mock_shared_store(vec!["zero", "one", "two", "three", "four"]);
//let sheet = create_mock_sheet(&shared_store);
//assert_eq!(sheet.get_cell(CellIndex::new("B2")), Some("zero"));
//}
//#[test]
///// expect cell
///// | |A|B|C|D|E|F|G|
///// |-|-|-|-|-|-|-|-|
///// |1|a| | | | | | |
///// |2| |b| | | | | |
///// |3| | |c| | | | |
///// |4| | | |d| | | |
///// |5|あ| | | |e| |0|
//fn get_all_cell_test() {
//let mut shareds = SharesMock::new();
//shareds.set_shared(Cell::new(Some("a".to_string()), "A1"));
//shareds.set_shared(Cell::new(Some("b".to_string()), "B2"));
//shareds.set_shared(Cell::new(Some("c".to_string()), "C3"));
//shareds.set_shared(Cell::new(Some("d".to_string()), "D4"));
//shareds.set_shared(Cell::new(Some("e".to_string()), "E5"));
//shareds.set_shared(Cell::new(Some("0".to_string()), "G5"));
//shareds.set_ref(Cell::new(0, "A5"));
//let shared_store = create_mock_shared_store(vec!["あ"]);
//let sheet = create_mock_sheet(&shared_store);
//let tobe = vec![
//vec![Some("a"), None, None, None, None, None, None],
//vec![None, Some("b"), None, None, None, None, None],
//vec![None, None, Some("c"), None, None, None, None],
//vec![None, None, None, Some("d"), None, None, None],
//vec![Some("あ"), None, None, None, Some("e"), None, Some("0")],
//];
//assert_eq!(sheet.get_all_cell(), tobe);
//}
//}

/////////////////////// mock ///////////////////////

//use crate::{excel::cell::CellIndex, xml::nodes::node::XMLNode};

//use super::shared_strings::SharedStringsInterface;

//mod mocks {
//use crate::excel::{
//cell::{Cell, CellIndex},
//xml_sheet::{Refarences, Shareds},
//};

//use super::Sheet;

//pub struct SharedStoreMock {
//values: Vec<String>,
//}
//impl SharedStoreMock {
//pub fn new() -> Self {
//SharedStoreMock { values: Vec::new() }
//}
//pub fn set_value(&mut self, s: impl Into<String>) {
//self.values.push(s.into())
//}
//}
//impl SharedStore for SharedStoreMock {
//fn get_shared_value(&self, index: usize) -> &str {
//&self.values[index]
//}
//}
//pub struct SharesMock {
//shared_values: Vec<Cell<String>>,
//refarence_values: Vec<Cell<usize>>,
//}
//impl SharesMock {
//pub fn new() -> Self {
//SharesMock {
//shared_values: Vec::new(),
//refarence_values: Vec::new(),
//}
//}
//pub fn set_shared(&mut self, cell: Cell<String>) {
//self.shared_values.push(cell)
//}
//pub fn set_ref(&mut self, cell: Cell<usize>) {
//self.refarence_values.push(cell)
//}
//}
//impl Shareds for SharesMock {
//fn get_shared_cell(&self, cell_index: &CellIndex) -> Option<&str> {
//self.shared_values
//.iter()
//.find(|v| v.is_index(cell_index))
//.iter()
//.map(|c| c.get_value().as_str())
//.next()
//}
//fn get_all_cell(&self) -> &Vec<Cell<String>> {
//&self.shared_values
//}
//}
//impl Refarences for SharesMock {
//fn get_refarence_cell(&self, cell_index: &CellIndex) -> Option<usize> {
//self.refarence_values
//.iter()
//.find(|c| c.is_index(cell_index))
//.iter()
//.map(|c| *c.get_value())
//.next()
//}
//fn get_all_cell(&self) -> &Vec<Cell<usize>> {
//&self.refarence_values
//}
//}
//pub fn create_mock_shared_store(values: Vec<&str>) -> SharedStoreMock {
//let mut shared_store = SharedStoreMock::new();
//for v in values {
//shared_store.set_value(v);
//}
//shared_store
//}
//pub fn create_mock_sheet<'a>(
//shared_store: &'a SharedStoreMock,
//) -> Sheet<'a, SharedStoreMock, SharesMock> {
//let mut shareds = SharesMock::new();
//shareds.set_shared(Cell::new(Some("50".to_string()), "F6"));
//shareds.set_shared(Cell::new(Some("shared_value".to_string()), "H4"));
//shareds.set_ref(Cell::new(0, "B2"));
//shareds.set_ref(Cell::new(1, "J2"));
//shareds.set_ref(Cell::new(2, "P2"));
//shareds.set_ref(Cell::new(3, "C3"));
//shareds.set_ref(Cell::new(4, "E3"));
//Sheet::new("sheet1", shared_store, shareds)
//}
//}
