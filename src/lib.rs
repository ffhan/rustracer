extern crate image;

pub mod base;
pub mod scene;
pub mod vector;
pub mod objects;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vector::Vector;

    #[test]
    fn test_add() {
        let v1 = vector::Vector::new(1.0, 2.0, 0.0);
        let v2 = vector::Vector::new(-1.0, -1.0, 2.0);

        assert_eq!(v1 + v2, Vector::new(0.0, 1.0, 2.0));
    }

    #[test]
    fn test_neg() {
        let v = vector::Vector::new(-1.0, 2.0, -3.0);
        let expected = vector::Vector::new(1.0, -2.0, 3.0);
        assert_eq!(-v, expected);
    }

    #[test]
    fn test_dot() {
        let v = vector::Vector::new(1.0, 2.0, 3.0);
        let v2 = vector::Vector::new(-1.0, 0.5, -1.0);

        assert_eq!(v.dot(&v2), -3.0_f64);
    }

    #[test]
    fn test_cross() {
        let v1 = vector::Vector::new(1.0, -1.0, 2.0);
        let v2 = vector::Vector::new(-1.0, 1.0, -1.0);

        assert_eq!(v1.cross(&v2), vector::Vector::new(-1.0, -1.0, 0.0));
    }

    #[test]
    fn test_distance() {
        let v1 = vector::Vector::new(1.0, 2.0, 3.0);
        assert_eq!(v1.euclidian_distance(), 14.0_f64.sqrt());
    }

    #[test]
    fn test_normalize() {
        let v1 = vector::Vector::new(1.0, 2.0, 5.0);
        assert_eq!(v1.normalize(), vector::Vector::new(1.0_f64 / 30.0_f64.sqrt(), (2.0_f64 / 15.0).sqrt(), (5.0_f64 / 6.0).sqrt()));
    }
}
