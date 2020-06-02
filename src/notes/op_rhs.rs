use std::ops::{Add, Sub, Mul, Div};

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {x: self.x + other.x, y: self.y + other.y}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_add() {
        assert_eq!(Point {x: 3, y: 3} + Point {x: 4, y: 4}, Point {x: 7, y: 7});
    }
}