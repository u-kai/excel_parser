use excel::cell::CellIndex;
use excel::excel::{Excel, XLSXOperator};
use excel::file_operator::XLSXFile;
use excel::sheet_names::sheet_names::UserDefineSheetName;

mod excel;
mod html;
mod xml;
fn main() {
    let mut xlsx_operator = XLSXFile::open("test.xlsx");
    let excel = Excel::new(&mut xlsx_operator);
    let sheet_name = UserDefineSheetName::new("term1");
    let sheet = excel.get_sheet(&sheet_name);
    println!("{:?}", sheet.get_cell(CellIndex::new("B2")));
    println!("{:?}", sheet.get_cell(CellIndex::new("P2")));
}
