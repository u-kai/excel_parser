use std::{
    cell::Cell,
    fs::{rename, File},
    io::{BufReader, Read},
    marker::PhantomData,
    path::{Path, PathBuf},
    process::Command,
};

use crate::xml::nodes::node::XMLNode;

use super::{
    excel::XLSXOperator,
    sheet_names::sheet_names::{ExcelDefineSheetName, SheetName},
};
#[derive(PartialEq, Eq, Clone, Copy)]
enum XLSXFileState {
    Excel,
    Zip,
    Decompress,
}
pub struct XLSXFile<'a> {
    filename: &'a Path,
    decompress_root: &'a Path,
    zip_name: PathBuf,
    state: Cell<XLSXFileState>,
    content: XLSXFileContent<'a>,
}
#[derive(Debug)]
struct XLSXFileContent<'a> {
    phantom: PhantomData<&'a str>,
    shared_strings: Option<XMLNode>,
    workbook: Option<XMLNode>,
}
impl<'a> XLSXFileContent<'a> {
    pub fn new() -> Self {
        XLSXFileContent {
            phantom: PhantomData,
            shared_strings: None,
            workbook: None,
        }
    }
    pub fn get_shared_strings(&'a self) -> Option<&'a XMLNode> {
        self.shared_strings.as_ref()
    }
    pub fn get_workbook(&'a self) -> Option<&'a XMLNode> {
        self.workbook.as_ref()
    }
    pub fn set_shared_strings(&mut self, source: String) {
        self.shared_strings = Some(XMLNode::from(source.as_str()))
    }
    pub fn set_workbook(&mut self, source: String) {
        self.workbook = Some(XMLNode::from(source.as_str()))
    }
}

impl<'a> XLSXFile<'a> {
    pub fn open(filename: &'a str) -> Self {
        let remove_xlsx_range = 0..(filename.len() - 5);
        let filename = Path::new(filename);
        let zip_name = filename.with_extension("zip");
        let decompress_root = Path::new(filename.to_str().unwrap().get(remove_xlsx_range).unwrap());
        let content = XLSXFileContent::new();
        XLSXFile {
            filename,
            zip_name,
            decompress_root,
            state: Cell::new(XLSXFileState::Excel),
            content,
        }
    }
    pub fn file_content(&self, filepath: &str) -> String {
        let mut buf = String::new();
        let filepath = self.decompress_root.join(filepath);
        let mut reader = BufReader::new(File::open(filepath).unwrap());
        let _ = reader.read_to_string(&mut buf);
        buf
    }
}

impl<'a> XLSXOperator<'a> for XLSXFile<'a> {
    fn read_content(&mut self) -> () {
        let shared_strings = self.file_content("xl/sharedStrings.xml");
        let workbook = self.file_content("xl/workbook.xml");
        self.content.set_shared_strings(shared_strings);
        self.content.set_workbook(workbook);
    }
    fn get_shared_strings(&'a self) -> &'a XMLNode {
        self.content.get_shared_strings().unwrap()
    }
    fn get_workbook(&'a self) -> &'a XMLNode {
        self.content.get_workbook().unwrap()
    }
    fn get_sheet(&self, sheet: &ExcelDefineSheetName) -> Option<String> {
        let source =
            self.file_content(format!("xl/worksheets/{}.xml", sheet.get_sheet_name()).as_str());
        Some(source)
    }
    fn decompress(&self) -> () {
        if self.state.get() == XLSXFileState::Zip {
            let command_arg = format!(
                "unzip {} -d {}",
                self.zip_name.to_str().unwrap(),
                self.decompress_root.to_str().unwrap()
            );
            Command::new("sh")
                .arg("-c")
                .arg(command_arg)
                .output()
                .unwrap();
            self.state.set(XLSXFileState::Decompress)
        }
    }
    fn to_excel(&self) -> () {
        match self.state.get() {
            XLSXFileState::Excel => {
                println!("{:?} is already .xlsx", self.filename);
            }
            XLSXFileState::Zip => {
                let _ = rename(&self.zip_name, &self.filename);
            }
            XLSXFileState::Decompress => {
                let _ = rename(&self.decompress_root, &self.zip_name);
                let _ = rename(&self.zip_name, &self.filename);
                let rm_commad = format!("rm -rf {}", self.decompress_root.to_str().unwrap());
                Command::new("sh")
                    .arg("-c")
                    .arg(rm_commad)
                    .output()
                    .unwrap();
            }
        }
        self.state.set(XLSXFileState::Excel)
    }
    fn to_zip(&self) -> () {
        if self.state.get() == XLSXFileState::Excel {
            let _ = rename(&self.filename, &self.zip_name);
            self.state.set(XLSXFileState::Zip)
        }
    }
}
