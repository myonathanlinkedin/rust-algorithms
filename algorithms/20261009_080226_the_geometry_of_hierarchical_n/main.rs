mod core;
use core::{BoundingBox, Point, Quadtree};

fn main() {
    // Simple demonstration and sanity check
    let bbox = BoundingBox {
        min_x: 0.0,
        min_y: 0.0,
        max_x: 10.0,
        max_y: 10.0,
    };
    let mut qt = Quadtree::new(4, bbox);
    qt.insert(Point { x: 1.0, y: 1.0 });
    qt.insert(Point { x: 2.0, y: 2.0 });
    qt.insert(Point { x: 3.0, y: 3.0 });
    qt.insert(Point { x: 4.0, y: 4.0 });
    qt.insert(Point { x: 5.0, y: 5.0 });

    let query = BoundingBox {
        min_x: 0.0,
        min_y: 0.0,
        max_x: 3.0,
        max_y: 3.0,
    };
    let (points, visited) = qt.query_range(&query);
    assert_eq!(points.len(), 3);
    assert!(visited > 0);
    println!("Main: Query returned {} points after visiting {} nodes.", points.len(), visited);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_query() {
        let bbox = BoundingBox {
            min_x: 0.0,
            min_y: 0.0,
            max_x: 10.0,
            max_y: 10.0,
        };
        let mut qt = Quadtree::new(4, bbox);
        qt.insert(Point { x: 1.0, y: 1.0 });
        qt.insert(Point { x: 2.0, y: 2.0 });
        qt.insert(Point { x: 3.0, y: 3.0 });
        qt.insert(Point { x: 4.0, y: 4.0 });
        qt.insert(Point { x: 5.0, y: 5.0 });

        let query = BoundingBox {
            min_x: 0.0,
            min_y: 0.0,
            max_x: 3.0,
            max_y: 3.0,
        };
        let (points, _visited) = qt.query_range(&query);
        assert_eq!(points.len(), 3);
        assert!(points.contains(&Point { x: 1.0, y: 1.0 }));
        assert!(points.contains(&Point { x: 2.0, y: 2.0 }));
        assert!(points.contains(&Point { x: 3.0, y: 3.0 }));
    }

    #[test]
    fn test_subdivision() {
        let bbox = BoundingBox {
            min_x: 0.0,
            min_y: 0.0,
            max_x: 10.0,
            max_y: 10.0,
        };
        let mut qt = Quadtree::new(4, bbox);
        for i in 0..5 {
            qt.insert(Point { x: i as f64, y: i as f64 });
        }
        assert!(qt.is_subdivided(), "Quadtree should have subdivided after exceeding capacity");
    }

    #[test]
    fn test_query_cost() {
        let bbox = BoundingBox {
            min_x: 0.0,
            min_y: 0.0,
            max_x: 10.0,
            max_y: 10.0,
        };
        let mut qt = Quadtree::new(4, bbox);
        for i in 0..10 {
            qt.insert(Point { x: i as f64, y: i as f64 });
        }
        let query = BoundingBox {
            min_x: 0.0,
            min_y: 0.0,
            max_x: 5.0,
            max_y: 5.0,
        };
        let (_points, visited) = qt.query_range(&query);
        assert!(visited > 0, "At least one node should be visited during query");
    }
}