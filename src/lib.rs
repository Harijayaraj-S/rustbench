// Lib

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn benchmark(_input: TokenStream, item: TokenStream) -> TokenStream {
    let input: ItemFn = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;
    let fn_body = &input.block;
    let expanded = quote! {
        fn #fn_name() {
            let start = std::time::Instant::now();
            #fn_body
            let duration = start.elapsed();
            println!("Function {} executed in {:?}", stringify!(#fn_name), duration);
        }
    };

    expanded.into()
}
