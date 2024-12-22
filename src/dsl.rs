
trait QueryDsl{
    fn filter<Predicate>(self , predicate : Predicate) ->  Filter<Self,Predicate>
        where
            Self : FilterDsl<Predicate>
    {
        FilterDsl::filter(self,predicate)
    }

    fn select<Selection>(self , selection : Selection) -> Select<Self,Selection>
        where
            Self : SelectDsl<Selection>
    {
        SelectDsl::select(self, selection)
    }
    // Order Dsl
    //fn order_by()

    // Limit Dsl
    // fn limit()

    //Update ??
    //Delete ??

}