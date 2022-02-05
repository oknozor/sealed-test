//! # Sealed test
//!
//! This crate expose the `#[sealed_test]` macro attribute to run your test in an isolated environment.
//!
//! It provides the following :
//! - an isolated process using [rusty-fork](https://crates.io/crates/two-rusty-forks) and
//! - a temporary work dir [tempfile](https://crates.io/crates/tempfile).
//!
//! **Caution:** `using #[sealed_test]` instead of `#[test]` will create a temporary file
//! and set it to be the test current directory but, nothing stops you from changing that directory
//! using `std::env::set_current_dir`.
//!
//! ## Example
//!
//! **Without `sealed_test`** :
//!
//! The below `bar` test will fail because the environment variable will be concurrently altered
//! by the `foo` test.
//!
//!```rust, no_run
//!     #[test]
//!     fn foo() -> Result<(), VarError> {
//!        std::env::set_var("VAR", "foo");
//!        let var = std::env::var("VAR")?;
//!        assert_eq!(var, "foo");
//!        Ok(())
//!    }
//!
//!    #[test]
//!    fn bar() -> Result<(), VarError> {
//!        std::env::set_var("VAR", "bar");
//!        // During the thread sleep, the `foo` test will run
//!        // and set the environment variable to "foo"
//!        std::thread::sleep(Duration::from_secs(1));
//!        let var = std::env::var("VAR")?;
//!        // If running tests on multiple threads the below assertion will fail
//!        assert_eq!(var, "bar");
//!        Ok(())
//!    }
//!```
//!
//! **With `sealed_test`** :
//!
//! Here each test has its own environment, the tests will always pass.
//!
//! ```rust
//! # fn main() {
//! use sealed_test::prelude::*;
//!
//! #[sealed_test]
//! fn foo() -> Result<(), VarError> {
//!    std::env::set_var("VAR", "bar");
//!     let var = std::env::var("VAR")?;
//!     assert_eq!(var, "bar");
//!     Ok(())
//! }
//!
//! #[sealed_test]
//! fn bar() -> Result<(), VarError> {
//!     std::env::set_var("VAR", "bar");
//!     std::thread::sleep(Duration::from_secs(1));
//!     let var = std::env::var("VAR")?;
//!     assert_eq!(var, "bar");
//!     Ok(())
//! }
//! # }
//! ```

extern crate sealed_test_derive;

pub mod prelude;

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use cmd_lib::run_cmd;
    use std::env::VarError;
    use std::time::Duration;

    #[sealed_test]
    fn a_dummy_test_with_git() {
        let current_dir = std::env::current_dir().unwrap();
        run_cmd! (
            info "Initializing test repo in $current_dir";
            git init;
            git commit -m c1 --allow-empty;
            git commit -m c2 --allow-empty;
            git shortlog;
        )
        .unwrap();

        // Make some assertion in the current test dir
    }

    #[sealed_test]
    fn a_dummy_test_with_var() -> Result<(), VarError> {
        std::env::set_var("VAR", "bar");

        let var = std::env::var("VAR")?;

        assert_eq!(var, "bar");
        Ok(())
    }

    #[sealed_test]
    fn a_dummy_test_with_another_var() -> Result<(), VarError> {
        std::env::set_var("VAR", "foo");

        let var = std::env::var("VAR")?;

        assert_eq!(var, "foo");
        Ok(())
    }

    #[sealed_test]
    fn another_dummy_test_with_git() {
        let current_dir = std::env::current_dir().unwrap();
        run_cmd! (
            info "Initializing test repo in $current_dir";
            git init;
            git commit -m "a commit" --allow-empty;
            git checkout -b branch1;
            git shortlog;
        )
        .unwrap();

        // Make some assertion in the current test dir
    }

    #[sealed_test]
    fn a_dummy_test_with_return_type() -> Result<&'static str, &'static str> {
        let current_dir = std::env::current_dir().unwrap();
        run_cmd! (
            info "Initializing test repo in $current_dir";
            git init;
            git commit -m "a commit" --allow-empty;
            git checkout -b branch1;
            git shortlog;
        )
        .unwrap();

        Ok("ok")

        // Make some assertion in the current test dir
    }

    #[sealed_test]
    fn foo() -> Result<(), VarError> {
        std::env::set_var("VAR", "foo");

        let var = std::env::var("VAR")?;

        assert_eq!(var, "foo");
        Ok(())
    }

    #[sealed_test]
    fn bar() -> Result<(), VarError> {
        std::env::set_var("VAR", "bar");
        std::thread::sleep(Duration::from_secs(1));
        let var = std::env::var("VAR")?;
        assert_eq!(var, "bar");
        Ok(())
    }

    #[sealed_test]
    #[should_panic]
    fn question_mark_unwrapping_works() -> Result<(), &'static str> {
        let err = Err("Oh no");
        let _err = err?;
        Ok(())
    }
}
