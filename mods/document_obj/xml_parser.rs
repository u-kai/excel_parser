use std::str::from_utf8;
use std::{fs::File, io::BufReader};

use quick_xml::Reader;

use quick_xml::events::Event;

pub fn test_xml(filename:&str){


let mut reader = Reader::from_reader(BufReader::new(File::open(filename).unwrap()));
reader.trim_text(true);

let mut count = 0;
let mut txt = Vec::new();
let mut buf = Vec::new();

// The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
loop {
    // NOTE: this is the generic case when we don't know about the input BufRead.
    // when the input is a &str or a &[u8], we don't actually need to use another
    // buffer, we could directly call `reader.read_event_unbuffered()`
    match reader.read_event(&mut buf).unwrap() {
        Event::Start(ref e) => {
            println!("{}",
            from_utf8(e.local_name()).unwrap().to_string()
       );
            from_utf8(e.local_name()).unwrap().to_string()
        },
        Event::Text(e) => {
            txt.push(e.unescape_and_decode(&reader).unwrap());
            "".to_string()
        },
        Event::Eof => break, // exits the loop when reaching end of file
        //Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
        _ => "".to_string(), // There are several other `Event`s we do not consider here
    };
    println!("{:?}",txt);
    // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
    buf.clear();
    }
}