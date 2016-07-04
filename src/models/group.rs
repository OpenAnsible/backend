
pub struct Group {
    gid   : String,
    name  : String,
    descp : String,
    parent: String
}

impl ToJson for Group {

}
impl ToString for Group {

}

impl ToDoc for Group {

}

impl Group {
    pub fn new() -> Group {

    }
    pub fn keys(&self) -> Vec<String> {

    }
    pub fn parent(&self) -> Option<Group> {

    }
    pub fn children(&self) -> Option<Vec<Group>> {
        
    }
    
}

