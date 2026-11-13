mod core;
use core::{convex_hull, Point};

fn main() {
    // Simple sanity check: square points.
    let square = vec![
        Point::new(0, 0),
        Point::new(0, 1),
        Point::new(1, 1),
        Point::new(1, 0),
        Point::new(0, 0), // duplicate
    ];
    let hull = convex_hull(&square);
    // Expected hull: four corners in CCW order starting from (0,0)
    let expected = vec![
        Point::new(0, 0),
        Point::new(1, 0),
        Point::new(1, 1),
        Point::new(0, 1),
    ];
    assert_eq!(hull, expected);
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_triangle() {
        let pts = vec![
            Point::new(0, 0),
            Point::new(2, 0),
            Point::new(1, 2),
            Point::new(1, 1), // interior point
        ];
        let hull = convex_hull(&pts);
        let expected = vec![
            Point::new(0, 0),
            Point::new(2, 0),
            Point::new(1, 2),
        ];
        assert_eq!(hull, expected);
    }

    #[test]
    fn test_colinear() {
        let pts = vec![
            Point::new(0, 0),
            Point::new(1, 1),
            Point::new(2, 2),
            Point::new(3, 3),
        ];
        let hull = convex_hull(&pts);
        let expected = vec![
            Point::new(0, 0),
            Point::new(3, 3),
        ];
        assert_eq!(hull, expected);
    }

    #[test]
    fn test_duplicate_and_inside() {
        let pts = vec![
            Point::new(0, 0),
            Point::new(2, 0),
            Point::new(2, 2),
            Point::new(0, 2),
            Point::new(1, 1), // interior
            Point::new(0, 0), // duplicate
            Point::new(2, 2), // duplicate
        ];
        let hull = convex_hull(&pts);
        let expected = vec![
            Point::new(0, 0),
            Point::new(2, 0),
            Point::new(2, 2),
            Point::new(0, 2),
        ];
        assert_eq!(hull, expected);
    }

    #[test]
    fn test_two_points() {
        let pts = vec![Point::new(5, 5), Point::new(5, 5)];
        let hull = convex_hull(&pts);
        let expected = vec![Point::new(5, 5)];
        assert_eq!(hull, expected);
    }

    #[test]
    fn test_one_point() {
        let pts = vec![Point::new(-3, 7)];
        let hull = convex_hull(&pts);
        let expected = vec![Point::new(-3, 7)];
        assert_eq!(hull, expected);
    }
}