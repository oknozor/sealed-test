extern crate temp_test_derive;

pub mod prelude;

#[cfg(test)]
mod tests {
    use std::env::VarError;
    use cmd_lib::run_cmd;
    use crate::prelude::*;


    #[temp_test]
    fn a_dummy_test_with_git() {
        let current_dir = std::env::current_dir().unwrap();
        run_cmd! (
            info "Initializing test repo in $current_dir";
            git init;
            git commit -m c1 --allow-empty;
            git commit -m c2 --allow-empty;
            git shortlog;
        ).unwrap();

        // Make some assertion in the current test dir
    }

    #[temp_test]
    fn a_dummy_test_with_var() -> Result<(), VarError> {
        std::env::set_var("VAR", "bar");

        let var = std::env::var("VAR")?;

        assert_eq!(var, "bar");
        Ok(())
    }

    #[temp_test]
    fn a_dummy_test_with_another_var() -> Result<(), VarError> {
        std::env::set_var("VAR", "foo");

        let var = std::env::var("VAR")?;

        assert_eq!(var, "foo");
        Ok(())
    }

    #[temp_test]
    fn another_dummy_test_with_git() {
        let current_dir = std::env::current_dir().unwrap();
        run_cmd! (
            info "Initializing test repo in $current_dir";
            git init;
            git commit -m "a commit" --allow-empty;
            git checkout -b branch1;
            git shortlog;
        ).unwrap();

        // Make some assertion in the current test dir
    }

    #[temp_test]
    fn a_dummy_test_with_return_type() -> Result<&'static str, &'static str> {
        let current_dir = std::env::current_dir().unwrap();
        run_cmd! (
            info "Initializing test repo in $current_dir";
            git init;
            git commit -m "a commit" --allow-empty;
            git checkout -b branch1;
            git shortlog;
        ).unwrap();

        Ok("ok")

        // Make some assertion in the current test dir
    }
}
