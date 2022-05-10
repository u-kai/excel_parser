use std::{
    cell::Cell,
    fs::{rename, File},
    io::{BufReader, BufWriter, Read, Write},
    path::{Path, PathBuf},
    process::Command,
};

pub trait XLSXOperator {
    fn to_zip(&self) -> ();
    fn to_excel(&self) -> ();
    fn decompress(&self) -> ();
    fn read_sheet(&self, e_sheet_name: &str) -> String;
    fn read_workbook(&self) -> String;
    fn read_shared_strings(&self) -> String;
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
    decompress_root: &'a Path,
    zip_name: PathBuf,
    state: Cell<XLSXFileState>,
}

impl<'a> XLSXFile<'a> {
    pub fn open(filename: &'a str) -> Self {
        let remove_xlsx_range = 0..(filename.len() - 5);
        let filename = Path::new(filename);
        let zip_name = filename.with_extension("zip");
        let decompress_root = Path::new(filename.to_str().unwrap().get(remove_xlsx_range).unwrap());
        XLSXFile {
            filename,
            zip_name,
            decompress_root,
            state: Cell::new(XLSXFileState::Excel),
        }
    }
    fn read_file(&self, filepath: &str) -> String {
        let mut buf = String::new();
        let filepath = self.decompress_root.join(filepath);
        let mut reader = BufReader::new(
            File::open(&filepath).expect(format!("{:?} is not found", filepath.display()).as_str()),
        );
        let _ = reader.read_to_string(&mut buf);
        buf
    }
    fn write_file(&self, filepath: &str, content: &str) {
        let filepath = self.decompress_root.join(filepath);
        let mut writer = BufWriter::new(
            File::open(&filepath).expect(format!("{:?} is not found", filepath.display()).as_str()),
        );
        let _ = writer.write_all(content.as_bytes());
    }
    fn workbook_path(&self) -> &str {
        "xl/workbook.xml"
    }
    fn sheet_path(&self, e_sheet_name: &str) -> String {
        format!("xl/worksheets/{}.xml", e_sheet_name)
    }
    fn shared_strings(&self) -> &str {
        "xl/sharedStrings.xml"
    }
}

impl<'a> XLSXOperator for XLSXFile<'a> {
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
    fn read_sheet(&self, e_sheet_name: &str) -> String {
        self.read_file(self.sheet_path(e_sheet_name).as_str())
    }
    fn read_shared_strings(&self) -> String {
        self.read_file(self.shared_strings())
    }
    fn read_workbook(&self) -> String {
        self.read_file(self.workbook_path())
    }
    fn write_sheet(&self, e_sheet_name: &str, content: &str) -> () {
        self.write_file(self.sheet_path(e_sheet_name).as_str(), content);
    }
}
