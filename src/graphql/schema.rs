use juniper::{FieldError, FieldResult, IntoFieldError};

use crate::database::{self, Constraints, Location, Organizer, QueryBuilder};
use crate::graphql::{Context, GraphQLError, MutationRoot, QueryRoot};
use crate::graphql::inputs::{EventInput, LocationInput, OrganizerInput};
use crate::graphql::queries::{EventQuery, LocationQuery, OrganizerQuery};
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
    fn event(context: &Context, query: Option<EventQuery>) -> FieldResult<Vec<models::Event>> {
        match query {
            Some(query) => {
                query
                    .into_builder(&context.connection.0)
                    .execute()
            }
            None => database::Event::create_query_builder(Constraints::default(), &context.connection.0).execute()
        }
            .map_err(Into::into)
    }

    fn location(context: &Context, query: Option<LocationQuery>) -> FieldResult<Vec<Location>> {
        match query {
            Some(query) => {
                query
                    .into_builder(&context.connection.0)
                    .execute()
            }
            None => Location::get(Constraints::default(), &context.connection.0)
        }
            .map_err(Into::into)
    }

    fn organizer(context: &Context, query: Option<OrganizerQuery>) -> FieldResult<Vec<Organizer>> {
        match query {
            Some(query) => {
                query
                    .into_builder(&context.connection.0)
                    .execute()
            }
            None => Organizer::get(Constraints::default(), &context.connection.0)
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
}
