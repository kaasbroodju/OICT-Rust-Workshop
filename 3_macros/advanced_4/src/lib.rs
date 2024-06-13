extern crate proc_macro;

use proc_macro::TokenStream;

use quote::{quote, ToTokens};
use syn::{ItemStruct, parse_macro_input, parse_quote};
use syn::parse::Parse;

mod parsing;

use parsing::IsHTMLTag;


// defineer onze unieke macro
#[proc_macro]
pub fn is_self_closing_html(input: TokenStream) -> TokenStream {
    // geef de AST door aan onze parser
    // Dit geeft onze eigen gedefinieerde AST terug.
    let html_tag = parse_macro_input!(input as IsHTMLTag);

    // haal het resultaat uit onze eigen AST
    let value = html_tag.is_html;

    // creer een nieuwe rust AST dmv de quote macro
    let result = quote!(#value);

    let expanded = result.to_token_stream();

    expanded.into()
}