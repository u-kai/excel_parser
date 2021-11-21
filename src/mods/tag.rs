pub mod tag {
    use std::collections::HashMap;
    #[derive(PartialEq, Debug)]
    pub struct Tag {
        element: String,
        properties: Option<HashMap<String, String>>,
        children: String,
    }

    impl Tag {
        pub fn new(
            element: String,
            children: Option<String>,
            properties: Option<HashMap<String, String>>,
        ) -> Self {
            let children = match children {
                Some(chil) => chil,
                None => String::new(),
            };
            Tag {
                element,
                properties,
                children,
            }
        }
        pub fn get_children(&self)->&String {
            &self.children
        }
    }
}
