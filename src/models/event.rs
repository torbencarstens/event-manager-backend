use chrono::NaiveDateTime;

use crate::database::{self, Location, Organizer};
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
    pub(crate) tags: Vec<InnerEventTag>,
}

impl Into<InnerEventTag> for database::Tag {
    fn into(self) -> InnerEventTag {
        InnerEventTag {
            id: self.id,
            name: self.name,
            description: self.description,
        }
    }
}

#[derive(Debug, GraphQLObject)]
pub struct InnerEventTag {
    id: i32,
    name: String,
    description: Option<String>,
}
