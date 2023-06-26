#![feature(proc_macro_expand)]
use proc_macro::TokenStream as TS;
use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::quote;
use syn::{
    parse::{Parse, Parser},
    parse_macro_input,
    punctuated::Punctuated,
    ExprStruct, Field, Fields, FieldsNamed, Generics, ItemStruct, Lifetime, LitStr, Path, TypePath,
    TypeReference,
};

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
    let st = quote!(str);

    println!("keywords = {:#?}", keywords);
    let kwws = keywords
        .iter()
        .map(|kw| {
            let name = Ident::new(&kw.key, Span::call_site());
            let value = LitStr::new(&kw.value, Span::call_site());

            quote!(#name: #value,)
        })
        .collect::<Vec<_>>();

    let kws = quote! {
        Keywords {
            #(#kwws)*
        }
    };

    let kwstruct = ItemStruct {
        fields: Fields::Named(FieldsNamed {
            brace_token: Default::default(),
            named: Punctuated::from_iter(keywords.into_iter().map(|kw| Field {
                attrs: vec![],
                colon_token: Some(Default::default()),
                vis: syn::Visibility::Public(Default::default()),
                ident: Some(Ident::new(&kw.key, Span::call_site())),
                mutability: syn::FieldMutability::None,
                ty: syn::Type::Reference(TypeReference {
                    and_token: Default::default(),
                    lifetime: Some(Lifetime::new("'static", Span::call_site())),
                    elem: Box::new(Parser::parse2(syn::Type::parse, st.clone()).unwrap()),
                    mutability: None,
                }),
            })),
        }),
        attrs: vec![],
        generics: Generics::default(),
        ident: Ident::new("Keywords", Span::call_site()),
        semi_token: None,
        struct_token: Default::default(),
        vis: syn::Visibility::Public(Default::default()),
    };

    quote! {
        #[allow(non_snake_case)]
        #kwstruct

        #[allow(non_snake_case)]
        pub const KEYWORDS: Keywords = #kws;

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
