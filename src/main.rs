use std::time;

use crate::excel::cells::cell::{CellIndex, ECell};
use crate::excel::excel::Excel;
use crate::excel::file_operator::XLSXFile;
use crate::excel::xmls::sheet::WorkSheet;

mod excel;
mod html;
mod xml;
fn main() {
    let time = time::Instant::now();
    let mut xlsx = XLSXFile::open("test_buf.xlsx");
    let mut excel = Excel::new(&mut xlsx);
    excel.read_sheet("term1");
    let mut sheet1 = excel.get_sheet("term1");
    let cell = ECell::new("0", "H");
    sheet1.set_cell(&cell);
    //println!("{:?}", sheet1.get_all_cell());
    println!("{:?}", sheet1.to_xml());
    println!("{:?}", time.elapsed());
    excel.save(sheet1);
}
