use std::ops::{Add, Div, Mul, Sub};
use std::vec::Vec;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug, PartialEq)]
struct MyVec(Vec<u32>);

impl Add for MyVec {
    type Output = MyVec;

    fn add(self, other: MyVec) -> MyVec {
        if self.0.is_empty() {
            return other;
        }
        let mut v3: Vec<u32> = Vec::new();
        self.0.iter().for_each(|x| v3.push(*x));
        let len1 = self.0.len();
        let len2 = other.0.len();
        if len1 < len2 {
            for i in len1..len2 {
                v3.push(other.0[i]);
            }
        }
        MyVec(v3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_add() {
        assert_eq!(
            Point { x: 3, y: 3 } + Point { x: 4, y: 4 },
            Point { x: 7, y: 7 }
        );
    }

    #[test]
    fn test_vec_plus() {
        let a1: MyVec = MyVec(vec![1, 2, 3]);
        let a2: MyVec = MyVec(vec![1, 2, 4, 5, 6]);
        let a3: MyVec = a1 + a2;
        assert_eq!(a3, MyVec(vec![1, 2, 3, 5, 6]))
    }
}
