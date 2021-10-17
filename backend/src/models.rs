#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub pwhash: String,
}
