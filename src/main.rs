// MAIN

fn main() {
    println!("Ray Tracing in Rust!");
}

// TUPLE

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

    fn is_point(self) -> bool {
        self.w == 1.0
    }

    fn is_vector(self) -> bool {
        self.w == 0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tuple_constructor_works() {
        let tuple = Tuple::new(1.0, 2.0, 3.0, 1.0);

        assert_eq!(tuple.x, 1.0);
        assert_eq!(tuple.y, 2.0);
        assert_eq!(tuple.z, 3.0);
        assert_eq!(tuple.w, 1.0);
    }

    #[test]
    fn test_is_point_is_true_when_w_equals_one() {
        let tuple: Tuple = Tuple::new(1.0, 2.0, 3.0, 1.0);
        let boolean: bool = Tuple::is_point(tuple);

        assert!(boolean);
    }

    #[test]
    fn test_is_point_is_false_when_w_equals_zero() {
        let tuple: Tuple = Tuple::new(1.0, 2.0, 3.0, 0.0);
        let boolean: bool = Tuple::is_point(tuple);

        assert!(!boolean);
    }

    #[test]
    fn test_is_vector_is_true_when_w_equals_zero() {
        let tuple: Tuple = Tuple::new(1.0, 2.0, 3.0, 0.0);
        let boolean: bool = Tuple::is_vector(tuple);

        assert!(boolean);
    }

    #[test]
    fn test_is_vector_is_false_when_w_equals_one() {
        let tuple: Tuple = Tuple::new(1.0, 2.0, 3.0, 1.0);
        let boolean: bool = Tuple::is_vector(tuple);

        assert!(!boolean);
    }
}
