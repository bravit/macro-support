extern crate proc_macro;

use proc_macro::{Ident, Punct, Literal, Spacing, Span, TokenStream, TokenTree};
use itertools::Itertools;

#[proc_macro]
pub fn make_answer(items: TokenStream) -> TokenStream {
    let answer = if items.is_empty() {
        String::from("42")
    } else {
        items.to_string()
    };
    format!("fn answer() -> u32 {{ {} }}", answer).parse().unwrap()
}

#[proc_macro]
pub fn declare_variables(items: TokenStream) -> TokenStream {

    fn is_comma(item: &TokenTree) -> bool {
        match item {
            TokenTree::Punct(p) => p.as_char() == ',',
            _ => false
        }
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