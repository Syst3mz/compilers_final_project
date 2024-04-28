use crate::testing::s_expr::SExpr;

pub trait ToSExpr {
    fn to_s_expr(self) -> SExpr<String>;
}