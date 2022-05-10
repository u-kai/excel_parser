use crate::excel::cells::cell::ECell;
use excel::excel::Excel;
use excel::xmls::sheet::WorkSheet;

mod excel;
mod html;
mod xml;
fn main() {
    let excel = Excel::open("test_buf.xlsx");
    let mut sheet1 = excel.get_sheet("term1");
    let mut sheet2 = excel.get_sheet("term2");
    sheet1.set_cell(ECell::new("new-data", "C2"));
    sheet2.set_cell(ECell::new("new3", "A1"));
    println!("{:?}", sheet1.to_xml());
    println!("{:?}", sheet2.to_xml());
    excel.save(sheet1);
    excel.save(sheet2);
    excel.close();
}
