use crate::database::Location;
use crate::graphql::GraphQLError;

#[derive(Debug, GraphQLInputObject)]
pub struct LocationInput {
    pub(crate) name: String,
    pub(crate) website: Option<String>,
    pub(crate) street: String,
    pub(crate) street_number: i32,
    pub(crate) city: String,
    pub(crate) postal_code: i32,
    pub(crate) country: String,
    pub(crate) building: Option<String>,
    pub(crate) maps_link: String,
}

impl LocationInput {
    pub(crate) fn validate(&self) -> Result<(), GraphQLError> {
        if let Some(website) = &self.website {
            if url::Url::parse(website).is_err() {
                return Err(GraphQLError::ValidationError("`website` field contains a non parsable string.".to_string()));
            }
        }

        let mut error = None;
        if url::Url::parse(&self.maps_link).is_err() {
            return Err(GraphQLError::ValidationError("`maps_link` field contains a non parsable string.".to_string()));
        }
        if self.name.len() < 2 {
            error = Some("name");
        }
        if self.street.is_empty() {
            error = Some("street");
        }
        if self.city.is_empty() {
            error = Some("city");
        }
        if self.country.is_empty() {
            error = Some("country");
        }

        if let Some(field_name) = error {
            return Err(GraphQLError::ValidationError(format!("`{}` field must at least contain 2 characters", field_name)));
        }

        if self.street_number < 0 {
            return Err(GraphQLError::ValidationError(format!("`{}` can't be negative", "street_number")));
        }
        if self.postal_code <= 0 {
            return Err(GraphQLError::ValidationError(format!("`{}` can't be 0 or negative", "postal_code")));
        }

        Ok(())
    }
}

impl Into<Location> for LocationInput {
    fn into(self) -> Location {
        Location {
            id: -1,
            name: self.name,
            website: self.website,
            street: self.street,
            street_number: self.street_number,
            city: self.city,
            postal_code: self.postal_code,
            country: self.country,
            building: self.building,
            maps_link: self.maps_link,
        }
    }
}
