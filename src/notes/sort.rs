
pub fn simple_bubble(arr: Vec<i32>) ->Vec<i32> {
    let mut a = arr.clone();
    // a.sort();
    for i in 0..a.len() {
        for j in 0..a.len() {
            
        }
    }

    println!("{:?}", a);
    return a;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_bubble() {
        let arr1 = vec![11, 2, 34, 29, 8, 19];
        assert_eq!(
            vec![2, 8, 11, 19, 29, 34],
            simple_bubble(arr1)
        )
    }
}

