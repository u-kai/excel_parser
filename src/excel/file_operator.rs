use std::{
    cell::Cell,
    fs::{rename, File, OpenOptions},
    io::{BufReader, BufWriter, Read, Write},
    path::{Path, PathBuf},
    process::Command,
};

pub trait XLSXOperator<'a> {
    //fn to_zip(&self) -> ();
    fn to_excel(&self) -> ();
    //fn decompress(&self) -> ();
    fn read_sheet(&'a self, e_sheet_name: &str) -> &'a str;
    fn read_workbook(&'a self) -> &'a str;
    fn read_shared_strings(&'a self) -> &'a str;
    fn add_sheet(&mut self, e_sheet_name: &str) -> ();
    fn write_sheet(&self, e_sheet_name: &str, content: &str) -> ();
}
#[derive(PartialEq, Eq, Clone, Copy)]
enum XLSXFileState {
    Excel,
    Zip,
    Decompress,
}
pub struct XLSXFile<'a> {
    filename: &'a Path,
    zip_name: PathBuf,
    //state: Cell<XLSXFileState>,
    workbook: String,
    shared_strings: String,
    sheets: Vec<String>,
}

impl<'a> XLSXFile<'a> {
    pub fn open(filename: &'a str) -> Self {
        let filename = Path::new(filename);
        let zip_name = filename.with_extension("zip");
        to_zip(filename, &zip_name);
        decompress(&zip_name);
        let workbook = XLSXFile::read_workbook_file();
        let shared_strings = XLSXFile::read_shared_strings_file();
        XLSXFile {
            filename,
            zip_name,
            workbook,
            shared_strings,
            sheets: Vec::new(),
            //state: Cell::new(XLSXFileState::Excel),
        }
    }
    fn read_file(filepath: &str) -> String {
        let mut buf = String::new();
        let mut reader = BufReader::new(
            File::open(&filepath).expect(format!("{} is not found", filepath).as_str()),
        );
        let _ = reader.read_to_string(&mut buf);
        buf
    }
    fn write_file(&self, filepath: &str, content: &str) {
        let mut file_options = OpenOptions::new();
        let file = file_options
            .read(true)
            .write(true)
            .open(&filepath)
            .expect(format!("{:?} is not found", filepath).as_str());
        let mut writer = BufWriter::new(file);
        let _ = writer.write_all(content.as_bytes()).unwrap();
    }
    fn workbook_path() -> &'a str {
        "./xl/workbook.xml"
    }
    fn sheet_path(e_sheet_name: &str) -> String {
        format!("./xl/worksheets/{}.xml", e_sheet_name)
    }
    fn shared_strings_path() -> &'a str {
        "./xl/sharedStrings.xml"
    }
    fn read_sheet_file(e_sheet_name: &str) -> String {
        XLSXFile::read_file(XLSXFile::sheet_path(e_sheet_name).as_str())
    }
    fn read_workbook_file() -> String {
        XLSXFile::read_file(XLSXFile::workbook_path())
    }
    fn read_shared_strings_file() -> String {
        XLSXFile::read_file(XLSXFile::shared_strings_path())
    }
}

fn to_zip(filename: &Path, zip_name: &PathBuf) -> () {
    let command_arg = format!(
        "mv {} {}",
        filename.to_str().unwrap(),
        zip_name.to_str().unwrap()
    );
    Command::new("zsh")
        .arg("-c")
        .arg(command_arg)
        .output()
        .unwrap();
}
fn decompress(zip_name: &PathBuf) -> () {
    let command_arg = format!("unzip {} -d ./", zip_name.to_str().unwrap(),);
    Command::new("zsh")
        .arg("-c")
        .arg(command_arg)
        .output()
        .unwrap();
    let rm_commad = format!("rm -rf {}", zip_name.to_str().unwrap());
    Command::new("zsh")
        .arg("-c")
        .arg(rm_commad)
        .output()
        .unwrap();
}
impl<'a> XLSXOperator<'a> for XLSXFile<'a> {
    fn add_sheet(&mut self, e_sheet_name: &str) -> () {
        ()
    }
    fn to_excel(&self) -> () {
        let command_arg = format!(
            r#"zip {} -r _rels docProps xl \[Content_Types\].xml"#,
            self.zip_name.to_str().unwrap(),
            //&self.decompress_root.to_str().unwrap()
        );
        Command::new("zsh")
            .arg("-c")
            .arg(command_arg)
            .output()
            .unwrap();
        let rm_commad = r#"rm -rf _rels docProps xl \[Content_Types\].xml"#; //);//, self.decompress_root.to_str().unwrap());
        Command::new("zsh")
            .arg("-c")
            .arg(rm_commad)
            .output()
            .unwrap();
        let command_arg = format!(
            "mv {} {}",
            self.zip_name.to_str().unwrap(),
            self.filename.to_str().unwrap(),
        );
        Command::new("zsh")
            .arg("-c")
            .arg(command_arg)
            .output()
            .unwrap();
    }
    fn read_sheet(&'a self, e_sheet_name: &str) -> &'a str {
        &self.sheets[0]
    }
    fn read_shared_strings(&'a self) -> &'a str {
        &self.shared_strings
    }
    fn read_workbook(&'a self) -> &'a str {
        &self.workbook
    }
    fn write_sheet(&self, e_sheet_name: &str, content: &str) -> () {
        self.write_file(XLSXFile::sheet_path(e_sheet_name).as_str(), content);
    }
}
