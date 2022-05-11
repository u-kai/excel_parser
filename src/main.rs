use crate::excel::cells::cell::ECell;
use excel::excel::Excel;
use excel::xmls::sheet::WorkSheet;

mod excel;
mod html;
mod xml;
fn main() {
    let excel = Excel::open("test.xlsx");
    let mut sheet1 = excel.get_sheet("term1");
    sheet1.set_cell(ECell::new("new-data", "B2"));
    println!("{}", sheet1.to_xml());
    excel.save(sheet1);
    excel.close();
    //let excel = Excel::open("test_buf.xlsx");
    //let mut sheet1 = excel.get_sheet("term1");
    //println!("second:{}\n{:?}", "#".repeat(50), sheet1.to_xml());
    //excel.close();
}
