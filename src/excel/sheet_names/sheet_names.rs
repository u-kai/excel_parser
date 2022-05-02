use std::collections::HashMap;

use crate::xml::nodes::node::XMLNode;

#[derive(PartialEq, Eq, Debug)]
pub struct SheetNames<'a>(HashMap<ExcelDefineSheetName, UserDefineSheetName<'a>>);

impl<'a> SheetNames<'a> {
    pub fn new() -> Self {
        SheetNames(HashMap::new())
    }
    fn set(&mut self, e_sheet_name: ExcelDefineSheetName, u_sheet_name: UserDefineSheetName<'a>) {
        self.0.insert(e_sheet_name, u_sheet_name);
    }
    pub fn get_excel_sheet_name(
        &self,
        sheet_name: &UserDefineSheetName,
    ) -> Option<&ExcelDefineSheetName> {
        self.0
            .keys()
            .filter_map(|key| self.0.get_key_value(key))
            .find(|(_e, u)| *u == sheet_name)
            .map(|s| s.0)
    }
    pub fn get_user_sheet_name(
        &self,
        sheet_name: &'a ExcelDefineSheetName,
    ) -> Option<&UserDefineSheetName> {
        self.0.get(sheet_name).iter().map(|s| *s).next()
    }
}

impl<'a> From<&'a XMLNode> for SheetNames<'a> {
    fn from(sheets_node: &'a XMLNode) -> Self {
        let mut sheet_names = SheetNames::new();
        let sheets = sheets_node
            .search_all_nodes("sheet")
            .expect(format!("invalid node {:?}", sheets_node).as_str());
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
pub struct UserDefineSheetName<'a>(&'a str);
impl<'a> UserDefineSheetName<'a> {
    pub fn new(sheet_name: &'a str) -> Self {
        UserDefineSheetName(sheet_name)
    }
}
impl<'a> SheetName for UserDefineSheetName<'a> {
    fn get_sheet_name(&self) -> &str {
        self.0
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
mod workbook_tests {
    use crate::xml::nodes::node::XMLNode;

    use super::*;

    #[test]
    fn test_new() {
        let mut sheet_nams = SheetNames::new();
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
            sheet_nams.get_excel_sheet_name(&UserDefineSheetName::new("user1")),
            Some(&ExcelDefineSheetName::from(1))
        );
        assert_eq!(
            sheet_nams.get_excel_sheet_name(&UserDefineSheetName::new("user4")),
            Some(&ExcelDefineSheetName::from(4))
        );
        assert_eq!(sheet_nams.get_excel_sheet_name(&user5), None);
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
        let mut tobe = SheetNames::new();
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
        assert_eq!(SheetNames::from(&source_node), tobe);
    }
}
