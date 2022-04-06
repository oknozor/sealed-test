use syn::__private::TokenStream2;
use syn::{
    braced, bracketed, parenthesized,
    parse::{Parse, ParseStream},
    Expr, LitStr, Token,
};

pub struct SealedTestAttributes {
    pub files: Vec<String>,
    pub env: Vec<EnvVar>,
    pub before: Option<Expr>,
    pub after: Option<Expr>,
    pub cmd_before: Option<TokenStream2>,
    pub cmd_after: Option<TokenStream2>,
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
        Ok(Self { name, value })
    }
}

impl Parse for SealedTestAttributes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut attributes = SealedTestAttributes {
            files: vec![],
            env: vec![],
            before: None,
            after: None,
            cmd_before: None,
            cmd_after: None,
        };

        while let Ok(ident) = input.parse::<syn::Ident>() {
            input.parse::<Token!(=)>()?;

            match ident.to_string().as_str() {
                "files" => attributes.files = Self::parse_files(input)?,
                "env" => attributes.env = Self::parse_env(input)?,
                "before" => attributes.before = Some(input.parse::<Expr>()?),
                "after" => attributes.after = Some(input.parse::<Expr>()?),
                "cmd_before" => attributes.cmd_before = Some(Self::parse_cmd(input)?),
                "cmd_after" => attributes.cmd_after = Some(Self::parse_cmd(input)?),
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

    fn parse_cmd(input: ParseStream) -> syn::Result<TokenStream2> {
        let content;
        braced!(content in input);
        let cmds = content.parse::<TokenStream2>()?;
        Ok(cmds)
    }
}
