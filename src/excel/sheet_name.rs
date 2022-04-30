use std::collections::HashMap;

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
pub struct ExcelDefineSheetName<'a>(&'a str);

impl<'a> ExcelDefineSheetName<'a> {
    pub fn new(s: &'a str) -> Self {
        ExcelDefineSheetName(s)
    }
}
impl<'a> SheetName for ExcelDefineSheetName<'a> {
    fn get_sheet_name(&self) -> &str {
        self.0
    }
}
#[derive(PartialEq, Eq, Debug)]
struct SheetNames<'a>(HashMap<ExcelDefineSheetName<'a>, UserDefineSheetName<'a>>);
pub trait SheetNameConvertor<'a> {
    fn get_excel_sheet_name(
        &self,
        sheet_name: &UserDefineSheetName,
    ) -> Option<&ExcelDefineSheetName>;
    fn get_user_sheet_name(
        &self,
        sheet_name: &'a ExcelDefineSheetName,
    ) -> Option<&'a UserDefineSheetName>;
}
impl<'a> SheetNames<'a> {
    pub fn new() -> Self {
        SheetNames(HashMap::new())
    }
    pub fn set(
        &mut self,
        e_sheet_name: ExcelDefineSheetName<'a>,
        u_sheet_name: UserDefineSheetName<'a>,
    ) {
        self.0.insert(e_sheet_name, u_sheet_name);
    }
}
impl<'a> SheetNameConvertor<'a> for SheetNames<'a> {
    fn get_excel_sheet_name(
        &self,
        sheet_name: &UserDefineSheetName,
    ) -> Option<&ExcelDefineSheetName> {
        self.0
            .keys()
            .filter_map(|key| self.0.get_key_value(key))
            .find(|(_e, u)| *u == sheet_name)
            .map(|s| s.0)
    }
    fn get_user_sheet_name(
        &self,
        sheet_name: &'a ExcelDefineSheetName,
    ) -> Option<&UserDefineSheetName> {
        self.0.get(sheet_name).iter().map(|s| *s).next()
    }
}
#[cfg(test)]
mod workbook_tests {
    use super::*;

    #[test]
    fn test_new() {
        let mut sheet_nams = SheetNames::new();
        let sheet1 = ExcelDefineSheetName::new("sheet1");
        let user1 = UserDefineSheetName::new("user1");
        sheet_nams.set(sheet1, user1);
        let sheet2 = ExcelDefineSheetName::new("sheet2");
        let user2 = UserDefineSheetName::new("user2");
        sheet_nams.set(sheet2, user2);
        let sheet3 = ExcelDefineSheetName::new("sheet3");
        let user3 = UserDefineSheetName::new("user3");
        sheet_nams.set(sheet3, user3);
        let sheet4 = ExcelDefineSheetName::new("sheet4");
        let user4 = UserDefineSheetName::new("user4");
        sheet_nams.set(sheet4, user4);
        let user5 = UserDefineSheetName::new("user5");
        let sheet5 = ExcelDefineSheetName::new("sheet5");
        assert_eq!(
            sheet_nams.get_excel_sheet_name(&UserDefineSheetName::new("user1")),
            Some(&ExcelDefineSheetName::new("sheet1"))
        );
        assert_eq!(
            sheet_nams.get_excel_sheet_name(&UserDefineSheetName::new("user4")),
            Some(&ExcelDefineSheetName::new("sheet4"))
        );
        assert_eq!(sheet_nams.get_excel_sheet_name(&user5), None);
        assert_eq!(
            sheet_nams.get_user_sheet_name(&ExcelDefineSheetName::new("sheet1")),
            Some(&UserDefineSheetName::new("user1"))
        );
        assert_eq!(
            sheet_nams.get_user_sheet_name(&ExcelDefineSheetName::new("sheet4")),
            Some(&UserDefineSheetName::new("user4"))
        );
        assert_eq!(sheet_nams.get_user_sheet_name(&sheet5), None);
    }
}
