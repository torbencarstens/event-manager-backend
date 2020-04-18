use crate::database;
use crate::database::Constraints;
use crate::database::tag::TagQueryBuilder;
use crate::graphql::GraphQLError;

#[derive(Debug, GraphQLInputObject)]
pub(crate) struct TagQuery {
    pub(crate) id: Option<i32>,
    pub(crate) name: Option<String>,
    pub(crate) description: Option<String>,
}

impl<'a> TagQuery {
    pub(crate) fn into_builder(self, constraints: Constraints, connection: &'a diesel::PgConnection) -> TagQueryBuilder<'a> {
        let mut builder = database::Tag::create_query_builder(constraints, connection);

        if let Some(id) = self.id {
            builder = builder.with_id(id);
        }
        if let Some(name) = self.name {
            builder = builder.with_name(name);
        }
        if let Some(description) = self.description {
            builder = builder.with_description(description);
        }

        builder
    }
}
