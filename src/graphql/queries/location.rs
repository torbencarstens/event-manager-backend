use crate::database;
use crate::database::Constraints;
use crate::database::location::LocationQueryBuilder;
use crate::graphql::GraphQLError;

#[derive(Debug, GraphQLInputObject)]
pub(crate) struct LocationQuery {
    pub(crate) id: Option<i32>,
    pub(crate) name: Option<String>,
    pub(crate) website: Option<String>,
    pub(crate) street: Option<String>,
    pub(crate) street_number: Option<i32>,
    pub(crate) city: Option<String>,
    pub(crate) postal_code: Option<i32>,
    pub(crate) country: Option<String>,
    pub(crate) building: Option<String>,
    pub(crate) maps_link: Option<String>,
}

impl<'a> LocationQuery {
    pub(crate) fn into_builder(self, connection: &'a diesel::PgConnection) -> LocationQueryBuilder<'a> {
        let mut builder = database::Location::create_query_builder(Constraints::default(), connection);

        if let Some(id) = self.id {
            builder = builder.with_id(id);
        }
        if let Some(name) = self.name {
            builder = builder.with_name(name);
        }
        if let Some(website) = self.website {
            builder = builder.with_website(website);
        }
        if let Some(street) = self.street {
            builder = builder.with_street(street);
        }
        if let Some(street_number) = self.street_number {
            builder = builder.with_street_number(street_number);
        }
        if let Some(city) = self.city {
            builder = builder.with_city(city);
        }
        if let Some(postal_code) = self.postal_code {
            builder = builder.with_postal_code(postal_code);
        }
        if let Some(country) = self.country {
            builder = builder.with_country(country);
        }
        if let Some(building) = self.building {
            builder = builder.with_building(building);
        }
        if let Some(maps_link) = self.maps_link {
            builder = builder.with_maps_link(maps_link);
        }

        builder
    }
}
