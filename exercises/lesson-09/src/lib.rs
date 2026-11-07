//! Lesson 09 practice problems: traits and generics.
//!
//! Replace every `todo!()` with a real implementation, then run:
//!
//! ```text
//! cd exercises && cargo test -p lesson-09
//! ```

use std::fmt;
use std::str::FromStr;

// ---------------------------------------------------------------------------
// Generics
// ---------------------------------------------------------------------------

/// Problem 1: Return the largest item, or `None` for an empty slice.
/// Works for any type that can be compared and copied.
///
/// ```text
/// largest(&[3, 9, 2]) == Some(9)
/// largest(&['a', 'z', 'q']) == Some('z')
/// ```
pub fn largest<T: PartialOrd + Copy>(items: &[T]) -> Option<T> {
    todo!()
}

/// A pair of two values of the same type.
#[derive(Debug, PartialEq)]
pub struct Pair<T> {
    pub a: T,
    pub b: T,
}

impl<T> Pair<T> {
    /// Problem 2a: Construct a pair.
    pub fn new(a: T, b: T) -> Pair<T> {
        todo!()
    }

    /// Problem 2b: Return a new pair with `a` and `b` swapped (consumes self).
    pub fn swap(self) -> Pair<T> {
        todo!()
    }
}

impl<T: PartialOrd> Pair<T> {
    /// Problem 2c: Return a reference to the larger of the two values
    /// (`a` if they are equal). This method only exists when `T: PartialOrd`.
    pub fn larger(&self) -> &T {
        todo!()
    }
}

// ---------------------------------------------------------------------------
// Defining and implementing traits
// ---------------------------------------------------------------------------

/// Something with an area. `name` and `describe` have default implementations.
pub trait Shape {
    fn area(&self) -> f64;

    fn name(&self) -> String {
        String::from("shape")
    }

    /// e.g. "circle with area 3.14"
    fn describe(&self) -> String {
        format!("{} with area {:.2}", self.name(), self.area())
    }
}

pub struct Circle {
    pub radius: f64,
}

pub struct Rect {
    pub width: f64,
    pub height: f64,
}

/// Problem 3a: Area is π·r² (use `std::f64::consts::PI`). Override `name`
/// to return "circle".
impl Shape for Circle {
    fn area(&self) -> f64 {
        todo!()
    }
}

/// Problem 3b: Area is width·height. Keep the default `name` ("shape"), or
/// override it to "rectangle". The tests accept either.
impl Shape for Rect {
    fn area(&self) -> f64 {
        todo!()
    }
}

/// Problem 4: Sum the areas of a slice of shapes that all have the same type.
pub fn total_area<S: Shape>(shapes: &[S]) -> f64 {
    todo!()
}

/// Problem 5: Return the `describe()` of whichever shape has the larger area
/// (`a` on a tie). The two shapes may be different types.
pub fn describe_bigger(a: &impl Shape, b: &impl Shape) -> String {
    todo!()
}

// ---------------------------------------------------------------------------
// Standard library traits
// ---------------------------------------------------------------------------

/// A 2D point with integer coordinates.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

/// Problem 6: Display a point as `(x, y)`, e.g. `(3, -4)`.
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!()
    }
}

/// An RGB colour.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

/// Problem 7: Build an `Rgb` from a tuple, so `Rgb::from((1, 2, 3))` and
/// `let c: Rgb = (1, 2, 3).into();` both work.
impl From<(u8, u8, u8)> for Rgb {
    fn from(value: (u8, u8, u8)) -> Rgb {
        todo!()
    }
}

/// Problem 8: Parse "#rrggbb" hex colours (case-insensitive), e.g. "#FF8800".
/// Anything else (wrong length, missing '#', bad hex digits) is an error
/// holding the original input. Hint: `u8::from_str_radix(&s[1..3], 16)`.
/// Check the length and that the string is ASCII before slicing.
impl FromStr for Rgb {
    type Err = String;

    fn from_str(s: &str) -> Result<Rgb, String> {
        todo!()
    }
}

/// Problem 9: Priority levels. Add the derives needed so that:
/// - levels can be compared with `<` and sorted (Low < Medium < High)
/// - they can be used in `assert_eq!` and printed with `{:?}`
/// - they can be copied
///
/// Replace the `#[derive(Debug)]` line below with the right list.
#[derive(Debug)]
pub enum Level {
    Low,
    Medium,
    High,
}

/// Problem 9 (cont.): Sort the levels from highest to lowest.
pub fn sort_desc(levels: &mut Vec<Level>) {
    todo!()
}

/// Settings with sensible defaults.
#[derive(Debug, PartialEq)]
pub struct Config {
    pub name: String,
    pub retries: u32,
    pub verbose: bool,
}

/// Problem 10: Implement `Default` by hand: name "untitled", 3 retries,
/// verbose false. (A plain `#[derive(Default)]` would give "", 0 and false.)
impl Default for Config {
    fn default() -> Config {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_largest() {
        assert_eq!(largest(&[3, 9, 2]), Some(9));
        assert_eq!(largest(&['a', 'z', 'q']), Some('z'));
        assert_eq!(largest(&[1.5, -2.0]), Some(1.5));
        assert_eq!(largest::<i32>(&[]), None);
    }

    #[test]
    fn p2_pair() {
        let p = Pair::new(1, 2);
        assert_eq!(*p.larger(), 2);
        let s = p.swap();
        assert_eq!(s, Pair { a: 2, b: 1 });
        assert_eq!(*s.larger(), 2);
        let words = Pair::new(String::from("pear"), String::from("apple"));
        assert_eq!(words.larger(), "pear");
    }

    #[test]
    fn p3_shapes() {
        let c = Circle { radius: 1.0 };
        assert!((c.area() - std::f64::consts::PI).abs() < 1e-9);
        assert_eq!(c.name(), "circle");
        assert_eq!(c.describe(), "circle with area 3.14");
        let r = Rect { width: 2.0, height: 3.5 };
        assert_eq!(r.area(), 7.0);
        assert!(r.describe().ends_with("with area 7.00"));
    }

    #[test]
    fn p4_total_area() {
        let rects = [Rect { width: 1.0, height: 2.0 }, Rect { width: 3.0, height: 3.0 }];
        assert_eq!(total_area(&rects), 11.0);
        assert_eq!(total_area::<Circle>(&[]), 0.0);
    }

    #[test]
    fn p5_describe_bigger() {
        let c = Circle { radius: 1.0 }; // ~3.14
        let r = Rect { width: 2.0, height: 2.0 }; // 4
        assert!(describe_bigger(&c, &r).ends_with("with area 4.00"));
        assert_eq!(describe_bigger(&c, &c), "circle with area 3.14");
    }

    #[test]
    fn p6_point_display() {
        assert_eq!(Point { x: 3, y: -4 }.to_string(), "(3, -4)");
        assert_eq!(format!("at {}", Point { x: 0, y: 0 }), "at (0, 0)");
    }

    #[test]
    fn p7_rgb_from() {
        assert_eq!(Rgb::from((1, 2, 3)), Rgb { r: 1, g: 2, b: 3 });
        let c: Rgb = (255, 0, 10).into();
        assert_eq!(c, Rgb { r: 255, g: 0, b: 10 });
    }

    #[test]
    fn p8_rgb_from_str() {
        assert_eq!("#FF8800".parse::<Rgb>(), Ok(Rgb { r: 255, g: 136, b: 0 }));
        assert_eq!("#0a0b0c".parse::<Rgb>(), Ok(Rgb { r: 10, g: 11, b: 12 }));
        assert_eq!("FF8800".parse::<Rgb>(), Err("FF8800".to_string()));
        assert_eq!("#FF88".parse::<Rgb>(), Err("#FF88".to_string()));
        assert_eq!("#GG0000".parse::<Rgb>(), Err("#GG0000".to_string()));
        assert_eq!("#é0000".parse::<Rgb>(), Err("#é0000".to_string()));
    }

    // Problem 9's test is commented out because it needs derives you haven't
    // added yet. Add them, then uncomment this test.
    //
    // #[test]
    // fn p9_level_ordering() {
    //     assert!(Level::Low < Level::Medium && Level::Medium < Level::High);
    //     let mut v = vec![Level::Medium, Level::High, Level::Low, Level::High];
    //     sort_desc(&mut v);
    //     assert_eq!(v, vec![Level::High, Level::High, Level::Medium, Level::Low]);
    //     let copy = v[0];
    //     assert_eq!(copy, v[0]);
    // }

    #[test]
    fn p10_config_default() {
        let c = Config::default();
        assert_eq!(c, Config { name: "untitled".into(), retries: 3, verbose: false });
        let custom = Config { verbose: true, ..Default::default() };
        assert_eq!(custom.retries, 3);
        assert!(custom.verbose);
    }
}
