
pub struct Visual {
    pub content: u32,
    pub x: u32,
    pub y: u32,
    pub id: Option<u32>,
}

pub enum Text {
    Title(String),
    ItemAnnotation(u32, String),
}
