use chrono::{Datelike, NaiveDateTime, Utc};

use crate::database::Event;
use crate::graphql::graphqli64::GraphQLi64;
use crate::graphql::GraphQLError;

#[derive(Debug, GraphQLInputObject)]
pub struct EventInput {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) timestamp: GraphQLi64,
    pub(crate) timestamp_end: GraphQLi64,
    pub(crate) price: Option<i32>,
    pub(crate) currency: Option<String>,
    pub(crate) location_id: i32,
    pub(crate) organizer_id: Option<i32>,
}

impl EventInput {
    pub(crate) fn validate(&self) -> Result<(), GraphQLError> {
        if self.name.len() < 2 {
            return Err(GraphQLError::ValidationError(format!("`{}` field must at least contain 2 characters", "name")));
        }
        if let Some(price) = self.price {
            if price < 0 {
                return Err(GraphQLError::ValidationError(format!("`{}` cannot be negative", "price")));
            }
        }
        if let Some(organizer_id) = self.organizer_id {
            if organizer_id < 0 {
                return Err(GraphQLError::ValidationError(format!("`{}` cannot be negative", "organizer_id")));
            }
        }
        if self.location_id < 0 {
            return Err(GraphQLError::ValidationError(format!("`{}` cannot be negative", "location_id")));
        }
        if self.timestamp.0 <= Utc::now().timestamp() {
            return Err(GraphQLError::ValidationError(format!("`{}` cannot be in the past", "timestamp")));
        }

        Ok(())
    }
}

impl Into<Event> for EventInput {
    fn into(self) -> Event {
        Event {
            id: -1,
            name: self.name,
            description: self.description,
            timestamp: NaiveDateTime::from_timestamp(self.timestamp.0, 0),
            timestamp_end: NaiveDateTime::from_timestamp(self.timestamp_end.0, 0),
            price: self.price,
            currency: self.currency,
            location_id: self.location_id,
            organizer_id: self.organizer_id,
        }
    }
}
