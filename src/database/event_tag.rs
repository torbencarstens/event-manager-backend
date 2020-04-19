use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, RunQueryDsl};
use diesel::pg::data_types::PgTimestamp;
use diesel::pg::Pg;
use diesel::prelude::*;

use crate::database::{Constraints, DieselResult, QueryBuilder};
use crate::database::schema::event_tags::{self, *};

#[derive(Clone, Debug, GraphQLInputObject, Queryable)]
pub struct EventTag {
    pub(crate) tag_id: i32,
    pub(crate) event_id: i32,
}

pub(crate) struct EventTagQueryBuilder<'a> {
    query: event_tags::BoxedQuery<'a, Pg>,
    connection: &'a diesel::PgConnection,
}

impl<'a> EventTagQueryBuilder<'a> {
    pub fn with_tag_id(mut self, value: i32) -> EventTagQueryBuilder<'a> {
        self.query = self.query.filter(tag_id.eq(value));
        self
    }

    pub fn with_event_id(mut self, value: i32) -> EventTagQueryBuilder<'a> {
        self.query = self.query.filter(event_id.eq(value));
        self
    }
}

impl<'a> QueryBuilder for EventTagQueryBuilder<'a> {
    type Item = EventTag;

    fn execute(self) -> DieselResult<Vec<EventTag>> {
        self.query
            .load::<EventTag>(self.connection)
    }
}

impl EventTag {
    pub(crate) fn count(connection: &diesel::PgConnection) -> DieselResult<i64> {
        event_tags::table
            .select(diesel::dsl::count(tag_id))
            .first(connection)
    }

    pub(crate) fn get(constraints: Constraints, connection: &diesel::PgConnection) -> DieselResult<Vec<EventTag>> {
        event_tags::table
            .limit(constraints.limit.0)
            .offset(constraints.offset.0)
            .load::<EventTag>(connection)
    }

    pub(crate) fn create_query_builder(constraints: Constraints, connection: &diesel::PgConnection) -> EventTagQueryBuilder {
        EventTagQueryBuilder {
            query: event_tags::table
                .limit(constraints.limit.0)
                .offset(constraints.offset.0)
                .into_boxed(),
            connection,
        }
    }

    pub(crate) fn insert(self, connection: &diesel::PgConnection) -> DieselResult<EventTag> {
        diesel::insert_into(event_tags::table)
            .values(vec![(
                tag_id.eq(self.tag_id),
                event_id.eq(self.event_id),
            )])
            .load::<EventTag>(connection)?
            .pop()
            .ok_or(diesel::result::Error::NotFound)
    }
}
