pub(crate) use event::Event;
pub(crate) use event_tag::EventTag;
pub(crate) use location::Location;
pub(crate) use organizer::Organizer;
pub use primary::PrimaryDb;
pub(crate) use tag::Tag;

use crate::graphql::graphqli64::GraphQLi64;

pub(crate) mod event;
pub(crate) mod event_tag;
pub(crate) mod location;
pub(crate) mod mock;
pub(crate) mod organizer;
pub(crate) mod schema;
pub(crate) mod tag;
mod primary;

pub(crate) type DieselResult<T> = Result<T, diesel::result::Error>;

#[derive(Clone, GraphQLInputObject)]
pub(crate) struct Constraints {
    pub(crate) offset: GraphQLi64,
    pub(crate) limit: GraphQLi64,
}

impl Default for Constraints {
    fn default() -> Self {
        Constraints {
            offset: GraphQLi64(0),
            limit: GraphQLi64(50),
        }
    }
}

pub(crate) trait QueryBuilder {
    type Item;

    // execute can't have a default impl since we would have to type `query` for that to work, which I can't
    fn execute(self) -> DieselResult<Vec<Self::Item>>;
}
