pub trait FilterDsl<Predicate>{
    type Output;
    fn filter(self, predicate : Predicate) -> Self::Output;
}
pub type Filter<Source, Predicate> = <Source as FilterDsl<Predicate>>::Output;