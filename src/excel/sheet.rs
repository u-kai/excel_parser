use super::{
    cell::{Cell, CellIndex},
    shared_strings::SharedStore,
    xml_sheet::{Refarences, Shareds},
};

#[derive(PartialEq, Eq, Debug)]
pub struct Sheet<'a, S: SharedStore, T: Refarences + Shareds> {
    name: &'a str,
    cells: Vec<Cell<String>>,
    shared_store: S,
    shared: T,
}
impl<'a, S: SharedStore, T: Refarences + Shareds> Sheet<'a, S, T> {
    fn new(name: &'a str, shared_store: S, shared: T) -> Self {
        Sheet {
            name,
            cells: Vec::new(),
            shared_store,
            shared,
        }
    }
    fn get_cell(&self, cell_index: CellIndex) -> Option<&str> {
        if self.shared.get_shared_cell(&cell_index).is_some() {
            return self.shared.get_shared_cell(&cell_index);
        }
        if self.shared.get_refarence_cell(&cell_index).is_some() {
            let index = self.shared.get_refarence_cell(&cell_index).unwrap();
            return Some(self.shared_store.get_shared_value(index));
        }
        None
    }
}

#[cfg(test)]
mod sheet_test {
    use crate::excel::{
        cell::{Cell, CellIndex},
        shared_strings::SharedStore,
        xml_sheet::{Refarences, Shareds},
    };

    use super::Sheet;

    struct SharedStoreMock {
        values: Vec<String>,
    }
    impl SharedStoreMock {
        pub fn new() -> Self {
            SharedStoreMock { values: Vec::new() }
        }
        pub fn set_value(&mut self, s: impl Into<String>) {
            self.values.push(s.into())
        }
    }
    impl SharedStore for SharedStoreMock {
        fn get_shared_value(&self, index: usize) -> &str {
            &self.values[index]
        }
    }
    struct SharesMock {
        shared_values: Vec<Cell<String>>,
        refarence_values: Vec<Cell<usize>>,
    }
    impl SharesMock {
        pub fn new() -> Self {
            SharesMock {
                shared_values: Vec::new(),
                refarence_values: Vec::new(),
            }
        }
        pub fn set_shared(&mut self, cell: Cell<String>) {
            self.shared_values.push(cell)
        }
        pub fn set_ref(&mut self, cell: Cell<usize>) {
            self.refarence_values.push(cell)
        }
    }
    impl Shareds for SharesMock {
        fn get_shared_cell(&self, cell_index: &CellIndex) -> Option<&str> {
            self.shared_values
                .iter()
                .find(|v| v.is_index(cell_index))
                .iter()
                .map(|c| c.get_value().as_str())
                .next()
        }
    }
    impl Refarences for SharesMock {
        fn get_refarence_cell(&self, cell_index: &CellIndex) -> Option<usize> {
            self.refarence_values
                .iter()
                .find(|c| c.is_index(cell_index))
                .iter()
                .map(|c| *c.get_value())
                .next()
        }
    }
    #[test]
    fn sheet_new_test() {
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
        let mut shared_store = SharedStoreMock::new();
        shared_store.set_value("zero");
        shared_store.set_value("one");
        shared_store.set_value("two");
        shared_store.set_value("three");
        shared_store.set_value("four");
        let mut shareds = SharesMock::new();
        shareds.set_shared(Cell::new("50".to_string(), "F6"));
        shareds.set_shared(Cell::new("shared_value".to_string(), "H4"));
        shareds.set_ref(Cell::new(0, "B2"));
        shareds.set_ref(Cell::new(1, "J2"));
        shareds.set_ref(Cell::new(2, "P2"));
        shareds.set_ref(Cell::new(3, "C3"));
        shareds.set_ref(Cell::new(4, "E3"));
        let sheet = Sheet::new("sheet1", shared_store, shareds);
        assert_eq!(sheet.get_cell(CellIndex::new("B2")), Some("zero"));
    }
}
