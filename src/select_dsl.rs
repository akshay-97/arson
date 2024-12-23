pub trait SelectDsl<Selection>{
    type Output;
    fn select(self, selection: Selection) -> Self::Output;
}

pub type Select<Source, Predicate> = <Source as SelectDsl<Predicate>>::Output;