use crate::gen::SealedTest;
use attributes::SealedTestAttributes;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse, parse_macro_input, ItemFn};

mod attributes;
mod gen;

#[proc_macro_attribute]
pub fn sealed_test(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as SealedTestAttributes);

    let mut input_fn = parse::<ItemFn>(item).expect("Expected a function");
    let input = input_fn.block.stmts.clone();

    let test_fn = SealedTest::new()
        .with_files(args.files)
        .with_env(args.env)
        .with_cmd_before(args.cmd_before)
        .with_expr(args.before)
        .with_test(input)
        .with_expr(args.after)
        .with_cmd_before(args.cmd_after)
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
