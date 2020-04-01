use diesel::backend::Backend;
use diesel::deserialize::FromSql;
use diesel::serialize::{Output, ToSql};
use diesel::sql_types::BigInt;
use juniper::{ParseScalarResult, ParseScalarValue, Value};
use serde::{Deserialize, Serialize, Serializer};
use serde::de::{self, Deserializer, Visitor};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, FromSqlRow, AsExpression)]
#[sql_type = "BigInt"]
pub struct GraphQLi64(pub i64);

impl Serialize for GraphQLi64 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_newtype_struct("i64", &self.0)
    }
}

struct GraphQLi64Visitor;

impl<'de> Visitor<'de> for GraphQLi64Visitor {
    type Value = GraphQLi64;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a i64 represented as a string.")
    }

    fn visit_i32<E>(self, value: i32) -> Result<GraphQLi64, E>
        where
            E: de::Error,
    {
        Ok(GraphQLi64(i64::from(value)))
    }

    fn visit_str<E>(self, value: &str) -> Result<GraphQLi64, E>
        where
            E: de::Error,
    {
        Ok(GraphQLi64(i64::from_str(&value).unwrap()))
    }
}

impl<'de> Deserialize<'de> for GraphQLi64 {
    fn deserialize<D>(deserializer: D) -> Result<GraphQLi64, D::Error>
        where
            D: Deserializer<'de>,
    {
        deserializer.deserialize_any(GraphQLi64Visitor)
    }
}

impl<DB: Backend> ToSql<BigInt, DB> for GraphQLi64
    where
        i64: ToSql<BigInt, DB>,
{
    fn to_sql<W: std::io::Write>(&self, out: &mut Output<W, DB>) -> ::diesel::serialize::Result {
        ToSql::<BigInt, DB>::to_sql(&self.0, out)
    }
}

impl<DB: Backend> FromSql<BigInt, DB> for GraphQLi64
    where
        i64: FromSql<BigInt, DB>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> diesel::deserialize::Result<Self> {
        FromSql::<BigInt, DB>::from_sql(bytes).map(GraphQLi64)
    }
}

juniper::graphql_scalar!(GraphQLi64 where Scalar = <S> {
    description: "String which is used as an i64 internally."

    resolve(&self) -> Value {
        Value::scalar(self.0.to_string())
    }

    from_input_value(v: &InputValue) -> Option<GraphQLi64> {
        v.as_scalar_value::<String>()
        .and_then(|str_val| match i64::from_str(&str_val) {
            Ok(i64) => Some(GraphQLi64(i64)),
            Err(_) => None,
        })
    }


    from_str<'a>(value: ScalarToken<'a>) -> ParseScalarResult<'a, S> {
        <String as ParseScalarValue<S>>::from_str(value)
    }
});

impl Into<GraphQLi64> for &i64 {
    fn into(self) -> GraphQLi64 {
        GraphQLi64(*self)
    }
}

impl Into<GraphQLi64> for i64 {
    fn into(self) -> GraphQLi64 {
        GraphQLi64(self)
    }
}

impl Into<i64> for GraphQLi64 {
    fn into(self) -> i64 {
        self.0
    }
}
