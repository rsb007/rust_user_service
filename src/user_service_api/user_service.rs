use actix_web::{Json, Path, Result};
use user_service_impl::controller::error::CustomError;

use user_service_impl::models::user::User;
use user_service_impl::models::user_login::UserLogin;
use user_service_impl::models::user_registration::UserRegistration;

pub trait UserService {
    fn create_user(user_reg: Json<UserRegistration>) -> Result<Json<User>, CustomError>;
    fn get_user(user_id: Path<i32>) -> Result<Json<User>, CustomError>;
    fn get_all_users() -> Result<Vec<User>>;
    fn user_login(user_login: Json<UserLogin>) -> Result<&'static str, CustomError>;
}