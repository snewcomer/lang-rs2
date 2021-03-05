use crate::utils;
use crate::env::Env;
use crate::val::Val;
use crate::expr::FuncCall;

#[derive(Clone, Debug, PartialEq)]
pub struct BindingUsage {
    pub(crate) name: String,
}

// get "let" from "let a"
impl BindingUsage {
    pub fn new(s: &str) -> Result<(&str, Self), String> {
        let (s, name) = utils::extract_ident(s)?;

        Ok((s, Self { name: name.to_owned() }))
    }

    pub(super) fn eval(&self, env: &Env) -> Result<Val, String> {
        env.get_binding(&self.name).or_else(|error_msg| {
            if env.get_func(&self.name).is_ok() {
                FuncCall {
                    callee: self.name.clone(),
                    params: Vec::new(),
                }
                .eval(env)
            } else {
                Err(error_msg)
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_binding_usage() {
        assert_eq!(
            BindingUsage::new("abc"),
            Ok((
                "",
                BindingUsage {
                    name: "abc".to_string(),
                },
            )),
        );
        assert_eq!(
            BindingUsage::new("let abc"),
            Ok((
                " abc",
                BindingUsage {
                    name: "let".to_string(),
                },
            )),
        );
        assert_eq!(
            BindingUsage::new("let "),
            Ok((
                " ",
                BindingUsage {
                    name: "let".to_string(),
                },
            )),
        );
    }

    #[test]
    fn parser_error_binding_usage() {
        let empty_env = Env::default();
        assert_eq!(
            BindingUsage { name: "wrong".to_owned() }.eval(&empty_env),
            Err("binding with name 'wrong' does not exist".to_owned())
        );
    }
}
