use std::{
    cell::Cell,
    fs::{rename, File},
    io::{BufReader, Read},
    ops::RemAssign,
    path::{Path, PathBuf},
};

use crate::xml::nodes::node::XMLNode;

use super::{excel::XLSXOperator, sheet_names::sheet_names::ExcelDefineSheetName};
#[derive(PartialEq, Eq, Clone, Copy)]
enum XLSXFileState {
    Excel,
    Zip,
}
pub struct XLSXFile<'a> {
    name: &'a Path,
    zip_name: PathBuf,
    state: Cell<XLSXFileState>,
    shared_strigs: &'a str,
    workbook: &'a str,
}
impl<'a> XLSXFile<'a> {
    fn open(file_name: &'a str) -> Self {
        let file_name = Path::new(file_name);
        let zip_name = file_name.with_extension("zip");
        XLSXFile {
            name: file_name,
            zip_name,
            state: Cell::new(XLSXFileState::Excel),
            shared_strigs: "",
            workbook: "",
        }
    }
}

impl<'a> XLSXOperator<'a> for XLSXFile<'a> {
    fn read_shared_strings(&'a mut self) -> &'a XMLNode {
        let mut buf = String::new();
        let shared_strings_path = self.zip_name.join("/xl/sharedStrings.xml");
        let mut reader = BufReader::new(File::open(shared_strings_path).unwrap());
        reader.read_to_string(&mut buf);
        self.shared_strigs = buf.as_str();
        &XMLNode::from(self.shared_strigs)
    }
    fn read_sheet(&'a mut self, sheet: &ExcelDefineSheetName) -> Option<&'a str> {
        Some(self.workbook)
    }
    fn read_workbook(&'a mut self) -> &'a XMLNode {
        let mut buf = String::new();
        let workbook = self.zip_name.join("/xl/workbook.xml");
        let reader = BufReader::new(File::open(workbook).unwrap());
        reader.read_to_string(&mut buf);
        self.workbook = buf.as_str();
        &XMLNode::from(self.workbook)
    }
    fn to_excel(&self) -> () {
        if self.state.get() == XLSXFileState::Excel {
            return;
        }
        rename(self.zip_name, self.name);
        self.state.set(XLSXFileState::Excel)
    }
    fn to_zip(&self) -> () {
        if self.state.get() == XLSXFileState::Zip {
            return;
        }
        rename(self.name, self.zip_name);
        self.state.set(XLSXFileState::Zip)
    }
}
