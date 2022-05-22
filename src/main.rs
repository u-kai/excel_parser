use std::fs::File;
use std::io::{BufReader, Read};

use crate::excel::cells::cell::CellIndex;
use crate::xml::nodes::node::XMLNode;
use excel::excel::Excel;
use excel::xmls::sheet::WorkSheet;

mod excel;
mod html;
mod xml;
fn main() {
    let excel = Excel::open("test.xlsx");
    let sheet1 = excel.get_sheet("テーブル一覧");
    let c3 = sheet1.get_cell(CellIndex::new("C3"));
    println!("{:?}", c3);
    println!("{:?}", sheet1.to_xml());
    excel.save(sheet1)
}
