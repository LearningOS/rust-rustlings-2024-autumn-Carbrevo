/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: None,
        }
    }
}
#[derive(Debug)]
struct LinkedList<'a, T: std::cmp::PartialOrd + 'a> 
where &'a T: std::ops::Deref {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
    phantom: core::marker::PhantomData<&'a T>,    
}

impl<'a, T: std::cmp::PartialOrd> Default for LinkedList<'a, T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T: std::cmp::PartialOrd> LinkedList<'a, T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
            phantom: core::marker::PhantomData,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(mut end_ptr) => unsafe { end_ptr.as_mut().next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(next_ptr.as_ref().val) }),
                _ => self.get_ith_node(unsafe { next_ptr.as_ref().next }, index - 1),
            },
        }
    }

    pub fn concat(mut list_a:LinkedList<'a, T>, list_b:LinkedList<'a, T>) -> Self
    {
        if list_b.length <= 0 {
            return list_a;
        }

        if list_a.length <= 0 {
            return list_b;
        }

        let itemend_a = list_a.end;
        let itemstart_b = list_b.start;
        assert!(itemend_a.is_some());
        assert!(itemstart_b.is_some());

        unsafe { itemend_a.unwrap().as_mut().next = itemstart_b };
        list_a.end = list_b.end;
        list_a.length += list_b.length;
        list_a
    }

	pub fn merge(mut list_a:LinkedList<'a, T>, list_b:LinkedList<'a, T>) -> Self
	{
		//TODO
        let mut prvnext_a = &mut list_a.start;
        let mut item_a = *prvnext_a;
        let mut item_b = list_b.start;

        while item_b.is_some() {
            let mut value_b = unsafe { &item_b.unwrap().as_ref().val };

            if item_a.is_none() {
                *prvnext_a = item_b;
                list_a.end = list_b.end;
                break;
            }

            let mut value_a = unsafe { &item_a.unwrap().as_ref().val };
            if value_a > value_b {
                let itemstart_b = item_b;
                let mut itemend_b = item_b;
                item_b = unsafe { item_b.unwrap().as_ref().next };
                while item_b.is_some() {
                    value_b = unsafe { &item_b.unwrap().as_ref().val };
    
                    if value_a < value_b {
                        break;
                    }
                    itemend_b = item_b;
                    item_b = unsafe { item_b.unwrap().as_ref().next };
                }
                *prvnext_a = itemstart_b;
                unsafe { itemend_b.unwrap().as_mut().next = item_a };
                prvnext_a = unsafe { &mut item_a.unwrap().as_mut().next };
                item_a = unsafe { item_a.unwrap().as_ref().next };
            } else {
                prvnext_a = unsafe { &mut item_a.unwrap().as_mut().next };
                item_a = unsafe { item_a.unwrap().as_ref().next };
                while item_a.is_some() {
                    value_a = unsafe { &item_a.unwrap().as_ref().val };
    
                    if value_a > value_b {
                        break;
                    }
                    prvnext_a = unsafe { &mut item_a.unwrap().as_mut().next };
                    item_a = unsafe { item_a.unwrap().as_ref().next };
                }
            }
        }
        list_a
    }
}

impl<'a, T: std::cmp::PartialOrd> Display for LinkedList<'a, T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![1,3,5,7];
		let vec_b = vec![2,4,6,8];
		let target_vec = vec![1,2,3,4,5,6,7,8];
		
		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
	#[test]
	fn test_merge_linked_list_2() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![11,33,44,88,89,90,100];
		let vec_b = vec![1,22,30,45];
		let target_vec = vec![1,11,22,30,33,44,45,88,89,90,100];

		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
	#[test]
	fn test_merge_linked_list_empty() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let target_vec = vec![1,11,22,30,33,44,45,88,89,90,100];

        println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
	}

	//#[test]
	fn test_merge_linked_list_solo() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![11,33,44,88,89,90,100];
		let vec_b = vec![1,22,30,45];
		let target_vec = vec![1,11,22,30,33,44,45,88,89,90,100];

		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			//list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..vec_a.len(){
			assert_eq!(vec_a[i],*list_c.get(i as i32).unwrap());
		}
    }

	//#[test]
	fn test_merge_linked_list_1X1() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![11];
		let vec_b = vec![22];
		let target_vec = vec![11,22,33];

		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			//assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
        assert!(false);
    }

	#[test]
	fn test_concat_linked_list_1X1() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
        let mut list_c = LinkedList::<i32>::new();
		let vec_a = vec![11];
		let vec_b = vec![22];
		let target_vec = vec![11,22];

		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let list_c = LinkedList::<i32>::concat(list_a,list_b);
		println!("merged List is {}", list_c);

        assert_eq!(list_c.length as usize, target_vec.len());
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
        //assert!(false);
    }
}