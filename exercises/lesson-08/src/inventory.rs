//! The module `crate::inventory`. Its child module `item` lives in
//! `src/inventory/item.rs`.

pub mod item;

// Problem 5 hint: you'll need `normalize_name` from `crate::util`. Bring it
// into scope with a `use` line here.
use item::Item;

/// A collection of items. The field is private: other modules can only
/// use the methods below.
pub struct Inventory {
    items: Vec<Item>,
}

impl Inventory {
    /// Problem 5a: An empty inventory.
    pub fn new() -> Inventory {
        todo!()
    }

    /// Problem 5b: Add `quantity` of `name`. If an item with the same
    /// *normalized* name exists, increase its quantity (use
    /// `Item::add_quantity`); otherwise push a new `Item`.
    pub fn add(&mut self, name: &str, quantity: u32) {
        todo!()
    }

    /// Problem 5c: The quantity for `name` (normalized), or 0 if absent.
    pub fn quantity_of(&self, name: &str) -> u32 {
        todo!()
    }

    /// Problem 5d: The sum of all quantities.
    pub fn total(&self) -> u32 {
        todo!()
    }

    /// Problem 6: All item names, sorted alphabetically.
    pub fn names(&self) -> Vec<String> {
        todo!()
    }
}
