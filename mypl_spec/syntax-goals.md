# mypl-spec

## syntax goals

### coherency

this is achieved by providing a single definition and initialization structure to all language components.

this is implemented using decleration with standard form
<pre>
[modifier] identifier [: type] = {expression};
</pre>

we try to spare the problem of other languages where there are many edge cases and keyword ambigouty or contextual semantics. i like java, but can you tell me what the ```final``` keyword is doing? or cpp's ```const``` keyword? to answer those question we need context.

#### examples

variable definition
<pre>
// no modifier
// identifer is `x`
// type is u32
// expression is 4

x: u32 = 4;
</pre>

function definition
<pre>
// no modifier
// identifer is `increment`
// type is "a function from u32 to u32"
// expression is the function implemntation - `return x = 1`

increment: (x: u32) u32 = {
    return x = 1;
}
</pre>

record definition
<pre>
// no modifier
// identifier is `Point
// type is `record`
// expression is a record type initialization

Point = record {
    // record definition
};

</pre>


note that this is not an edge case of the standard form.
the same form of `identifier: type = expression;` still holds.
we can read the above expression to have the `record` type.
the `record` type is a is builtin and is basically a type constructor.

The Point's type is inferred and is basically an anonmous type, but just for the sake of the argument one could write

<pre>
Point: 'PointConstructor = record {
    // record definition
};
</pre>
