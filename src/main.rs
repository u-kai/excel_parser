use std::io::{stdout, Result, Write};
use std::time;

use crate::excel::cells::cell::ECell;
use crate::excel::excel::Excel;
use crate::excel::file_operator::XLSXFile;
use crate::excel::xmls::sheet::WorkSheet;

mod excel;
mod html;
mod xml;
fn main() -> Result<()> {
    //let time = time::Instant::now();
    //let mut xlsx = XLSXFile::open("test_buf.xlsx");
    //let mut excel = Excel::new(&mut xlsx);
    //excel.read_sheet("term1");
    //let mut sheet1 = excel.get_sheet("term1");
    //let cell = ECell::new("0", "H");
    //sheet1.set_cell(&cell);
    ////println!("{:?}", sheet1.get_all_cell());
    //println!("{:?}", sheet1.to_xml());
    //println!("{:?}", time.elapsed());
    //excel.save(sheet1);
    let mut out = stdout();
    let mut buf = String::new();
    buf = "hello".to_string();
    println!("{:?}", out);
    out.write(buf.as_bytes())?;
    out.write(b" ")?;
    println!("{:?}", out);
    Ok(())
}
