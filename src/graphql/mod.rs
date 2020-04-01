use juniper::{FieldError, IntoFieldError};

use crate::database::{self, PrimaryDb};
use crate::graphql::graphqli64::GraphQLi64;

pub(crate) mod inputs;
pub(crate) mod queries;
pub mod graphqli64;
pub mod schema;

pub struct Context {
    pub connection: PrimaryDb
}

impl juniper::Context for Context {}

pub struct MutationRoot;

pub struct QueryRoot;

#[derive(GraphQLInputObject)]
pub struct Constraints {
    pub(crate) limit: GraphQLi64,
    pub(crate) offset: GraphQLi64,
}

impl Default for Constraints {
    fn default() -> Self {
        Constraints {
            limit: GraphQLi64(50),
            offset: GraphQLi64(0),
        }
    }
}

impl Into<database::Constraints> for Constraints {
    fn into(self) -> database::Constraints {
        database::Constraints {
            limit: self.limit.into(),
            offset: self.offset.into(),
        }
    }
}

#[derive(Debug)]
pub enum GraphQLError {
    ValidationError(String)
}
