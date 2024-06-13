
use syn::parse::{Parse, ParseStream};
use syn::Token;
use syn::Ident;

pub struct IsHTMLTag {
    pub is_html: bool
}

impl Parse for IsHTMLTag {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // // is de volgende token een < ?
        match input.parse::<Token![<]>() {
            Ok(_) => {}
            Err(_) => { return Ok(IsHTMLTag { is_html: false }); }
        };

        // is de volgende token een p of iet anders in de zin van <p> <model>
        match input.parse::<Ident>() {
            Ok(_) => {}
            Err(_) => { return Ok(IsHTMLTag { is_html: false }); }
        };

        // is de volgende token een / ?
        match input.parse::<Token![/]>() {
            Ok(_) => {}
            Err(_) => { return Ok(IsHTMLTag { is_html: false }); }
        };

        // is de volgende token een > ?
        match input.parse::<Token![>]>() {
            Ok(_) => {}
            Err(_) => { return Ok(IsHTMLTag { is_html: false }); }
        };

        return Ok(IsHTMLTag { is_html: true });
    }
}