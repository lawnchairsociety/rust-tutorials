//! The module `crate::inventory::item`.

/// One kind of thing in the inventory. Its fields are private, so the only way
/// to build an `Item` is `Item::new`, and that guarantees the name is normalized.
pub struct Item {
    name: String,
    quantity: u32,
}

impl Item {
    /// Problem 4a: Create an item. Store the name normalized with
    /// `crate::util::normalize_name` (call it with the full path, or add a
    /// `use` line at the top of this file).
    pub fn new(name: &str, quantity: u32) -> Item {
        todo!()
    }

    /// Problem 4b: Read-only access to the name.
    pub fn name(&self) -> &str {
        todo!()
    }

    /// Problem 4c: Read-only access to the quantity.
    pub fn quantity(&self) -> u32 {
        todo!()
    }

    /// Problem 5 helper: increase the quantity. `pub(super)` means only the
    /// parent module (`inventory`) can call this, not the rest of the crate.
    pub(super) fn add_quantity(&mut self, extra: u32) {
        todo!()
    }
}
