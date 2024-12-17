mod table;
use proc_macro::TokenStream;

#[proc_macro]
pub fn table(ts: TokenStream) -> TokenStream {
    table::table_inner(ts)
}
