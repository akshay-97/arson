
/*
    FromCql implemented for Tuple of Rust Types to CqlTypes
    ? How do compile check if CqlType is supported by Backend
        - check HasSqlType trait
*/
use scylla_cql::types::deserialize::row::DeserializeRow;
use crate::backend::{Backend, scylla};
use std::fmt;
pub trait FromCqlRow<CT, DB: Backend>
{
    fn build_from_row<'a>(row : DB::ResultValue<'a>) -> Result<Self, DeserializeError>
        where
            Self: Sized,
            Self : DeserializeRow<'a,'a>; // P1: put this behind a feature flag
}

#[derive(Debug,Clone)]
pub struct DeserializeError(String);

impl DeserializeError{
    fn new(msg : &str) -> Self{
        DeserializeError(msg.to_string())
    }
}

impl std::error::Error for DeserializeError{
    fn description(&self) -> &str {
        self.0.as_str()
    }
}
impl fmt::Display for DeserializeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl <CT, T> FromCqlRow<CT, scylla> for T
{
    fn build_from_row<'a>(row : <scylla as Backend>::ResultValue<'a>) -> Result<Self, DeserializeError>
    where
        Self: Sized,
        Self : DeserializeRow<'a, 'a>
    {
        T::deserialize(row)
            .map_err(|e| {
                e.downcast_ref::<DeserializeError>()
                    .map_or_else(|| DeserializeError::new("Deserialization error"),
                        |err| err.clone())
        })
    }
}

// pub trait FromCql<CT, DB>{
//     fn from_cql(db_value: DB::ColumnValue) -> Result<Self>;
// }