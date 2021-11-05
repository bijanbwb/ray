use std::fs;
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::path::Path;

// MAIN

fn main() {
    println!("Ray Tracing in Rust!");
}

// TUPLE

#[derive(Clone, Copy, Debug, PartialEq)]
struct Tuple {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Tuple {
    fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Tuple { x, y, z, w }
    }

    fn point(x: f32, y: f32, z: f32) -> Self {
        Tuple { x, y, z, w: 1.0 }
    }

    fn vector(x: f32, y: f32, z: f32) -> Self {
        Tuple { x, y, z, w: 0.0 }
    }

    fn is_point(self) -> bool {
        self.w == 1.0
    }

    fn is_vector(self) -> bool {
        self.w == 0.0
    }

    fn magnitude(self) -> f32 {
        let float: f32 = (self.x * self.x) + (self.y * self.y) + (self.z * self.z);
        float.sqrt()
    }

    fn normalize(self) -> Self {
        Tuple {
            x: self.x / Tuple::magnitude(self),
            y: self.y / Tuple::magnitude(self),
            z: self.z / Tuple::magnitude(self),
            w: self.w / Tuple::magnitude(self),
        }
    }

    fn dot(a: Self, b: Self) -> f32 {
        a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w
    }

    fn cross(a: Self, b: Self) -> Self {
        Tuple {
            x: a.y * b.z - a.z * b.y,
            y: a.z * b.x - a.x * b.z,
            z: a.x * b.y - a.y * b.x,
            w: 0.0,
        }
    }
}

impl Add for Tuple {
    type Output = Self;

    fn add(self, other: Tuple) -> Tuple {
        Tuple {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Sub for Tuple {
    type Output = Self;

    fn sub(self, other: Tuple) -> Tuple {
        Tuple {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Tuple {
        Tuple {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl Mul<f32> for Tuple {
    type Output = Self;

    fn mul(self, scalar: f32) -> Tuple {
        Tuple {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
            w: self.w * scalar,
        }
    }
}

impl Div<f32> for Tuple {
    type Output = Self;

    fn div(self, scalar: f32) -> Tuple {
        Tuple {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
            w: self.w / scalar,
        }
    }
}

// PROJECTILE

#[derive(Clone, Copy, Debug, PartialEq)]
struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

impl Projectile {
    fn new(position: Tuple, velocity: Tuple) -> Self {
        Projectile { position, velocity }
    }
}

// ENVIRONMENT

#[derive(Clone, Copy, Debug, PartialEq)]
struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

impl Environment {
    fn new(gravity: Tuple, wind: Tuple) -> Self {
        Environment { gravity, wind }
    }

    fn tick(environment: Environment, projectile: Projectile) -> Projectile {
        Projectile {
            position: projectile.position + projectile.velocity,
            velocity: projectile.velocity + environment.gravity + environment.wind,
        }
    }
}

// COLOR

#[derive(Clone, Copy, Debug)]
struct Color {
    red: f32,
    green: f32,
    blue: f32,
}

impl Color {
    fn new(red: f32, green: f32, blue: f32) -> Self {
        Color { red, green, blue }
    }

    fn to_int(_color: Color) -> i32 {
        // TODO: Turn rgb into integer between 0 and 255.
        0
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, other: Color) -> Self {
        Color {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
        }
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, other: Color) -> Self {
        Color {
            red: self.red - other.red,
            green: self.green - other.green,
            blue: self.blue - other.blue,
        }
    }
}

impl Mul<f32> for Color {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Color {
            red: self.red * scalar,
            green: self.green * scalar,
            blue: self.blue * scalar,
        }
    }
}

impl Mul<Color> for Color {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Color {
            red: self.red * other.red,
            green: self.green * other.green,
            blue: self.blue * other.blue,
        }
    }
}

// CANVAS

struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Vec<Color>>,
}

impl Canvas {
    fn new(width: usize, height: usize) -> Self {
        // TODO: Replace constants with width and height args
        const W: usize = 10;
        const H: usize = 20;

        let color: Color = Color::new(0.0, 0.0, 0.0);
        let pixels: Vec<Vec<Color>> = vec![vec![color; W]; H];

        Canvas {
            width,
            height,
            pixels: pixels,
        }
    }

    fn pixel_at(canvas: Self, x: usize, y: usize) -> Color {
        let pixels: Vec<Vec<Color>> = canvas.pixels;

        pixels[x][y]
    }

    fn write_pixel(canvas: Self, x: usize, y: usize, color: Color) -> Canvas {
        let mut pixels: Vec<Vec<Color>> = canvas.pixels;

        pixels[x][y] = color;

        Canvas {
            width: canvas.width,
            height: canvas.height,
            pixels: pixels,
        }
    }

    fn canvas_to_ppm(canvas: Self) -> String {
        // TODO: Convert canvas.pixels into integers.
        // TODO: Add integers to string.
        let maximum_color_value: i32 = 255;

        format!(
            "P3\n{} {}\n{}\n",
            canvas.width, canvas.height, maximum_color_value
        )
    }

    fn write_ppm_to_file(ppm: String) {
        fs::write("./assets/canvas.ppm", ppm).expect("Unable to write file");
    }
}

// TESTS

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tuple_constructor_for_points() {
        let tuple: Tuple = Tuple::new(4.3, -4.2, 3.1, 1.0);
        assert_eq!(tuple.x, 4.3);
        assert_eq!(tuple.y, -4.2);
        assert_eq!(tuple.z, 3.1);
        assert_eq!(tuple.w, 1.0);

        let is_point: bool = Tuple::is_point(tuple);
        assert!(is_point);

        let is_vector: bool = Tuple::is_vector(tuple);
        assert!(!is_vector);
    }

    #[test]
    fn test_tuple_constructor_for_vectors() {
        let tuple: Tuple = Tuple::new(4.3, -4.2, 3.1, 0.0);
        assert_eq!(tuple.x, 4.3);
        assert_eq!(tuple.y, -4.2);
        assert_eq!(tuple.z, 3.1);
        assert_eq!(tuple.w, 0.0);

        let is_point: bool = Tuple::is_point(tuple);
        assert!(!is_point);

        let is_vector: bool = Tuple::is_vector(tuple);
        assert!(is_vector);
    }

    #[test]
    fn test_point_creates_a_point() {
        let tuple: Tuple = Tuple::point(4.0, -4.0, 3.0);
        assert_eq!(tuple.x, 4.0);
        assert_eq!(tuple.y, -4.0);
        assert_eq!(tuple.z, 3.0);
        assert_eq!(tuple.w, 1.0);

        let is_point: bool = Tuple::is_point(tuple);
        assert!(is_point);
    }

    #[test]
    fn test_vector_creates_a_vector() {
        let tuple: Tuple = Tuple::vector(4.0, -4.0, 3.0);
        assert_eq!(tuple.x, 4.0);
        assert_eq!(tuple.y, -4.0);
        assert_eq!(tuple.z, 3.0);
        assert_eq!(tuple.w, 0.00);

        let is_vector: bool = Tuple::is_vector(tuple);
        assert!(is_vector);
    }

    #[test]
    fn test_add_tuples() {
        let tuple1: Tuple = Tuple::new(3.0, -2.0, 5.0, 1.0);
        let tuple2: Tuple = Tuple::new(-2.0, 3.0, 1.0, 0.0);
        let result: Tuple = tuple1 + tuple2;

        assert_eq!(result.x, 1.0);
        assert_eq!(result.y, 1.0);
        assert_eq!(result.z, 6.0);
        assert_eq!(result.w, 1.0);
    }

    #[test]
    fn test_subtract_point_from_point() {
        let point1: Tuple = Tuple::point(3.0, 2.0, 1.0);
        let point2: Tuple = Tuple::point(5.0, 6.0, 7.0);
        let result: Tuple = point1 - point2;

        assert_eq!(result.x, -2.0);
        assert_eq!(result.y, -4.0);
        assert_eq!(result.z, -6.0);
        assert_eq!(result.w, 0.0);
    }

    #[test]
    fn test_subtract_vector_from_point() {
        let point: Tuple = Tuple::point(3.0, 2.0, 1.0);
        let vector: Tuple = Tuple::vector(5.0, 6.0, 7.0);
        let result: Tuple = point - vector;

        assert_eq!(result.x, -2.0);
        assert_eq!(result.y, -4.0);
        assert_eq!(result.z, -6.0);
        assert_eq!(result.w, 1.0);
    }

    #[test]
    fn test_subtract_vector_from_vector() {
        let vector1: Tuple = Tuple::vector(3.0, 2.0, 1.0);
        let vector2: Tuple = Tuple::vector(5.0, 6.0, 7.0);
        let result: Tuple = vector1 - vector2;

        assert_eq!(result.x, -2.0);
        assert_eq!(result.y, -4.0);
        assert_eq!(result.z, -6.0);
        assert_eq!(result.w, 0.0);
    }

    #[test]
    fn test_subtract_vector_from_zero_vector() {
        let vector1: Tuple = Tuple::vector(0.0, 0.0, 0.0);
        let vector2: Tuple = Tuple::vector(1.0, -2.0, 3.0);
        let result: Tuple = vector1 - vector2;

        assert_eq!(result.x, -1.0);
        assert_eq!(result.y, 2.0);
        assert_eq!(result.z, -3.0);
        assert_eq!(result.w, 0.0);
    }

    #[test]
    fn test_negate_tuple() {
        let tuple: Tuple = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let result: Tuple = -tuple;

        assert_eq!(result.x, -1.0);
        assert_eq!(result.y, 2.0);
        assert_eq!(result.z, -3.0);
        assert_eq!(result.w, 4.0);
    }

    #[test]
    fn test_multiply_tuple_by_scalar() {
        let tuple: Tuple = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let result: Tuple = tuple * 3.5;

        assert_eq!(result.x, 3.5);
        assert_eq!(result.y, -7.0);
        assert_eq!(result.z, 10.5);
        assert_eq!(result.w, -14.0);
    }

    #[test]
    fn test_divide_tuple_by_scalar() {
        let tuple: Tuple = Tuple::new(1.0, -2.0, 3.0, -4.0);
        let result: Tuple = tuple / 2.0;

        assert_eq!(result.x, 0.5);
        assert_eq!(result.y, -1.0);
        assert_eq!(result.z, 1.5);
        assert_eq!(result.w, -2.0);
    }

    #[test]
    fn test_vector_magnitude() {
        let vector: Tuple = Tuple::vector(1.0, 0.0, 0.0);
        let result: f32 = Tuple::magnitude(vector);

        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_vector_magnitude_pythagoras() {
        let vector: Tuple = Tuple::vector(1.0, 2.0, 3.0);

        let expected: f32 = (14.0 as f32).sqrt();
        let actual: f32 = Tuple::magnitude(vector);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_normalize_simple_vector() {
        let vector: Tuple = Tuple::vector(4.0, 0.0, 0.0);
        let result: Tuple = Tuple::normalize(vector);

        assert_eq!(result.x, 1.0);
        assert_eq!(result.y, 0.0);
        assert_eq!(result.z, 0.0);
        assert_eq!(result.w, 0.0);
    }

    #[test]
    fn test_normalize_vector() {
        let vector: Tuple = Tuple::vector(1.0, 2.0, 3.0);
        let result: Tuple = Tuple::normalize(vector);

        assert_eq!(result.x, 1.0 / (14.0 as f32).sqrt());
        assert_eq!(result.y, 2.0 / (14.0 as f32).sqrt());
        assert_eq!(result.z, 3.0 / (14.0 as f32).sqrt());
        assert_eq!(result.w, 0.0);
    }

    #[test]
    fn test_magnitude_of_normalized_vector() {
        let vector: Tuple = Tuple::vector(1.0, 2.0, 3.0);
        let normalized_vector: Tuple = Tuple::normalize(vector);
        let result: f32 = Tuple::magnitude(normalized_vector);

        assert!(float_eq(result, 1.0));
    }

    #[test]
    fn test_dot_product() {
        let vector1: Tuple = Tuple::vector(1.0, 2.0, 3.0);
        let vector2: Tuple = Tuple::vector(2.0, 3.0, 4.0);
        let result: f32 = Tuple::dot(vector1, vector2);

        assert_eq!(result, 20.0);
    }

    #[test]
    fn test_cross_product() {
        let vector1: Tuple = Tuple::vector(1.0, 2.0, 3.0);
        let vector2: Tuple = Tuple::vector(2.0, 3.0, 4.0);
        let result: Tuple = Tuple::cross(vector1, vector2);

        assert_eq!(result.x, -1.0);
        assert_eq!(result.y, 2.0);
        assert_eq!(result.z, -1.0);
        assert_eq!(result.w, 0.0);
        assert!(Tuple::is_vector(result));
    }

    #[test]
    fn test_cross_product_order() {
        let vector1: Tuple = Tuple::vector(1.0, 2.0, 3.0);
        let vector2: Tuple = Tuple::vector(2.0, 3.0, 4.0);
        let result: Tuple = Tuple::cross(vector2, vector1);

        assert_eq!(result.x, 1.0);
        assert_eq!(result.y, -2.0);
        assert_eq!(result.z, 1.0);
        assert_eq!(result.w, 0.0);
        assert!(Tuple::is_vector(result));
    }

    #[test]
    fn test_projectile_constructor() {
        let position: Tuple = Tuple::point(0.0, 1.0, 0.0);
        let velocity: Tuple = Tuple::normalize(Tuple::vector(1.0, 1.0, 0.0));
        let result: Projectile = Projectile::new(position, velocity);

        assert_eq!(result.position, position);
        assert_eq!(result.velocity, velocity);
    }

    #[test]
    fn test_environment_constructor() {
        let gravity: Tuple = Tuple::vector(0.0, -0.1, 0.0);
        let wind: Tuple = Tuple::vector(-0.01, 0.0, 0.0);
        let result: Environment = Environment::new(gravity, wind);

        assert_eq!(result.gravity, gravity);
        assert_eq!(result.wind, wind);
    }

    #[test]
    fn test_environment_tick() {
        let position: Tuple = Tuple::point(0.0, 1.0, 0.0);
        let velocity: Tuple = Tuple::normalize(Tuple::vector(1.0, 1.0, 0.0));
        let projectile: Projectile = Projectile::new(position, velocity);

        let gravity: Tuple = Tuple::vector(0.0, -0.1, 0.0);
        let wind: Tuple = Tuple::vector(-0.01, 0.0, 0.0);
        let environment: Environment = Environment::new(gravity, wind);

        let updated_projectile: Projectile = Environment::tick(environment, projectile);

        assert_eq!(updated_projectile.position.x, 0.70710677);
        assert_eq!(updated_projectile.position.y, 1.7071068);
        assert_eq!(updated_projectile.velocity.x, 0.6971068);
        assert_eq!(updated_projectile.velocity.y, 0.60710675);
    }

    #[test]
    fn test_environment_tick_repeated() {
        let position: Tuple = Tuple::point(0.0, 1.0, 0.0);
        let velocity: Tuple = Tuple::normalize(Tuple::vector(1.0, 1.0, 0.0));
        let mut projectile: Projectile = Projectile::new(position, velocity);

        let gravity: Tuple = Tuple::vector(0.0, -0.1, 0.0);
        let wind: Tuple = Tuple::vector(-0.01, 0.0, 0.0);
        let environment: Environment = Environment::new(gravity, wind);

        while projectile.position.y > 0.0 {
            projectile = Environment::tick(environment, projectile);
        }

        assert!(projectile.position.y <= 0.0);
    }

    #[test]
    fn test_color_constructor() {
        let color: Color = Color::new(-0.5, 0.4, 1.7);

        assert_eq!(color.red, -0.5);
        assert_eq!(color.green, 0.4);
        assert_eq!(color.blue, 1.7);
    }

    #[test]
    fn test_color_to_int() {
        let color: Color = Color::new(0.0, 0.0, 0.0);
        let color_int: i32 = Color::to_int(color);

        assert_eq!(color_int, 0);
    }

    #[test]
    fn test_add_colors() {
        let color1: Color = Color::new(0.9, 0.6, 0.75);
        let color2: Color = Color::new(0.7, 0.1, 0.25);
        let result: Color = color1 + color2;

        assert!(float_eq(result.red, 1.6));
        assert!(float_eq(result.green, 0.7));
        assert!(float_eq(result.blue, 1.0));
    }

    #[test]
    fn test_sub_colors() {
        let color1: Color = Color::new(0.9, 0.6, 0.75);
        let color2: Color = Color::new(0.7, 0.1, 0.25);
        let result: Color = color1 - color2;

        assert!(float_eq(result.red, 0.2));
        assert!(float_eq(result.green, 0.5));
        assert!(float_eq(result.blue, 0.5));
    }

    #[test]
    fn test_multiply_color_by_scalar() {
        let color: Color = Color::new(0.9, 0.6, 0.75);
        let result: Color = color * 2.0;

        assert!(float_eq(result.red, 1.8));
        assert!(float_eq(result.green, 1.2));
        assert!(float_eq(result.blue, 1.5));
    }

    #[test]
    fn test_multiply_colors() {
        let color1: Color = Color::new(1.0, 0.2, 0.4);
        let color2: Color = Color::new(0.9, 1.0, 0.1);
        let result: Color = color1 * color2;

        assert!(float_eq(result.red, 0.9));
        assert!(float_eq(result.green, 0.2));
        assert!(float_eq(result.blue, 0.04));
    }

    #[test]
    fn test_canvas_constructor() {
        let canvas: Canvas = Canvas::new(10, 20);

        assert_eq!(canvas.width, 10);
        assert_eq!(canvas.height, 20);

        for pixel in canvas.pixels.iter().flatten() {
            assert_eq!(pixel.red, 0.0);
            assert_eq!(pixel.green, 0.0);
            assert_eq!(pixel.blue, 0.0);
        }
    }

    #[test]
    fn test_get_pixel_from_canvas() {
        let canvas: Canvas = Canvas::new(10, 20);
        let pixel: Color = Canvas::pixel_at(canvas, 2, 3);

        assert_eq!(pixel.red, 0.0);
        assert_eq!(pixel.green, 0.0);
        assert_eq!(pixel.blue, 0.0);
    }

    #[test]
    fn test_write_pixel_to_canvas() {
        let canvas: Canvas = Canvas::new(10, 20);
        let color: Color = Color::new(1.0, 0.0, 0.0);
        let updated_canvas: Canvas = Canvas::write_pixel(canvas, 2, 3, color);
        let pixel: Color = Canvas::pixel_at(updated_canvas, 2, 3);

        assert_eq!(pixel.red, 1.0);
        assert_eq!(pixel.green, 0.0);
        assert_eq!(pixel.blue, 0.0);
    }

    #[test]
    fn test_canvas_to_ppm() {
        let canvas: Canvas = Canvas::new(5, 3);
        let ppm: String = Canvas::canvas_to_ppm(canvas);
        let expected_output: String = "P3\n5 3\n255\n".to_string();

        assert_eq!(ppm, expected_output);
    }

    #[test]
    fn test_write_ppm_to_file() {
        let canvas: Canvas = Canvas::new(256, 240);
        let ppm: String = Canvas::canvas_to_ppm(canvas);

        Canvas::write_ppm_to_file(ppm);

        let file_exists: bool = Path::new("./assets/canvas.ppm").exists();
        assert!(file_exists);

        // Clean up
        fs::remove_file("./assets/canvas.ppm").expect("Failed to remove ppm file.");
    }

    // TEST HELPERS

    const EPSILON: f32 = 0.00001;

    fn float_eq(a: f32, b: f32) -> bool {
        (a - b).abs() < EPSILON
    }
}
