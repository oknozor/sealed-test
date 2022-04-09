## Sealed test

This crate exposes the `#[sealed_test]` macro attribute to run your tests in an isolated environment.
It provides the following :
- an isolated process using [rusty-fork](https://crates.io/crates/two-rusty-forks)
- a temporary work dir with [tempfile](https://crates.io/crates/tempfile).
- a set of handy attributes to configure your tests including:
  - `before`/`after`: setup and teardown functions for your tests.
  - `env`: set environment variables in the test process.
  - `files`: copy files from your crate directory to the test temporary directory.
  
**Caution:** using `#[sealed_test]` instead of `#[test]` will create a temporary file
and set it to be the test current directory but, nothing stops you from changing that directory
using `std::env::set_current_dir`.

### Why ?

If you run `cargo test` your tests will run in parallel, in some case this could be problematic.
Let us examine a concrete example.

```rust
 #[test]
fn foo() -> Result<(), VarError> {
    std::env::set_var("VAR", "foo");
    let var = std::env::var("VAR")?;
    assert_eq!(var, "foo");
    Ok(())
}

#[test]
fn bar() -> Result<(), VarError> {
    std::env::set_var("VAR", "bar");
    // During the thread sleep, the `foo` test will run
    // and set the environment variable to "foo"
    std::thread::sleep(Duration::from_secs(1));
    let var = std::env::var("VAR")?;
    // If running tests on multiple threads the below assertion will fail
    assert_eq!(var, "bar");
    Ok(())
}
```

### With `sealed_test`

Here each test has its own environment, the tests will always pass !

```rust
use sealed_test::prelude::*;

#[sealed_test]
fn foo() -> Result<(), VarError> {
   std::env::set_var("VAR", "bar");
    let var = std::env::var("VAR")?;
    assert_eq!(var, "bar");
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
```
## Examples

### The `env` attribute

The `env` attribute allow to quickly set environment variable in your tests.
This is only syntactic sugar and you can still normally manipulate environment variables with `std::env`.

```rust
#[sealed_test(env = [ ("FOO", "foo"), ("BAR", "bar") ])]
fn should_set_env() {
    let foo = std::env::var("FOO").expect("Failed to get $FOO");
    let bar = std::env::var("BAR").expect("Failed to get $BAR");
    assert_eq!(foo, "foo");
    assert_eq!(bar, "bar");
}
```

**Tip**: Sealed tests have their own environment and variable from the parent
won't be affected by whatever you do with the test environment.

### The `files` attribute

If you need test behaviors that mutate the file system, you can use the `files`
attribute to quickly copy files from your crate root to the test working directory.
The test working directory lives in tmpfs and will be cleaned up after the test execution.

```rust
#[sealed_test(files = ["tests/foo", "tests/bar"])]
fn should_copy_files() {
    assert!(PathBuf::from("foo").exists());
    assert!(PathBuf::from("bar").exists());
}
```
### Setup and teardown

Use `before` and `after` to run a rust expression around your tests, typically a function, for instance `setup = setup_function()`.

#### `before` and `after`

```rust
#[sealed_test(before = setup(), after = teardown())]
fn should_run_setup_and_tear_down() {
    // ...
}

fn setup() {
    println!("Hello from setup")
}

fn teardown() {
    println!("Hello from teardown")
}
```
