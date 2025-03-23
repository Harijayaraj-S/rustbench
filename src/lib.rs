use proc_macro::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::{parse::Parse, parse_macro_input, Expr, ItemFn, Lit};
use syn::{ExprLit, Token};

#[derive(Default, Debug)]
struct MacroParamInput {
    pub times: Option<i64>,
}

impl Parse for MacroParamInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut output_value = MacroParamInput::default();
        let values: Punctuated<Expr, Token![,]> = Punctuated::parse_terminated(input)?;

        let mut iter = values.iter();

        if let Some(expr) = iter.next() {
            if let Expr::Lit(ExprLit {
                lit: Lit::Int(lit_int),
                ..
            }) = expr
            {
                output_value.times = Some(lit_int.base10_parse::<i64>().unwrap());
            }
        }

        Ok(output_value)
    }
}

#[proc_macro_attribute]
pub fn benchmark(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = &input_fn.sig.ident;
    let fn_body = &input_fn.block;

    let parsed_attr = parse_macro_input!(attr as MacroParamInput);
    let times = parsed_attr.times.unwrap_or(1);

    let expanded = if times > 1 {
        quote! {
            fn #fn_name() {
                use std::time::Instant;
                let mut total_time = std::time::Duration::new(0, 0);

                for _ in 0..#times {
                    let start = Instant::now();
                    #fn_body
                    let duration = start.elapsed();
                    total_time += duration;
                    println!("Iteration took: {:?}", duration);
                }

                let avg_time = total_time.as_nanos() / (#times as u128);
                println!("Function '{}' executed {} times. Avg time: {:?} ns",stringify!(#fn_name), #times, avg_time);
            }
        }
    } else {
        quote! {
            fn #fn_name() {
                use std::time::Instant;
                let start = Instant::now();
                #fn_body
                let duration = start.elapsed().as_nanos();
                println!("Function '{}' executed in {:?} ns", stringify!(#fn_name), duration);
            }
        }
    };

    expanded.into()
}
