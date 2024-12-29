use scylla_cql::types::{deserialize::row::ColumnIterator,
        serialize::row::SerializedValues};
use crate::result::QueryResult;
use crate::serialize::ToCql;

pub trait SelectableExpression<QS> : AppearsOnTable<QS> {}

pub trait AppearsOnTable<QS> : crate::expr::Expression {}

pub trait QuerySource : Sized{
    type DefaultSelection: SelectableExpression<Self>;
    type FromClause;

    fn from_clause(&self) -> Self::FromClause;
    fn default_selection(&self) -> Self::DefaultSelection;
}




//****** Backend Trait */

pub trait BindCollector<DB>
where
    DB : Backend
{
    type Container;

    fn push_serialized_value<T,U>(&mut self, bind : U) -> QueryResult<()>
        where
            //DB : HasSqlType<T>,
            U : ToCql<T, DB>;
}
pub trait Backend : Sized {
    type ResultValue<'a>;
    //
    //type QueryBuilder : QueryBuilder<Self>;

    type BindCollector : BindCollector<Self>;
}


// scylla driver as Backend
pub struct scylla;

pub struct ScyllaBindCollector{
    inner_buffer : SerializedValues
}

impl BindCollector<scylla> for ScyllaBindCollector{
    type Container = SerializedValues;

    fn push_serialized_value<T,U>(&mut self, bind : U) -> QueryResult<()>
            where
                //DB : HasSqlType<T>,
                U : ToCql<T, scylla>
    {
        bind.to_cql(self)
    }
}
impl Backend for scylla{
    type ResultValue<'a> = ColumnIterator<'a, 'a>;
    type BindCollector = ScyllaBindCollector;
}