#![feature(proc_macro_hygiene, decl_macro)]
extern crate event_manager;
#[macro_use]
extern crate rocket;

use std::{env, io};
use std::collections::HashMap;

use diesel::{Connection, PgConnection};
use rocket::Config;
use rocket::config::{Environment, Value};

use event_manager::graphql::{MutationRoot, QueryRoot};
use event_manager::PrimaryDb;
use event_manager::routes;

fn main() -> io::Result<()> {
    let database_default = "postgres://postgres:password@localhost/event_manager".to_string();
    if env::args()
        .filter(|arg| arg.contains("populate"))
        .next()
        .is_some() {
        let connection = PgConnection::establish(
            std::env::var("DATABASE_URL")
                .unwrap_or(database_default)
                .as_ref())
            .expect("Failed to create database connection");

        println!("{:#?}", event_manager::mock(300, &connection));
        return Ok(());
    }

    let mut config = Config::active().expect("Couldn't find active rocket config");
    let mut database_url = std::env::var("DATABASE_URL");

    let environment = std::env::var("ROCKET_ENV").unwrap_or("DEV".into());

    let database_url = if environment.starts_with("prod") {
        database_url.expect("Couldn't find DATABASE_URL in environment")
    } else {
        database_url.unwrap_or(database_default)
    };

    let mut database_config = HashMap::new();
    let mut databases = HashMap::new();
    let mut extras = HashMap::new();

    database_config.insert("url".to_string(), Value::from(database_url));
    databases.insert("primary_db".to_string(), Value::from(database_config));
    extras.insert("databases".to_string(), Value::from(databases));
    config.set_extras(extras);

    rocket::custom(config)
        .attach(PrimaryDb::fairing())
        .manage(routes::Schema::new(QueryRoot, MutationRoot))
        .mount("/", routes![
            routes::graphiql,
            routes::get_graphql_handler,
            routes::post_graphql_handler,
        ])
        .launch();

    Ok(())
}
