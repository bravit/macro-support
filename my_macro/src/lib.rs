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

fn is_comma(item: &TokenTree) -> bool {
    match item {
        TokenTree::Punct(p) => p.as_char() == ',',
        _ => false
    }
}

#[proc_macro]
pub fn declare_variables(items: TokenStream) -> TokenStream {
    let decl_groups = items.clone().into_iter().group_by(is_comma);
    let declarations = decl_groups.into_iter()
        .filter_map(|(comma, group)| if comma { None } else { Some(group) } );

    let mut result = TokenStream::new();
    for mut in_decl in declarations {
        let variable = in_decl.next().unwrap();

        let value = if let Some(_) = in_decl.next() { // declares some value
            in_decl.next().unwrap()
        } else {
            Literal::u8_unsuffixed(0).into() // default value
        };

        result.extend([
            Ident::new("let", Span::mixed_site()).into(),
            variable,
            Punct::new('=', Spacing::Alone).into(),
            value,
            Punct::new(';', Spacing::Alone).into()
        ]);
    }

    result
}