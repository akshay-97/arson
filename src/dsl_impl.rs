use crate::{filter_dsl::FilterDsl
        , select_dsl::SelectDsl
        , expr::{Expression, NoWhereClause, WhereAnd, CqlBool, SelectClause}
        , dsl::QueryDsl
        , backend::SelectableExpression};

struct SelectStatement
    <F,
    S = DefaultSelectClause,
    W = NoWhereClause,
    O = NoOrderClause,
    L = NoLimitClause
    >
{
    from  : F,
    select_clause : S,
    where_clause : W,
    order_by_clause : O,
    limit_clause : L,
}

struct NoOrderClause;
struct NoLimitClause;
struct DefaultSelectClause;

// impl <Predicate, O,L, F> FilterDsl<Predicate> for SelectStatement<F, NoWhereClause, O,L>
// where
//     Predicate : Expression,
//     Predicate::CqlType : CqlBool,
//     //W : WhereAnd<Predicate>,
// {
//     type Output = SelectStatement<F, WhereClause<Predicate> , O, L>;

//     fn filter(self, predicate : Predicate) -> Self::Output {
//         SelectStatement{
//             from : self.from,
//             where_clause: self.where_clause.and(predicate),
//             order_by_clause : self.order_by_clause,
//             limit_clause : self.limit_clause,
//         }
//     }
// }

impl <Predicate, F, S, W, O, L> FilterDsl<Predicate> for SelectStatement<F, S, W, O, L>
where
    Predicate : Expression,
    Predicate::CqlType : CqlBool,
    W : WhereAnd<Predicate>,
{
    type Output = SelectStatement<F, S, W::Output, O, L>;

    fn filter(self, predicate : Predicate) -> Self::Output {
        SelectStatement{
            from : self.from,
            select_clause: self.select_clause,
            where_clause : self.where_clause.and(predicate),
            order_by_clause : self.order_by_clause,
            limit_clause : self.limit_clause,
        }
    }
}

impl <F, W, O, L> QueryDsl for SelectStatement<F,W,O,L> {}


/*
    impl for SelectDsl
*/
impl <Selection, F,S, W,O,L> SelectDsl<Selection> for SelectStatement<F,S,W,O,L>
where
    Selection : SelectableExpression<F>,
    //SelectStatement<F, SelectClause<Selection>, W, O, L> : SelectClauseExpress
    //SelectStatement<F, SelectClause<Selection>, W, O, L> : SelectQuery,
    /*
    SQL type of SELECT clause
    trait SqlQuery {
        type SqlType
    }
    */
{
    type Output = SelectStatement<F, SelectClause<Selection>, W, O,L>;

    fn select(self, selection : Selection) -> Self::Output{
        SelectStatement{
            from : self.from,
            select_clause : SelectClause(selection),
            where_clause : self.where_clause,
            order_by_clause : self.order_by_clause,
            limit_clause: self.limit_clause,
        }
    }
}
