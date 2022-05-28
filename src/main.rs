use std::time;

use crate::excel::excel::Excel;
use crate::excel::file_operator::XLSXFile;

mod excel;
mod html;
mod xml;
fn main() {
    let time = time::Instant::now();
    let mut xlsx = XLSXFile::open("test.xlsx");
    let mut excel = Excel::new(&mut xlsx);
    excel.read_sheet("テーブル一覧");
    let sheet1 = excel.get_sheet("テーブル一覧");
    println!("{:?}", time.elapsed());
    println!("{}", sheet1.to_xml());
    excel.save(sheet1);
}
