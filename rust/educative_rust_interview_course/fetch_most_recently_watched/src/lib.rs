use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub struct Node {
    key: usize,
    value: usize,
    prev_node: Option<Rc<RefCell<Node>>>,
    next_node: Option<Rc<RefCell<Node>>>,
}

#[derive(Default)]
pub struct LinkedList {
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
}

impl LinkedList {
    fn new() -> LinkedList {
        Default::default()
    }

    fn move_to_tail(&mut self, node: &Rc<RefCell<Node>>) {
        let prev = node.borrow().prev_node.clone();
        let next = node.borrow().next_node.clone();

        match (prev, next) {
            (None, None) => {
                // Singular node
                // NOOP
            }
            (None, Some(next)) => {
                // Node is at the end now
                node.borrow_mut().next_node = None;
                // The node after node moves to beginning
                next.borrow_mut().prev_node = None;
                self.head = Some(next.clone());

                // Previous tail point to node now, new tail is node
                let prev_tail = self.tail.as_ref().unwrap();
                node.borrow_mut().prev_node = Some(prev_tail.clone());
                prev_tail.borrow_mut().next_node = Some(node.clone());
                self.tail = Some(node.clone());
            }
            (Some(_), None) => {
                // Already a tail
                // NOOP
            }
            (Some(prev), Some(next)) => {
                // Remove node from the chain
                prev.borrow_mut().next_node = node.borrow().next_node.clone();
                next.borrow_mut().prev_node = node.borrow().prev_node.clone();

                // Set successor to current tail to be node
                let prev_tail = self.tail.as_ref().unwrap();
                prev_tail.borrow_mut().next_node = Some(node.clone());

                // Set correct neighbors for node
                node.borrow_mut().next_node = None;
                node.borrow_mut().prev_node = Some(prev_tail.clone());

                // Set new tail
                self.tail = Some(node.clone());
            }
        }
    }

    fn push_back(&mut self, node: &Rc<RefCell<Node>>) {
        match &self.tail {
            Some(prev_tail) => {
                node.borrow_mut().prev_node = Some(prev_tail.clone());
                prev_tail.borrow_mut().next_node = Some(node.clone());
                self.tail = Some(node.clone());
            }
            None => {
                self.tail = Some(node.clone());
                self.head = Some(node.clone());
            }
        }
    }

    fn remove_head(&mut self) -> Option<Rc<RefCell<Node>>> {
        if let Some(head) = self.head.clone() {
            if let Some(head_next) = head.borrow().next_node.as_ref() {
                head_next.borrow_mut().prev_node = None;
                self.head = Some(head_next.clone());
            } else {
                self.head = None;
            }
            Some(head.clone())
        } else {
            None
        }
    }
}

#[derive(Default)]
pub struct LRUCache {
    map: HashMap<usize, Rc<RefCell<Node>>>,
    cache_vals: LinkedList,
    size: usize,
    capacity: usize,
}

impl LRUCache {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            ..Default::default()
        }
    }

    pub fn set(&mut self, key: usize, value: usize) {
        if let Some(node) = self.map.get(&key) {
            self.cache_vals.move_to_tail(node);
            node.borrow_mut().value = value;
        } else {
            if self.size >= self.capacity {
                if let Some(node) = self.cache_vals.remove_head() {
                    self.map.remove(&node.borrow().key);
                };
            }
            let node = Rc::new(RefCell::new(Node {
                key,
                value,
                prev_node: None,
                next_node: None,
            }));
            self.cache_vals.push_back(&node);
            self.map.insert(key, node);
            self.size += 1;
        }
    }

    pub fn get(&mut self, key: usize) -> usize {
        if let Some(node) = self.map.get(&key) {
            self.cache_vals.move_to_tail(node);
            node.borrow().value
        } else {
            0
        }
    }

    pub fn print(&self) {
        let mut head = self.cache_vals.head.clone();
        while head.is_some() {
            let head_local = head.clone().unwrap();
            let head_local = head_local.borrow();
            print!("({}, {})", head_local.key, head_local.value);
            head = head_local.next_node.clone();
        }
        println!("");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch_most_recently_watched_0() {
        let mut cache = LRUCache::new(3);

        cache.set(10, 20);
        cache.print();
        cache.set(15, 25);
        cache.print();
        cache.set(20, 30);
        cache.print();
        cache.set(25, 35);
        cache.print();
        cache.set(5, 40);
        cache.print();
        cache.get(25);
        cache.print();
    }
}
