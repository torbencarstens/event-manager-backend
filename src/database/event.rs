use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, RunQueryDsl};
use diesel::expression::AsExpression;
use diesel::pg::data_types::PgTimestamp;
use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::query_builder::AsQuery;
use diesel::sql_types::Text;

use crate::database::{Constraints, DieselResult, Location, Organizer, QueryBuilder};
use crate::database::schema::events::{self, *};
use crate::graphql::graphqli64::GraphQLi64;
use crate::models;

#[derive(Clone, Debug, Queryable)]
pub struct Event {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) timestamp: NaiveDateTime,
    pub(crate) timestamp_end: NaiveDateTime,
    pub(crate) price: Option<i32>,
    pub(crate) currency: Option<String>,
    pub(crate) location_id: i32,
    pub(crate) organizer_id: Option<i32>,
}

pub(crate) struct EventQueryBuilder<'a> {
    query: events::BoxedQuery<'a, Pg>,
    connection: &'a diesel::PgConnection,
}

impl<'a> EventQueryBuilder<'a> {
    pub fn with_id(mut self, value: i32) -> EventQueryBuilder<'a> {
        self.query = self.query.filter(id.eq(value));
        self
    }

    pub fn with_name<T>(mut self, value: T) -> EventQueryBuilder<'a>
        where
            T: AsExpression<Text>,
            T::Expression: BoxableExpression<events::table, Pg>,
            <T as diesel::expression::AsExpression<diesel::sql_types::Text>>::Expression: 'a
    {
        self.query = self.query.filter(name.eq(value));
        self
    }

    pub fn with_description(mut self, value: String) -> EventQueryBuilder<'a> {
        self.query = self.query.filter(description.eq(value));
        self
    }

    pub fn with_timestamp(mut self, value: NaiveDateTime) -> EventQueryBuilder<'a> {
        self.query = self.query.filter(timestamp.eq(value));
        self
    }

    pub fn with_timestamp_end(mut self, value: NaiveDateTime) -> EventQueryBuilder<'a> {
        self.query = self.query.filter(timestamp_end.eq(value));
        self
    }

    pub fn with_price(mut self, value: i32) -> EventQueryBuilder<'a> {
        self.query = self.query.filter(price.eq(value));
        self
    }

    pub fn with_currency(mut self, value: String) -> EventQueryBuilder<'a> {
        self.query = self.query.filter(currency.eq(value));
        self
    }

    pub fn with_location_id(mut self, value: i32) -> EventQueryBuilder<'a> {
        self.query = self.query.filter(location_id.eq(value));
        self
    }

    pub fn with_organizer_id(mut self, value: i32) -> EventQueryBuilder<'a> {
        self.query = self.query.filter(organizer_id.eq(value));
        self
    }
}

impl<'a> QueryBuilder for EventQueryBuilder<'a> {
    type Item = models::Event;

    fn execute(self) -> DieselResult<Vec<Self::Item>> {
        let connection = self.connection;

        self.query
            .order((timestamp.asc(), timestamp_end.asc()))
            .load::<Event>(connection)?
            .into_iter()
            .map(|event| Event::from_database_event(event, connection))
            .collect()
    }
}

impl Event {
    pub(crate) fn from_database_event(event: Event, connection: &diesel::PgConnection) -> DieselResult<models::Event> {
        let location = Location::create_query_builder(Constraints::default(), connection)
            .with_id(event.location_id)
            .execute()?
            .pop()
            .ok_or(diesel::NotFound)?;
        let organizer = event.organizer_id.and_then(|org_id|
            Organizer::create_query_builder(Constraints::default(), connection)
                .with_id(org_id)
                .execute()
                .ok()?
                .pop());

        Ok(models::Event {
            id: event.id,
            name: event.name,
            description: event.description,
            timestamp: GraphQLi64(event.timestamp.timestamp()),
            timestamp_end: GraphQLi64(event.timestamp_end.timestamp()),
            price: event.price,
            currency: event.currency,
            location,
            organizer,
        })
    }

    pub(crate) fn from_database_join((event, location, organizer): (Event, Location, Option<Organizer>)) -> models::Event {
        models::Event {
            id: event.id,
            name: event.name,
            description: event.description,
            timestamp: GraphQLi64(event.timestamp.timestamp()),
            timestamp_end: GraphQLi64(event.timestamp_end.timestamp()),
            price: event.price,
            currency: event.currency,
            location,
            organizer,
        }
    }

    pub(crate) fn get(constraints: Constraints, connection: &diesel::PgConnection) -> DieselResult<Vec<models::Event>> {
        Ok(events::table
            .inner_join(crate::database::schema::locations::table)
            .left_join(crate::database::schema::organizers::table)
            .limit(constraints.limit)
            .offset(constraints.offset)
            .load::<(Event, Location, Option<Organizer>)>(connection)?
            .into_iter()
            .map(Event::from_database_join)
            .collect())
    }

    pub(crate) fn create_query_builder(constraints: Constraints, connection: &diesel::PgConnection) -> EventQueryBuilder {
        EventQueryBuilder {
            query: events::table
                .limit(constraints.limit)
                .offset(constraints.offset)
                .into_boxed(),
            connection,
        }
    }

    pub(crate) fn insert(self, connection: &diesel::PgConnection) -> DieselResult<Event> {
        diesel::insert_into(events::table)
            .values(vec![(
                name.eq(self.name),
                description.eq(self.description),
                timestamp.eq(self.timestamp),
                timestamp_end.eq(self.timestamp_end),
                price.eq(self.price),
                currency.eq(self.currency),
                location_id.eq(self.location_id),
                organizer_id.eq(self.organizer_id)
            )])
            .load::<Event>(connection)?
            .pop()
            .ok_or(diesel::result::Error::NotFound)
    }
}
