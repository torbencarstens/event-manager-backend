use diesel::dsl::{Eq, SqlTypeOf};
use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::query_builder::BoxedSelectStatement;
use diesel::query_dsl::boxed_dsl::BoxedDsl;
use diesel::sql_types::Text;

use crate::database::{Constraints, DieselResult, QueryBuilder};
use crate::database::event::Event;
use crate::database::schema::locations::{self, *};

#[derive(Debug, GraphQLObject, Queryable)]
pub struct Location {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) website: Option<String>,
    pub(crate) street: String,
    pub(crate) street_number: i32,
    pub(crate) city: String,
    pub(crate) postal_code: i32,
    pub(crate) building: Option<String>,
    pub(crate) maps_link: String,
}

pub(crate) struct LocationQueryBuilder<'a> {
    query: locations::BoxedQuery<'a, Pg>,
    connection: &'a diesel::PgConnection,
}

impl<'a> LocationQueryBuilder<'a> {
    pub fn with_id(mut self, value: i32) -> LocationQueryBuilder<'a> {
        self.query = self.query.filter(id.eq(value));
        self
    }

    pub fn with_name(mut self, location_name: String) -> LocationQueryBuilder<'a> {
        self.query = self.query.filter(name.eq(location_name));
        self
    }

    pub fn with_website(mut self, location_website: String) -> LocationQueryBuilder<'a> {
        self.query = self.query.filter(website.eq(location_website));
        self
    }

    pub fn with_street(mut self, street_name: String) -> LocationQueryBuilder<'a> {
        self.query = self.query.filter(street.eq(street_name));
        self
    }

    pub fn with_street_number(mut self, number: i32) -> LocationQueryBuilder<'a> {
        self.query = self.query.filter(street_number.eq(number));
        self
    }

    pub fn with_city(mut self, city_name: String) -> LocationQueryBuilder<'a> {
        self.query = self.query.filter(city.eq(city_name));
        self
    }

    pub fn with_postal_code(mut self, code: i32) -> LocationQueryBuilder<'a> {
        self.query = self.query.filter(postal_code.eq(code));
        self
    }

    pub fn with_building(mut self, building_name: String) -> LocationQueryBuilder<'a> {
        self.query = self.query.filter(building.eq(building_name));
        self
    }

    pub fn with_maps_link(mut self, link: String) -> LocationQueryBuilder<'a> {
        self.query = self.query.filter(maps_link.eq(link));
        self
    }
}

impl<'a> QueryBuilder for LocationQueryBuilder<'a> {
    type Item = Location;

    fn execute(self) -> DieselResult<Vec<Location>> {
        self.query
            .load::<Location>(self.connection)
    }
}

impl Location {
    pub(crate) fn get(constraints: Constraints, connection: &diesel::PgConnection) -> DieselResult<Vec<Location>> {
        locations::table
            .limit(constraints.limit)
            .offset(constraints.offset)
            .load::<Location>(connection)
    }

    pub(crate) fn create_query_builder(constraints: Constraints, connection: &diesel::PgConnection) -> LocationQueryBuilder {
        LocationQueryBuilder {
            query: locations::table
                .limit(constraints.limit)
                .offset(constraints.offset)
                .into_boxed(),
            connection,
        }
    }

    pub(crate) fn insert(self, connection: &diesel::PgConnection) -> DieselResult<Location> {
        diesel::insert_into(locations::table)
            .values(vec![(
                name.eq(self.name),
                website.eq(self.website),
                street.eq(self.street),
                street_number.eq(self.street_number),
                city.eq(self.city),
                postal_code.eq(self.postal_code),
                building.eq(self.building),
                maps_link.eq(self.maps_link)
            )])
            .load::<Location>(connection)?
            .pop()
            .ok_or(diesel::NotFound)
    }
}
