use std::{cmp::Reverse, collections::BinaryHeap};

struct MedianOfAges {
    smaller_half: BinaryHeap<usize>,
    bigger_half: BinaryHeap<Reverse<usize>>,
}

impl MedianOfAges {
    fn new() -> Self {
        Self {
            smaller_half: BinaryHeap::new(), // Max heap
            bigger_half: BinaryHeap::new(), // Min heap
        }
    }

    fn add_age(&mut self, age: usize) {
        if self.bigger_half.is_empty() || age > self.bigger_half.peek().unwrap().0 {
            self.bigger_half.push(Reverse(age));
        } else {
            self.smaller_half.push(age);
        }

        if self.bigger_half.len() >= self.smaller_half.len() + 1 {
            self.smaller_half.push(self.bigger_half.pop().unwrap().0);
        } else if self.bigger_half.len() + 1 <= self.smaller_half.len() {
            self.bigger_half.push(Reverse(self.smaller_half.pop().unwrap()));
        }
    }

    fn get_median(&self) -> f64 {
        println!("Smaller: {:?}", self.smaller_half);
        println!("Bigger: {:?}", self.bigger_half);
        if self.smaller_half.len() == self.bigger_half.len() {
            (*self.smaller_half.peek().unwrap() as f64 + self.bigger_half.peek().unwrap().0 as f64) * 0.5
        } else if self.smaller_half.len() > self.bigger_half.len() {
            *self.smaller_half.peek().unwrap() as f64
        } else {
            self.bigger_half.peek().unwrap().0 as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_median_age_0() {
        let mut median = MedianOfAges::new();
        median.add_age(22);
        median.add_age(35);
        assert_eq!(median.get_median(), 28.5);
        median.add_age(30);
        assert_eq!(median.get_median(), 30.0);
        median.add_age(25);
        assert_eq!(median.get_median(), 27.5);
    }
}
