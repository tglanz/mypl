use crate::ast::Expr;

pub struct AstFormatter;

impl AstFormatter {
    pub fn format_ast(ast: &Expr) -> String {
        let repr = format!("{:#?}", ast);
        repr.replace("    ", "  ")
    }
}
