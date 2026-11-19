pub mod engine {
    use crate::types::types::{KdNode, KdTree, Point};
    use std::cmp::Ordering;

    impl KdTree {
        /// Build a K‑d tree from a vector of points.
        /// `k` is the dimensionality and must match each point's coordinate length.
        pub fn from_points(mut points: Vec<Point>, k: usize) -> Self {
            assert!(k > 0, "Dimensionality k must be positive");
            // Validate dimensionality
            for p in &points {
                assert_eq!(
                    p.coords.len(),
                    k,
                    "All points must have exactly k coordinates"
                );
            }
            let root = build_subtree(&mut points[..], 0, k);
            KdTree { k, root }
        }

        /// Return the nearest neighbor to `target` using Euclidean distance.
        /// If the tree is empty, `None` is returned.
        pub fn nearest(&self, target: &Point) -> Option<Point> {
            if self.root.is_none() {
                return None;
            }
            let mut best: Option<(Point, f64)> = None;
            search(&self.root, target, 0, self.k, &mut best);
            best.map(|(p, _)| p)
        }
    }

    fn build_subtree(points: &mut [Point], depth: usize, k: usize) -> Option<Box<KdNode>> {
        if points.is_empty() {
            return None;
        }
        let axis = depth % k;
        // Sort by the current axis
        points.sort_by(|a, b| {
            a.coords[axis]
                .partial_cmp(&b.coords[axis])
                .unwrap_or(Ordering::Equal)
        });
        let median = points.len() / 2;
        let median_point = points[median].clone();

        let left = build_subtree(&mut points[..median], depth + 1, k);
        let right = build_subtree(&mut points[median + 1..], depth + 1, k);

        Some(Box::new(KdNode {
            point: median_point,
            left,
            right,
        }))
    }

    fn search(
        node: &Option<Box<KdNode>>,
        target: &Point,
        depth: usize,
        k: usize,
        best: &mut Option<(Point, f64)>,
    ) {
        let node_ref = match node {
            Some(n) => n,
            None => return,
        };

        let dist_sq = node_ref.point.distance_squared(target);
        if best
            .as_ref()
            .map_or(true, |(_, best_dist_sq)| dist_sq < *best_dist_sq)
        {
            *best = Some((node_ref.point.clone(), dist_sq));
        }

        let axis = depth % k;
        let target_coord = target.coords[axis];
        let node_coord = node_ref.point.coords[axis];

        let (first, second) = if target_coord < node_coord {
            (&node_ref.left, &node_ref.right)
        } else {
            (&node_ref.right, &node_ref.left)
        };

        // Explore the side that target lies in first
        search(first, target, depth + 1, k, best);

        // Determine whether we need to explore the opposite side
        let diff = target_coord - node_coord;
        if diff * diff < best.as_ref().map_or(f64::INFINITY, |(_, d)| *d) {
            search(second, target, depth + 1, k, best);
        }
    }
}