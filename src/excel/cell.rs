#[derive(PartialEq, Eq, Debug)]
pub struct Cell<T: PartialEq + Eq> {
    value: T,
    index: CellIndex,
}
impl<T: PartialEq + Eq> Cell<T> {
    #[allow(warnings)]
    pub fn new(value: T, cell_index: &str) -> Self {
        Cell {
            value,
            index: CellIndex::new(cell_index),
        }
    }
}
#[derive(PartialEq, Eq, Debug)]
struct CellIndex {
    column: usize,
    row: usize,
}
impl CellIndex {
    pub fn new(s: &str) -> Self {
        let split = CellIndex::split_alphabet_number(s);
        CellIndex {
            column: split.0,
            row: split.1,
        }
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
struct ColumnAlphabet(String);
impl ColumnAlphabet {
    pub fn new(s: &str) -> Self {
        let s = s
            .chars()
            .filter(|c| c.is_uppercase())
            .collect::<Vec<_>>()
            .iter()
            .fold("".to_string(), |acc, cur| format!("{}{}", acc, cur));
        ColumnAlphabet(s)
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
    use super::CellIndex;
    #[test]
    fn new_test() {
        let cell_index = CellIndex::new("A1");
        assert_eq!(cell_index, CellIndex { column: 1, row: 1 });
        let cell_index = CellIndex::new("B1");
        assert_eq!(cell_index, CellIndex { column: 2, row: 1 })
    }
}
#[cfg(test)]
mod cell_tests {

    use super::{Cell, CellIndex};

    #[test]
    fn new_cell_test() {
        let cell = Cell::new("test", "A123");
        assert_eq!(
            cell,
            Cell {
                value: "test",
                index: CellIndex {
                    column: 1,
                    row: 123
                }
            }
        );
        let cell = Cell::new("test", "AA1");
        assert_eq!(
            cell,
            Cell {
                value: "test",
                index: CellIndex { column: 27, row: 1 }
            }
        );
        let cell = Cell::new("test", "LRO1");
        assert_eq!(
            cell,
            Cell {
                value: "test",
                index: CellIndex {
                    column: 8595,
                    row: 1
                }
            }
        );
        let cell = Cell::new("test2", "B1");
        assert_eq!(
            cell,
            Cell {
                value: "test2",
                index: CellIndex { column: 2, row: 1 }
            }
        )
    }
}
