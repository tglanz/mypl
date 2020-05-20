```

/* Arithmetic Expressions */

<arithmetic_expression>
        = <arithmetic_expression> + <arithmetic_expression>
        | <arithmetic_expression> - <arithmetic_expression>
        | <arithmetic_expression> * <arithmetic_expression>
        | <arithmetic_expression> / <arithmetic_expression>

/* Boolean Expressions */

<boolean_expression>
        = <expression> == expression
        | <boolean_expression> and <boolean_expression>
        | <boolean_expression> or <boolean_expression>
        | not <boolean_expression>

```
