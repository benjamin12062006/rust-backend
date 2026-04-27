use uuid::Uuid;

#[derive(Debug)]
pub struct User {
    pub user_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub user_name: String,
    pub password: String,
}
