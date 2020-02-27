pub fn test_func() {
    println!("this is mod of copy-trait");
}

#[derive(Debug)]
struct Point {
    x_value: i32,
    y_value: i32,
}

pub fn no_copy() {
    let v1 = Point {x_value: 1, y_value: 2};
    let v2 = v1;
    // println!("{:?}", v1);
}

#[derive(Debug)]
struct Point2 {
    x_value: i32,
    y_value: i32,
}

impl Copy for Point2 {
   
}

impl Clone for Point2{
    fn clone(&self) -> Point2 {
        *self
    }
}

pub fn has_copy() {
    let v1 = Point2 {
        x_value: 1,
        y_value: 2,
    };
    let mut v2 = v1;
    println!("{:?}", v1);
    v2.x_value += 1;
    println!("{:?}", v1);
    println!("{:?}", v2);
}
