//! Lesson 05 practice problems: enums, `match`, and `Option`.
//!
//! Replace each `todo!()` with your implementation, then run:
//!
//! ```text
//! cd exercises
//! cargo test -p lesson-05
//! ```

// ---------------------------------------------------------------------------
// Problem 1: Shape
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle { base: f64, height: f64 },
}

impl Shape {
    /// Area of the shape. Circle: π r², Rectangle: w × h, Triangle: ½ × b × h.
    /// Use `std::f64::consts::PI`.
    pub fn area(&self) -> f64 {
        todo!()
    }
}

// ---------------------------------------------------------------------------
// Problem 2: Coin
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

impl Coin {
    /// Penny 1, Nickel 5, Dime 10, Quarter 25.
    pub fn value_in_cents(&self) -> u32 {
        todo!()
    }
}

// ---------------------------------------------------------------------------
// Problem 3: TrafficLight
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    /// The next light: Red → Green → Yellow → Red.
    pub fn next(&self) -> TrafficLight {
        todo!()
    }

    /// How long the light stays on: Red 30, Green 25, Yellow 5.
    pub fn seconds(&self) -> u32 {
        todo!()
    }
}

// ---------------------------------------------------------------------------
// Problems 4–5: returning Option
// ---------------------------------------------------------------------------

/// Problem 4: `Some(a / b)`, or `None` if `b` is zero.
pub fn safe_divide(a: i32, b: i32) -> Option<i32> {
    todo!()
}

/// Problem 5: the first even number in `nums`, if any.
///
/// Example: `first_even(&[1, 3, 4, 6])` → `Some(4)`
pub fn first_even(nums: &[i32]) -> Option<i32> {
    todo!()
}

// ---------------------------------------------------------------------------
// Problem 6: parsing text into an enum
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

/// Accept "n"/"north", "s"/"south", "e"/"east", "w"/"west", in any letter case
/// (hint: `s.to_lowercase()` returns a `String`; match on `.as_str()`). Anything else → None.
pub fn parse_direction(s: &str) -> Option<Direction> {
    todo!()
}

// ---------------------------------------------------------------------------
// Problem 7: match guards
// ---------------------------------------------------------------------------

/// Use a `match` with guards:
/// - below 0 → "negative"
/// - 0 → "zero"
/// - 1..=9 and even → "small even"
/// - 1..=9 and odd → "small odd"
/// - 10 and above → "large"
pub fn classify(n: i32) -> String {
    todo!()
}

// ---------------------------------------------------------------------------
// Problem 8: combining Options
// ---------------------------------------------------------------------------

/// `Some(a + b)` if both are `Some`, otherwise `None`.
pub fn add_options(a: Option<i32>, b: Option<i32>) -> Option<i32> {
    todo!()
}

// ---------------------------------------------------------------------------
// Problem 9: destructuring data-carrying variants
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

/// Describe a message:
/// - `Quit` → `"quit"`
/// - `Move { x: 3, y: -1 }` → `"move to (3, -1)"`
/// - `Write("hi")` → `"write: hi"`
/// - `ChangeColor(255, 0, 16)` → `"color #ff0010"` (lowercase hex, 2 digits each: `{:02x}`)
pub fn describe(msg: &Message) -> String {
    todo!()
}

// ---------------------------------------------------------------------------
// Problem 10: let ... else
// ---------------------------------------------------------------------------

/// Uppercase initials of the first and last word, using `let ... else`.
///
/// - `"ada lovelace"` → `Some("AL")`
/// - `"Grace Brewster Hopper"` → `Some("GH")` (first and *last* word)
/// - `"Plato"` → `None`
///
/// Hints: `split_once(' ')` and `rsplit_once(' ')`; `c.to_ascii_uppercase()` on a `char`.
pub fn initials(full_name: &str) -> Option<String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    #[test]
    fn shape_areas() {
        assert!(close(Shape::Circle { radius: 1.0 }.area(), std::f64::consts::PI));
        assert!(close(Shape::Circle { radius: 2.0 }.area(), 4.0 * std::f64::consts::PI));
        assert!(close(Shape::Rectangle { width: 3.0, height: 4.0 }.area(), 12.0));
        assert!(close(Shape::Triangle { base: 3.0, height: 4.0 }.area(), 6.0));
    }

    #[test]
    fn coins() {
        let purse = [Coin::Quarter, Coin::Dime, Coin::Nickel, Coin::Penny, Coin::Penny];
        let mut total = 0;
        for c in purse {
            total += c.value_in_cents();
        }
        assert_eq!(total, 42);
    }

    #[test]
    fn traffic_light_cycle() {
        assert_eq!(TrafficLight::Red.next(), TrafficLight::Green);
        assert_eq!(TrafficLight::Green.next(), TrafficLight::Yellow);
        assert_eq!(TrafficLight::Yellow.next(), TrafficLight::Red);
        let mut light = TrafficLight::Red;
        let mut total = 0;
        for _ in 0..3 {
            total += light.seconds();
            light = light.next();
        }
        assert_eq!(total, 60);
        assert_eq!(light, TrafficLight::Red);
    }

    #[test]
    fn division() {
        assert_eq!(safe_divide(10, 2), Some(5));
        assert_eq!(safe_divide(7, -2), Some(-3));
        assert_eq!(safe_divide(1, 0), None);
    }

    #[test]
    fn evens() {
        assert_eq!(first_even(&[1, 3, 4, 6]), Some(4));
        assert_eq!(first_even(&[-2]), Some(-2));
        assert_eq!(first_even(&[1, 3, 5]), None);
        assert_eq!(first_even(&[]), None);
    }

    #[test]
    fn directions() {
        assert_eq!(parse_direction("n"), Some(Direction::North));
        assert_eq!(parse_direction("North"), Some(Direction::North));
        assert_eq!(parse_direction("SOUTH"), Some(Direction::South));
        assert_eq!(parse_direction("e"), Some(Direction::East));
        assert_eq!(parse_direction("west"), Some(Direction::West));
        assert_eq!(parse_direction("up"), None);
        assert_eq!(parse_direction(""), None);
    }

    #[test]
    fn classification() {
        assert_eq!(classify(-5), "negative");
        assert_eq!(classify(0), "zero");
        assert_eq!(classify(4), "small even");
        assert_eq!(classify(9), "small odd");
        assert_eq!(classify(1), "small odd");
        assert_eq!(classify(10), "large");
        assert_eq!(classify(1000), "large");
    }

    #[test]
    fn option_sum() {
        assert_eq!(add_options(Some(2), Some(3)), Some(5));
        assert_eq!(add_options(None, Some(3)), None);
        assert_eq!(add_options(Some(2), None), None);
        assert_eq!(add_options(None, None), None);
    }

    #[test]
    fn messages() {
        assert_eq!(describe(&Message::Quit), "quit");
        assert_eq!(describe(&Message::Move { x: 3, y: -1 }), "move to (3, -1)");
        assert_eq!(describe(&Message::Write("hi".to_string())), "write: hi");
        assert_eq!(describe(&Message::ChangeColor(255, 0, 16)), "color #ff0010");
    }

    #[test]
    fn name_initials() {
        assert_eq!(initials("ada lovelace"), Some("AL".to_string()));
        assert_eq!(initials("Grace Brewster Hopper"), Some("GH".to_string()));
        assert_eq!(initials("Plato"), None);
        assert_eq!(initials(""), None);
    }
}
