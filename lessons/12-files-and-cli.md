# Lesson 12 — Files, Command-Line Arguments & `Box<dyn Error>`

> Make your programs remember things between runs and accept arguments, while handling all the ways I/O can fail. Estimated time: 3–4 hours.

## What you'll learn

- Reading and writing files: `fs::read_to_string`, `fs::write`, `File`, `OpenOptions`
- Buffered I/O with `BufReader` / `BufWriter` and reading line by line
- `Path` and `PathBuf`
- `io::Error` and `io::Result<T>`
- Reading command-line arguments with `std::env::args`
- Returning `Result` from `main`, and `Box<dyn Error>` as a "catch-all" error type
- Building your own error type that wraps other errors (`From`, `std::error::Error`)
- Exit codes with `std::process::exit`

---

## 1. The quick way: whole-file helpers

For small files, `std::fs` has one-liners:

```rust
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    fs::write("hello.txt", "first line\nsecond line\n")?; // create or overwrite
    let text = fs::read_to_string("hello.txt")?;           // read it all into a String
    for (i, line) in text.lines().enumerate() {
        println!("{}: {line}", i + 1);
    }
    fs::remove_file("hello.txt")?;
    Ok(())
}
```

Things to notice:

- Every I/O function returns `io::Result<T>`, which is just an alias for `Result<T, io::Error>`.
- `main` can return `io::Result<()>` (or any `Result<(), E>` where `E: Debug`). If it returns
  `Err`, Rust prints the error and the process exits with code **1**.
- So `?` works directly in `main`.
- `str::lines()` splits on `\n` (and strips a trailing `\r`), and doesn't yield a final empty line.

Paths can be `&str`, `String`, `&Path`, or `PathBuf`. The functions take `impl AsRef<Path>`,
which accepts all of those.

### What can go wrong?

```rust
use std::fs;
use std::io::ErrorKind;

fn main() {
    match fs::read_to_string("does-not-exist.txt") {
        Ok(text) => println!("{text}"),
        Err(e) if e.kind() == ErrorKind::NotFound => println!("no file yet, starting fresh"),
        Err(e) => println!("some other problem: {e}"),
    }
}
```

`io::Error::kind()` returns an `ErrorKind` enum (`NotFound`, `PermissionDenied`,
`AlreadyExists`, ...) that you can match on. You'll use exactly this pattern in the homework:
**a missing save file isn't an error, it just means "no tasks yet".**

---

## 2. `File`, `OpenOptions`, and buffering

When you need more control, open a `File` handle:

```rust
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() -> io::Result<()> {
    // Create (truncate) and write through a buffer
    let file = File::create("log.txt")?;
    let mut w = BufWriter::new(file);
    for i in 1..=3 {
        writeln!(w, "entry {i}")?;   // `writeln!` works on anything implementing `Write`
    }
    w.flush()?;                      // push buffered bytes to the OS, *and see any error*

    // Append without truncating
    let mut f = OpenOptions::new().append(true).create(true).open("log.txt")?;
    writeln!(f, "entry 4")?;

    // Read line by line, without loading the whole file at once
    let reader = BufReader::new(File::open("log.txt")?);
    for line in reader.lines() {
        let line = line?;            // each line is an io::Result<String>
        println!("read: {line}");
    }
    std::fs::remove_file("log.txt")
}
```

Why buffer? Each `write` on a raw `File` is a system call. `BufWriter` collects small writes in
memory and sends them in big chunks. `BufReader` does the same for reading, and adds the
`lines()` method through the `BufRead` trait.

> ⚠️ A `BufWriter` flushes when it's dropped, but **errors during that final flush are
> silently ignored**. Call `.flush()?` yourself when the data matters.

Note the `use std::io::{BufRead, Write}`. Methods like `lines()` and `write_all()` come from
**traits**, so the trait must be in scope to call them (lesson 9). If you get
"no method named `lines` found", you're probably missing `use std::io::BufRead;`.

### Atomic-ish saves

If your program crashes halfway through writing `tasks.db`, you could lose everything. A common
trick is to write to a temporary file and then **rename** it over the real one. A rename within
one filesystem is atomic on all mainstream OSes:

```rust
use std::fs;
use std::io;
use std::path::Path;

fn save_atomically(path: &Path, contents: &str) -> io::Result<()> {
    let tmp = path.with_extension("tmp");
    fs::write(&tmp, contents)?;
    fs::rename(&tmp, path)
}

fn main() -> io::Result<()> {
    save_atomically(Path::new("demo.db"), "hello\n")?;
    fs::remove_file("demo.db")
}
```

You'll use this in lesson 14.

---

## 3. `Path` and `PathBuf`

`Path` is to `PathBuf` what `str` is to `String`: a borrowed view vs an owned, growable value.
They handle OS differences like `/` vs `\` for you.

```rust
use std::path::{Path, PathBuf};

fn main() {
    let p = Path::new("data/tasks.db");
    println!("{:?}", p.file_name());   // Some("tasks.db")
    println!("{:?}", p.extension());   // Some("db")
    println!("{:?}", p.parent());      // Some("data")
    println!("{}", p.exists());        // false (probably!)

    let mut owned: PathBuf = PathBuf::from("data");
    owned.push("backup");
    owned.push("tasks.db");
    println!("{}", owned.display());   // data/backup/tasks.db
    println!("{}", owned.with_extension("bak").display()); // data/backup/tasks.bak

    let home_relative = Path::new("notes").join("todo.txt"); // join returns a new PathBuf
    println!("{}", home_relative.display());
}
```

Paths aren't guaranteed to be valid UTF-8, so they don't implement `Display` directly. Use
`.display()` to print them.

---

## 4. Command-line arguments

`std::env::args()` returns an iterator of `String`s. The first item is the program's own path:

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("program: {}", args[0]);
    for (i, arg) in args.iter().enumerate().skip(1) {
        println!("arg {i}: {arg}");
    }
}
```

With cargo, put your program's arguments after `--` so cargo doesn't try to interpret them:

```bash
cargo run -- --file my.db add Buy milk
```

### Hand-rolled flag parsing

For a small tool, a loop over the arguments is enough. The trick is using the iterator's
`next()` to grab a flag's **value**:

```rust
#[derive(Debug, Default)]
struct Options {
    file: Option<String>,
    rest: Vec<String>,
}

fn parse(args: impl Iterator<Item = String>) -> Result<Options, String> {
    let mut opts = Options::default();
    let mut args = args;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-f" | "--file" => {
                let value = args.next().ok_or("--file needs a path")?;
                opts.file = Some(value);
            }
            _ => opts.rest.push(arg),
        }
    }
    Ok(opts)
}

fn main() {
    match parse(std::env::args().skip(1)) {
        Ok(opts) => println!("{opts:?}"),
        Err(msg) => {
            eprintln!("error: {msg}");     // eprintln! prints to stderr
            std::process::exit(2);         // non-zero exit code = failure
        }
    }
}
```

- `eprintln!` writes to **stderr**, so error messages don't get mixed into output that someone
  might pipe to a file.
- `std::process::exit(code)` ends the program immediately. By convention `0` means success,
  `1` a general error, and `2` a usage error. (`exit` skips destructors, so only call it once
  everything is saved and flushed.)
- `ok_or("...")?` converts `Option` → `Result` and propagates. `?` turns the `&str` into a
  `String` because `String: From<&str>`.

Real-world tools use the [`clap`](https://docs.rs/clap) crate, which generates all of this plus
`--help` output. You'll get a chance to try it as a stretch goal in lesson 15.

---

## 5. Errors from many sources: `Box<dyn Error>`

A function that reads a file *and* parses numbers can fail in two different ways:
`io::Error` and `ParseIntError`. What should it return?

### Option A: `Box<dyn Error>`, the catch-all

```rust
use std::error::Error;
use std::fs;

fn total(path: &str) -> Result<i64, Box<dyn Error>> {
    let text = fs::read_to_string(path)?;     // io::Error → Box<dyn Error>
    let mut sum = 0;
    for line in text.lines() {
        sum += line.trim().parse::<i64>()?;   // ParseIntError → Box<dyn Error>
    }
    Ok(sum)
}

fn main() -> Result<(), Box<dyn Error>> {
    fs::write("nums.txt", "1\n2\n3\n")?;
    println!("{}", total("nums.txt")?);
    fs::remove_file("nums.txt")?;
    Ok(())
}
```

`dyn Error` means "some type that implements the `std::error::Error` trait". Which one exactly
isn't known until runtime. `Box` puts it on the heap, because different error types have
different sizes. Lesson 13 covers `dyn` in depth. For now, `?` converts any error type into
`Box<dyn Error>` automatically, even a plain message:

```rust
fn check(n: i32) -> Result<(), Box<dyn std::error::Error>> {
    if n < 0 {
        return Err("negative numbers not allowed".into()); // &str → Box<dyn Error>
    }
    Ok(())
}
```

**Pros:** effortless. **Cons:** callers can't easily `match` on *which* error happened.
That makes it great for `main` and application glue, and less great for library APIs.

### Option B: your own error enum that wraps others

This is what you did in lesson 7, extended to wrap errors from the standard library:

```rust
use std::error::Error;
use std::fmt;
use std::io;
use std::num::ParseIntError;

#[derive(Debug)]
enum LoadError {
    Io(io::Error),
    BadNumber { line: usize, source: ParseIntError },
}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoadError::Io(e) => write!(f, "I/O error: {e}"),
            LoadError::BadNumber { line, source } => write!(f, "line {line}: {source}"),
        }
    }
}

// Implementing the Error trait makes LoadError usable as a Box<dyn Error> too.
// All its methods have defaults, so an empty impl block is enough.
impl Error for LoadError {}

// Teach `?` how to turn an io::Error into a LoadError.
impl From<io::Error> for LoadError {
    fn from(e: io::Error) -> Self {
        LoadError::Io(e)
    }
}

fn load(path: &str) -> Result<Vec<i64>, LoadError> {
    let text = std::fs::read_to_string(path)?;   // uses From<io::Error>
    text.lines()
        .enumerate()
        .map(|(i, l)| {
            l.trim()
                .parse()
                .map_err(|source| LoadError::BadNumber { line: i + 1, source })
        })
        .collect()                                // Iterator<Result> → Result<Vec>
}

fn main() {
    std::fs::write("bad.txt", "1\ntwo\n").unwrap();
    match load("bad.txt") {
        Ok(v) => println!("{v:?}"),
        Err(e) => println!("Error: {e}"),          // Error: line 2: invalid digit found in string
    }
    std::fs::remove_file("bad.txt").unwrap();
}
```

How `?` works, precisely: `expr?` on an `Err(e)` does `return Err(From::from(e))`. Any error type
for which a `From` conversion exists is converted automatically.

**Rule of thumb:** library code (taskr's `storage` module) uses a precise enum. `main` uses
`Box<dyn Error>` and just prints whatever comes up.

---

## Practice problems

All problems live in `exercises/lesson-12/`. Run them with:

```bash
cd exercises && cargo test -p lesson-12
```

1. `write_lines`: create/overwrite a file with given lines.
2. `count_lines`: `BufReader` + `lines()`, propagating I/O errors.
3. `append_line`: `OpenOptions` in append mode.
4. `sum_file`: mixing two error types with `Box<dyn Error>`.
5. `LoadError` + `load_scores`: implement `From<io::Error>`, `Display`, and a loader that reports
   1-based line numbers.
6. `parse_args`: hand-rolled flag parsing with values and error messages.
7. `change_extension`: `Path` methods.

Written questions in `exercises/lesson-12/ANSWERS.md`:

- **Q1** fix `?` in `main`
- **Q2** `Box<dyn Error>` vs a custom enum
- **Q3** buffered writing
- **Q4** what `env::args()` contains

---

## Homework — taskr step 12

### Starting point

After lesson 11, `taskr` has a library (`task`, `task_list`, `command`, `error` modules) and a
REPL binary. Commands: `add` (with `#tags` and `!priority`), `done`, `remove`, `show`, `edit`,
`list` with combinable filters (`pending`/`done`/`#tag`/`!priority`), `search`, `clear`, `tags`,
`stats`, `sort`, `help`, `quit`. It has plenty of tests, and the code uses iterators throughout.

But every time you quit, **all your tasks vanish**. Time to fix that.

### Requirements

1. **New module `src/storage.rs`** with:
   ```rust
   pub enum StorageError {
       Io(std::io::Error),
       Parse { line: usize, msg: String },
   }
   pub fn load(path: &Path) -> Result<TaskList, StorageError>;
   pub fn save(path: &Path, list: &TaskList) -> Result<(), StorageError>;
   ```
   `StorageError` implements `Debug`, `Display`, `std::error::Error`, and `From<io::Error>`.

2. **File format.** One task per line, fields separated by `|`:
   ```text
   id|priority|done|tag1,tag2|title
   ```
   Example `tasks.db`:
   ```text
   1|high|false|errand,home|Buy milk
   2|medium|true||Write report
   ```
   - `priority` is the same text your `Priority` `FromStr` accepts (reuse it!).
   - `done` is `true`/`false`.
   - Tags are comma-separated, and empty if there are none.
   - The title is the **last** field, so `splitn(5, '|')` keeps everything after the 4th `|`
     together. Even so, **reject `|` in titles**: make `add` and `edit` return a `ParseError`
     (e.g. `InvalidTitle`) if the title contains `|`. That keeps the format unambiguous.

3. **Loading.**
   - A missing file means an empty list (check `e.kind() == io::ErrorKind::NotFound`). That's not an error.
   - Blank lines are ignored.
   - A malformed line gives `StorageError::Parse { line, msg }` with a 1-based line number, for
     example: `tasks.db line 3: invalid priority 'urgent'`.
   - After loading, `next_id` must be **max id + 1** (or 1 for an empty list). Otherwise new
     tasks would reuse ids. You'll probably need a constructor like
     `TaskList::from_tasks(tasks: Vec<Task>) -> TaskList`, or a method that inserts a task with a known id.

4. **Saving.** After every command that changes the list (`add`, `done`, `remove`, `edit`,
   `clear`, `sort` if it reorders storage), save the whole list. Read-only commands (`list`,
   `show`, `search`, `tags`, `stats`, `help`) don't save. A good way to do this is a method
   `Command::is_mutating(&self) -> bool`. Saving failures are printed as errors but don't crash the REPL.

5. **Command-line arguments.**
   - `taskr` uses `tasks.db` in the current directory.
   - `taskr --file <path>` (or `-f <path>`) uses another file.
   - Any remaining arguments are joined with spaces and run as **one command** (one-shot mode),
     then the program saves (if mutating) and exits:
     ```bash
     taskr add Buy milk #home !high
     taskr -f work.db list pending
     ```
   - No remaining arguments → start the REPL as before.
   - `--file` with no value, or an unknown flag → print a usage message to **stderr** and exit
     with code 2.

6. **`main` returns `Result<(), Box<dyn Error>>`.** A load failure at startup (e.g. a corrupt
   file) should print a clear message and exit non-zero. Don't silently start with an empty
   list and then overwrite the user's file on the next save!

7. **Tests** (in `storage.rs` and/or `tests/storage.rs`):
   - round trip: save a list with several tasks (some done, some tagged), load it, and compare
   - loading a missing file gives an empty list
   - a malformed line reports the right line number
   - `next_id` continues after the max id
   - `add` with a `|` in the title is rejected

   Use unique temp paths (`std::env::temp_dir().join(...)`) and delete them at the end.

### Example session

```text
$ cargo run -q -- add Buy milk #home !high
Added task 1.
$ cargo run -q -- add Write report #work
Added task 2.
$ cargo run -q -- done 2
Completed task 2.
$ cat tasks.db
1|high|false|home|Buy milk
2|medium|true|work|Write report
$ cargo run -q -- list
  1. [ ] (High) Buy milk #home
  2. [x] (Medium) Write report #work
$ cargo run -q -- -f other.db list
No tasks.
$ cargo run -q -- --file
usage: taskr [--file <path>] [command...]
$ echo $?
2
$ echo "7|urgent|false||Oops" > broken.db
$ cargo run -q -- -f broken.db list
Error: broken.db line 1: invalid priority 'urgent'
```

### Hints

- To convert a line to a `Task`, build it up with `let mut parts = line.splitn(5, '|');` and call
  `parts.next().ok_or(...)?` for each field. Wrap every failure with the line number using `map_err`.
- Give `Task` (or `storage.rs`) a `to_line(&self) -> String` and a `from_line(&str) -> Result<Task, String>`.
  Then `load` only has to add line numbers to the error.
- Serialization needs read access to private fields. Either put the conversion in `task.rs`, or
  use the accessor methods you added in lesson 8.
- Write with a `BufWriter` and call `flush()?` at the end, or build one `String` and use `fs::write`.
- `tasks.db` is already in the repo's `.gitignore` (`*.db`), so your real tasks won't end up in git.
- For one-shot mode: `rest.join(" ").parse::<Command>()`, then use the same "execute command"
  function the REPL uses. If you don't have one yet, extract it: `fn execute(list: &mut TaskList, cmd: Command) -> ...`.

### Stretch goals

- `--help` flag that prints the usage message and exits with code 0.
- Respect a `TASKR_FILE` environment variable (`std::env::var`) when `--file` isn't given.
- Escape `|` and `\` in titles instead of rejecting them.

---

## Submitting for review

1. Make sure everything passes:
   ```bash
   (cd exercises && cargo test -p lesson-12)
   (cd taskr && cargo test)
   ```
2. Commit on your `work-12` branch:
   ```bash
   git add -A && git commit -m "Lesson 12: files & CLI"
   ```
3. Ask Claude: **"Please review lesson 12"**.

---

## Further reading

- The Rust Book, ch. 12: [An I/O Project: Building a Command Line Program](https://doc.rust-lang.org/book/ch12-00-an-io-project.html). Very close to what you're doing!
- The Rust Book, ch. 9.2: [Recoverable Errors with `Result`](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html) (re-read the `?` / `Box<dyn Error>` parts)
- Rust by Example: [File I/O](https://doc.rust-lang.org/rust-by-example/std_misc/file.html), [Program arguments](https://doc.rust-lang.org/rust-by-example/std_misc/arg.html)
- std docs: [`std::fs`](https://doc.rust-lang.org/std/fs/index.html), [`std::io`](https://doc.rust-lang.org/std/io/index.html), [`std::path::Path`](https://doc.rust-lang.org/std/path/struct.Path.html)
- [Command Line Applications in Rust](https://rust-cli.github.io/book/), a free short book
