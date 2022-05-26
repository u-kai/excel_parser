use crate::excel::cells::cell::CellIndex;
use crate::excel::excel::Excel;
use crate::excel::file_operator::XLSXFile;
use crate::excel::xmls::sheet::WorkSheet;

mod excel;
mod html;
mod xml;
fn main() {
    let mut xlsx = XLSXFile::open("test.xlsx");
    let excel = Excel::new(&mut xlsx);
    let sheet1 = excel.get_sheet("テーブル一覧");
    let c3 = sheet1.get_cell(CellIndex::new("C3"));
    println!("{:?}", c3);
    println!("{:?}", sheet1.to_xml());
    excel.save(sheet1)
}
