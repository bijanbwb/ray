// MAIN

fn main() {
    println!("Ray Tracing in Rust!");
}

// TUPLE

#[derive(Clone, Copy)]
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
}
