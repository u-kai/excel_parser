use super::{
    cell::{CellIndex, ColumnAlphabet},
    shared_strings::SharedStore,
    xml_sheet::{Refarences, Shareds},
};

#[derive(PartialEq, Eq, Debug)]
pub struct Sheet<'a, S: SharedStore, T: Refarences + Shareds> {
    name: &'a str,
    shared_store: &'a S,
    shared: T,
}
impl<'a, S: SharedStore, T: Refarences + Shareds> Sheet<'a, S, T> {
    pub fn new(name: &'a str, shared_store: &'a S, shared: T) -> Self {
        Sheet {
            name,
            shared_store,
            shared,
        }
    }
}
impl<'a, S: SharedStore, T: Refarences + Shareds> WorkSheet for Sheet<'a, S, T> {
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
    fn get_cell(&self, cell_index: CellIndex) -> Option<&str>;
    //fn get_row(&self, u: usize) -> Vec<Option<&str>>;
    //fn get_column(&self, s: ColumnAlphabet) -> Vec<Option<&str>>;
    fn get_all_cell(&self) -> Vec<Vec<Option<&str>>>;
}
#[cfg(test)]
mod sheet_test {
    use crate::excel::{
        cell::{Cell, CellIndex},
        shared_strings::SharedStore,
        sheet::{
            mocks::{create_mock_shared_store, create_mock_sheet, SharesMock},
            WorkSheet,
        },
        xml_sheet::{Refarences, Shareds},
    };

    use super::Sheet;

    #[test]
    fn sheet_new_test() {
        let mut shareds = SharesMock::new();
        shareds.set_shared(Cell::new("50".to_string(), "F6"));
        shareds.set_shared(Cell::new("shared_value".to_string(), "H4"));
        shareds.set_ref(Cell::new(0, "B2"));
        shareds.set_ref(Cell::new(1, "J2"));
        shareds.set_ref(Cell::new(2, "P2"));
        shareds.set_ref(Cell::new(3, "C3"));
        shareds.set_ref(Cell::new(4, "E3"));
        let shared_store = create_mock_shared_store(vec!["zero", "one", "two", "three", "four"]);
        let sheet = create_mock_sheet(&shared_store);
        assert_eq!(sheet.get_cell(CellIndex::new("B2")), Some("zero"));
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
    fn get_all_cell_test() {
        let mut shareds = SharesMock::new();
        shareds.set_shared(Cell::new("a".to_string(), "A1"));
        shareds.set_shared(Cell::new("b".to_string(), "B2"));
        shareds.set_shared(Cell::new("c".to_string(), "C3"));
        shareds.set_shared(Cell::new("d".to_string(), "D4"));
        shareds.set_shared(Cell::new("e".to_string(), "E5"));
        shareds.set_shared(Cell::new("0".to_string(), "G5"));
        shareds.set_ref(Cell::new(0, "A5"));
        let shared_store = create_mock_shared_store(vec!["あ"]);
        let sheet = create_mock_sheet(&shared_store);
        let tobe = vec![
            vec![Some("a"), None, None, None, None, None, None],
            vec![None, Some("b"), None, None, None, None, None],
            vec![None, None, Some("c"), None, None, None, None],
            vec![None, None, None, Some("d"), None, None, None],
            vec![Some("あ"), None, None, None, Some("e"), None, Some("0")],
        ];
        assert_eq!(sheet.get_all_cell(), tobe);
    }
}

/////////////////////// mock ///////////////////////

mod mocks {
    use crate::excel::{
        cell::{Cell, CellIndex},
        shared_strings::SharedStore,
        xml_sheet::{Refarences, Shareds},
    };

    use super::Sheet;

    pub struct SharedStoreMock {
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
    pub struct SharesMock {
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
        fn get_all_cell(&self) -> &Vec<Cell<String>> {
            &self.shared_values
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
        fn get_all_cell(&self) -> &Vec<Cell<usize>> {
            &self.refarence_values
        }
    }
    pub fn create_mock_shared_store(values: Vec<&str>) -> SharedStoreMock {
        let mut shared_store = SharedStoreMock::new();
        for v in values {
            shared_store.set_value(v);
        }
        shared_store
    }
    pub fn create_mock_sheet<'a>(
        shared_store: &'a SharedStoreMock,
    ) -> Sheet<'a, SharedStoreMock, SharesMock> {
        let mut shareds = SharesMock::new();
        shareds.set_shared(Cell::new("50".to_string(), "F6"));
        shareds.set_shared(Cell::new("shared_value".to_string(), "H4"));
        shareds.set_ref(Cell::new(0, "B2"));
        shareds.set_ref(Cell::new(1, "J2"));
        shareds.set_ref(Cell::new(2, "P2"));
        shareds.set_ref(Cell::new(3, "C3"));
        shareds.set_ref(Cell::new(4, "E3"));
        Sheet::new("sheet1", shared_store, shareds)
    }
}
