// Part A: this code WORKS (all tests pass) but it's unidiomatic.
//
// Your job: run
//
//     cd exercises && cargo clippy -p lesson-15
//
// and fix every warning that clippy reports in this file, without changing behaviour (the
// tests must still pass). Then also try `cargo clippy -p lesson-15 -- -W clippy::pedantic`
// and fix or consciously decide to ignore what it suggests.
//
// Don't silence warnings with #[allow(...)]. Fix the code. (The one exception is a
// pedantic lint you've decided you disagree with; if so, explain why in ANSWERS.md Q2.)

pub struct Inventory {
    pub items: Vec<(String, u32)>,
}

impl Inventory {
    pub fn new() -> Inventory {
        return Inventory { items: Vec::new() };
    }

    pub fn is_empty(&self) -> bool {
        if self.items.len() == 0 {
            return true;
        } else {
            return false;
        }
    }

    pub fn add(&mut self, name: &String, qty: u32) {
        let name = name.clone();
        let mut found = false;
        for i in 0..self.items.len() {
            if self.items[i].0 == name {
                self.items[i].1 = self.items[i].1 + qty;
                found = true;
            }
        }
        if found == false {
            self.items.push((name.clone(), qty));
        }
    }

    pub fn quantity(&self, name: &str) -> u32 {
        let found = self.items.iter().find(|(n, _)| n == name);
        match found {
            Some(item) => item.1,
            None => 0,
        }
    }
}

pub fn count_in_range(values: &Vec<i32>, low: i32, high: i32) -> usize {
    let mut count = 0;
    for v in values.iter() {
        if *v >= low && *v <= high {
            count += 1;
        }
    }
    count
}

pub fn first_part(s: &str) -> String {
    let parts: Vec<&str> = s.split(",").collect();
    if parts.len() > 0 {
        if parts[0].len() > 0 {
            return parts[0].to_string();
        }
    }
    String::from("")
}

pub fn total_length(words: &[String]) -> usize {
    words.iter().map(|w| w.len()).fold(0, |acc, n| acc + n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inventory_works() {
        let mut inv = Inventory::new();
        assert!(inv.is_empty());
        inv.add(&"apple".to_string(), 2);
        inv.add(&"apple".to_string(), 3);
        inv.add(&"pear".to_string(), 1);
        assert!(!inv.is_empty());
        assert_eq!(inv.quantity("apple"), 5);
        assert_eq!(inv.quantity("pear"), 1);
        assert_eq!(inv.quantity("kiwi"), 0);
        assert_eq!(inv.items.len(), 2);
    }

    #[test]
    fn helpers_work() {
        assert_eq!(count_in_range(&vec![1, 5, 10, 15], 5, 10), 2);
        assert_eq!(first_part("a,b,c"), "a");
        assert_eq!(first_part(",b"), "");
        assert_eq!(first_part(""), "");
        assert_eq!(total_length(&["ab".to_string(), "cde".to_string()]), 5);
    }
}
