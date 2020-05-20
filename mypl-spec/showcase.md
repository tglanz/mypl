# mypl-spec

## Showcase

### variable definitions and comments

there is a strong emphasis 

<pre>
// this is a comment
// there is no other way to make a comment

cst x: u8 = 30;
var y: u8 = 10;

z: u8 = 40;
</pre>


### simple for loop

- define two 32 bit unsigned integer values
- one should sum even numbers up to 300
- the other should sum odd numbers from 500 to 1000

<pre>
x: u32 = 0;
y: u32 = 0;

for (i in 0..1000) {
    if (i mod 2 == 0 and i < 300) {
        x = x + i;
    } else if (i mod 2 == 1 and i > 500 and i < 1000) {
        y = y +i;
    }
}
</pre>

note that i is constant like every other definition with implicit mutability. this is like writing in javascript the following

<pre>
for (const i of ...) {
    ...
}
</pre>

### simple functions

define an in/de crement functions and apply them to a variable

<pre>

increment: (value: i8) -> i8 = {
    return value + 1;
}

decrement: (value: i8) -> i8 = {
    return value - 1;
}

x: i8 = 0;
x = increment(x);
x = decrement(x);

</pre>

### ADT

#### Records

<pre>
// definition of the Point record
Point = record {
    x: u32,
    y: u32,
};

// instantiation of a Point record
point: Point = {
    x = 4,
    y = 3,
};
</pre>

#### Unions

<pre>
// define a Geometry union
Geometry = union {
    Circle = record {
        center: Point,
        radius: u32,
    },
    Square = record {
        center: Point,
        size: u32,
    },
    Rectangle = record {
        center: Point,
        width: u32,
        height: 32,
    },
};
</pre>

for some reason, other languages as far as i can tell doesn't support embedding pre-existing record equivalents inside a new union.

i don't know why, perhaps there is some implementation, or conceptual issue with it. let's see how it goes when it goes

potentially, we aim for the following capability

#### Multiple usages

<pre>
// define a rectangle record
Rectangle = record {
    center: Point,
    width: u32,
    height: 32,
};

// define a square record
Square = record {
    center: Point,
    width: u32,
    height: 32,
};

// define a geometry union using already defined records
Geometry = union {
    Circle = record {
        center: Point,
        radius: u32,
    },
    Square,
    Rectagle,
}

// define a quadrilateral union using already defined, and already used in other union records
Quadrilateral = union { Square, Rectangle };

</pre>

### Type Classes

TBD

am not exactly sure how to spec this, breaking coherency with any attempt.

### Modules

TBD