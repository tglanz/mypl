// The basic syntax rules of the language.
// These rules are a subset of the entire language.
//
// Abbreviations:
// - expr : expression
// - bin  : binary
// - op   : operator

expr = literal | unary_expr | binary_expr | group_expr;


literal = NUMBER | STRING | BOOLEAN;

group_expr = "(" expr ")";

unary_expr = ("-" | "!") expr;

bin_expr = expr bin_op expr;

bin_op = "==" | "!=" | "<" | "<=" | ">" | ">="
       | "+" | "-" | "*" | "/";
