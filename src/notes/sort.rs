pub fn simple_bubble(arr: Vec<i32>) -> Vec<i32> {
    let mut a = arr.clone();
    // a.sort();
    for i in 0..a.len() {
        for j in 0..a.len() {}
    }

    println!("{:?}", a);
    return a;
}

/// 插入排序
pub fn insert(arr: &mut Vec<i32>) {
    let arr_l = arr.len();
    for i in 1..arr_l {
        let mut j = i;
        while j >= 1 {
            if arr[j] < arr[j - 1] {
                let tmp = arr[j - 1];
                arr[j - 1] = arr[j];
                arr[j] = tmp;
            } else {
                break;
            }
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_bubble() {
        let arr1 = vec![11, 2, 34, 29, 8, 19];
        assert_eq!(vec![2, 8, 11, 19, 29, 34], simple_bubble(arr1))
    }

    #[test]
    fn test_insert() {
        let mut arr1 = vec![12, 4, 3, 9];
        insert(&mut arr1);
        println!("{:?}", arr1);
        assert_eq!(arr1, vec![3, 4, 9, 12]);
    }

    #[test]
    fn test_get_a_vec() {
        let mut arr = vec![];
        for num in 0..=9 {
            arr.push(num);
        }
        println!("{:?}", arr);
        assert!(false)
    }
}
