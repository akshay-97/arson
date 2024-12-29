/*
Struct Representation of all CQl Types Supported

FromSql is implemented for Rust types that can be represented as CQL Type

adding a udt should essentially implement FromCql and necessary traits of Backend
*/

pub struct Boolean;
pub struct Text; // same as Varchar
pub struct Timestamp;
pub struct Integer;
pub struct Double;

/*
    impl <CT, DB, T> FromCql<CT, DB> for T
    where
        T : DB::Deserializer
*/
