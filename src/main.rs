extern crate actix_web;
extern crate listenfd;
extern crate env_logger;

use user_service_impl::controller::handler::{create_user,user_login,get_user,get_all_users};

use actix_web::{App,http,server};
use listenfd::ListenFd;

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let mut listenfd: ListenFd = ListenFd::from_env();
    let mut server = server::new(|| {
        App::new()
            .resource("/create_user", |r| r.method(http::Method::POST)
                .with(create_user))
            .resource("/login", |r| r.method(http::Method::POST)
                .with(user_login))
            .resource("/get_user/{user_id}", |r| r.method(http::Method::GET)
                .with(get_user))
            .resource("/get_user", |r| r.method(http::Method::GET)
                .with(get_all_users))
    });
    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)
    } else {
        server.bind("127.0.0.1:3080").unwrap()
    };

    server.run();
}