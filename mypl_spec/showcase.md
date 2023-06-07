# mypl-spec

## Showcase

### variable definitions and comments

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

for i in 0..1000 {
    if i mod 2 == 0 && i < 300 {
        x = x + i;
    } else if i mod 2 == 1 && i > 500 && i < 1000 {
        y = y + i;
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

increment: (value: i8) i8 = {
    return value + 1;
}

decrement: (value: i8) i8 = {
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
    height: u32,
};

// define a square record
Square = record {
    center: Point,
    width: u32,
    height: u32,
};

// define a geometry union using already defined records
Geometry = union {
    Circle = record {
        center: Point,
        radius: u32,
    },
    square,
    rectangle,
}

// define a quadrilateral union using already defined, and already used in other union records
Quadrilateral = union { square: Square, rectangle: Rectangle };

</pre>

### Traits 

Point2d = record {
    x: f32,
    y: f32,
}

Point3d = record {
    x: f32,
    y: f32,
    z: f32,
}

trait Metric {
  distance: () f32;
}

impl Metric for f32 {
  distance: () f32 = {
    if this > 0 {
      return this;
    }

    return -1 * this;
  }
}

impl Metric for Point2d {
  // l1 norm
  distance: () f32 = {
    return this.x.distance() + this.y.distance();
  }
}

impl Metric for Point3d {
  // l1 norm
  distance: () f32 = {
    return this.x.distance() + this.y.distance() + this.z.distance();
  }
}

### Modules

Modules is a set of elements.
Package is a set of modules.

You define a Module using the `module` keyword:

```
module my_mod {
}
```

Or, if the file contain a single modules

```
module my_mod;
```

For example


- `public`: Visible to anyown
- `internal`: Visible to the package
- `private`: Visible to the module

```
module shapes;

private Point2d {
  x: f32,
  y: f32,
}

public Circle {
  center: Point2d,
  radius: f32,
}

impl Circle {
  diameter: () f32 = {
    return 2 * this.radius;
  }
}
```


