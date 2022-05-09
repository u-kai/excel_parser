use std::net::ToSocketAddrs;
use std::sync::Arc;
use std::time::{Duration, Instant};

use excel::cells::cell::ColumnAlphabet;
use excel::excel::Excel;
use excel::file_operator::XLSXFile;
use excel::xmls::sheet::WorkSheet;
use html::dom::Dom;
use xml::nodes::node::XMLNode;

mod excel;
mod html;
mod xml;
fn main() {
    let mut excel = Excel::open("test_buf.xlsx");
    let mut now = Instant::now();
    println!(
        "{:?}",
        excel
            .get_sheet("term1")
            .get_column_range(ColumnAlphabet::new("A"), ColumnAlphabet::new("ZZZ"))
    );
    println!("{:?}", now.elapsed());
    excel.close();
}
