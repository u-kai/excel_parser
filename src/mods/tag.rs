pub mod tag {
use std::collections::HashMap;
pub struct Tag {
    element:String,
    properties:Option<HashMap<String,String>>,
    children:String
}

impl Tag {
    pub fn new(element:String,children:Option<String>,properties:Option<HashMap<String,String>>) ->Self {
        let children = match children {
            Some(chil)=>chil,
            None => String::new()
        };
        Tag{element,properties,children}
    }
}

}
