use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, RunQueryDsl};
use diesel::pg::data_types::PgTimestamp;
use diesel::pg::Pg;
use diesel::prelude::*;

use crate::database::{Constraints, DieselResult, Event, EventTag, QueryBuilder};
use crate::database::schema::tags::{self, *};
use crate::models;

#[derive(Clone, Debug, GraphQLObject, Identifiable, Queryable)]
pub struct Tag {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) description: Option<String>,
}

pub(crate) struct TagQueryBuilder<'a> {
    query: tags::BoxedQuery<'a, Pg>,
    connection: &'a diesel::PgConnection,
}

impl<'a> TagQueryBuilder<'a> {
    pub fn with_id(mut self, value: i32) -> TagQueryBuilder<'a> {
        self.query = self.query.filter(id.eq(value));
        self
    }

    pub fn with_name(mut self, location_name: String) -> TagQueryBuilder<'a> {
        self.query = self.query.filter(name.eq(location_name));
        self
    }

    pub fn with_description(mut self, location_description: String) -> TagQueryBuilder<'a> {
        self.query = self.query.filter(description.eq(location_description));
        self
    }
}

impl<'a> QueryBuilder for TagQueryBuilder<'a> {
    type Item = Tag;

    fn execute(self) -> DieselResult<Vec<Self::Item>> {
        let connection = self.connection;

        self.query
            .load::<Tag>(connection)
    }
}

impl Tag {
    pub(crate) fn into_model(self, constraints: Constraints, connection: &diesel::PgConnection) -> DieselResult<models::Tag> {
        let events = EventTag::create_query_builder(constraints.clone(), connection)
            .with_tag_id(self.id)
            .execute()?
            .into_iter()
            .map(|event_tag|
                Event::create_query_builder(constraints.clone(), connection)
                    .with_id(event_tag.event_id)
                    .execute()?
                    .pop()
                    .ok_or(diesel::NotFound))
            .collect::<DieselResult<Vec<models::Event>>>()?;

        Ok(models::Tag {
            id: self.id,
            name: self.name,
            description: self.description,
            events,
        })
    }

    pub(crate) fn count(connection: &diesel::PgConnection) -> DieselResult<i64> {
        tags::table
            .select(diesel::dsl::count(id))
            .first(connection)
    }

    pub(crate) fn create_query_builder(constraints: Constraints, connection: &diesel::PgConnection) -> TagQueryBuilder {
        TagQueryBuilder {
            query: tags::table
                .limit(constraints.limit.0)
                .offset(constraints.offset.0)
                .into_boxed(),
            connection,
        }
    }

    pub(crate) fn insert(self, constraints: Constraints, connection: &diesel::PgConnection) -> DieselResult<models::Tag> {
        diesel::insert_into(tags::table)
            .values(vec![(
                name.eq(self.name),
                description.eq(self.description),
            )])
            .load::<Tag>(connection)?
            .pop()
            .ok_or(diesel::result::Error::NotFound)?
            .into_model(constraints, connection)
    }
}
