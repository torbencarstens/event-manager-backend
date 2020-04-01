#![feature(proc_macro_hygiene, decl_macro)]
extern crate event_manager;
#[macro_use]
extern crate rocket;

use std::env;

use diesel::{Connection, PgConnection};

use event_manager::graphql::{MutationRoot, QueryRoot};
use event_manager::PrimaryDb;
use event_manager::routes;

fn main() {
    let database_default = "postgres://postgres:password@localhost/event_manager".to_string();
    let connection = PgConnection::establish(
        std::env::var("DATABASE_URL")
            .unwrap_or(database_default)
            .as_ref())
        .expect("Failed to create database connection");

    if env::args()
        .filter(|arg| arg.contains("populate"))
        .next()
        .is_some() {
        println!("{:#?}", event_manager::mock(10, &connection));
        return ();
    }

    rocket::ignite()
        .attach(PrimaryDb::fairing())
        .manage(routes::Schema::new(QueryRoot, MutationRoot))
        .mount("/", routes![
            routes::graphiql,
            routes::get_graphql_handler,
            routes::post_graphql_handler,
        ])
        .launch();
}
