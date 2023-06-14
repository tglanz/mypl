use crate::ast::Stmt;

pub struct AstFormatter;

impl AstFormatter {
    pub fn format_ast(ast: &Vec<Stmt>) -> String {
        let repr = format!("{:#?}", ast);
        repr.replace("    ", "  ")
    }
}
