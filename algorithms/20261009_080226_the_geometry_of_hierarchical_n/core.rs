use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,

}

#[derive(Debug, Clone, PartialEq)]
pub struct BoundingBox {
    pub min_x: f64,
    pub min_y: f64,
    pub max_x: f64,
    pub max_y: f64,

}

impl BoundingBox {
    pub fn contains(&self, p: &Point) -> bool {
        p.x >= self.min_x && p.x <= self.max_x && p.y >= self.min_y && p.y <= self.max_y
    }

    pub fn intersects(&self, other: &BoundingBox) -> bool {
        !(self.max_x < other.min_x
            || self.min_x > other.max_x
            || self.max_y < other.min_y
            || self.min_y > other.max_y)
    }
}

pub struct Node {
    pub bbox: BoundingBox,
    pub points: Vec<Point>,
    pub children: Option<[Box<Node>; 4]>,

}

impl Node {
    fn new(bbox: BoundingBox) -> Self {
        Node {
            bbox,
            points: Vec::new(),
            children: None,
        }
    }

    fn subdivide(&mut self) {
        let mid_x = (self.bbox.min_x + self.bbox.max_x) / 2.0;
        let mid_y = (self.bbox.min_y + self.bbox.max_y) / 2.0;

        let sw = BoundingBox {
            min_x: self.bbox.min_x,
            min_y: self.bbox.min_y,
            max_x: mid_x,
            max_y: mid_y,
        };
        let se = BoundingBox {
            min_x: mid_x,
            min_y: self.bbox.min_y,
            max_x: self.bbox.max_x,
            max_y: mid_y,
        };
        let nw = BoundingBox {
            min_x: self.bbox.min_x,
            min_y: mid_y,
            max_x: mid_x,
            max_y: self.bbox.max_y,
        };
        let ne = BoundingBox {
            min_x: mid_x,
            min_y: mid_y,
            max_x: self.bbox.max_x,
            max_y: self.bbox.max_y,
        };

        self.children = Some([
            Box::new(Node::new(sw)),
            Box::new(Node::new(se)),
            Box::new(Node::new(nw)),
            Box::new(Node::new(ne)),
        ]);
    }

    fn insert(&mut self, point: Point, capacity: usize) {
        if !self.bbox.contains(&point) {
            return;
        }

        if self.children.is_none() {
            if self.points.len() < capacity {
                self.points.push(point);
                return;
            } else {
                self.subdivide();
            }
        }

        if let Some(ref mut children) = self.children {
            let mid_x = (self.bbox.min_x + self.bbox.max_x) / 2.0;
            let mid_y = (self.bbox.min_y + self.bbox.max_y) / 2.0;
            let idx = if point.x <= mid_x {
                if point.y <= mid_y { 0 } else { 2 }
            } else {
                if point.y <= mid_y { 1 } else { 3 }
            };
            children[idx].insert(point, capacity);
        }
    }

    fn query_range(&self, query: &BoundingBox, visited: &mut usize) -> Vec<Point> {
        *visited += 1;
        let mut result = Vec::new();

        if !self.bbox.intersects(query) {
            return result;
        }

        if self.children.is_none() {
            for p in &self.points {
                if query.contains(p) {
                    result.push(p.clone());
                }
            }
        } else if let Some(ref children) = self.children {
            for child in children.iter() {
                result.extend(child.query_range(query, visited));
            }
        }

        result
    }
}

pub struct Quadtree {
    pub root: Node,
    pub capacity: usize,

}

impl Quadtree {
    pub fn new(capacity: usize, bbox: BoundingBox) -> Self {
        Quadtree {
            root: Node::new(bbox),
            capacity,
        }
    }

    pub fn insert(&mut self, point: Point) {
        self.root.insert(point, self.capacity);
    }

    pub fn query_range(&self, query: &BoundingBox) -> (Vec<Point>, usize) {
        let mut visited = 0usize;
        let points = self.root.query_range(query, &mut visited);
        (points, visited)
    }

    pub fn is_subdivided(&self) -> bool {
        self.root.children.is_some()
    }
}