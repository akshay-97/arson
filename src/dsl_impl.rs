use crate::{filter_dsl::FilterDsl
        , select_dsl::SelectDsl
        , expr::{Expression, NoWhereClause, WhereAnd, WhereClause, CqlBool}};

struct SelectStatement
    <F,
    W = NoWhereClause,
    O = NoOrderClause,
    L = NoLimitClause
    >
{
    from  : F,
    where_clause : W,
    order_by_clause : O,
    limit_clause : L,
}

struct NoOrderClause;
struct NoLimitClause;

impl <Predicate, O,L, F> FilterDsl<Predicate> for SelectStatement<F, NoWhereClause, O,L>
where
    Predicate : Expression,
    Predicate::CqlType : CqlBool,
    //W : WhereAnd<Predicate>,
{
    type Output = SelectStatement<F, WhereClause<Predicate> , O, L>;

    fn filter(self, predicate : Predicate) -> Self::Output {
        SelectStatement{
            from : self.from,
            where_clause: self.where_clause.and(predicate),
            order_by_clause : self.order_by_clause,
            limit_clause : self.limit_clause,
        }
    }
}

