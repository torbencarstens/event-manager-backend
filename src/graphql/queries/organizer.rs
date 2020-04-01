use crate::database;
use crate::database::Constraints;
use crate::database::organizer::OrganizerQueryBuilder;
use crate::graphql::GraphQLError;

#[derive(Debug, GraphQLInputObject)]
pub(crate) struct OrganizerQuery {
    pub(crate) id: Option<i32>,
    pub(crate) name: Option<String>,
    pub(crate) website: Option<String>,
}

impl<'a> OrganizerQuery {
    pub(crate) fn into_builder(self, connection: &'a diesel::PgConnection) -> OrganizerQueryBuilder<'a> {
        let mut builder = database::Organizer::create_query_builder(Constraints::default(), connection);

        if let Some(id) = self.id {
            builder = builder.with_id(id);
        }
        if let Some(name) = self.name {
            builder = builder.with_name(name);
        }
        if let Some(website) = self.website {
            builder = builder.with_website(website);
        }

        builder
    }
}
