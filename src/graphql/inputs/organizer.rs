use crate::database::Organizer;
use crate::graphql::GraphQLError;

#[derive(Debug, GraphQLInputObject)]
pub struct OrganizerInput {
    pub(crate) name: String,
    pub(crate) website: Option<String>,
}

impl OrganizerInput {
    pub(crate) fn validate(&self) -> Result<(), GraphQLError> {
        if let Some(website) = &self.website {
            if url::Url::parse(website).is_err() {
                return Err(GraphQLError::ValidationError("`website` field contains a non parsable string.".to_string()));
            }
        }
        if self.name.len() < 2 {
            return Err(GraphQLError::ValidationError(format!("`{}` field must at least contain 2 characters", "name")));
        }

        Ok(())
    }
}

impl Into<Organizer> for OrganizerInput {
    fn into(self) -> Organizer {
        Organizer {
            id: -1,
            name: self.name,
            website: self.website,
        }
    }
}
