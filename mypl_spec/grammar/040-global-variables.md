The basic syntax rules of the language.

These rules are a subset of the entire language.

We aim for non-left recursions.

Abbreviations:
- arit : arithmetic
- expr : expression
- bin  : binary
- op   : operator

Following operatoes have a low to high precedence.

Name       | Operators | Associates
-----------|-----------|-------------
Equality   | == !=     | Left
Comparison | > >= < <= | Left
Term       | - +       | Left
Factor     | / *       | Left
Unary      | ! -       | Right

```ebnf

program = decl* EOF;





decl = constDecl
     | stmt
     ;

constDecl = "const" IDENTIFIER "=" expr ";" ;




stmt = exprStmt
     ;

exprStmt = expr ";";





expr = equality

equality = comparison (("==" "!=") comparison)*;

comparison = term ((">" ">=" "<" "<=") term)*;

term = factor (("-" | "+") factory)*;

factor = unary (("/" | "*") unary)*;

unary = ("!" | "-") unary
      | primary
      ;






primary =
        | NUMBER
        | STRING 
        | "true"
        | "false"
        | "(" expr ")"
        | IDENTIFIER
        ;
```
