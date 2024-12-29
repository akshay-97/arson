/*
    Expression trait determines the CqlType for the expression, 
    implemented for all table column types
*/
pub trait Expression{
    type CqlType;
}

// *** CqlTypes
pub struct CBool;
pub struct Varchar;
pub struct Int;

impl Expression for bool{
    type CqlType = CBool;
}

trait AsExpression<T>{
    type Expression : Expression<CqlType = T>;

    fn as_expression(self) -> Self::Expression;
}

impl <T,ST> AsExpression<ST> for T
where
    T : Expression<CqlType = ST>,
{
    type Expression = T;
    fn as_expression(self) -> Self::Expression{
        self
    }
}

// sql types of expressions
pub trait CqlBool{}

impl CqlBool for CBool{}

pub trait WhereAnd<Predicate>
{
    type Output;
    fn and(self, predicate: Predicate) -> Self::Output;
}

pub trait WhereOr<Predicate>
{
    type Output;
    fn or(self, predicate: Predicate) -> Self::Output;
}

pub struct NoWhereClause;

impl <Predicate> WhereAnd<Predicate> for NoWhereClause
where
    Predicate : Expression,
    Predicate::CqlType : CqlBool,
{
    type Output = WhereClause<Predicate>;

    fn and(self, predicate : Predicate) -> Self::Output{
        WhereClause(predicate)
    }
}
pub struct WhereClause<Expr>(Expr);


impl <Predicate, Expr> WhereAnd<Predicate> for WhereClause<Expr>
where
    Predicate : Expression,
    Predicate::CqlType : CqlBool,
    Expr : Expression,
    Expr::CqlType : CqlBool
{
    type Output = WhereClause<Grouped<And<Expr, Predicate>>>;

    fn and(self, predicate : Predicate) -> Self::Output{
        WhereClause(
            Grouped(
                And(self.0, predicate)
            )
        )
    }
}

// ****  //
// WhereClause expression types 
// TODO
/*
    impl SelectableExpression
    impl AppearsOnTable

*/
pub struct Grouped<Expr>(Expr);

impl <Expr:Expression> Expression for Grouped<Expr>{
    type CqlType = Expr::CqlType;
}

pub struct And<Left,Right>(Left, Right);

// 
impl <T: Expression, U : Expression> Expression for And<T, U>
where
    T::CqlType : CqlBool,
    U::CqlType : CqlBool,
{
    type CqlType = CBool;
}

pub struct Eq<Left,Right>(Left, Right);

impl <Left, Right> Expression for Eq<Left, Right>
where
    Left  : Expression,
    Right : Expression,
{
    type CqlType = CBool;
}

type CqlTypeOf<V> = <V as Expression>::CqlType;
type AsExpr<L,R> = <L as AsExpression<CqlTypeOf<R>>>::Expression;
type EqGrouped<Left, Right> = Grouped<Eq<Left, AsExpr<Right, Left>>>;

//type EqRes<L,R> = Grouped<Eq<L,R>>;
// Expression methods traits//
trait ExpressionMethods : Expression + Sized{
    fn eq<T>(self, value : T) -> EqGrouped<Self, T>//EqRes<Self, <T as AsExpression<Self::CqlType>>::Expression>
    where
        T: AsExpression<Self::CqlType>,
    {
        Grouped(Eq(self, value.as_expression()))
    }
}

/// ***** //
pub struct SelectClause<Expr>(pub Expr);
