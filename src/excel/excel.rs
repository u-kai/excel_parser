use std::{collections::HashMap, sync::Arc};

use crate::xml::nodes::{generators, node::XMLNode};

use super::{
    file_operator::{XLSXFile, XLSXOperator},
    xmls::{
        shared_strings::{self, SharedStrings, SharedStringsInterface},
        sheet::Sheet,
        workbook::WorkBook,
    },
};

type SheetA<'a> = Sheet<'a, SharedStrings>;
#[derive(Debug, PartialEq, Eq)]
pub struct Excel<'a, T: XLSXOperator> {
    xlsx_operator: T,
    workbook: WorkBook,
    shared_strings: SharedStrings,
    sheet_map: SheetMap<'a>,
}
#[derive(Debug, PartialEq, Eq)]
struct SheetMap<'a>(HashMap<String, SheetA<'a>>);

impl<'a> SheetMap<'a> {
    pub fn new() -> Self {
        SheetMap(HashMap::new())
    }
    pub fn register_sheet(&mut self, sheet_name: &str, sheet: SheetA<'a>) {
        self.0.insert(sheet_name.to_string(), sheet);
    }
    pub fn get_sheet(&mut self, sheet_name: &str) -> Option<&'a mut SheetA> {
        self.0.get_mut(sheet_name)
    }
}
impl<'a, XOpe: XLSXOperator> Excel<'a, XOpe> {
    pub fn new(xlsx_operator: XOpe) -> Self {
        xlsx_operator.to_zip();
        xlsx_operator.decompress();
        let workbook = WorkBook::new(&xlsx_operator.read_workbook());
        let shared_strings = SharedStrings::new(&xlsx_operator.read_shared_strings());
        let sheet_map = SheetMap::new();
        Excel {
            xlsx_operator,
            workbook,
            shared_strings,
            sheet_map,
        }
    }
    pub fn close(&self) {
        self.xlsx_operator.to_excel()
    }
    pub fn get_sheet(&'a mut self, sheet_name: &str) -> &'a mut SheetA {
        let e_sheet_name = self.workbook.get_excel_sheet_name(&sheet_name);
        let source = self.xlsx_operator.read_sheet(e_sheet_name);
        let shared_strings = self.shared_strings.get_mut();
        let sheet = SheetA::new(sheet_name, source, shared_strings);
        self.sheet_map.register_sheet(sheet_name, sheet);
        let sheet = self.sheet_map.get_sheet(sheet_name).unwrap();
        sheet
    }
}
impl<'a> Excel<'a, XLSXFile<'a>> {
    pub fn open(xlsx_file: &'a str) -> Self {
        let xlsx_operator = XLSXFile::open(xlsx_file);
        xlsx_operator.to_zip();
        xlsx_operator.decompress();
        let workbook = WorkBook::new(&xlsx_operator.read_workbook());
        let shared_strings = SharedStrings::new(&xlsx_operator.read_shared_strings());
        let sheet_map = SheetMap::new();
        Excel {
            xlsx_operator,
            workbook,
            shared_strings,
            sheet_map,
        }
    }
}

#[cfg(test)]
mod excel_tests {
    use crate::excel::{
        file_operator::XLSXOperator,
        xmls::{
            shared_strings::{self, SharedStrings, SharedStringsInterface},
            sheet::{self, WorkSheet},
        },
    };

    use super::Excel;
    #[derive(Debug)]
    struct XLSXOperatorMock {
        shared_strings: String,
        workbook: String,
        sheet: String,
    }
    impl<'a> XLSXOperatorMock {
        pub fn new(
            sheet: impl Into<String>,
            shared_strings: impl Into<String>,
            workbook: impl Into<String>,
        ) -> Self {
            XLSXOperatorMock {
                sheet: sheet.into(),
                shared_strings: shared_strings.into(),
                workbook: workbook.into(),
            }
        }
    }
    impl<'a> XLSXOperator for XLSXOperatorMock {
        fn to_zip(&self) -> () {
            println!("ziped!")
        }
        fn to_excel(&self) -> () {
            println!("exceled!")
        }
        fn decompress(&self) -> () {
            println!("decompress")
        }
        fn read_sheet(&self, _: &str) -> String {
            self.sheet.clone()
        }
        fn read_workbook(&self) -> String {
            self.workbook.clone()
        }
        fn read_shared_strings(&self) -> String {
            self.shared_strings.clone()
        }
    }
    #[test]
    fn excel_test() {
        let shared_strings = r#"
            <?xml version="1.0" encoding="UTF-8" standalone="yes"?>
            <sst xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" count="938" uniqueCount="265">
                <si>
                    <t>詳細画面レイアウト</t>
                    <rPh sb="0" eb="2">
                        <t>ショウサイ</t>
                    </rPh>
                    <rPh sb="2" eb="4">
                        <t>ガメン</t>
                    </rPh>
                    <phoneticPr fontId="2"/>
                </si>
                <si>
                    <t>会社名</t>
                    <rPh sb="0" eb="3">
                        <t>カイシャメイ</t>
                    </rPh>
                    <phoneticPr fontId="2"/>
                </si>
                <si>
                    <t>タイトル</t>
                    <phoneticPr fontId="2"/>
                </si>
            </sst>
            "#;
        let workbook = r#"
                <?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006" mc:Ignorable="x15 xr xr6 xr10 xr2" xmlns:x15="http://schemas.microsoft.com/office/spreadsheetml/2010/11/main" xmlns:xr="http://schemas.microsoft.com/office/spreadsheetml/2014/revision" xmlns:xr6="http://schemas.microsoft.com/office/spreadsheetml/2016/revision6" xmlns:xr10="http://schemas.microsoft.com/office/spreadsheetml/2016/revision10" xmlns:xr2="http://schemas.microsoft.com/office/spreadsheetml/2015/revision2">
                    <fileVersion appName="xl" lastEdited="7" lowestEdited="7" rupBuild="20372"/>
                    <workbookPr defaultThemeVersion="166925"/>
                    <mc:AlternateContent xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006">
                        <mc:Choice Requires="x15">
                            <x15ac:absPath url="dist" xmlns:x15ac="http://schemas.microsoft.com/office/spreadsheetml/2010/11/ac"/>
                        </mc:Choice>
                    </mc:AlternateContent>
                    <xr:revisionPtr revIDLastSave="0" documentId="13_ncr:1_{23D3C209-A72C-406C-978A-0FFFF7F72B00}" xr6:coauthVersionLast="36" xr6:coauthVersionMax="36" xr10:uidLastSave="{00000000-0000-0000-0000-000000000000}"/>
                    <bookViews>
                        <workbookView xWindow="0" yWindow="0" windowWidth="28800" windowHeight="12135" firstSheet="6" activeTab="9" xr2:uid="{EFD3C6D3-98FA-468E-8050-FFAA76D4661F}"/>
                    </bookViews>
                    <sheets>
                        <sheet name="term1" sheetId="1" state="hidden" r:id="rId1"/>
                        <sheet name="term2" sheetId="2" state="hidden" r:id="rId2"/>
                        <sheet name="テーブル一覧" sheetId="8" r:id="rId3"/>
                        <sheet name="求人情報テーブル(job_info)" sheetId="3" r:id="rId4"/>
                    </sheets>
                    <calcPr calcId="191029"/>
                    <extLst>
                        <ext uri="{140A7094-0E35-4892-8432-C4D2E57EDEB5}" xmlns:x15="http://schemas.microsoft.com/office/spreadsheetml/2010/11/main">
                            <x15:workbookPr chartTrackingRefBase="1"/>
                        </ext>
                    </extLst>
                </workbook>
        "#;
        let sheet1 = r#"
                <?xml version="1.0" encoding="UTF-8" standalone="yes"?>
                <worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006" mc:Ignorable="x14ac xr xr2 xr3" xmlns:x14ac="http://schemas.microsoft.com/office/spreadsheetml/2009/9/ac" xmlns:xr="http://schemas.microsoft.com/office/spreadsheetml/2014/revision" xmlns:xr2="http://schemas.microsoft.com/office/spreadsheetml/2015/revision2" xmlns:xr3="http://schemas.microsoft.com/office/spreadsheetml/2016/revision3" xr:uid="{44FEEDED-D128-4496-B199-BCD526D1EB2C}">
                    <dimension ref="B2:S50"/>
                    <sheetViews>
                        <sheetView workbookViewId="0">
                            <selection activeCell="G1" sqref="G1:G1048576"/>
                        </sheetView>
                    </sheetViews>
                    <sheetFormatPr defaultRowHeight="18.75" x14ac:dyDescent="0.4"/>
                    <cols>
                        <col min="5" max="5" width="19.25" bestFit="1" customWidth="1"/>
                        <col min="7" max="7" width="15" bestFit="1" customWidth="1"/>
                        <col min="8" max="8" width="22.5" bestFit="1" customWidth="1"/>
                        <col min="11" max="11" width="17.25" bestFit="1" customWidth="1"/>
                        <col min="16" max="16" width="8.875" customWidth="1"/>
                        <col min="17" max="17" width="16.75" customWidth="1"/>
                    </cols>
                    <sheetData>
                        <row r="2" spans="2:19" x14ac:dyDescent="0.4">
                            <c r="B2" s="15" t="s">
                                <v>0</v>
                            </c>
                            <c r="C2" s="12"/>
                            <c r="D2" s="16"/>
                            <c r="E2" s="13"/>
                            <c r="J2" s="15" t="s">
                                <v>1</v>
                            </c>
                            <c r="K2" s="13"/>
                            <c r="P2" s="15" t="s">
                                <v>2</v>
                            </c>
                            <c r="Q2" s="13"/>
                        </row>
                        <row r="3" spans="2:19" x14ac:dyDescent="0.4">
                            <c r="B3" s="4"/>
                            <c r="C3" s="15" t="s">
                                <v>3</v>
                            </c>
                            <c r="D3" s="16"/>
                            <c r="J3" s="4"/>
                            <c r="L3">
                                <v>10</v>
                            </c>
                            <c r="M3" t="s">
                                <v>109</v>
                            </c>
                            <c r="P3" s="4"/>
                            <c r="R3">
                                <v>100</v>
                            </c>
                        </row>
                    </sheetData>
                </worksheet>
                        "#;
        let oprator = XLSXOperatorMock::new(sheet1, shared_strings, workbook);
        let mut excel: Excel<XLSXOperatorMock> = Excel::new(oprator);
        let sheet = excel.get_sheet("term1"); //.get_sheet("term1");
                                              //sheet.set_cell(Cell::new("", "A1"));
    }
}
