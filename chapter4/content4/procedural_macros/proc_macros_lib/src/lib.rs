use proc_macro;
use quote;
use syn;

#[proc_macro]
pub fn str_with_len(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::LitStr);

    let input_str = input.value();
    let len = input_str.len();

    let output = quote::quote! {
        (#len, #input_str)
    };

    output.into()
}

#[proc_macro_attribute]
pub fn log_input(
    ann: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut input = syn::parse_macro_input!(item as syn::ItemFn);
    let ann_str = ann.to_string();

    let fn_name = &input.sig.ident;

    let args: Vec<_> = input
        .sig
        .inputs
        .iter()
        .map(|arg| match arg {
            syn::FnArg::Typed(pat_type) => {
                if let syn::Pat::Ident(ident) = &*pat_type.pat {
                    let arg_value = &ident.ident;
                    quote::quote! { #arg_value }
                } else {
                    quote::quote! {}
                }
            }
            _ => quote::quote! {},
        })
        .collect();

    let log_stmt = quote::quote! {
        println!(
            "{}: Calling function '{}' with args: {:?}",
            #ann_str,
            stringify!(#fn_name),
            (#(#args),*)
        );
    };

    let original_body = input.block;

    input.block = Box::new(syn::parse_quote!({
        #log_stmt
        #original_body
    }));

    quote::quote! { #input }.into()
}

#[proc_macro_derive(Hello)]
pub fn derive_hello(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    let name = &input.ident;

    let expanded = quote::quote! {
        pub trait Hello {
            fn hello(&self);
        }

        impl Hello for #name {
            fn hello(&self) {
                println!("Hello from {}!", stringify!(#name));
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}
