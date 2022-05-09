use std::net::ToSocketAddrs;

use excel::excel::Excel;
use excel::file_operator::XLSXFile;
use excel::xmls::sheet::WorkSheet;
use html::dom::Dom;
use xml::nodes::node::XMLNode;

mod excel;
mod html;
mod xml;
fn main() {
    let mut x = X::new();
    x.change("data");
    let mut y = Y::new(&mut x);
    let mut x = X::new();
    x.change("data2");
    y.add(&mut x);
    y.print_ptr();
    let y = y.drain();
    y.iter()
        .for_each(|x| println!("{:?}", x.get_value().as_ptr()));
}

struct X {
    d: String,
}
impl X {
    pub fn new() -> Self {
        X { d: String::new() }
    }
    pub fn change(&mut self, s: &str) {
        self.d = s.to_string()
    }
    pub fn get_value(&self) -> &String {
        &self.d
    }
    pub fn get_mut(&mut self) -> &mut X {
        self
    }
}
struct Y<'a> {
    d: Vec<&'a mut X>,
}

impl<'a> Y<'a> {
    pub fn new(d: &'a mut X) -> Self {
        Y { d: vec![d] }
    }
    pub fn add(&mut self, d: &'a mut X) {
        self.d.push(d)
    }
    pub fn drain(&mut self) -> Vec<&'a mut X> {
        self.d.drain(..).collect()
    }
    pub fn print_ptr(&self) {
        self.d
            .iter()
            .for_each(|x| println!("{:?}", x.get_value().as_ptr()))
    }
}
