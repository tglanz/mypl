// this is a comment

record Point {
    x: i32,
    y: i32,
}

record Rectangle {
    topLeft: Point,
}

record Circle {
    radius: u32,
}

union Shape {
    Rectangle,
    Circle,
}

function createPoint(x: u32, y: u32) -> Point {
    return Point { x, y }
}

record RGB {
    r: u8,
    g: u8,
    b: u8
}

union Color {
    RGB,
    Gray,
}

function createRGB(r: u8, g: u8, b: u8) -> RGB {
    return RGB { r, g, b }
}

function createRGB(Gray) {
    return RGB { 100, 100, 100 }
}

function createRGB(Color color) {
    match color {
        RGB rgb {
            return rgb
        }
        Gray {
            return { 100, 100, 100 }
        }
    }
}