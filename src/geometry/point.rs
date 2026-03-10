use nalgebra::DVector;

/// n-dimensional space e ekta point
#[derive(Debug, Clone)]
pub struct Point {
    pub coords: DVector<f64>,
}

impl Point {
    /// Vec<f64> theke Point banao
    pub fn new(coords: Vec<f64>) -> Self {
        Point {
            coords: DVector::from_vec(coords),
        }
    }

    /// Dimension koto?
    pub fn dim(&self) -> usize {
        self.coords.len()
    }

    /// Origin theke distance: sqrt(x1² + x2² + ...)
    pub fn norm(&self) -> f64 {
        self.coords.norm()
    }

    /// Dot product: x1*y1 + x2*y2 + ...
    pub fn dot(&self, other: &Point) -> f64 {
        self.coords.dot(&other.coords)
    }
}

// point_a + point_b
impl std::ops::Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point { coords: self.coords + other.coords }
    }
}

// point + &other
impl std::ops::Add<&Point> for Point {
    type Output = Point;
    fn add(self, other: &Point) -> Point {
        Point { coords: self.coords + &other.coords }
    }
}

// point * 2.5
impl std::ops::Mul<f64> for Point {
    type Output = Point;
    fn mul(self, scalar: f64) -> Point {
        Point { coords: self.coords * scalar }
    }
}

// point_a - point_b
impl std::ops::Sub for Point {
    type Output = Point;
    fn sub(self, other: Point) -> Point {
        Point { coords: self.coords - other.coords }
    }
}
