use system_schema::keyspaces;

fn main() {
    //Keyspaces::check_compat::<Scylla>();
}


struct Keyspaces{
    table_name : String,
    keyspace_name :  String,
}

type CqlTypeOf<Expr> = <Expr as Expression>::CqlType;

const _  : ()= {
    fn check_compat<PG: Backend>()
    where
        String : FromCql<CqlTypeOf<system_schema::keyspaces::dsl::table_name>, PG>,
        String : FromCql<CqlTypeOf<system_schema::keyspaces::dsl::keyspace_name>, PG>
    {}

};

impl <PG: Backend> FromCql<types::Text, PG> for String{
    type Error = ();
    fn from_cql(_db_result : <PG as Backend>::Value<'_>) -> Result<Self, Self::Error> {
        Err(())
    }
}
/////////////////////////
trait Backend{
    type Value<'a>;
}

trait FromStaticSqlRow<STuple, DB: Backend>: Sized{
    fn build_from_row<'a>(row: &impl Row<'a, DB>) -> Result<Self, ()>;
}

trait Field<'a, DB: Backend>{
    fn field_name(&self) -> Option<&str>;
    fn field_value(&self) -> Option<DB::Value<'_>>;
    fn is_null(&self) -> bool{
        self.field_value().is_none()
    }
}
trait RowIndex<I>{
    fn idx(&self, index : I) -> usize;
}

// represent row from scylla or astra
/*
 field types will change with  
*/
trait Row<'a, DB: Backend> {//: RowIndex<usize> + for<'b> RowIndex<&'b str> {
    type Field<'f> : Field<'f, DB>
        where
            'a : 'f,
            Self : 'f;

    fn field_count(&self) -> usize;

}
trait Queryable<Tuple, DB: Backend> : Sized{
    type Row : FromStaticSqlRow<Tuple, DB>;
    fn build(row: Self::Row) -> Result<Self, ()>;
}

struct Scylla;
impl Backend for Scylla{
    type Value<'a> = &'a str;
}

trait CqlType{}

trait ToCqlRow<CqlType>{
    fn to_sql_row(&self) -> CqlType;
}

/*
    scylla driver connection object to implement Backend trait
*/
trait FromCql<A, DB: Backend> : Sized{
    type Error;
    fn from_cql(db_result : DB::Value<'_>) -> Result<Self, Self::Error>;
}



trait Expression{
    type CqlType;
}
trait AppearsOnTable<QS:Table> {}
trait Column {
    const COL_NAME : &'static str;
}


trait Table {
    type PartitionKeys;
    type ClusteringKeys;

    fn partition_keys(&self) -> &Self::PartitionKeys;
    fn clustering_keys(&self) -> &Self::ClusteringKeys;
}


mod types{
    //have implementation of driver serialization and de serialization
    pub struct Text;
    pub struct Int;
    pub struct Boolean;
    pub struct Timestamp;
}

mod system_schema{
    pub mod keyspaces{
        #[allow(dead_code, non_camel_case_types)]
        pub struct table;

        use self::columns::*;
        use crate::{Queryable, Table};

        impl Table for table{
            type PartitionKeys = keyspace_name;
            type ClusteringKeys = table_name;

            fn partition_keys(&self) -> &Self::PartitionKeys{
                &keyspace_name{}
            }

            fn clustering_keys(&self) -> &Self::ClusteringKeys{
                &table_name{}
            }
        }

        #[allow(unused_imports)]
        pub mod dsl{
            pub use super::columns::keyspace_name;
            pub use super::columns::table_name;
            pub use super::table as keyspaces;
        }

        #[allow(non_camel_case_types)]
        mod columns{
            use crate::*;
            use crate::types as CqlType;

            pub struct table_name;
            pub struct keyspace_name;

            impl Expression for table_name{
                type CqlType =  CqlType::Text;
            }

            impl Column for table_name{
                const COL_NAME : &'static str = "table_name";
            }

            impl Expression for keyspace_name{
                type CqlType = CqlType::Text;
            }

            impl Column for keyspace_name{
                const COL_NAME : &'static str = "keyspace_name";
            }


        }
    }
}