//! Lesson 13 — Lifetimes, trait objects & smart pointers practice problems.
//!
//! Replace each `todo!()` with a working implementation, then run:
//!
//! ```text
//! cd exercises && cargo test -p lesson-13
//! ```
//!
//! (Unused-variable warnings are expected until you implement each function.)

use std::cell::RefCell;
use std::rc::Rc;

// ---------------------------------------------------------------------------
// Part A — Lifetimes
// ---------------------------------------------------------------------------

/// Problem 1: Return whichever of `a` and `b` is longer (in bytes). On a tie, return `a`.
///
/// The `'a` says: "the result borrows from `a` or `b`, so it's valid for as long as *both* are".
pub fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    todo!()
}

/// Problem 2: Return the first whitespace-separated word of `s`, or `""` if there is none.
/// Leading whitespace is skipped.
///
/// Notice there's no lifetime annotation: the elision rules fill it in. Can you say which rule?
/// (Write it in ANSWERS.md Q1.)
pub fn first_word(s: &str) -> &str {
    todo!()
}

/// A struct that **borrows** a piece of text instead of owning it.
pub struct Highlighter<'a> {
    pub text: &'a str,
}

impl<'a> Highlighter<'a> {
    /// Problem 3a: Return every whitespace-separated word of `self.text` that contains `needle`,
    /// in order.
    ///
    /// Look at the return type: the words borrow from the *text* (`'a`), not from `self`
    /// and not from `needle`. That means the result can outlive the `Highlighter` itself (see the test).
    pub fn words_containing(&self, needle: &str) -> Vec<&'a str> {
        todo!()
    }

    /// Problem 3b: Return `self.text` with every occurrence of `needle` wrapped in `[` `]`.
    /// Example: text `"a cat sat"`, needle `"at"` → `"a c[at] s[at]"`.
    pub fn highlight(&self, needle: &str) -> String {
        todo!()
    }
}

// ---------------------------------------------------------------------------
// Part B — Trait objects
// ---------------------------------------------------------------------------

pub trait Shape {
    fn area(&self) -> f64;
    fn name(&self) -> String;

    /// A default method: every shape gets this for free.
    fn describe(&self) -> String {
        format!("{} with area {:.2}", self.name(), self.area())
    }
}

pub struct Circle {
    pub radius: f64,
}

pub struct Square {
    pub side: f64,
}

/// Problem 4a: Implement `Shape` for `Circle` (name `"circle"`, area πr²) and
/// `Square` (name `"square"`, area side²). Use `std::f64::consts::PI`.
impl Shape for Circle {
    fn area(&self) -> f64 {
        todo!()
    }
    fn name(&self) -> String {
        todo!()
    }
}

impl Shape for Square {
    fn area(&self) -> f64 {
        todo!()
    }
    fn name(&self) -> String {
        todo!()
    }
}

/// Problem 4b: Build a shape chosen **at runtime**:
/// `"circle"` → `Circle { radius: size }`, `"square"` → `Square { side: size }`, anything else → `None`.
pub fn make_shape(kind: &str, size: f64) -> Option<Box<dyn Shape>> {
    todo!()
}

/// Problem 4c: Sum the areas of a mixed collection of shapes.
pub fn total_area(shapes: &[Box<dyn Shape>]) -> f64 {
    todo!()
}

// ---------------------------------------------------------------------------
// Part C — Box<T> for recursive types
// ---------------------------------------------------------------------------

/// A tiny arithmetic expression tree. Without `Box`, `Expr` would contain itself directly
/// and have infinite size.
#[derive(Debug)]
pub enum Expr {
    Num(i64),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
}

/// Problem 5: Evaluate an expression (recursively).
/// Example: `Add(Num(2), Mul(Num(3), Num(4)))` → `14`.
pub fn eval(e: &Expr) -> i64 {
    todo!()
}

// ---------------------------------------------------------------------------
// Part D — Rc<RefCell<T>> and Drop
// ---------------------------------------------------------------------------

/// A logger that can be cloned; all clones share the **same** list of lines.
#[derive(Clone)]
pub struct Logger {
    lines: Rc<RefCell<Vec<String>>>,
}

impl Logger {
    /// Problem 6a: Create a logger with no lines.
    pub fn new() -> Self {
        todo!()
    }

    /// Problem 6b: Append a line. Note `&self`, not `&mut self`! That's interior mutability.
    pub fn log(&self, msg: &str) {
        todo!()
    }

    /// Problem 6c: Return a copy of all lines logged so far (by this logger or any clone).
    pub fn lines(&self) -> Vec<String> {
        todo!()
    }

    /// Problem 6d: How many `Logger`s share this list right now? (`Rc::strong_count`)
    pub fn handles(&self) -> usize {
        todo!()
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self::new()
    }
}

/// A value that records its own destruction in a shared logger.
pub struct Noisy {
    pub name: String,
    pub logger: Logger,
}

/// Problem 7: When a `Noisy` is dropped, log `"drop <name>"`.
impl Drop for Noisy {
    fn drop(&mut self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_longer() {
        assert_eq!(longer("hello", "hi"), "hello");
        assert_eq!(longer("hi", "hello"), "hello");
        assert_eq!(longer("ab", "cd"), "ab");
    }

    #[test]
    fn p2_first_word() {
        assert_eq!(first_word("hello world"), "hello");
        assert_eq!(first_word("   padded  text"), "padded");
        assert_eq!(first_word("   "), "");
        assert_eq!(first_word(""), "");
    }

    #[test]
    fn p3_highlighter() {
        let text = String::from("the cat sat on the mat");
        let words;
        {
            let h = Highlighter { text: &text };
            let needle = String::from("at");
            words = h.words_containing(&needle);
            assert_eq!(h.highlight("at"), "the c[at] s[at] on the m[at]");
        } // `h` and `needle` are gone, but `words` borrows from `text`, which is still alive
        assert_eq!(words, vec!["cat", "sat", "mat"]);
        let h = Highlighter { text: "no match" };
        assert!(h.words_containing("zzz").is_empty());
        assert_eq!(h.highlight("zzz"), "no match");
    }

    #[test]
    fn p4_shapes() {
        let c = Circle { radius: 1.0 };
        assert!((c.area() - std::f64::consts::PI).abs() < 1e-9);
        assert_eq!(c.name(), "circle");
        assert_eq!(Square { side: 3.0 }.describe(), "square with area 9.00");

        let shapes: Vec<Box<dyn Shape>> = ["square", "circle", "square"]
            .iter()
            .map(|k| make_shape(k, 2.0).unwrap())
            .collect();
        let expected = 4.0 + std::f64::consts::PI * 4.0 + 4.0;
        assert!((total_area(&shapes) - expected).abs() < 1e-9);
        assert!(make_shape("hexagon", 1.0).is_none());
    }

    #[test]
    fn p5_eval() {
        use Expr::*;
        let e = Add(Box::new(Num(2)), Box::new(Mul(Box::new(Num(3)), Box::new(Num(4)))));
        assert_eq!(eval(&e), 14);
        assert_eq!(eval(&Neg(Box::new(Num(5)))), -5);
        assert_eq!(eval(&Num(0)), 0);
    }

    #[test]
    fn p6_logger_shares_state() {
        let a = Logger::new();
        assert_eq!(a.handles(), 1);
        let b = a.clone();
        assert_eq!(a.handles(), 2);
        a.log("from a");
        b.log("from b");
        assert_eq!(a.lines(), vec!["from a", "from b"]);
        assert_eq!(b.lines(), a.lines());
        drop(b);
        assert_eq!(a.handles(), 1);
    }

    #[test]
    fn p7_drop_order() {
        let log = Logger::new();
        {
            let _first = Noisy { name: "first".into(), logger: log.clone() };
            let second = Noisy { name: "second".into(), logger: log.clone() };
            drop(second); // dropped early, explicitly
            log.log("end of scope");
        } // `_first` dropped here
        assert_eq!(log.lines(), vec!["drop second", "end of scope", "drop first"]);
    }
}
