trait SelectDsl<Selection>{
    type Output;
    fn select(self, selection: Selection) -> Self::Output;
}