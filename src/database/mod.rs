pub(crate) use event::Event;
pub(crate) use location::Location;
pub(crate) use organizer::Organizer;
pub use primary::PrimaryDb;

pub(crate) mod event;
pub(crate) mod location;
pub(crate) mod mock;
pub(crate) mod organizer;
pub(crate) mod schema;
mod primary;

pub(crate) type DieselResult<T> = Result<T, diesel::result::Error>;

pub(crate) struct Constraints {
    pub(crate) offset: i64,
    pub(crate) limit: i64,
}

impl Default for Constraints {
    fn default() -> Self {
        Constraints {
            offset: 0,
            limit: 50,
        }
    }
}

pub(crate) trait QueryBuilder {
    type Item;

    // execute can't have a default impl since we would have to type `query` for that to work, which I can't
    fn execute(self) -> DieselResult<Vec<Self::Item>>;
}
