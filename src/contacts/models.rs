use serde::Deserialize;

pub struct Contact {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub phone: Option<String>
}

#[derive(Deserialize)]
pub struct NewContact {
    pub name: String,
    pub email: String,
    pub phone: Option<String>
}
