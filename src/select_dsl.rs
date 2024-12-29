use crate::{expr::Expression, backend::QuerySource};
pub trait SelectDsl<Selection>{
    type Output;
    fn select(self, selection: Selection) -> Self::Output;
}

pub type Select<Source, Predicate> = <Source as SelectDsl<Predicate>>::Output;


// *** SelectExpression

struct DefaultSelectClause<QS: QuerySource>{
    selection : QS::DefaultSelection
}

pub trait SelectClauseExpression<QS>{
    type Selection;
    type SelectClauseCqlType;
}

impl <QS> SelectClauseExpression<QS> for DefaultSelectClause<QS>
where
    QS : QuerySource
{
    type Selection = QS::DefaultSelection;
    type SelectClauseCqlType = <Self::Selection as Expression>::CqlType;
}