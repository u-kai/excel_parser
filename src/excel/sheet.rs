use super::cell::Cell;

#[derive(PartialEq, Eq, Debug)]
pub struct Sheet {
    name: String,
    cells: Vec<Vec<Cell<String>>>,
}

#[cfg(test)]
mod sheet_test {
    #[test]
    fn sheet_new_test() {}
}
