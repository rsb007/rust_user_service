use crate::user_service_impl::user_command::models::UserCommand;
use crate::models::user_registration::UserRegistration;
use crate::models::p_user::PUser;

#[derive(Serialize, Deserialize, Debug, Clone, Event)]
#[event_type_version("1.0")]
#[event_source("")]
pub enum UserEvent {
    UserCreated(PUser),
}

impl From<UserCommand> for UserEvent {
    fn from(source: UserCommand) -> Self {
        match source {
            UserCommand::CreateUser(UserRegistration) =>
                UserEvent::UserCreated(PUser{
                    id:"".to_string() ,
                    name: UserRegistration.name,
                    email: UserRegistration.email,
                    password: UserRegistration.password
                }),
            }
    }
}
