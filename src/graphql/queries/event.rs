use chrono::NaiveDateTime;

use crate::database;
use crate::database::Constraints;
use crate::database::event::EventQueryBuilder;
use crate::graphql::GraphQLError;

#[derive(Debug, GraphQLInputObject)]
pub(crate) struct EventQuery {
    pub(crate) id: Option<i32>,
    pub(crate) name: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) timestamp: Option<NaiveDateTime>,
    pub(crate) timestamp_end: Option<NaiveDateTime>,
    pub(crate) price: Option<i32>,
    pub(crate) currency: Option<String>,
    pub(crate) location_id: Option<i32>,
    pub(crate) organizer_id: Option<i32>,
}

impl<'a> EventQuery {
    pub(crate) fn into_builder(self, connection: &'a diesel::PgConnection) -> EventQueryBuilder<'a> {
        let mut builder = database::Event::create_query_builder(Constraints::default(), connection);

        if let Some(id) = self.id {
            builder = builder.with_id(id);
        }
        if let Some(name) = self.name {
            builder = builder.with_name(name);
        }
        if let Some(description) = self.description {
            builder = builder.with_description(description);
        }
        if let Some(timestamp) = self.timestamp {
            builder = builder.with_timestamp(timestamp);
        }
        if let Some(timestamp_end) = self.timestamp_end {
            builder = builder.with_timestamp_end(timestamp_end);
        }
        if let Some(price) = self.price {
            builder = builder.with_price(price);
        }
        if let Some(currency) = self.currency {
            builder = builder.with_currency(currency);
        }
        if let Some(location_id) = self.location_id {
            builder = builder.with_location_id(location_id);
        }
        if let Some(organizer_id) = self.organizer_id {
            builder = builder.with_organizer_id(organizer_id);
        }

        builder
    }
}
