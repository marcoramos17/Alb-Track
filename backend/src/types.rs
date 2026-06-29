use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub first_name: String,
    pub last_name: String,
    pub phone: Option<String>,
    pub birth_date: String,
    pub password: String,
}
