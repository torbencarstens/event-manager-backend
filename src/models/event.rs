use chrono::NaiveDateTime;

use crate::database::Location;
use crate::database::Organizer;
use crate::graphql::graphqli64::GraphQLi64;

#[derive(Debug, GraphQLObject)]
pub struct Event {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) timestamp: GraphQLi64,
    pub(crate) timestamp_end: GraphQLi64,
    pub(crate) price: Option<i32>,
    pub(crate) currency: Option<String>,
    pub(crate) location: Location,
    pub(crate) organizer: Option<Organizer>,
}
