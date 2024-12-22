struct Scylla;

struct ScyllaQueryBuilder;


impl Backend for Scylla{
    type QueryBuilder = ScyllaQueryBuilder;
    type RawValue<'a> = ScyllaValue<'a>;
}


/// A type that can be deserialized from a row that was returned from a query.
///
/// For tips on how to write a custom implementation of this trait, see the
/// documentation of the parent module.
///
/// The crate also provides a derive macro which allows to automatically
/// implement the trait for a custom type. For more details on what the macro
/// is capable of, see its documentation.
pub trait DeserializeRow<'frame, 'metadata>
where
    Self: Sized,
{
    /// Checks that the schema of the result matches what this type expects.
    ///
    /// This function can check whether column types and names match the
    /// expectations.
    fn type_check(specs: &[ColumnSpec]) -> Result<(), TypeCheckError>;

    /// Deserializes a row from given column iterator.
    ///
    /// This function can assume that the driver called `type_check` to verify
    /// the row's type. Note that `deserialize` is not an unsafe function,
    /// so it should not use the assumption about `type_check` being called
    /// as an excuse to run `unsafe` code.
    fn deserialize(row: ColumnIterator<'frame, 'metadata>) -> Result<Self, DeserializationError>;
}

T  : DeserializeRow<'frame, 'metadata>


trait Executor<PG : Backend>{
    type Statement;
    type Result;
    fn execute() -> BytesResult
}

pub trait LoadConnection<B = DefaultLoadingMode>: Connection {
    /// The cursor type returned by [`LoadConnection::load`]
    ///
    /// Users should handle this as opaque type that implements [`Iterator`]
    type Cursor<'conn, 'query>: Iterator<
        Item = QueryResult<<Self as LoadConnection<B>>::Row<'conn, 'query>>,
    >
    where
        Self: 'conn;

    /// The row type used as [`Iterator::Item`] for the iterator implementation
    /// of [`LoadConnection::Cursor`]
    type Row<'conn, 'query>: crate::row::Row<'conn, Self::Backend>
    where
        Self: 'conn;

    /// Executes a given query and returns any requested values
    ///
    /// This function executes a given query and returns the
    /// query result as given by the database. **Normal users
    /// should not use this function**. Use
    /// [`QueryDsl::load`](crate::QueryDsl) instead.
    ///
    /// This function is useful for people trying to build an alternative
    /// dsl on top of diesel. It returns an [`impl Iterator<Item = QueryResult<&impl Row<Self::Backend>>`](Iterator).
    /// This type can be used to iterate over all rows returned by the database.
    #[diesel_derives::__diesel_public_if(
        feature = "i-implement-a-third-party-backend-and-opt-into-breaking-changes"
    )]
    fn load<'conn, 'query, T>(
        &'conn mut self,
        source: T,
    ) -> QueryResult<Self::Cursor<'conn, 'query>>
    where
        T: Query + QueryFragment<Self::Backend> + QueryId + 'query,
        Self::Backend: QueryMetadata<T::SqlType>;
}


pub trait RunQueryDsl<Conn> : Sized {
    fn execute<Conn : Connection, Self: ExecuteDsl<Conn>> (self, conn : &mut Conn) -> QueryResult<usize>;

    fn load<'query, U>(self, conn: &mut Conn) -> QueryResult<Vec<U>>
    where
        Self: LoadQuery<'query, Conn, U>
    {

    }
}


trait LoadQuery<Conn, PG : Backend, U>{
    type RowIter<'conn> : Iterator<Item = QueryResult<U>>
        where Conn : 'conn;
    
    fn internal_load(self, conn : &mut Conn) -> QueryResult<Self::RowIter<'_>>;
}
// diesel

impl FromSql<scylla_typ, Scylla> for <RustType>{
    fn from_sql(bytes: Scylla::RawValue<'_>) -> Result<Self,DeserializationError>{

    }
}




// please work
trait SelectDsl{}
trait FilterDsl<Predicate>{
    type Output;
    fn filter(self, predicate: Predicate) -> Self::Output;
}
//    pub type Filter<Source, Predicate> = <Source as FilterDsl<Predicate>>::Output;

impl <T, Predicate> FilterDsl<Predicate> for T
where
    T: Table,
    T::Query : FilterDsl<Predicate>
{
    type Output = <Self as FilterDsl<Predicate>>::Output;

    fn filter(self, predicate : Predicate) -> Self::Output{
        self.as_query().filter(predicate)
    }
}

trait QueryDsl : Sized{
    fn filter<Predicate>(self, predicate : Predicate) -> Filter<Self, Predicate>
        where
            Self: FilterDsl<Predicate>
    {

    }
}

struct Post{
    user_id : String,
    date_created : DateTime
}

{
    post.filter()
}