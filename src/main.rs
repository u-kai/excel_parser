use std::{
    collections::HashMap,
    fs::{read, File},
    io::{BufRead, BufReader, Read},
};

use excel_parser::mods::do_parser::DOParser;

fn main() {
    let filename = "excel/xl/worksheets/sheet1.xml";
    let mut content = String::new();
    let mut reader = BufReader::new(File::open(filename).unwrap());
    reader.read_to_string(&mut content);
    content = "<div>data</div>".to_string();
    let parser = DOParser::new(content);
    println!("{:?}", parser)
}
