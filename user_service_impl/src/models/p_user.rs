use actix_web::{App, error, http, HttpRequest, HttpResponse};
use cdrs::{self, types::prelude::*};
use cdrs::frame::IntoBytes;
use cdrs::frame::TryFromRow;
use cdrs::types::from_cdrs::FromCDRSByName;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq,IntoCDRSValue,TryFromRow)]
pub struct PUser {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String
}