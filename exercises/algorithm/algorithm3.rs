/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/
use std::fmt::{Debug};
use std::cmp;

fn sort<T: Ord + Debug>(array: &mut [T]){
	//TODO
    let len = array.len();
    let mut check = 0;
    println!("{}:{}<->{}:{:?}", check, 0, 0, array);
    while check < len {
        let mut first_swap = len;
        for i in check..array.len()-1 {
            if array[i] > array[i+1] {
                array.swap(i, i + 1);
                first_swap = cmp::min(first_swap, cmp::max(1, i)-1);
                println!("{}:{}<->{}:{:?}", first_swap, i, i+1, array);
            }
        }
        check = first_swap;    
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
        //assert!(false);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}