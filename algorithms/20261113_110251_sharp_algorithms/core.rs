use std::cmp::Ordering;

/// A 2‑D point with integer coordinates.
///
/// Derives `Debug`, `Clone`, and `PartialEq` for testing and debugging.
#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    pub x: i64,
    pub y: i64,

}

impl Point {
    /// Creates a new `Point`.
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

/// Returns the orientation of the ordered triplet (p, q, r).
///
/// * `0`  – colinear  
/// * `1`  – clockwise  
/// * `2`  – counter‑clockwise
pub fn orientation(p: &Point, q: &Point, r: &Point) -> i32 {
    let val = (q.y - p.y) * (r.x - q.x) - (q.x - p.x) * (r.y - q.y);
    if val == 0 {
        0
    } else if val > 0 {
        1
    } else {
        2
    }
}

/// Squared Euclidean distance between two points.
pub fn dist_sq(p1: &Point, p2: &Point) -> i64 {
    let dx = p1.x - p2.x;
    let dy = p1.y - p2.y;
    dx * dx + dy * dy
}

/// Computes the convex hull of a set of points using Graham's scan.
///
/// The returned hull is a vector of points ordered counter‑clockwise,
/// starting from the point with the lowest y‑coordinate (and lowest x on tie).
///
/// # Panics
///
/// Panics if `points` contains more than `usize::MAX` elements (practically impossible).
pub fn convex_hull(points: &[Point]) -> Vec<Point> {
    let n = points.len();
    if n < 3 {
        // The hull of fewer than three points is the set itself (deduplicated).
        let mut uniq = points.to_vec();
        uniq.sort_by(|a, b| {
            match a.x.cmp(&b.x) {
                Ordering::Equal => a.y.cmp(&b.y),
                other => other,
            }
        });
        uniq.dedup();
        return uniq;
    }

    // 1. Find the pivot point with the lowest y (and lowest x on tie).
    let mut pivot_idx = 0usize;
    for (i, pt) in points.iter().enumerate().skip(1) {
        if pt.y < points[pivot_idx].y
            || (pt.y == points[pivot_idx].y && pt.x < points[pivot_idx].x)
        {
            pivot_idx = i;
        }
    }

    // 2. Clone points and move pivot to the front.
    let mut pts = points.to_vec();
    pts.swap(0, pivot_idx);
    let pivot = pts[0].clone();

    // 3. Sort the remaining points by polar angle w.r.t. pivot.
    pts[1..].sort_by(|a, b| {
        let o = orientation(&pivot, a, b);
        if o == 0 {
            // Colinear points: keep the farther one first.
            let d1 = dist_sq(&pivot, a);
            let d2 = dist_sq(&pivot, b);
            d2.cmp(&d1)
        } else if o == 2 {
            Ordering::Less // counter‑clockwise -> a before b
        } else {
            Ordering::Greater // clockwise -> b before a
        }
    });

    // 4. Remove points that are colinear with the pivot and closer than a later point.
    let mut filtered: Vec<Point> = Vec::with_capacity(pts.len());
    filtered.push(pivot.clone());
    for pt in pts.iter().skip(1) {
        while filtered.len() >= 2
            && orientation(&filtered[filtered.len() - 2], &filtered[filtered.len() - 1], pt) == 0
        {
            // Keep the farthest point (the later one) by discarding the previous.
            filtered.pop();
        }
        filtered.push(pt.clone());
    }

    // 5. Graham scan using a stack.
    let mut hull: Vec<Point> = Vec::with_capacity(filtered.len());
    hull.push(filtered[0].clone());
    hull.push(filtered[1].clone());
    hull.push(filtered[2].clone());

    for pt in filtered.iter().skip(3) {
        while hull.len() >= 2
            && orientation(&hull[hull.len() - 2], &hull[hull.len() - 1], pt) != 2
        {
            hull.pop();
        }
        hull.push(pt.clone());
    }

    hull
}