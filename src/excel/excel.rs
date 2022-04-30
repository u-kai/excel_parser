use super::{
    cell::{CellIndex, ColumnAlphabet},
    sheet_names::sheet_names::SheetNames,
};

pub struct Excel<'a> {
    file_name: &'a str,
    sheet_names: SheetNames<'a>,
}
trait WorkSheet {
    fn get_cell(&self, cell_index: CellIndex) -> Option<&str>;
    fn get_row(&self, u: usize) -> Vec<Option<&str>>;
    fn get_column(&self, s: ColumnAlphabet) -> Vec<Option<&str>>;
    fn get_all_cell(&self) -> Vec<Vec<Option<&str>>>;
}
