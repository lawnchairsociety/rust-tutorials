//! Lesson 04 practice problems: structs and methods.
//!
//! The struct definitions are given. Implement every method marked `todo!()`, then run:
//!
//! ```text
//! cd exercises
//! cargo test -p lesson-04
//! ```

// ---------------------------------------------------------------------------
// Problem 1: Rectangle
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    /// Create a rectangle with the given width and height.
    pub fn new(width: u32, height: u32) -> Rectangle {
        todo!()
    }

    /// Create a square with sides of length `size`.
    pub fn square(size: u32) -> Rectangle {
        todo!()
    }

    /// Width × height.
    pub fn area(&self) -> u32 {
        todo!()
    }

    /// 2 × (width + height).
    pub fn perimeter(&self) -> u32 {
        todo!()
    }

    /// True if `other` fits strictly inside `self` (both dimensions strictly smaller).
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        todo!()
    }

    /// True if width == height.
    pub fn is_square(&self) -> bool {
        todo!()
    }
}

// ---------------------------------------------------------------------------
// Problem 2: Point (a tuple struct)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point(pub i32, pub i32);

impl Point {
    /// The point (0, 0).
    pub fn origin() -> Point {
        todo!()
    }

    /// |x1 - x2| + |y1 - y2|. Hint: `i32::abs_diff` returns a `u32`.
    pub fn manhattan_distance(&self, other: &Point) -> u32 {
        todo!()
    }

    /// Move the point by (dx, dy) in place.
    pub fn translate(&mut self, dx: i32, dy: i32) {
        todo!()
    }
}

// ---------------------------------------------------------------------------
// Problem 3: Counter
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub struct Counter {
    count: u32,
}

impl Counter {
    /// A counter starting at 0.
    pub fn new() -> Counter {
        todo!()
    }

    /// Add 1.
    pub fn increment(&mut self) {
        todo!()
    }

    /// Add `n`.
    pub fn increment_by(&mut self, n: u32) {
        todo!()
    }

    /// Current value.
    pub fn get(&self) -> u32 {
        todo!()
    }

    /// Back to 0.
    pub fn reset(&mut self) {
        todo!()
    }
}

// ---------------------------------------------------------------------------
// Problem 4: BankAccount
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub struct BankAccount {
    owner: String,
    balance: u64,
}

impl BankAccount {
    /// A new account for `owner` with a balance of 0.
    pub fn new(owner: &str) -> BankAccount {
        todo!()
    }

    /// Add `amount` to the balance.
    pub fn deposit(&mut self, amount: u64) {
        todo!()
    }

    /// Remove `amount` from the balance and return `true`, or — if the balance is too
    /// low — change nothing and return `false`.
    pub fn withdraw(&mut self, amount: u64) -> bool {
        todo!()
    }

    /// Borrow the owner's name.
    pub fn owner(&self) -> &str {
        todo!()
    }

    /// Current balance.
    pub fn balance(&self) -> u64 {
        todo!()
    }
}

// ---------------------------------------------------------------------------
// Problem 5: Inventory (a struct owning a Vec of structs)
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub struct Item {
    pub name: String,
    pub quantity: u32,
}

#[derive(Debug)]
pub struct Inventory {
    items: Vec<Item>,
}

impl Inventory {
    /// An empty inventory.
    pub fn new() -> Inventory {
        todo!()
    }

    /// Add `quantity` of `name`. If an item with that exact name already exists, increase
    /// its quantity instead of adding a second entry.
    pub fn add(&mut self, name: &str, quantity: u32) {
        todo!()
    }

    /// How many of `name` we have (0 if none).
    pub fn quantity_of(&self, name: &str) -> u32 {
        todo!()
    }

    /// Sum of all quantities.
    pub fn total_items(&self) -> u32 {
        todo!()
    }

    /// Number of distinct item entries. (Already implemented — used by the tests.)
    pub fn distinct_items(&self) -> usize {
        self.items.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rectangle_basics() {
        let r = Rectangle::new(3, 4);
        assert_eq!(r, Rectangle { width: 3, height: 4 });
        assert_eq!(r.area(), 12);
        assert_eq!(r.perimeter(), 14);
        assert!(!r.is_square());
        let s = Rectangle::square(5);
        assert_eq!(s, Rectangle { width: 5, height: 5 });
        assert!(s.is_square());
    }

    #[test]
    fn rectangle_can_hold() {
        let big = Rectangle::new(10, 8);
        assert!(big.can_hold(&Rectangle::new(9, 7)));
        assert!(!big.can_hold(&Rectangle::new(10, 7)));
        assert!(!big.can_hold(&Rectangle::new(1, 9)));
        assert!(!Rectangle::new(1, 9).can_hold(&big));
    }

    #[test]
    fn point_methods() {
        let o = Point::origin();
        assert_eq!(o, Point(0, 0));
        let p = Point(3, -4);
        assert_eq!(o.manhattan_distance(&p), 7);
        assert_eq!(p.manhattan_distance(&o), 7);
        let mut q = Point(1, 1);
        q.translate(-3, 5);
        assert_eq!(q, Point(-2, 6));
    }

    #[test]
    fn counter() {
        let mut c = Counter::new();
        assert_eq!(c.get(), 0);
        c.increment();
        c.increment();
        assert_eq!(c.get(), 2);
        c.increment_by(10);
        assert_eq!(c.get(), 12);
        c.reset();
        assert_eq!(c.get(), 0);
    }

    #[test]
    fn bank_account() {
        let mut acct = BankAccount::new("Ann");
        assert_eq!(acct.owner(), "Ann");
        assert_eq!(acct.balance(), 0);
        acct.deposit(100);
        assert!(acct.withdraw(30));
        assert_eq!(acct.balance(), 70);
        assert!(!acct.withdraw(71), "overdraft must fail");
        assert_eq!(acct.balance(), 70, "failed withdrawal must not change balance");
        assert!(acct.withdraw(70));
        assert_eq!(acct.balance(), 0);
    }

    #[test]
    fn inventory() {
        let mut inv = Inventory::new();
        assert_eq!(inv.total_items(), 0);
        assert_eq!(inv.quantity_of("apple"), 0);
        inv.add("apple", 3);
        inv.add("pear", 2);
        inv.add("apple", 4);
        assert_eq!(inv.quantity_of("apple"), 7);
        assert_eq!(inv.quantity_of("pear"), 2);
        assert_eq!(inv.quantity_of("plum"), 0);
        assert_eq!(inv.total_items(), 9);
        assert_eq!(inv.distinct_items(), 2, "adding an existing name must not duplicate it");
    }
}
