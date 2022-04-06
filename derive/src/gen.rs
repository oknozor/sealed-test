use std::path::PathBuf;
use syn::{Stmt, parse_quote, Expr};
use crate::attributes::EnvVar;

pub struct SealedTest {
    stmt: Vec<Stmt>,
}

impl SealedTest {
    pub(crate) fn new() -> Self {
        Self {
            stmt: parse_quote! {
                let temp_dir = tempfile::TempDir::new().unwrap();
                std::env::set_current_dir(&temp_dir).unwrap();
                let crate_dir: String = std::env::var("CARGO_MANIFEST_DIR").unwrap();
            }
        }
    }

    pub fn build(self) -> Vec<Stmt> {
        self.stmt
    }


    pub fn with_expr(mut self, expr: Option<Expr>) -> Self {
        if let Some(expr) = expr {
            self.stmt.push(parse_quote!(
                #expr;
            ));
        }
        self
    }

    pub fn with_test(mut self, test_stmt: Vec<Stmt>) -> Self {
        self.stmt.extend(test_stmt);
        self
    }

    pub fn with_files(mut self, files: Vec<String>) -> Self {
        for file in files {
            let target = PathBuf::from(file.clone());
            let target = target.file_name()
                .unwrap()
                .to_str()
                .unwrap();

            self.stmt.push(parse_quote!(
            {
                let src = std::path::PathBuf::from(&crate_dir).join(#file);
                let dest = std::env::current_dir().unwrap().join(#target);

                let mut path = std::path::PathBuf::from(#file);

                if src.is_dir() {
                    let mut opt = fs_extra::dir::CopyOptions::new();
                    opt.copy_inside = true;
                    let copy = fs_extra::dir::copy(&src, &dest, &opt);

                    if let Err(error) = copy {
                        panic!("failed to copy {:?} to test directory {:?}. Error = {}", src, dest, error);
                    };
                } else {
                    let copy = std::fs::copy(&src, &dest);

                    if let Err(error) = copy {
                        panic!("failed to copy {:?} to test directory {:?}. Error = {}", src, dest, error);
                    };
                }
            }));
        }

        self
    }

    pub fn with_env(mut self, env: Vec<EnvVar>) -> Self {
        for var in env {
            let key = var.name;
            let value = var.value;
            self.stmt.push(parse_quote!(
                std::env::set_var(#key, #value);
            ));
        }

        self
    }
}
