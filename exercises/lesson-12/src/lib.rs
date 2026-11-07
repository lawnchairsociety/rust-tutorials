//! Lesson 12 — File I/O, command-line arguments & `Box<dyn Error>` practice problems.
//!
//! Replace each `todo!()` with a working implementation, then run:
//!
//! ```text
//! cd exercises && cargo test -p lesson-12
//! ```
//!
//! The tests create small files in your system's temporary directory and delete them afterwards.
//! (Unused-variable warnings are expected until you implement each function.)

use std::error::Error;
use std::fmt;
use std::io;
use std::path::{Path, PathBuf};

/// Problem 1: Write every string in `lines` to the file at `path`, each followed by `\n`.
/// Create the file if it doesn't exist and **replace** its contents if it does.
///
/// Example: `write_lines(p, &["a", "b"])` leaves the file containing `"a\nb\n"`.
pub fn write_lines(path: &Path, lines: &[&str]) -> io::Result<()> {
    todo!()
}

/// Problem 2: Return the number of lines in the file at `path`.
/// A missing file is an error (just pass the `io::Error` up with `?`).
///
/// Hint: `BufReader::new(File::open(path)?).lines()`, or `fs::read_to_string`.
pub fn count_lines(path: &Path) -> io::Result<usize> {
    todo!()
}

/// Problem 3: Append `line` plus a `\n` to the end of the file at `path`,
/// creating the file if it doesn't exist. Existing contents must be kept.
///
/// Hint: `std::fs::OpenOptions`.
pub fn append_line(path: &Path, line: &str) -> io::Result<()> {
    todo!()
}

/// Problem 4: Read a file where every non-blank line is an integer (surrounding whitespace allowed)
/// and return their sum. Blank lines are skipped.
///
/// Errors: a missing file (an `io::Error`) **or** a bad number (a `ParseIntError`).
/// Both convert into `Box<dyn Error>` automatically with `?`.
pub fn sum_file(path: &Path) -> Result<i64, Box<dyn Error>> {
    todo!()
}

/// An error from [`load_scores`].
#[derive(Debug)]
pub enum LoadError {
    /// The file couldn't be read.
    Io(io::Error),
    /// Line number `line` (1-based) was not in `name:score` form.
    BadLine { line: usize },
}

/// Problem 5a: Make `?` convert an `io::Error` into a `LoadError::Io`.
impl From<io::Error> for LoadError {
    fn from(err: io::Error) -> Self {
        todo!()
    }
}

/// Problem 5b: Display a `LoadError`.
/// - `Io(e)`            → `"could not read file: <e>"`
/// - `BadLine { line }` → `"line <line>: expected name:score"`
impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl Error for LoadError {}

/// Problem 5c: Load a file of `name:score` lines (e.g. `ann:12`) into a vector, in file order.
/// Blank lines are skipped. Whitespace around the name and score is trimmed.
/// Any other malformed line (no `:`, empty name, or score not a `u32`) is a `BadLine` error
/// carrying the **1-based** line number.
///
/// Hint: `.lines().enumerate()` and `str::split_once(':')`.
pub fn load_scores(path: &Path) -> Result<Vec<(String, u32)>, LoadError> {
    todo!()
}

/// Parsed command-line options for an imaginary tool.
#[derive(Debug, PartialEq, Default)]
pub struct Args {
    pub verbose: bool,
    pub output: Option<String>,
    pub rest: Vec<String>,
}

/// Problem 6: Parse command-line arguments (program name already removed).
///
/// - `-v` or `--verbose` sets `verbose`.
/// - `-o <file>` or `--output <file>` sets `output`. A missing value is an error:
///   `Err("missing value for --output".to_string())`.
/// - Any other argument starting with `-` is an error: `Err(format!("unknown flag: {arg}"))`.
/// - Everything else goes into `rest`, in order.
///
/// In a real program you'd call it as `parse_args(&std::env::args().skip(1).collect::<Vec<_>>())`.
pub fn parse_args(args: &[String]) -> Result<Args, String> {
    todo!()
}

/// Problem 7: Return `path` with its extension replaced by `ext` (or added if it had none).
///
/// Examples: `"notes.txt"` + `"md"` → `"notes.md"`; `"data/tasks"` + `"db"` → `"data/tasks.db"`.
///
/// Hint: look at the methods on `Path`.
pub fn change_extension(path: &str, ext: &str) -> PathBuf {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    /// A unique temp-file path for one test.
    fn tmp(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!("lesson12-{}-{name}", std::process::id()))
    }

    #[test]
    fn p1_write_lines_creates_and_replaces() {
        let p = tmp("write");
        write_lines(&p, &["a", "b"]).unwrap();
        assert_eq!(fs::read_to_string(&p).unwrap(), "a\nb\n");
        write_lines(&p, &["only"]).unwrap();
        assert_eq!(fs::read_to_string(&p).unwrap(), "only\n");
        fs::remove_file(&p).unwrap();
    }

    #[test]
    fn p2_count_lines() {
        let p = tmp("count");
        fs::write(&p, "one\ntwo\nthree\n").unwrap();
        assert_eq!(count_lines(&p).unwrap(), 3);
        fs::write(&p, "").unwrap();
        assert_eq!(count_lines(&p).unwrap(), 0);
        fs::remove_file(&p).unwrap();
        assert!(count_lines(&p).is_err());
    }

    #[test]
    fn p3_append_line() {
        let p = tmp("append");
        let _ = fs::remove_file(&p);
        append_line(&p, "first").unwrap();
        append_line(&p, "second").unwrap();
        assert_eq!(fs::read_to_string(&p).unwrap(), "first\nsecond\n");
        fs::remove_file(&p).unwrap();
    }

    #[test]
    fn p4_sum_file() {
        let p = tmp("sum");
        fs::write(&p, "10\n  -3 \n\n5\n").unwrap();
        assert_eq!(sum_file(&p).unwrap(), 12);
        fs::write(&p, "1\nnope\n").unwrap();
        let err = sum_file(&p).unwrap_err();
        assert!(err.to_string().contains("invalid digit"));
        fs::remove_file(&p).unwrap();
        assert!(sum_file(&p).is_err());
    }

    #[test]
    fn p5_load_error_display_and_from() {
        let e: LoadError = io::Error::new(io::ErrorKind::NotFound, "gone").into();
        assert!(matches!(e, LoadError::Io(_)));
        assert_eq!(e.to_string(), "could not read file: gone");
        assert_eq!(
            LoadError::BadLine { line: 4 }.to_string(),
            "line 4: expected name:score"
        );
    }

    #[test]
    fn p5_load_scores() {
        let p = tmp("scores");
        fs::write(&p, "ann: 12\n\n bob :7\n").unwrap();
        assert_eq!(
            load_scores(&p).unwrap(),
            vec![("ann".to_string(), 12), ("bob".to_string(), 7)]
        );
        fs::write(&p, "ann:1\nbob\n").unwrap();
        assert!(matches!(load_scores(&p), Err(LoadError::BadLine { line: 2 })));
        fs::write(&p, "ann:x\n").unwrap();
        assert!(matches!(load_scores(&p), Err(LoadError::BadLine { line: 1 })));
        fs::write(&p, ":5\n").unwrap();
        assert!(matches!(load_scores(&p), Err(LoadError::BadLine { line: 1 })));
        fs::remove_file(&p).unwrap();
        assert!(matches!(load_scores(&p), Err(LoadError::Io(_))));
    }

    fn strings(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn p6_parse_args() {
        assert_eq!(parse_args(&[]).unwrap(), Args::default());
        assert_eq!(
            parse_args(&strings(&["-v", "in.txt", "--output", "out.txt", "more"])).unwrap(),
            Args {
                verbose: true,
                output: Some("out.txt".into()),
                rest: strings(&["in.txt", "more"]),
            }
        );
        assert_eq!(
            parse_args(&strings(&["--verbose", "-o", "x"])).unwrap(),
            Args { verbose: true, output: Some("x".into()), rest: vec![] }
        );
        assert_eq!(
            parse_args(&strings(&["-o"])),
            Err("missing value for --output".to_string())
        );
        assert_eq!(
            parse_args(&strings(&["--frobnicate"])),
            Err("unknown flag: --frobnicate".to_string())
        );
    }

    #[test]
    fn p7_change_extension() {
        assert_eq!(change_extension("notes.txt", "md"), PathBuf::from("notes.md"));
        assert_eq!(change_extension("data/tasks", "db"), PathBuf::from("data/tasks.db"));
    }
}
