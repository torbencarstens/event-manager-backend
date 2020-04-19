use juniper::{FieldError, FieldResult, IntoFieldError};

use crate::database::{self, Constraints, DieselResult, Location, Organizer, QueryBuilder};
use crate::graphql::{Context, GraphQLError, MutationRoot, Pagination, QueryRoot};
use crate::graphql::graphqli64::GraphQLi64;
use crate::graphql::inputs::{EventInput, LocationInput, OrganizerInput, TagInput};
use crate::graphql::queries::{EventQuery, LocationQuery, OrganizerQuery, TagQuery};
use crate::models;

impl IntoFieldError for GraphQLError {
    fn into_field_error(self) -> FieldError {
        match self {
            GraphQLError::ValidationError(value) => FieldError::new(value, graphql_value!({ "type": "INVALID_VALUE" }))
        }
    }
}


#[juniper::object(Context = Context)]
impl QueryRoot {
    fn event(context: &Context, constraints: Option<Constraints>, query: Option<EventQuery>) -> FieldResult<Vec<models::Event>> {
        let constraints = constraints.unwrap_or_default();

        match query {
            Some(query) => {
                query
                    .into_builder(constraints, &context.connection.0)
                    .execute()
            }
            None => database::Event::create_query_builder(constraints, &context.connection.0).execute()
        }
            .map_err(Into::into)
    }

    fn location(context: &Context, constraints: Option<Constraints>, query: Option<LocationQuery>) -> FieldResult<Vec<Location>> {
        let constraints = constraints.unwrap_or_default();

        match query {
            Some(query) => {
                query
                    .into_builder(constraints, &context.connection.0)
                    .execute()
            }
            None => Location::create_query_builder(constraints, &context.connection.0)
                .execute()
        }
            .map_err(Into::into)
    }

    fn organizer(context: &Context, constraints: Option<Constraints>, query: Option<OrganizerQuery>) -> FieldResult<Vec<Organizer>> {
        let constraints = constraints.unwrap_or_default();

        match query {
            Some(query) => {
                query
                    .into_builder(constraints, &context.connection.0)
                    .execute()
            }
            None => Organizer::create_query_builder(constraints, &context.connection.0)
                .execute()
        }
            .map_err(Into::into)
    }

    fn pagination(context: &Context) -> FieldResult<Pagination> {
        let event_count = GraphQLi64(database::Event::count(&context.connection.0)?);
        let location_count = GraphQLi64(database::Location::count(&context.connection.0)?);
        let organizer_count = GraphQLi64(database::Organizer::count(&context.connection.0)?);

        Ok(
            Pagination {
                event_count,
                location_count,
                organizer_count,
            }
        )
    }

    fn tag(context: &Context, constraints: Option<Constraints>, query: Option<TagQuery>) -> FieldResult<Vec<models::Tag>> {
        let constraints = constraints.unwrap_or_default();

        match query {
            Some(query) => {
                query
                    .into_builder(constraints.clone(), &context.connection.0)
                    .execute()?
                    .into_iter()
                    .map(|tag|
                        database::Tag::into_model(tag, constraints.clone(), &context.connection.0))
                    .collect::<DieselResult<Vec<models::Tag>>>()
            }
            None => database::Tag::create_query_builder(constraints.clone(), &context.connection.0)
                .execute()?
                .into_iter()
                .map(|tag|
                    database::Tag::into_model(tag, constraints.clone(), &context.connection.0))
                .collect()
        }
            .map_err(Into::into)
    }
}

#[juniper::object(Context = Context)]
impl MutationRoot {
    fn event(context: &Context, input: EventInput) -> FieldResult<models::Event> {
        input
            .validate()
            .map_err(|x|
                x.into_field_error())?;

        let event: database::Event = input.into();

        database::Event::from_database_event(
            event
                .insert(&context.connection.0)?,
            &context.connection.0,
        )
            .map_err(Into::into)
    }

    fn location(context: &Context, input: LocationInput) -> FieldResult<Location> {
        input
            .validate()
            .map_err(|x|
                x.into_field_error())?;

        let location: Location = input.into();

        location
            .insert(&context.connection.0)
            .map_err(Into::into)
    }

    fn organizer(context: &Context, input: OrganizerInput) -> FieldResult<Organizer> {
        input
            .validate()
            .map_err(|x|
                x.into_field_error())?;

        let organizer: Organizer = input.into();

        organizer
            .insert(&context.connection.0)
            .map_err(Into::into)
    }

    fn tag(context: &Context, input: TagInput) -> FieldResult<models::Tag> {
        input
            .validate()
            .map_err(|x|
                x.into_field_error())?;

        let tag: database::Tag = input.into();

        tag
            .insert(Constraints::default(), &context.connection.0)
            .map_err(Into::into)
    }
}
