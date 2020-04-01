#![feature(decl_macro, proc_macro_hygiene)]
extern crate chrono;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate juniper;
extern crate rand;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate rocket_http;
extern crate url;

use std::collections::HashMap;

use chrono::NaiveDateTime;
use diesel::PgConnection;
use diesel::result::Error;

use crate::database::{Constraints, DieselResult, Organizer, QueryBuilder};
use crate::database::event::Event;
use crate::database::location::Location;
use crate::database::mock::Mockable;
pub use crate::database::PrimaryDb;

pub(crate) mod database;
pub(crate) mod models;
pub mod graphql;
pub mod routes;

pub fn return_all_locations(connection: &diesel::PgConnection) -> DieselResult<Vec<Location>> {
    Location::create_query_builder(Constraints::default(), connection).execute()
}

pub fn return_all_events(connection: &diesel::PgConnection) -> DieselResult<Vec<models::Event>> {
    Event::create_query_builder(Constraints::default(), connection).execute()
}

pub fn return_all_organizers(connection: &diesel::PgConnection) -> DieselResult<Vec<Organizer>> {
    Organizer::create_query_builder(Constraints::default(), connection).execute()
}

pub fn mock(amount: u16, connection: &diesel::PgConnection) -> DieselResult<()> {
    for _ in 0..amount {
        let location = Location::mock(None).unwrap().insert(connection)?;
        let organizer = Organizer::mock(None).unwrap().insert(connection)?;

        let mut event_data = HashMap::new();
        event_data.insert("location_id".to_string(), location.id);
        event_data.insert("organizer_id".to_string(), organizer.id);
        let event = Event::mock(Some(event_data)).unwrap().insert(connection)?;
    }

    Ok(())
}
