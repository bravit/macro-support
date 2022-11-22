extern crate proc_macro;

use proc_macro::{Ident, Punct, Literal, Spacing, Span, TokenStream, TokenTree};
use itertools::Itertools;


#[proc_macro_derive(NameFn)]
pub fn derive_name_fn(items: TokenStream) -> TokenStream {

    fn ident_name(item: TokenTree) -> String {
        match item {
            TokenTree::Ident(i) => i.to_string(),
            _ => panic!("Not an ident")
        }
    }

    let item_name = ident_name(items.into_iter().nth(1).unwrap());

    format!("impl Name for {} {{
    fn name() -> String {{
        \"{}\".to_string()
    }}
}}", item_name, item_name).parse().unwrap()
}

#[proc_macro]
pub fn declare_variables(items: TokenStream) -> TokenStream {

    fn is_comma(item: &TokenTree) -> bool {
        matches!(item, TokenTree::Punct(p) if p.as_char() == ',')
    }

    items
        .into_iter()
        .group_by(is_comma)
        .into_iter()
        .filter_map(|(comma, group)| if comma { None } else { Some(group) } )
        .fold(TokenStream::new(), |mut tokens, mut decl| {

            let variable = decl.next().unwrap();

            let value =
                if decl.next().is_some() { // contains some value
                    decl.next().unwrap_or_else(
                        || panic!("The expected syntax for a declaration is: 'variable = value' OR 'variable'")
                    )
                } else {
                    Literal::u8_unsuffixed(0).into() // default value
                };

            tokens.extend([
                Ident::new("let", Span::mixed_site()).into(),
                variable,
                Punct::new('=', Spacing::Alone).into(),
                value,
                Punct::new(';', Spacing::Alone).into()
            ]);

            tokens
        }
    )
}