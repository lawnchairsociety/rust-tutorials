//! Lesson 08 practice problems: modules, paths, visibility and re-exports.
//!
//! This crate is split across several files. Read them in this order:
//!
//! ```text
//! src/lib.rs               <- crate root (you are here): declares the modules
//! src/temperature.rs       <- `mod temperature`
//! src/util.rs              <- `mod util` (private module!)
//! src/inventory.rs         <- `mod inventory`, which declares a child module...
//! src/inventory/item.rs    <- ...`mod inventory::item`
//! ```
//!
//! Replace every `todo!()` in those files, then run:
//!
//! ```text
//! cd exercises && cargo test -p lesson-08
//! ```
//!
//! The tests below only use paths that the starter code already makes visible.
//! Don't change the tests. If you think a test can't reach something, read the
//! `pub` keywords again.

pub mod inventory;
pub mod temperature;
mod util;

// Problem 3: re-export. Thanks to this line, users can write
// `lesson_08::celsius_to_fahrenheit` instead of `lesson_08::temperature::celsius_to_fahrenheit`.
pub use temperature::celsius_to_fahrenheit;

#[cfg(test)]
mod tests {
    // `crate::` is an absolute path starting at this crate's root.
    use crate::inventory::Inventory;
    use crate::inventory::item::Item;

    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    #[test]
    fn p1_temperature() {
        assert!(close(crate::temperature::celsius_to_fahrenheit(100.0), 212.0));
        assert!(close(crate::temperature::fahrenheit_to_celsius(32.0), 0.0));
        assert!(close(crate::temperature::fahrenheit_to_celsius(-40.0), -40.0));
    }

    #[test]
    fn p2_normalize_name() {
        // util is private, but tests is a child of the crate root, and the crate
        // root can see its own private modules. Children can see their ancestors' items.
        assert_eq!(crate::util::normalize_name("  Green Apple "), "green apple");
        assert_eq!(crate::util::normalize_name("PEAR"), "pear");
    }

    #[test]
    fn p3_reexport() {
        assert!(close(crate::celsius_to_fahrenheit(0.0), 32.0));
    }

    #[test]
    fn p4_item() {
        let item = Item::new("  Banana ", 3);
        assert_eq!(item.name(), "banana");
        assert_eq!(item.quantity(), 3);
    }

    #[test]
    fn p5_inventory_add_and_query() {
        let mut inv = Inventory::new();
        inv.add("Apple", 2);
        inv.add("pear", 1);
        inv.add(" APPLE ", 5); // same item after normalizing: quantities merge
        assert_eq!(inv.quantity_of("apple"), 7);
        assert_eq!(inv.quantity_of("Pear"), 1);
        assert_eq!(inv.quantity_of("kiwi"), 0);
        assert_eq!(inv.total(), 8);
    }

    #[test]
    fn p6_inventory_names_sorted() {
        let mut inv = Inventory::new();
        inv.add("pear", 1);
        inv.add("Apple", 1);
        inv.add("fig", 1);
        assert_eq!(inv.names(), vec!["apple", "fig", "pear"]);
    }
}
