use crate::parsing_utils::Spanned;

pub type SExpr = Spanned<Expr>;

#[derive(Debug)]
pub(super) enum Expr {
    Symbol(String),
    Number(i64),
    String(String),
    List(Vec<SExpr>),
    Quote(Box<SExpr>),
}


