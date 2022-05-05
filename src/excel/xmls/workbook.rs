use crate::xml::nodes::node::XMLNode;

use self::sheet_map::*;

use super::xl::XL;

pub struct WorkBook {
    node: XMLNode,
    sheet_map: SheetMap,
}
impl WorkBook {
    pub fn new(source: &str) -> Self {
        let node = XMLNode::from(source);
        let workbook_node = node.search_node("workbook").unwrap();
        let sheets_node = workbook_node.search_node("sheets").unwrap();
        let sheet_map = SheetMap::from(sheets_node);
        println!("{:?}", sheet_map);
        WorkBook { node, sheet_map }
    }
    pub fn get_excel_sheet_name(&self, sheet_name: &str) -> &str {
        let sheet_name = UserDefineSheetName::new(sheet_name);
        self.sheet_map.get_excel_sheet_name(sheet_name).unwrap()
    }
}
impl<'a> XL<'a> for WorkBook {
    fn get_xml_node(&'a self) -> &'a XMLNode {
        &self.node
    }
}
mod workbook_test {
    use crate::{excel::xmls::xl::XL, xml::nodes::node::XMLNode};

    use super::WorkBook;
    #[test]
    fn new_test() {
        let source = r#"
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
                        <sheet name="table" sheetId="3" r:id="rId3"/>
                    </sheets>
                    <calcPr calcId="191029"/>
                    <extLst>
                        <ext uri="{140A7094-0E35-4892-8432-C4D2E57EDEB5}" xmlns:x15="http://schemas.microsoft.com/office/spreadsheetml/2010/11/main">
                            <x15:workbookPr chartTrackingRefBase="1"/>
                        </ext>
                    </extLst>
                </workbook>
        "#;
        let node = XMLNode::from(source);
        let workbook = WorkBook::new(source);
        assert_eq!(workbook.get_xml_node(), &node);
    }
    #[test]
    fn get_excel_sheet_name_test() {
        let source = r#"
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
                        <sheet name="table" sheetId="3" r:id="rId3"/>
                    </sheets>
                    <calcPr calcId="191029"/>
                    <extLst>
                        <ext uri="{140A7094-0E35-4892-8432-C4D2E57EDEB5}" xmlns:x15="http://schemas.microsoft.com/office/spreadsheetml/2010/11/main">
                            <x15:workbookPr chartTrackingRefBase="1"/>
                        </ext>
                    </extLst>
                </workbook>
        "#;
        let workbook = WorkBook::new(source);
        assert_eq!(workbook.get_excel_sheet_name("term1"), "sheet1");
        assert_eq!(workbook.get_excel_sheet_name("term2"), "sheet2");
        assert_eq!(workbook.get_excel_sheet_name("table"), "sheet3");
    }
}

mod sheet_map {
    use std::collections::HashMap;

    use crate::xml::nodes::node::XMLNode;

    #[derive(PartialEq, Eq, Debug)]
    pub struct SheetMap(HashMap<ExcelDefineSheetName, UserDefineSheetName>);

    impl<'a> SheetMap {
        pub fn new() -> Self {
            SheetMap(HashMap::new())
        }
        fn set(&mut self, e_sheet_name: ExcelDefineSheetName, u_sheet_name: UserDefineSheetName) {
            self.0.insert(e_sheet_name, u_sheet_name);
        }
        pub fn get_excel_sheet_name(&'a self, sheet_name: UserDefineSheetName) -> Option<&'a str> {
            self.0
                .keys()
                .filter_map(|key| self.0.get_key_value(key))
                .find(|(_e, u)| u.get_sheet_name() == sheet_name.get_sheet_name())
                .map(|s| s.0.get_sheet_name())
        }
        pub fn get_user_sheet_name(
            &self,
            sheet_name: &ExcelDefineSheetName,
        ) -> Option<&UserDefineSheetName> {
            self.0.get(sheet_name).iter().map(|s| *s).next()
        }
    }

    impl From<&XMLNode> for SheetMap {
        fn from(sheets_node: &XMLNode) -> Self {
            let mut sheet_names = SheetMap::new();
            let sheets = sheets_node
                .search_all_nodes("sheet")
                .expect(format!("invalid node {:?}", sheets_node).as_str());
            println!("{:?}", sheets);
            sheets.iter().for_each(|sheet| {
                let e_sheet_id = sheet.search_element("sheetId").unwrap();
                let u_sheet = UserDefineSheetName::new(sheet.search_element("name").unwrap());
                let e_sheet = ExcelDefineSheetName::from(e_sheet_id);
                sheet_names.set(e_sheet, u_sheet);
            });
            sheet_names
        }
    }
    pub trait SheetName {
        fn get_sheet_name(&self) -> &str;
    }
    #[derive(PartialEq, Eq, Debug)]
    pub struct UserDefineSheetName(String);
    impl UserDefineSheetName {
        pub fn new(sheet_name: impl Into<String>) -> Self {
            UserDefineSheetName(sheet_name.into())
        }
    }
    impl SheetName for UserDefineSheetName {
        fn get_sheet_name(&self) -> &str {
            &self.0
        }
    }
    #[derive(PartialEq, Eq, Hash, Debug)]
    pub struct ExcelDefineSheetName(String);

    impl ExcelDefineSheetName {
        pub fn new(s: impl Into<String>) -> Self {
            ExcelDefineSheetName(s.into())
        }
    }
    impl From<&str> for ExcelDefineSheetName {
        fn from(id: &str) -> Self {
            ExcelDefineSheetName::new(format!("sheet{}", id))
        }
    }
    impl From<u8> for ExcelDefineSheetName {
        fn from(id: u8) -> Self {
            ExcelDefineSheetName::new(format!("sheet{}", id))
        }
    }
    impl SheetName for ExcelDefineSheetName {
        fn get_sheet_name(&self) -> &str {
            &self.0
        }
    }
    #[cfg(test)]
    mod sheet_name_tests {
        use crate::xml::nodes::node::XMLNode;

        use super::*;

        #[test]
        fn test_new() {
            let mut sheet_nams = SheetMap::new();
            let sheet1 = ExcelDefineSheetName::from(1);
            let user1 = UserDefineSheetName::new("user1");
            sheet_nams.set(sheet1, user1);
            let sheet2 = ExcelDefineSheetName::from(2);
            let user2 = UserDefineSheetName::new("user2");
            sheet_nams.set(sheet2, user2);
            let sheet3 = ExcelDefineSheetName::from(3);
            let user3 = UserDefineSheetName::new("user3");
            sheet_nams.set(sheet3, user3);
            let sheet4 = ExcelDefineSheetName::from(4);
            let user4 = UserDefineSheetName::new("user4");
            sheet_nams.set(sheet4, user4);
            let user5 = UserDefineSheetName::new("user5");
            let sheet5 = ExcelDefineSheetName::from(5);
            assert_eq!(
                sheet_nams.get_excel_sheet_name(UserDefineSheetName::new("user1")),
                Some("sheet1")
            );
            assert_eq!(
                sheet_nams.get_excel_sheet_name(UserDefineSheetName::new("user4")),
                Some("sheet4")
            );
            assert_eq!(sheet_nams.get_excel_sheet_name(user5), None);
            assert_eq!(
                sheet_nams.get_user_sheet_name(&ExcelDefineSheetName::from(1)),
                Some(&UserDefineSheetName::new("user1"))
            );
            assert_eq!(
                sheet_nams.get_user_sheet_name(&ExcelDefineSheetName::from(4)),
                Some(&UserDefineSheetName::new("user4"))
            );
            assert_eq!(sheet_nams.get_user_sheet_name(&sheet5), None);
        }
        #[test]
        fn create_sheet_names_test() {
            let source = r#"
                <sheets>
                    <sheet name="term1" sheetId="1" state="hidden" r:id="rId1"/>
                    <sheet name="term2" sheetId="2" state="hidden" r:id="rId2"/>
                    <sheet name="テーブル一覧" sheetId="8" r:id="rId3"/>
                </sheets>
        "#;
            let source_node = XMLNode::from(source);
            let mut tobe = SheetMap::new();
            tobe.set(
                ExcelDefineSheetName::new("sheet1"),
                UserDefineSheetName::new("term1"),
            );
            tobe.set(
                ExcelDefineSheetName::new("sheet2"),
                UserDefineSheetName::new("term2"),
            );
            tobe.set(
                ExcelDefineSheetName::new("sheet8"),
                UserDefineSheetName::new("テーブル一覧"),
            );
            assert_eq!(SheetMap::from(&source_node), tobe);
        }
    }
}
