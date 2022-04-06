//! ## Sealed test
//!
//! This crate expose the `#[sealed_test]` macro attribute to run your tests in an isolated environment.
//!
//! It provides the following :
//! - an isolated process using [rusty-fork](https://crates.io/crates/two-rusty-forks)
//! - a temporary work dir with [tempfile](https://crates.io/crates/tempfile).
//! - a set of handy attributes to configure your tests including:
//!   - `before`/`after`: teardown and setup functions for your tests.
//!   - `env`: set environment variables in the test process.
//!   - `files`: copy files from your crate directory to the test temporary directory.
//!   - `cmd_before`/`cmd_after`: setup and tear down using shell-script like tasks provided by the [cmd-lib](https://docs.rs/cmd_lib/1.3.0/cmd_lib/index.html) crate.
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
//! **with `sealed_test`** :
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
//! **The `env` attribute**
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
//! **The `files` attribute:**
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
//! **setup and teardown:**
//!
//! Sealed test provides two kinds of setup and teardown attribute :
//! - `before` and `after`: to run a expression around your tests, typically a function, for instance `setup = setup_function()`.
//! - `cmd_before` and `cmd_after` to run a  shell-script like tasks, typically a function, see the examples below.
//!
//! `before and after`
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
//! `cmd_before` and `cmd_after`:
//!
//! ```rust
//! # fn main() {
//! #[sealed_test(
//!      cmd_before = {
//!          git init;
//!          git commit --allow-empty -m "first commit";
//!      },
//!      cmd_after = {
//!          git --no-pager log;
//!      },
//!  )]
//!  fn git_cmd_setup_and_tear_down() {
//!      assert!(PathBuf::from(".git").exists());
//!  }
//! # }

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

    #[sealed_test(
        cmd_before = {
            echo "Hello from shell test setup!";
            ls -larth;
        },
        before = setup(),
        env = [ ("HOME", "la maison") ],
        files = [ "tests/bar" ],
    )]
    fn should_run_cmd() {
        let home = env::var("HOME").expect("Failed to get $HOME");
        assert_eq!(home, "la maison");
        assert!(PathBuf::from("bar").exists());
    }

    #[sealed_test(
        cmd_before = {
            git init;
            git commit --allow-empty -m "first commit";
        },
        cmd_after = {
            git --no-pager log;
        },
    )]
    fn should_run_cmd_after() {
        assert!(PathBuf::from(".git").exists());
    }
}
