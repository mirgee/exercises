use std::{borrow::Borrow, collections::LinkedList};

fn merge_two_lists(list1: &mut LinkedList<i32>, list2: &mut LinkedList<i32>) -> LinkedList<i32>{
    let mut new_list = LinkedList::new();
    while !list1.is_empty() && !list2.is_empty() {
        if list1.front() <= list2.front() {
            new_list.push_back(list1.pop_front().unwrap().clone());
        } else {
            new_list.push_back(list2.pop_front().unwrap().clone());
        }
    }
    if !list1.is_empty() {
        new_list.append(list1);
    }
    if !list2.is_empty() {
        new_list.append(list2);
    }
    new_list
}

pub fn fetch_top_movies(lists: &mut Vec<LinkedList<i32>>) -> LinkedList<i32> {
    let mut l1 = lists.pop().unwrap();
    while !lists.is_empty() {
        let mut l2 = lists.pop().unwrap();
        l1 = merge_two_lists(&mut l1, &mut l2);
    }
    return l1 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch_top_movies_0() {
        let mut a: LinkedList<i32> = LinkedList::new();

        a.push_back(11);
        a.push_back(41);
        a.push_back(51);

        let mut b: LinkedList<i32> = LinkedList::new();
        b.push_back(21);
        b.push_back(23);
        b.push_back(42);

        let mut c: LinkedList<i32> = LinkedList::new();
        c.push_back(25);
        c.push_back(56);
        c.push_back(66);
        c.push_back(72);

        let mut list1: Vec<LinkedList<i32>> = Vec::new();
        list1.push(a);
        list1.push(b);
        list1.push(c);

        let res = fetch_top_movies(&mut list1);

        assert_eq!(LinkedList::from_iter(vec![11, 21, 23, 25, 41, 42, 51, 56, 66, 72]), res);
        println!("{:?}", res);
    }
}
