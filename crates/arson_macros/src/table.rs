use quote::ToTokens;
use syn::{Attribute, Ident};

struct Table {
    partition_key: ColumnInfo,
    clustering_keys: Vec<ColumnInfo>,
    columns: Vec<ColumnInfo>,
}

#[derive(Clone)]
struct ColumnInfo {
    attr: Option<Attribute>,
    column_name: Ident,
    _sep: syn::Token![->],
    typ: Ident,
}

impl syn::parse::Parse for Table {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content = input;

        let mut columns: Vec<ColumnInfo> = Vec::new();
        let mut partition_key = Vec::new();
        let mut clustering_keys: Vec<ColumnInfo> = Vec::new();

        while !content.is_empty() {
            let column = content.parse::<ColumnInfo>()?;
            let attr = column
                .attr
                .as_ref()
                .map(|c| c.path.is_ident("partition_key"))
                .unwrap_or_default();
            if attr {
                partition_key.push(column.clone());
            }
            let attr = column
                .attr
                .as_ref()
                .map(|c| c.path.is_ident("clustering_key"))
                .unwrap_or_default();
            if attr {
                clustering_keys.push(column.clone())
            }
            if content.peek(syn::Token![,]) {
                content.parse::<syn::Token![,]>()?;
            }
            columns.push(column);
        }

        if partition_key.len() > 1 {
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "You cannot have two partition keys in a table",
            ));
        }

        Ok(Self {
            partition_key: partition_key
                .first()
                .cloned()
                .expect("partition_key should not be empty"),
            clustering_keys,
            columns,
        })
    }
}

impl syn::parse::Parse for ColumnInfo {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attr = input.call(Attribute::parse_outer)?;

        let name = input.parse::<Ident>()?;
        let _sep = input.parse::<syn::Token![->]>()?;
        let typ = input.parse::<Ident>()?;

        Ok(Self {
            attr: attr.first().cloned(),
            column_name: name,
            _sep,
            typ,
        })
    }
}

impl quote::ToTokens for Table {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.columns.iter().for_each(|c| {
            let name = &c.column_name;
            let typ = &c.typ;

            let token = quote::quote! {
                struct #name;
                impl Column for #name {
                    type CqlType = #typ;
                }
            };

            tokens.extend(token)
        });
    }
}

///
/// table! {
///   #[partition_key]
///   column1: Varchar
///   #[clustering_key]
///   column2: Varchar
/// }
pub(crate) fn table_inner(ts: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let t = syn::parse_macro_input!(ts as Table);
    let mut token = quote::quote! {};
    t.to_tokens(&mut token);
    token.into()
}
