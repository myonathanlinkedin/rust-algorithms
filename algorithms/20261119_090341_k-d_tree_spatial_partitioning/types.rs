pub mod types {
    #[derive(Debug, Clone, PartialEq)]
    pub struct Point {
        pub coords: Vec<f64>,

    }

    impl Point {
        pub fn new(coords: Vec<f64>) -> Self {
            Self { coords }
        }

        pub fn distance_squared(&self, other: &Self) -> f64 {
            self.coords
                .iter()
                .zip(other.coords.iter())
                .map(|(a, b)| (a - b) * (a - b))
                .sum()
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct KdNode {
        pub point: Point,
        pub left: Option<Box<KdNode>>,
        pub right: Option<Box<KdNode>>,

    }

    impl KdNode {
        pub fn new(point: Point) -> Self {
            Self {
                point,
                left: None,
                right: None,
            }
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct KdTree {
        pub k: usize,
        pub root: Option<Box<KdNode>>,

    }

    impl KdTree {
        pub fn new(k: usize) -> Self {
            Self { k, root: None }
        }
    }
}