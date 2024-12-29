use crate::{backend::{Backend, BindCollector}};

pub trait ToCql<T, DB>
where
    DB : Backend,
    DB::BindCollector : BindCollector<DB>,
{ 
    fn to_cql<'a>(&'a self , out : &'a mut DB::BindCollector) -> Result<(), Box<dyn std::error::Error>>;
}


