use crate::database::Tag;
use crate::graphql::GraphQLError;

#[derive(Debug, GraphQLInputObject)]
pub struct TagInput {
    pub(crate) name: String,
    pub(crate) description: Option<String>,
}

impl TagInput {
    pub(crate) fn validate(&self) -> Result<(), GraphQLError> {
        if self.name.len() < 2 {
            return Err(GraphQLError::ValidationError(format!("`{}` field must at least contain 2 characters", "name")));
        }

        Ok(())
    }
}

impl Into<Tag> for TagInput {
    fn into(self) -> Tag {
        Tag {
            id: -1,
            name: self.name,
            description: self.description,
        }
    }
}
