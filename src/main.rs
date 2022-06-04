use std::io::{stdout, Read, Write};
use std::{net, time};

use html::dom::Dom;

use crate::excel::cells::cell::ECell;
use crate::excel::excel::Excel;
use crate::excel::file_operator::XLSXFile;
use crate::excel::xmls::sheet::WorkSheet;

mod excel;
mod html;
mod xml;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let time = time::Instant::now();
    let mut xlsx = XLSXFile::open("test_buf.xlsx");
    let mut excel = Excel::new(&mut xlsx);
    excel.read_sheet("term1");
    let mut sheet1 = excel.get_sheet("term1");
    let cell = ECell::new("0", "H2");
    sheet1.set_cell(&cell);
    println!("{:?}", sheet1.get_all_cell());
    println!("{:?}", sheet1.to_xml());
    println!("{:?}", time.elapsed());
    excel.save(sheet1);
    Ok(())
}
