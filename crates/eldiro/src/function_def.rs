use crate::stmt::Stmt;
use crate::utils;
use crate::env::Env;

#[derive(Clone, Debug, PartialEq)]
pub struct FuncDef {
    pub(crate) name: String,
    pub(crate) params: Vec<String>,
    pub(crate) body: Box<Stmt>,
}

impl FuncDef {
    pub(crate) fn new(s: &str) -> Result<(&str, Self), String> {
        let s = utils::tag("fn", s)?;

        let (s, _) = utils::extract_whitespace(s);

        let (s, name) = utils::extract_ident(s)?;

        let (s, _) = utils::extract_whitespace(s);

        let (s, params) = utils::sequence(
            |s| utils::extract_ident(s).map(|(s, ident)| (s, ident.to_owned())),
            utils::extract_whitespace,
            s
        )?;

        let s = utils::tag("=>", s)?;
        let (s, _) = utils::extract_whitespace(s);

        let (s, body) = Stmt::new(s)?;

        Ok((
            s,
            Self {
                name: name.to_owned(),
                params,
                body: Box::new(body)
            }
       ))
    }

    pub(crate) fn eval(&self, env: &mut Env) -> Result<(), String> {
        env.store_func(self.name.clone(), self.params.clone(), *self.body.clone());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::{Block, Expr, Op, BindingUsage};

    #[test]
    fn parse_func_no_params() {
        assert_eq!(
            FuncDef::new("fn nothing => {}"),
            Ok((
                "",
                FuncDef {
                    name: "nothing".to_string(),
                    params: Vec::new(),
                    body: Box::new(Stmt::Expr(Expr::Block(Block { stmts: Vec::new() })))
                }
            ))
        );
    }

    #[test]
    fn parse_func_params() {
        assert_eq!(
            FuncDef::new("fn hi name => {}"),
            Ok((
                "",
                FuncDef {
                    name: "hi".to_string(),
                    params: vec!["name".to_owned()],
                    body: Box::new(Stmt::Expr(Expr::Block(Block { stmts: Vec::new() })))
                }
            ))
        );
    }

    #[test]
    fn parse_func_multiple_params() {
        assert_eq!(
            FuncDef::new("fn hi name last => {}"),
            Ok((
                "",
                FuncDef {
                    name: "hi".to_string(),
                    params: vec!["name".to_owned(), "last".to_owned()],
                    body: Box::new(Stmt::Expr(Expr::Block(Block { stmts: Vec::new() })))
                }
            ))
        );
    }

    #[test]
    fn parse_func_expr() {
        assert_eq!(
            FuncDef::new("fn hi x y => x + y"),
            Ok((
                "",
                FuncDef {
                    name: "hi".to_string(),
                    params: vec!["x".to_owned(), "y".to_owned()],
                    body: Box::new(Stmt::Expr(Expr::Operation {
                        lhs: Box::new(Expr::BindingUsage(BindingUsage {
                            name: "x".to_owned()
                        })),
                        rhs: Box::new(Expr::BindingUsage(BindingUsage {
                            name: "y".to_owned()
                        })),
                        op: Op::Add
                    }))
                }
            ))
        );
    }
}
