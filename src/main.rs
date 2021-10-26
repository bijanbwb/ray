use std::ops::{Add, Div, Mul, Neg, Sub};

// MAIN

fn main() {
    println!("Ray Tracing in Rust!");
}

// TUPLE

#[derive(Clone, Copy, Debug)]
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

        // TODO: Write EPSILON helper.
        // assert_eq!(result, 1.0);
        assert_eq!(result, 0.99999994);
    }
}
