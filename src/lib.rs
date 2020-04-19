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

use crate::database::{Constraints, DieselResult, EventTag, Organizer, QueryBuilder, Tag};
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
    let mut location_ids = vec![];
    let mut organizer_ids = vec![];
    let mut tag_ids = vec![];
    let mut event_ids = vec![];

    for _ in 0..10 {
        let tag = Tag::mock(None).unwrap().insert(Constraints::default(), connection)?;
        tag_ids.push(tag.id);
    }

    for _ in 0..(amount / 3) {
        let location = Location::mock(None).unwrap().insert(connection)?;
        location_ids.push(location.id);
        let organizer = Organizer::mock(None).unwrap().insert(connection)?;
        organizer_ids.push(organizer.id);
    }
    let mut event_data = HashMap::new();
    event_data.insert("locations".to_string(), location_ids);
    event_data.insert("organizers".to_string(), organizer_ids);

    for _ in 0..amount {
        let event = Event::mock(Some(event_data.clone())).unwrap().insert(connection)?;
        event_ids.push(event.id);
    }

    let mut event_tag_data = HashMap::new();
    event_tag_data.insert("tags".to_string(), tag_ids);
    for event_id in event_ids {
        event_tag_data.insert("event_id".to_string(), vec![event_id]);
        let event_tag = EventTag::mock(Some(event_tag_data.clone())).unwrap().insert(connection)?;
    }

    Ok(())
}
