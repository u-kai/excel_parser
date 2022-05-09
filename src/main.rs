use excel::cells::cell::ColumnAlphabet;
use excel::excel::Excel;
use excel::xmls::sheet::WorkSheet;
use std::time::Instant;

mod excel;
mod html;
mod xml;
fn main() {
    let mut excel = Excel::open("test_buf.xlsx");
    let now = Instant::now();
    println!(
        "{:?}",
        excel
            .get_sheet("term1")
            .get_column_range(ColumnAlphabet::new("A"), ColumnAlphabet::new("ZZZ"))
    );
    println!("{:?}", now.elapsed());
    excel.close();
}
