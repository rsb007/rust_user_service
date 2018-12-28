use crate::models::user_registration::UserRegistration;

#[derive(Debug)]
pub enum UserCommand {
    CreateUser(UserRegistration),
}