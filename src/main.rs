use std::{
    collections::HashMap,
    fs::{read, File},
    io::{BufRead, BufReader, Read},
};

use excel_parser::mods::document_obj::{do_parser::DOParser, do_searcher::DOSearcher};

fn main() {
    let filename = "excel/xl/worksheets/sheet1.xml";
    let mut content = String::new();
    let mut reader = BufReader::new(File::open(filename).unwrap());
    reader.read_to_string(&mut content);
    let mut parser = DOParser::new(content);
    parser.fmt_content();
    
    let mut searcher = DOSearcher::new(parser.content);
    let tag = searcher.news("sheetData").unwrap();
    let mut dop = DOParser::new(tag.get_children().to_string());
    dop.fmt_content();
    let searcher = DOSearcher::new(dop.content);
    let tag = searcher.news("c");
    println!("{:?}",tag);

}
