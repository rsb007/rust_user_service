#[macro_use]
extern crate cdrs;
#[macro_use]
extern crate cdrs_helpers_derive;
extern crate actix_web;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate eventsourcing;
#[macro_use]
extern crate eventsourcing_derive;

pub mod env_setup;

pub mod modals;

pub mod user_service_impl;

pub mod controller;

pub mod constants;

pub mod utilities;