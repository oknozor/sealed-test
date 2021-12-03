use proc_macro::TokenStream;
use syn::{parse, ItemFn, Stmt, parse_quote};
use quote::quote;

#[proc_macro_attribute]
pub fn temp_test(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let tmpdir: Stmt = parse_quote! {
        let temp_dir = tempfile::TempDir::new().unwrap();
    };

    let current_dir = parse_quote! {
            std::env::set_current_dir(&temp_dir).unwrap();
    };

    let mut input = parse::<ItemFn>(item).expect("Expected a function");
    let mut statements = vec![tmpdir, current_dir];
    statements.extend(input.block.stmts.clone());

    input.block.stmts = statements;


    TokenStream::from( quote! {
        rusty_fork_test! {
            #[test]
            #input
        }
    })
}