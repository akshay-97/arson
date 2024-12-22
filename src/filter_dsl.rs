trait FilterDsl<Predicate>{
    type Output;
    fn filter(self, predicate : Predicate) -> Self::Output;
}