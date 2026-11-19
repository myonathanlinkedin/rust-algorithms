mod types;
mod engine;

use types::types::{KdTree, Point};

fn main() {
    // Helper to create a point from a slice
    fn pt(coords: &[f64]) -> Point {
        Point::new(coords.to_vec())
    }

    // Test 1: Empty tree
    let empty_tree = KdTree::from_points(Vec::new(), 2);
    assert!(empty_tree.nearest(&pt(&[0.0, 0.0])).is_none());

    // Test 2: Single point
    let single = KdTree::from_points(vec![pt(&[1.0, 1.0])], 2);
    let nn = single.nearest(&pt(&[2.0, 2.0])).expect("should find a neighbor");
    assert_eq!(nn, pt(&[1.0, 1.0]));

    // Test 3: Classic 2‑D example
    // Points: (2,3), (5,4), (9,6), (4,7), (8,1), (7,2)
    let points = vec![
        pt(&[2.0, 3.0]),
        pt(&[5.0, 4.0]),
        pt(&[9.0, 6.0]),
        pt(&[4.0, 7.0]),
        pt(&[8.0, 1.0]),
        pt(&[7.0, 2.0]),
    ];
    let tree = KdTree::from_points(points, 2);

    // Query nearest to (9,2) -> expected (8,1)
    let query = pt(&[9.0, 2.0]);
    let result = tree.nearest(&query).expect("nearest neighbor must exist");
    assert_eq!(result, pt(&[8.0, 1.0]));

    // Query nearest to (5,5) -> expected (5,4) or (4,7) whichever is closer
    let query2 = pt(&[5.0, 5.0]);
    let result2 = tree.nearest(&query2).expect("nearest neighbor must exist");
    // Compute distances manually to verify
    let d1 = pt(&[5.0, 4.0]).distance_squared(&query2);
    let d2 = pt(&[4.0, 7.0]).distance_squared(&query2);
    let expected = if d1 <= d2 { pt(&[5.0, 4.0]) } else { pt(&[4.0, 7.0]) };
    assert_eq!(result2, expected);

    // Test 4: Higher dimensional data (3‑D)
    let points3d = vec![
        pt(&[1.0, 2.0, 3.0]),
        pt(&[4.0, 5.0, 6.0]),
        pt(&[7.0, 8.0, 9.0]),
        pt(&[2.0, 3.0, 1.0]),
    ];
    let tree3d = KdTree::from_points(points3d, 3);
    let query3 = pt(&[3.0, 3.0, 2.0]);
    let nn3 = tree3d.nearest(&query3).expect("must find neighbor");
    // Expected nearest is (2,3,1) with distance sqrt(2)
    assert_eq!(nn3, pt(&[2.0, 3.0, 1.0]));

    // All assertions passed
    println!("All K‑d tree unit tests passed.");
}