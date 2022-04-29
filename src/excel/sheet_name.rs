use std::collections::HashMap;

type UserDefineSheetName<'a> = &'a str;
type ExcelDefineSheetName<'a> = &'a str;
struct SheetNames<'a>(HashMap<ExcelDefineSheetName<'a>, UserDefineSheetName<'a>>);
pub trait SheetNameConvertor {
    fn get_excel_sheet_name(&self, sheet_name: UserDefineSheetName)
        -> Option<ExcelDefineSheetName>;
    fn get_user_sheet_name(&self, sheet_name: ExcelDefineSheetName) -> Option<UserDefineSheetName>;
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
impl<'a> SheetNameConvertor for SheetNames<'a> {
    fn get_excel_sheet_name(
        &self,
        sheet_name: UserDefineSheetName,
    ) -> Option<ExcelDefineSheetName> {
        self.0
            .keys()
            .filter_map(|key| self.0.get_key_value(*key))
            .find(|(_e, u)| **u == sheet_name)
            .map(|s| *s.0)
    }
    fn get_user_sheet_name(&self, sheet_name: UserDefineSheetName) -> Option<ExcelDefineSheetName> {
        self.0.get(sheet_name).iter().map(|s| **s).next()
    }
}
#[cfg(test)]
mod workbook_tests {
    use super::*;

    #[test]
    fn test_new() {
        let mut sheet_nams = SheetNames::new();
        sheet_nams.set("sheet1", "user1");
        sheet_nams.set("sheet2", "user2");
        sheet_nams.set("sheet3", "user3");
        sheet_nams.set("sheet4", "user4");
        assert_eq!(sheet_nams.get_excel_sheet_name("user1"), Some("sheet1"));
        assert_eq!(sheet_nams.get_excel_sheet_name("user4"), Some("sheet4"));
        assert_eq!(sheet_nams.get_excel_sheet_name("user5"), None);
        assert_eq!(sheet_nams.get_user_sheet_name("sheet1"), Some("user1"));
        assert_eq!(sheet_nams.get_user_sheet_name("sheet4"), Some("user4"));
        assert_eq!(sheet_nams.get_user_sheet_name("sheet5"), None);
    }
}
