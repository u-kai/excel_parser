#[derive(PartialEq, Eq, Debug)]
pub struct ECell<'a> {
    value: &'a str,
    index: CellIndex<'a>,
}
impl<'a> ECell<'a> {
    #[allow(dead_code)]
    pub fn new(value: &'a str, cell_index: &'a str) -> Self {
        ECell {
            value,
            index: CellIndex::new(cell_index),
        }
    }
    pub fn get_value(&'a self) -> &'a str {
        &self.value
    }
    pub fn is_index(&self, cell_index: &CellIndex) -> bool {
        &self.index == cell_index
    }
    pub fn get_index(&self) -> CellIndex<'a> {
        self.index.clone()
    }
    pub fn get_column_index(&self) -> usize {
        self.index.get_column_index()
    }
    pub fn get_row_index(&self) -> usize {
        self.index.get_row_index()
    }
}
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CellIndex<'a> {
    excel_index: &'a str,
    column: usize,
    row: usize,
}
impl<'a> CellIndex<'a> {
    pub fn new(s: &'a str) -> Self {
        let split = CellIndex::split_alphabet_number(s);
        CellIndex {
            excel_index: s,
            column: split.0,
            row: split.1,
        }
    }
    pub fn get_value(&self) -> &'a str {
        &self.excel_index
    }
    pub fn get_column_index(&self) -> usize {
        self.column
    }
    pub fn get_row_index(&self) -> usize {
        self.row
    }
    fn split_alphabet_number(s: &str) -> (usize, usize) {
        let alphabet_number = ColumnAlphabet::new(s).to_number();
        let number = s
            .chars()
            .filter(|c| c.is_numeric())
            .collect::<Vec<_>>()
            .iter()
            .fold("".to_string(), |acc, cur| format!("{}{}", acc, cur))
            .parse::<usize>()
            .unwrap();
        (alphabet_number, number)
    }
}
pub struct ColumnAlphabet<'a>(&'a str);
impl<'a> ColumnAlphabet<'a> {
    pub fn new(s: &'a str) -> Self {
        let len = s
            .chars()
            .enumerate()
            .filter(|(_, c)| c.is_uppercase())
            .map(|(i, _)| i)
            .max()
            .unwrap();
        ColumnAlphabet(s.get(0..=len).unwrap())
    }
    pub fn to_number(&self) -> usize {
        self.0
            .bytes()
            .rev()
            .enumerate()
            .fold(0, |sum, (digit, byte)| {
                let number = (byte - 64) as usize;
                let digit_effect = 26_usize.pow(digit as u32);
                let this_digit_number = number * digit_effect;
                sum + this_digit_number
            })
    }
}

#[cfg(test)]
mod cell_index_tests {
    use crate::excel::cells::cell::ColumnAlphabet;

    use super::CellIndex;
    #[test]
    fn new_test() {
        let cell_index = CellIndex::new("A1");
        assert_eq!(
            cell_index,
            CellIndex {
                excel_index: "A1",
                column: 1,
                row: 1
            }
        );
        let cell_index = CellIndex::new("B1");
        assert_eq!(
            cell_index,
            CellIndex {
                excel_index: "B1",
                column: 2,
                row: 1
            }
        );
        let alphabet = ColumnAlphabet::new("A");
        assert_eq!(alphabet.to_number(), 1)
    }
}
#[cfg(test)]
mod cell_tests {

    use super::{CellIndex, ECell};

    #[test]
    fn new_cell_test() {
        let e_cell = ECell::new("test", "A123");
        assert_eq!(
            e_cell,
            ECell {
                value: "test",
                index: CellIndex {
                    excel_index: "A123",
                    column: 1,
                    row: 123
                }
            }
        );
        let e_cell = ECell::new("test", "AA1");
        assert_eq!(
            e_cell,
            ECell {
                value: "test",
                index: CellIndex {
                    excel_index: "AA1",
                    column: 27,
                    row: 1
                }
            }
        );
        let e_cell = ECell::new("test", "LRO1");
        assert_eq!(
            e_cell,
            ECell {
                value: "test",
                index: CellIndex {
                    excel_index: "LRO1",
                    column: 8595,
                    row: 1
                }
            }
        );
        let e_cell = ECell::new("test2", "B1");
        assert_eq!(
            e_cell,
            ECell {
                value: "test2",
                index: CellIndex {
                    excel_index: "B1",
                    column: 2,
                    row: 1
                }
            }
        )
    }
}
