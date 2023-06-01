# ADT

Color = union {
    Red,
    Green,
    Blue,

    RGB: record {
        red: u8,
        green: u8,
        blue: u8
    }
};

color = Color.RGB {
  red = 128,
  green = 128,
  blue = 128,
};

Point = record {
    x: u32,
    y: u32,
};

point = Point {
  x = 3,
  y = 4,
};

Vertex = record {
    point: Point,

    texture: record {
        u: u32
        v: u32,
    },

    color: Color,
};

vertex = Vertex {
  point = point,
  texture = {
    u: 12,
    v: 32,
  },
  color = color,
};
