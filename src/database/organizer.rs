use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, RunQueryDsl};
use diesel::pg::data_types::PgTimestamp;
use diesel::pg::Pg;
use diesel::prelude::*;

use crate::database::{Constraints, DieselResult, QueryBuilder};
use crate::database::schema::organizers::{self, *};

#[derive(Clone, Debug, GraphQLObject, Identifiable, Queryable)]
pub struct Organizer {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) website: Option<String>,
}

pub(crate) struct OrganizerQueryBuilder<'a> {
    query: organizers::BoxedQuery<'a, Pg>,
    connection: &'a diesel::PgConnection,
}

impl<'a> OrganizerQueryBuilder<'a> {
    pub fn with_id(mut self, value: i32) -> OrganizerQueryBuilder<'a> {
        self.query = self.query.filter(id.eq(value));
        self
    }

    pub fn with_name(mut self, location_name: String) -> OrganizerQueryBuilder<'a> {
        self.query = self.query.filter(name.eq(location_name));
        self
    }

    pub fn with_website(mut self, location_website: String) -> OrganizerQueryBuilder<'a> {
        self.query = self.query.filter(website.eq(location_website));
        self
    }
}

impl<'a> QueryBuilder for OrganizerQueryBuilder<'a> {
    type Item = Organizer;

    fn execute(self) -> DieselResult<Vec<Organizer>> {
        self.query
            .load::<Organizer>(self.connection)
    }
}

impl Organizer {
    pub(crate) fn count(connection: &diesel::PgConnection) -> DieselResult<i64> {
        organizers::table
            .select(diesel::dsl::count(id))
            .first(connection)
    }

    pub(crate) fn get(constraints: Constraints, connection: &diesel::PgConnection) -> DieselResult<Vec<Organizer>> {
        organizers::table
            .limit(constraints.limit.0)
            .offset(constraints.offset.0)
            .load::<Organizer>(connection)
    }

    pub(crate) fn create_query_builder(constraints: Constraints, connection: &diesel::PgConnection) -> OrganizerQueryBuilder {
        OrganizerQueryBuilder {
            query: organizers::table
                .limit(constraints.limit.0)
                .offset(constraints.offset.0)
                .into_boxed(),
            connection,
        }
    }

    pub(crate) fn insert(self, connection: &diesel::PgConnection) -> DieselResult<Organizer> {
        diesel::insert_into(organizers::table)
            .values(vec![(
                name.eq(self.name),
                website.eq(self.website),
            )])
            .load::<Organizer>(connection)?
            .pop()
            .ok_or(diesel::result::Error::NotFound)
    }
}
