use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, parse_quote, Data, DeriveInput, Fields, GenericParam, Generics, Index,
};

// STORABLE

#[proc_macro_derive(Storable)]
pub fn derive_storable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    // Add a bound `T: CellStorable` to every field type.
    let generics = add_storable_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
      impl #impl_generics Storable for #name #ty_generics #where_clause {
        fn create_statement(name: &str) -> String {
          format!("CREATE TABLE IF NOT EXISTS {} (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL
          )", name)
        }
      }
    };

    proc_macro::TokenStream::from(expanded)
}

fn add_storable_trait_bounds(mut generics: Generics) -> Generics {
    let where_clause = generics.make_where_clause();

    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(CellStorable));
        }
    }

    generics
}

// DATABASE

#[proc_macro_derive(Database)]
pub fn derive_database(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    // Add a bound `T: CellStorable` to every field type.
    let generics = add_database_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
      impl #impl_generics Database for #name #ty_generics #where_clause {
        fn connect() -> Self {
          use std::rc::Rc;

          let conn = Rc::new(Connection::open_in_memory().unwrap());

          Self {
            users: Table::create(conn.clone(), "users")
          }
        }
      }
    };

    proc_macro::TokenStream::from(expanded)
}

fn add_database_trait_bounds(mut generics: Generics) -> Generics {
    let where_clause = generics.make_where_clause();

    for param in &mut generics.params {
        // if let GenericParam::Type(ref mut type_param) = *param {
        //     type_param.bounds.push(parse_quote!(CellStorable));
        // }
    }

    generics
}
