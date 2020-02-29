
pub fn test_func() {
    let content: Box<String> = Box::new(String::from("this is content"));
    println!("{}", *content);
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
        Point2 {
            x_value: self.x_value,
            y_value: self.y_value,
        }
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

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct StuResult {
    name: String,
    age: u8,
    score: u16,
}

/// struct sort
pub fn sort_for_struct() {
    let stu1 = StuResult {
        name: "王双喜".to_string(),
        age: 22,
        score: 89,
    };
    let stu3 = StuResult {
        name: "苏杰".to_string(),
        age: 21,
        score: 97,
    };
    let stu2 = StuResult {
        name: "李聪怡".to_string(),
        age: 23,
        score: 87,
    };
    let mut stus: Vec<StuResult> = vec!(stu1, stu2, stu3);
    println!("{:#?}", stus);
    stus.sort();
    println!("{:#?}", stus);
}


