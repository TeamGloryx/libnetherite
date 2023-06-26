#![feature(proc_macro_expand)]
use proc_macro::TokenStream as TS;
use proc_macro2::{Literal, TokenStream};
use quote::quote;
use syn::{parse_macro_input, LitStr};

fn literate(num: f32) -> Literal {
    if num.fract() == 0.0 {
        Literal::u8_unsuffixed(num as u8)
    } else {
        Literal::f32_unsuffixed(num)
    }
}

#[proc_macro]
pub fn im_spacing(b: TS) -> TS {
    let c = b.to_string();
    let items: Vec<TokenStream> = c
        .split(',')
        .map(|item| {
            let (num, rem) = item.split_once("=>").unwrap();
            let (num, rem) = (num.trim(), rem.trim());
            let (num, rem): (f32, f32) = (num.parse().unwrap(), rem.parse().unwrap());
            (literate(num), Literal::f32_suffixed(rem))
        })
        .map(|(num, rem)| quote!([#num] => (crate::ui::spacing::Spacing::Rem(#rem))))
        .collect();

    quote! {
        #[doc = include_str!("./SPACING.md")]
        pub macro tw_spacing {
            [px] => ( crate::ui::Spacing::Px(1f32) ),
            #(#items,)*
            [rem($n:literal)] => ( crate::ui::spacing::Spacing::Rem($n) ),
            [em($n:literal)] => ( crate::ui::spacing::Spacing::Em($n) ),
            [px($n:literal)] => ( crate::ui::spacing::Spacing::Px($n) )
        }
    }
    .into()
}

#[proc_macro]
pub fn org_mod(what: TS) -> TS {
    let what = what.expand_expr().unwrap();
    let what = parse_macro_input!(what as LitStr).value();
    let org = orgize::Org::parse(&what);
    let mut html = Vec::new();
    org.write_html(&mut html).unwrap();
    let html = String::from_utf8(html).unwrap();
    let html_lit = Literal::string(&html);
    let keywords = org.keywords().collect::<Vec<_>>();

    println!("{:#?}", keywords);

    quote! {
        pub const ORG_DATA: &'static str = #what;
        pub const HTML_DATA: &'static str = #html_lit;
        use dioxus::prelude::*;

        pub fn render(cx: Scope) -> Element {
            cx.render(rsx! {
                div {
                    dangerous_inner_html: HTML_DATA
                }
            })
        }
    }
    .into()
}
