//! ## Sealed test
//!
//! This crate exposes the `#[sealed_test]` macro attribute to run your tests in an isolated environment.
//!
//! It provides the following :
//! - an isolated process using [rusty-fork](https://crates.io/crates/two-rusty-forks)
//! - a temporary work dir with [tempfile](https://crates.io/crates/tempfile).
//! - a set of handy attributes to configure your tests including:
//!   - `before`/`after`: setup and teardown functions for your tests.
//!   - `env`: set environment variables in the test process.
//!   - `files`: copy files from your crate directory to the test temporary directory.
//!
//! **Caution:** using `#[sealed_test]` instead of `#[test]` will create a temporary file
//! and set it to be the test current directory but, nothing stops you from changing that directory
//! using `std::env::set_current_dir`.
//!
//! ### Why ?
//!
//! If you run `cargo test` your tests will run in parallel, in some case this could be problematic.
//! Let us examine a concrete example.
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
//! ### With `sealed_test`
//!
//! Here each test has its own environment, the tests will always pass !
//!
//! ```rust
//! # fn main() {
//!
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
//!
//! ## Examples
//!
//! ### The `env` attribute
//!
//! The `env` attribute allow to quickly set environment variable in your tests.
//! This is only syntactic sugar and you can still normally manipulate environment variables with `std::env`.
//!
//!
//! ```rust
//! # fn main() {
//!
//! use sealed_test::prelude::*;
//!
//! #[sealed_test(env = [ ("FOO", "foo"), ("BAR", "bar") ])]
//! fn should_set_env() {
//!     let foo = std::env::var("FOO").expect("Failed to get $FOO");
//!     let bar = std::env::var("BAR").expect("Failed to get $BAR");
//!     assert_eq!(foo, "foo");
//!     assert_eq!(bar, "bar");
//! }
//! # }
//! ```
//!
//! **Tip**: Sealed tests have their own environment and variable from the parent
//! won't be affected by whatever you do with the test environment.
//!
//! ### The `files` attribute
//!
//! If you need test behaviors that mutate the file system, you can use the `files`
//! attribute to quickly copy files from your crate root to the test working directory.
//! The test working directory lives in tmpfs and will be cleaned up after the test execution.
//!
//! ```rust
//! # fn main() {
//!
//! use sealed_test::prelude::*;
//!
//! #[sealed_test(files = ["tests/foo", "tests/bar"])]
//! fn should_copy_files() {
//!     assert!(PathBuf::from("foo").exists());
//!     assert!(PathBuf::from("bar").exists());
//! }
//! # }
//! ```
//!
//! ### Setup and teardown
//!
//! Use `before` and `after` to run a rust expression around your tests, typically a function, for instance `setup = setup_function()`.
//!
//! ```rust
//! # fn main() {
//!
//! use sealed_test::prelude::*;
//!
//! #[sealed_test(before = setup(), after = teardown())]
//! fn should_run_setup_and_tear_down() {
//!     // ...
//! }
//!
//! fn setup() {
//!     println!("Hello from setup")
//! }
//!
//! fn teardown() {
//!     println!("Hello from teardown")
//! }
//! # }
//!```
#![allow(clippy::test_attr_in_doctest)]
extern crate sealed_test_derive;

pub mod prelude;

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use cmd_lib::run_cmd;
    use std::env;
    use std::env::VarError;
    use std::path::PathBuf;
    use std::time::Duration;

    #[sealed_test]
    fn a_dummy_test_with_git() {
        let current_dir = std::env::current_dir().unwrap();
        run_cmd! (
            info "Initializing test repo in ${current_dir:?}";
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
            info "Initializing test repo in ${current_dir:?}";
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
            info "Initializing test repo in ${current_dir:?}";
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

    #[sealed_test(files = ["tests/foo", "tests/bar"])]
    fn should_copy_files() {
        assert!(PathBuf::from("foo").exists());
        assert!(PathBuf::from("bar").exists());
    }

    #[sealed_test(files = ["tests/baz"])]
    fn should_copy_directory() {
        assert!(PathBuf::from("baz/buzz").exists());
    }

    #[sealed_test(env = [ ("FOO", "foo"), ("BAR", "bar") ])]
    fn should_set_env() {
        let foo = std::env::var("FOO").expect("Failed to get $FOO");
        let bar = std::env::var("BAR").expect("Failed to get $BAR");
        assert_eq!(foo, "foo");
        assert_eq!(bar, "bar");
    }

    #[sealed_test(before = setup(), after = teardown())]
    fn should_run_setup_function() {
        std::env::set_var("BEFORE", "ok");
    }

    #[sealed_test(
        env = [ ("HOME", "la maison")],
        files = [ "tests/bar"],
        before = setup(),
        after = teardown()
    )]
    fn should_work_all_together() {
        let home = env::var("HOME").expect("Failed to get $HOME");
        let before = env::var("BEFORE").expect("Failed to get $BEFORE");

        assert!(PathBuf::from("bar").exists());
        assert_eq!(home, "la maison");
        assert_eq!(before, "ok");
    }

    fn setup() {
        std::env::set_var("BEFORE", "ok");
    }

    fn teardown() {
        println!("I run after the test");
    }
}
