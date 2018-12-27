use crate::modals::user_registration::UserRegistration;

#[derive(Debug)]
pub enum UserCommand {
    CreateUser(UserRegistration),
    GetUser{user_id: String},
    GetUsers
}