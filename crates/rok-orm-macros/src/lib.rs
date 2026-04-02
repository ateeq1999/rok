//! Procedural macros for rok-orm.

use heck::ToSnakeCase;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

/// Derive the [`rok_orm_core::Model`] trait for a struct.
///
/// Generates:
/// - `table_name()` — struct name in `snake_case` with an `"s"` suffix
/// - `columns()` — all named field identifiers in declaration order
/// - inherits `primary_key()` (defaults to `"id"`) and `query()` / `find()`
///
/// ```rust,ignore
/// use rok_orm::Model;
///
/// #[derive(Model)]
/// pub struct BlogPost {
///     pub id: i64,
///     pub title: String,
///     pub published: bool,
/// }
///
/// assert_eq!(BlogPost::table_name(), "blog_posts");
/// assert_eq!(BlogPost::columns(), &["id", "title", "published"]);
/// ```
#[proc_macro_derive(Model, attributes(rok_orm))]
pub fn derive_model(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_model(input).unwrap_or_else(|e| e.to_compile_error().into())
}

fn expand_model(input: DeriveInput) -> syn::Result<TokenStream> {
    let struct_name = &input.ident;

    // ── table name: snake_case(StructName) + "s" ─────────────────────────
    let raw_name = struct_name.to_string();
    let table = format!("{}s", raw_name.to_snake_case());

    // ── collect named field identifiers ───────────────────────────────────
    let fields = match &input.data {
        Data::Struct(s) => match &s.fields {
            Fields::Named(f) => &f.named,
            _ => {
                return Err(syn::Error::new(
                    Span::call_site(),
                    "#[derive(Model)] only supports structs with named fields",
                ))
            }
        },
        _ => {
            return Err(syn::Error::new(
                Span::call_site(),
                "#[derive(Model)] only supports structs",
            ))
        }
    };

    let column_names: Vec<String> = fields
        .iter()
        .filter_map(|f| f.ident.as_ref())
        .map(|id| id.to_string())
        .collect();

    let columns_len = column_names.len();

    let expanded = quote! {
        impl ::rok_orm_core::Model for #struct_name {
            fn table_name() -> &'static str {
                #table
            }

            fn columns() -> &'static [&'static str] {
                static COLS: [&str; #columns_len] = [#(#column_names),*];
                &COLS
            }
        }
    };

    Ok(expanded.into())
}
