The basic syntax rules of the language.

These rules are a subset of the entire language.

This grammar supports LL2 parsers (I think).
One example of when 2 token look ahead is required is at the assignment statment.
To check wether it is an assigmnment statment we need to match IDENTIFIER followed by "=".
We cannot just check IDENTIFIER because it is also a valid expression.

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
     | varDecl
     | stmt
     ;

constDecl = "const" IDENTIFIER "=" expr ";" ;
varDecl = "var" IDENTIFIER "=" expr ";" ;

stmt = printlnStmt 
     | printStmt
     | assignmentStmt 
     | exprStmt
     ;

printlnStmt = "println" expr ";" ;
printStmt = "print" expr ";" ;

assignmentStmt = IDENTIFIER "=" expr ;

exprStmt = expr ";" ;

expr = equality

equality = comparison (("==" | "!=") comparison)* ;

comparison = term ((">" | ">=" | "<" | "<=") term)* ;

term = factor (("-" | "+") factory)* ;

factor = unary (("/" | "*") unary)* ;

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
