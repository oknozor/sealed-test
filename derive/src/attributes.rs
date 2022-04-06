use syn::{parse::{Parse, ParseStream}, bracketed, Expr, LitStr, Token, parenthesized};

pub struct SealedTestAttributes {
    pub files: Vec<String>,
    pub env: Vec<EnvVar>,
    pub before: Option<Expr>,
    pub after: Option<Expr>,
}

pub struct EnvVar {
    pub name: String,
    pub value: String,
}

impl Parse for EnvVar {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        parenthesized!(content in input);
        let name = content.parse::<LitStr>()?.value();
        let _ = content.parse::<Token!(,)>()?;
        let value = content.parse::<LitStr>()?.value();
        Ok(Self {
            name,
            value,
        })
    }
}

impl Parse for SealedTestAttributes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut attributes = SealedTestAttributes {
            files: vec![],
            env: vec![],
            before: None,
            after: None,
        };

        while let Ok(ident) = input.parse::<syn::Ident>() {
            input.parse::<Token!(=)>()?;

            match ident.to_string().as_str() {
                "files" => attributes.files = Self::parse_files(input)?,
                "env" => attributes.env = Self::parse_env(input)?,
                "before" => attributes.before = Some(input.parse::<Expr>()?),
                "after" => attributes.after = Some(input.parse::<Expr>()?),
                other => panic!(
                    "unexpected attribute {}, use 'files', 'env', 'setup' or 'teardown'",
                    other
                ),
            }

            if input.peek(Token!(,)) {
                input.parse::<Token!(,)>()?;
            }
        }

        Ok(attributes)
    }
}

impl SealedTestAttributes {
    fn parse_files(input: ParseStream) -> syn::Result<Vec<String>> {
        let content;
        bracketed!(content in input);

        let mut files = vec![];


        while let Ok(file) = content.parse::<LitStr>() {
            files.push(file.value());
            if !content.peek(Token!(,)) && !content.is_empty() {
                content.parse::<Token!(,)>()?;
            } else if !content.is_empty() {
                content.parse::<Token!(,)>()?;
            }
        }

        Ok(files)
    }

    fn parse_env(input: ParseStream) -> syn::Result<Vec<EnvVar>> {
        let content;
        bracketed!(content in input);

        let mut env_vars = vec![];

        while let Ok(env_var) = content.parse::<EnvVar>() {
            env_vars.push(env_var);
            if !content.peek(Token!(,)) && !content.is_empty() {
                content.parse::<Token!(,)>()?;
            } else if !content.is_empty() {
                content.parse::<Token!(,)>()?;
            }
        }

        Ok(env_vars)
    }
}
