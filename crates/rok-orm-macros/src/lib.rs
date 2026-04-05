//! Procedural macros for rok-orm.

use heck::ToSnakeCase;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, LitStr};

/// Derive the `Model` trait (from `rok_orm_core`) for a struct.
///
/// Generates:
/// - `table_name()` — struct name in `snake_case` with an `"s"` suffix (overridable)
/// - `primary_key()` — defaults to `"id"` (overridable per field or struct-level)
/// - `columns()` — named fields in declaration order (fields marked `#[rok_orm(skip)]` are excluded)
///
/// # Attributes
///
/// **Struct-level:**
/// ```rust,ignore
/// #[rok_orm(table = "my_table")]          // override table name
/// #[rok_orm(primary_key = "user_id")]     // override primary key column name
/// ```
///
/// **Field-level:**
/// ```rust,ignore
/// #[rok_orm(primary_key)]     // mark this field as primary key
/// #[rok_orm(skip)]            // exclude from columns()
/// #[rok_orm(column = "col")]  // override column name in SQL
/// ```
///
/// # Example
///
/// ```rust,ignore
/// use rok_orm::Model;
///
/// #[derive(Model)]
/// #[rok_orm(table = "articles")]
/// pub struct BlogPost {
///     pub id: i64,
///     pub title: String,
///     #[rok_orm(skip)]
///     pub draft_notes: String,
/// }
///
/// assert_eq!(BlogPost::table_name(), "articles");
/// assert_eq!(BlogPost::columns(), &["id", "title"]);
/// ```
#[proc_macro_derive(Model, attributes(rok_orm))]
pub fn derive_model(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_model(input).unwrap_or_else(|e| e.to_compile_error().into())
}

fn expand_model(input: DeriveInput) -> syn::Result<TokenStream> {
    let struct_name = &input.ident;

    // ── parse struct-level rok_orm attributes ─────────────────────────────
    let mut custom_table: Option<String> = None;
    let mut struct_pk: Option<String> = None; // #[rok_orm(primary_key = "col")]

    for attr in &input.attrs {
        if !attr.path().is_ident("rok_orm") {
            continue;
        }
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("table") {
                let value = meta.value()?;
                let s: LitStr = value.parse()?;
                custom_table = Some(s.value());
                Ok(())
            } else if meta.path.is_ident("primary_key") {
                let value = meta.value()?;
                let s: LitStr = value.parse()?;
                struct_pk = Some(s.value());
                Ok(())
            } else {
                Err(meta.error("unknown rok_orm struct attribute"))
            }
        })?;
    }

    // ── derive table name ─────────────────────────────────────────────────
    let table = custom_table.unwrap_or_else(|| {
        format!("{}s", struct_name.to_string().to_snake_case())
    });

    // ── collect fields ────────────────────────────────────────────────────
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

    // ── parse field-level attributes ──────────────────────────────────────
    let mut column_names: Vec<String> = Vec::new();
    let mut field_pk: Option<String> = None; // set if a field has #[rok_orm(primary_key)]

    for field in fields.iter() {
        let field_ident = match &field.ident {
            Some(id) => id.to_string(),
            None => continue,
        };

        let mut skip = false;
        let mut col_override: Option<String> = None;
        let mut is_pk = false;

        for attr in &field.attrs {
            if !attr.path().is_ident("rok_orm") {
                continue;
            }
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("skip") {
                    skip = true;
                    Ok(())
                } else if meta.path.is_ident("primary_key") {
                    is_pk = true;
                    Ok(())
                } else if meta.path.is_ident("column") {
                    let value = meta.value()?;
                    let s: LitStr = value.parse()?;
                    col_override = Some(s.value());
                    Ok(())
                } else {
                    Err(meta.error("unknown rok_orm field attribute"))
                }
            })?;
        }

        if is_pk {
            field_pk = Some(col_override.clone().unwrap_or(field_ident.clone()));
        }

        if !skip {
            column_names.push(col_override.unwrap_or(field_ident));
        }
    }

    // ── resolve primary key ───────────────────────────────────────────────
    // Priority: field #[rok_orm(primary_key)] > struct #[rok_orm(primary_key = "...")] > "id"
    let pk = field_pk.or(struct_pk).unwrap_or_else(|| "id".to_string());

    let columns_len = column_names.len();

    let expanded = quote! {
        impl ::rok_orm_core::Model for #struct_name {
            fn table_name() -> &'static str {
                #table
            }

            fn primary_key() -> &'static str {
                #pk
            }

            fn columns() -> &'static [&'static str] {
                static COLS: [&str; #columns_len] = [#(#column_names),*];
                &COLS
            }
        }
    };

    Ok(expanded.into())
}
