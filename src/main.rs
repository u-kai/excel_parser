use crate::excel::cells::cell::ECell;
use excel::excel::Excel;
use excel::xmls::sheet::WorkSheet;

mod excel;
mod html;
mod xml;
fn main() {
    let mut excel = Excel::open("test_buf.xlsx");
    let mut sheet = excel.get_sheet("term1");
    println!("{:?}", sheet.get_all_cell());
    sheet.set_cell(ECell::new("new-data", "C2"));
    sheet.set_cell(ECell::new("new3", "A1"));
    //    excel.save(sheet);
    excel.close();
}
