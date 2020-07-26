# ADT

Color = union {
    Red,
    Green,
    Blue,

    RGB = record {
        red: u8,
        green: u8,
        blue: u8
    }
};

Point = record {
    x: u32,
    y: u32,
};

Vertex = record {
    point: Point,

    texture: record {
        u: u32
        v: u32,
    },

    color: Color,
};