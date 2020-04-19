use crate::database::Location;
use crate::graphql::graphqli64::GraphQLi64;
use crate::models::Event;

#[derive(Debug, GraphQLObject)]
pub struct Tag {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) description: Option<String>,
    pub(crate) events: Vec<Event>,
}
