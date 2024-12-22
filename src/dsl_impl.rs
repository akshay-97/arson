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

struct NoWhereClause;
struct NoOrderClause;
struct NoLimitClause;

impl <Predicate, O,L, F> FilterDsl<Predicate> for SelectStatement<F, NoWhereClause, O,L>
where
    Predicate : Expression
{

}