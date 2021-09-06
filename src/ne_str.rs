use {
    proc_macro::TokenStream,
    proc_macro2::{Literal as Literal2, TokenStream as TokenStream2, TokenTree as TokenTree2},
    quote::quote,
};

pub(crate) fn nestr_impl(item: TokenStream2) -> TokenStream {
    let mut iter = item.into_iter();

    let string_tt = iter
        .next()
        .expect("`nestr` macro takes one non-empty quoted string literal - none were provided");

    let result = match string_tt {
        TokenTree2::Literal(string_lit) => {
            // At least [" "].
            let orig_string = string_lit.to_string();
            assert!(
                orig_string.len() >= 3,
                "`nestr` macro takes one non-empty quoted string literal - `{}` was provided",
                orig_string
            );

            // Trim quotes: ["asdf"] -> [asdf].
            if let Some(string) = orig_string.strip_prefix("\"") {
                if let Some(_) = string.strip_suffix("\"") {
                    let string_lit: Literal2 = string_lit.into();

                    TokenStream::from(quote!(
                        unsafe { ministr::NonEmptyStr::new_unchecked(#string_lit) }
                    ))
                } else {
                    panic!("`nestr` macro takes one non-empty quoted string literal - `{}` does not end with a quote", orig_string);
                }
            } else {
                panic!("`nestr` macro takes one non-empty quoted string literal - `{}` does not start with a quote", orig_string);
            }
        }

        TokenTree2::Group(group) => nestr_impl(group.stream()),

        TokenTree2::Ident(ident) => {
            panic!(
                "`nestr` macro takes one non-empty quoted string literal - ident `{}` was provided",
                ident
            );
        }

        TokenTree2::Punct(punct) => {
            panic!(
                "`nestr` macro takes one non-empty quoted string literal - punct `{}` was provided",
                punct
            );
        }
    };

    assert!(
        iter.next().is_none(),
        "`nestr` macro takes one non-empty quoted string literal - multiple were provided"
    );

    result
}
