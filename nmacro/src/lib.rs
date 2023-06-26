#![feature(proc_macro_expand)]
use inflector::Inflector;
use proc_macro::TokenStream as TS;
use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::quote;
use syn::{
    parse::{Parse, Parser},
    parse_macro_input,
    punctuated::Punctuated,
    ExprStruct, Field, Fields, FieldsNamed, Generics, ItemStruct, Lifetime, LitStr, Path, Token,
    TypePath, TypeReference,
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
    let st = quote!(&'static str);
    let u3 = quote!(u32);

    println!("keywords = {:#?}", keywords);
    let kwws = keywords
        .iter()
        .map(|kw| {
            let name = Ident::new(&kw.key, Span::call_site());
            let val = if let Ok(u32) = kw.value.parse::<u32>() {
                Literal::u32_suffixed(u32)
            } else {
                Literal::string(&kw.value)
            };

            quote!(#name: #val,)
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
            named: Punctuated::from_iter(keywords.into_iter().map(|kw| {
                Field {
                    attrs: vec![],
                    colon_token: Some(Default::default()),
                    vis: syn::Visibility::Public(Default::default()),
                    ident: Some(Ident::new(&kw.key, Span::call_site())),
                    mutability: syn::FieldMutability::None,
                    ty: if kw.value.parse::<u32>().is_ok() {
                        Parser::parse2(syn::Type::parse, u3.clone())
                    } else {
                        Parser::parse2(syn::Type::parse, st.clone())
                    }
                    .unwrap(),
                }
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

#[proc_macro]
pub fn ranks(input: TS) -> TS {
    let input = Parser::parse(Punctuated::<Ident, Token!(,)>::parse_terminated, input).unwrap();
    let rks = input
        .iter()
        .map(|rank| Ident::new(&rank.to_string().to_pascal_case(), Span::call_site()));
    let rkws = input
        .iter()
        .map(|rank| {
            (
                rank,
                Ident::new(
                    &(rank.to_string().to_uppercase() + "_KEYWORDS"),
                    Span::call_site(),
                ),
            )
        })
        .map(|(rank, kws)| {
            quote! {
                static #kws: RankKeywords = RankKeywords {
                    name: #rank::KEYWORDS.title,
                    price: #rank::KEYWORDS.price
                };
            }
        });
    let rkws_match = input
        .iter()
        .map(|rank| {
            (
                Ident::new(&rank.to_string().to_pascal_case(), Span::call_site()),
                Ident::new(
                    &(rank.to_string().to_uppercase() + "_KEYWORDS"),
                    Span::call_site(),
                ),
            )
        })
        .map(|(rank, kws)| quote!(Self::#rank => &#kws,));
    let render_match = input
        .iter()
        .map(|rank| {
            (
                rank,
                Ident::new(&rank.to_string().to_pascal_case(), Span::call_site()),
            )
        })
        .map(|(rank, var)| quote!(Self::#var => #rank::render(cx)));
    let count = input.len();
    let values = input
        .iter()
        .map(|rank| Ident::new(&rank.to_string().to_pascal_case(), Span::call_site()))
        .map(|rank| quote!(Self::#rank));

    quote! {
        #[derive(Copy, Clone, Debug)]
        pub enum Rank {
            #(#rks)*
        }

        #(#rkws)*

        impl Rank {
            pub const VALUES: [Self; #count] = [#(#values),*];

            pub fn render<'a>(&self, cx: dioxus::prelude::Scope<'a>) -> dioxus::prelude::Element<'a> {
                match self {
                    #(#render_match)*
                }
            }

            #[doc(hidden)]
            pub fn _keywords(&self) -> &'static RankKeywords {
                match self {
                    #(#rkws_match)*
                }
            }
        }
    }
    .into()
}
