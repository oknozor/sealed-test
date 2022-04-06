use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse, parse_macro_input};
use attributes::SealedTestAttributes;
use crate::gen::SealedTest;

mod gen;
mod attributes;

#[proc_macro_attribute]
pub fn sealed_test(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as SealedTestAttributes);

    let mut input_fn = parse::<ItemFn>(item).expect("Expected a function");

    let test_fn = SealedTest::new()
        .with_files(args.files)
        .with_env(args.env)
        .with_expr(args.before)
        .with_test(input_fn.block.stmts.clone())
        .with_expr(args.after)
        .build();

    input_fn.block.stmts = test_fn;


    let augmented_test = quote! {
        rusty_fork_test! {
            #[test]
            #input_fn

        }
    };

    TokenStream::from(augmented_test)
}
