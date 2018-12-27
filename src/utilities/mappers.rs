use crate::modals::p_user::PUser;
use crate::modals::user::User;

pub fn mappers(user: PUser) -> User {
    User {
        id: user.id,
        name: user.name,
        email: user.email,
    }
}