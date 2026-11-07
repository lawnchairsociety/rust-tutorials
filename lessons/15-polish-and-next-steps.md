# Lesson 15 — Polish & Next Steps

> Use Rust's tooling to make your code idiomatic, documented and installable, then get a map of where to go next. Estimated time: 2–3 hours, plus as much time exploring as you like.

## What you'll learn

- Consistent formatting with `rustfmt` (`cargo fmt`)
- Linting with `clippy`: default and pedantic lints, and when to `#[allow]`
- Documentation comments (`//!`, `///`), doc tests, and `cargo doc`
- Release builds and Cargo profiles
- Installing your own tools with `cargo install --path`
- A tour of the crates ecosystem (`clap`, `serde`, `anyhow`/`thiserror`, `tokio`, ...)
- What to learn next: async, macros, `unsafe`, and more

---

## 1. Formatting: `cargo fmt`

Rust has one official style, and `rustfmt` applies it automatically:

```bash
cargo fmt            # reformat every file in the package
cargo fmt -- --check # only report; exits non-zero if anything would change (great for CI)
```

The point isn't that the style is perfect. The point is that **nobody argues about it**, and
every Rust codebase looks familiar. Most editors with rust-analyzer can format on save.

You can tweak it with a `rustfmt.toml` file (e.g. `max_width = 110`). Most projects keep the defaults.

---

## 2. Linting: `cargo clippy`

Clippy is a collection of 700+ lints that catch bugs, unidiomatic code and performance traps.

```bash
cargo clippy                           # default lints (the "clippy::all" group)
cargo clippy -- -D warnings            # treat warnings as errors (CI)
cargo clippy -- -W clippy::pedantic    # opt into stricter, more opinionated lints
cargo clippy --fix                     # apply machine-applicable fixes automatically (commit first!)
```

Some lints you'll meet constantly:

| Lint | Instead of... | write... |
|---|---|---|
| `needless_return` | `return x;` as the last line | `x` |
| `len_zero` | `v.len() == 0` | `v.is_empty()` |
| `ptr_arg` | `fn f(v: &Vec<T>)`, `fn g(s: &String)` | `fn f(v: &[T])`, `fn g(s: &str)` |
| `bool_comparison` | `if done == false` | `if !done` |
| `needless_range_loop` | `for i in 0..v.len() { v[i] ... }` | `for x in &v` / `iter_mut()` |
| `new_without_default` | `fn new()` with no `Default` impl | add `impl Default` (or derive it) |
| `redundant_clone` | `.clone()` on a value never used again | just move it |
| `manual_range_contains` | `x >= lo && x <= hi` | `(lo..=hi).contains(&x)` |

Every warning links to an explanation: `https://rust-lang.github.io/rust-clippy/master/index.html#<lint_name>`.

### Allowing a lint deliberately

Sometimes a lint is wrong for your situation. Allow it as narrowly as possible, and say why:

```rust
#[allow(clippy::cast_possible_truncation)] // ids are always < 2^32 by construction
fn to_id(n: usize) -> u32 {
    n as u32
}
```

You can also configure lints for a whole package in `Cargo.toml`:

```toml
[lints.clippy]
pedantic = { level = "warn", priority = -1 }
module_name_repetitions = "allow"
```

---

## 3. Documentation

### Doc comments

- `///` documents the item that **follows** it (a function, struct, enum, field, method...).
- `//!` documents the item that **contains** it. At the top of `lib.rs` that's the whole crate.

Doc comments are Markdown. Conventional sections are `# Examples`, `# Errors` (when it returns
`Result`), `# Panics` (if it can panic), and `# Safety` (for `unsafe` functions).

````rust
//! # taskr
//!
//! A tiny command-line task tracker.

/// Parses a priority name such as `"high"` or `"low"`.
///
/// # Errors
///
/// Returns [`ParseError::InvalidPriority`] if `s` isn't a known priority.
///
/// # Examples
///
/// ```
/// use taskr::{parse_priority, Priority};
///
/// assert_eq!(parse_priority("high").unwrap(), Priority::High);
/// assert!(parse_priority("urgent").is_err());
/// ```
pub fn parse_priority(s: &str) -> Result<Priority, ParseError> {
    // ...
}
````

Notes:

- Square brackets around a path (`` [`ParseError::InvalidPriority`] ``) make an **intra-doc
  link** that rustdoc resolves for you.
- The code block in `# Examples` is a **doc test**. `cargo test` compiles and runs it as if it
  were an external user of your crate, so it must `use` your crate's public items.
- Doc tests run only for **library** crates. That's another reason the lib/bin split from lesson 8 pays off.
- Lines starting with `# ` inside a doc test are hidden in the rendered docs but still run.
  That's handy for setup code:

  ````rust
  /// ```
  /// # use taskr::TaskList;
  /// let mut list = TaskList::default();
  /// let id = list.add("Buy milk").unwrap();
  /// assert_eq!(id, 1);
  /// ```
  ````
- Mark a block ` ```no_run ` to compile it without running it (e.g. it touches the filesystem),
  ` ```should_panic ` if it should panic, or ` ```text ` for non-Rust content.

### Building the docs

```bash
cargo doc --open              # your crate + all dependencies
cargo doc --no-deps --open    # just your crate
```

It produces the same kind of site as [docs.rs](https://docs.rs), which hosts docs for every
crate published to crates.io.

---

## 4. Release builds and profiles

```bash
cargo build              # debug profile: fast compile, slow code, debug info, overflow checks
cargo build --release    # release profile: optimized (opt-level 3), no overflow checks
cargo run --release
```

Release binaries land in `target/release/`. Debug builds can be **10–100× slower** for
compute-heavy code, so always benchmark in release mode.

Profiles are configurable in `Cargo.toml`:

```toml
[profile.release]
lto = true           # link-time optimization: smaller, faster binary, slower build
codegen-units = 1    # better optimization, slower build
strip = true         # strip symbols from the binary → smaller file
panic = "abort"      # don't unwind on panic → smaller binary
```

(Integer overflow panics in debug mode but wraps silently in release mode. If overflow matters,
use the explicit methods like `checked_add`, `saturating_add` and `wrapping_add`.)

## 5. Installing your own tools

```bash
cargo install --path taskr      # builds in release mode, copies the binary to ~/.cargo/bin
taskr list                       # now available everywhere (if ~/.cargo/bin is on your PATH)
cargo uninstall taskr
```

`cargo install <crate-name>` does the same for anything on crates.io, e.g.
`cargo install ripgrep`.

---

## 6. A tour of the ecosystem

You built `taskr` with **zero dependencies** on purpose, so you'd learn what the standard
library gives you. Real projects lean on crates. The ones you'll see everywhere:

| Need | Crate(s) | What it does |
|---|---|---|
| CLI parsing | [`clap`](https://docs.rs/clap) | `#[derive(Parser)]` on a struct gives you parsing, `--help`, subcommands and errors |
| Serialization | [`serde`](https://serde.rs) + [`serde_json`](https://docs.rs/serde_json) / `toml` / ... | `#[derive(Serialize, Deserialize)]` and you're done |
| App errors | [`anyhow`](https://docs.rs/anyhow) | a better `Box<dyn Error>` with `.context("while loading tasks")` |
| Library errors | [`thiserror`](https://docs.rs/thiserror) | `#[derive(Error)]` writes your `Display`/`Error`/`From` impls |
| Async runtime | [`tokio`](https://tokio.rs) | async I/O, timers and tasks for networked services |
| HTTP | [`reqwest`](https://docs.rs/reqwest) (client), [`axum`](https://docs.rs/axum) (server) | |
| Data parallelism | [`rayon`](https://docs.rs/rayon) | `.par_iter()`: lesson 14's thread splitting in one word |
| Logging | [`tracing`](https://docs.rs/tracing) / [`log`](https://docs.rs/log) | structured, leveled logs |
| Dates & times | [`chrono`](https://docs.rs/chrono) / [`jiff`](https://docs.rs/jiff) | |
| Testing | [`proptest`](https://docs.rs/proptest), [`insta`](https://docs.rs/insta) | property-based and snapshot tests |

Add a dependency with `cargo add`. It updates `Cargo.toml` with the latest compatible version:

```bash
cargo add clap --features derive
cargo add serde --features derive
cargo add serde_json
```

Here's what `thiserror` would do to your lesson 7 error type:

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TaskError {
    #[error("no task with id {0}")]
    NotFound(u32),
    #[error("task {0} is already done")]
    AlreadyDone(u32),
}
```

Now that you've written those impls by hand, you know exactly what the macro generates.

To find crates, try [crates.io](https://crates.io), [lib.rs](https://lib.rs) (curated
categories), and [blessed.rs](https://blessed.rs) (a list of well-regarded crates).

---

## 7. What to learn next

You now know the core of Rust. Some directions, roughly in order of usefulness:

1. **Async/await**: `async fn`, `Future`s, and `tokio`. Essential for network services.
   The [Tokio tutorial](https://tokio.rs/tokio/tutorial) is excellent.
2. **More traits and generics**: associated types, iterator implementations, `impl Trait`
   everywhere, blanket impls, operator overloading (`std::ops`). See the Rust Book ch. 20.
3. **Macros**: `macro_rules!` for declarative macros, then procedural/derive macros.
   [The Little Book of Rust Macros](https://veykril.github.io/tlborm/).
4. **Unsafe Rust and FFI**: raw pointers, calling C, and what `unsafe` actually promises.
   Book ch. 20.1, then [the Rustonomicon](https://doc.rust-lang.org/nomicon/).
5. **Performance**: profiling (`cargo flamegraph`), benchmarking (`criterion`), allocation awareness.
6. **Practice**: [Rustlings](https://github.com/rust-lang/rustlings) (small exercises),
   [Exercism's Rust track](https://exercism.org/tracks/rust) (mentored), and
   [Advent of Code](https://adventofcode.com) (puzzles every December).
7. **Build something real**: a web API with `axum`, a TUI with `ratatui`, a game with `bevy`,
   or contribute to an open-source crate you use.

---

## Practice problems

All problems live in `exercises/lesson-15/`.

**Part A: Clippy cleanup (`src/messy.rs`)**

1. Run `cd exercises && cargo clippy -p lesson-15`. Fix every warning in `messy.rs` *without*
   `#[allow]`, keeping `cargo test -p lesson-15` passing for its two tests.
2. Then run `cargo clippy -p lesson-15 -- -W clippy::pedantic` and fix or justify what it reports.
3. Run `cargo fmt -p lesson-15`.

**Part B: Implement and document (`src/lib.rs`)**

4. `slugify`
5. `truncate` (characters, not bytes! Remember lesson 6's UTF-8 discussion.)
6. `parse_duration`

For each of 4–6, add a `# Examples` section with at least one doc test (`parse_duration` needs a
failing-input example too). `cargo test -p lesson-15` must show your doc tests running in the
`Doc-tests lesson_15` section. Then look at the result with `cargo doc -p lesson-15 --no-deps --open`.

Written questions in `exercises/lesson-15/ANSWERS.md`:

- **Q1** three clippy fixes explained
- **Q2** a pedantic lint you fixed or allowed
- **Q3** why doc tests matter
- **Q4** debug vs release
- **Q5** looking back over the course

---

## Homework — taskr step 15 (the final step!)

### Starting point

After lesson 14, `taskr` is a full-featured task tracker:

- library modules: `task`, `task_list`, `command`, `error`, `storage`, `report`, `export`, `saver`
- persistence to `tasks.db` (or `--file`), with atomic writes by a background saver thread that
  is joined on exit
- one-shot mode and the REPL, with `add`, `done`, `remove`, `show`, `edit`, `list` (filters),
  `search`, `clear`, `tags`, `stats`, `sort`, `export`, `undo`, `help`, `quit`
- a solid test suite

Now make it something you'd be proud to publish.

### Requirements

1. **Format.** `cargo fmt` and commit, so the formatting change is separate from your other changes.
2. **Clippy-clean.** `cargo clippy --all-targets -- -D warnings` passes (tests included).
3. **Pedantic pass.** Run `cargo clippy --all-targets -- -W clippy::pedantic`. Fix what you
   agree with. For anything you choose to allow, add a narrowly scoped `#[allow(...)]` with a
   comment explaining why, or a `[lints.clippy]` entry in `Cargo.toml`. List what you allowed
   and why in your commit message.
4. **Documentation.**
   - A crate-level `//!` doc comment in `lib.rs`: what taskr is, a module overview, and a short
     doc test that creates a `TaskList`, adds a task and completes it.
   - `///` on **every** public item: types, functions, methods, enum variants and public fields.
     Use `# Errors` on functions returning `Result`.
   - At least **five** doc tests in total, across different modules.
   - Add `#![warn(missing_docs)]` at the top of `lib.rs` to have the compiler enforce this.
   - `cargo doc --no-deps --open` and read through it like a stranger would.
5. **`taskr/README.md`**: what it is, how to build/install it, every command with an example,
   the `--file` option and one-shot mode, the file format, and how to run the tests.
6. **Install and use it.** `cargo install --path taskr`, then use your installed `taskr` for real
   tasks for a few days. Fix anything that annoys you!
7. **Release profile.** Add a `[profile.release]` section (at least `lto = true` and
   `strip = true`) and compare the binary size before and after (`ls -lh target/release/taskr`).
   Put the numbers in your commit message.

### Example

```text
$ cargo clippy --all-targets -- -D warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.42s
$ cargo test 2>&1 | grep "test result"
test result: ok. 41 passed; 0 failed; 0 ignored; ...
test result: ok. 6 passed; 0 failed; 0 ignored; ...
test result: ok. 7 passed; 0 failed; 0 ignored; ...    ← doc tests
$ cargo install --path .
  Installed package `taskr v0.1.0` (executable `taskr`)
$ cd ~ && taskr add Celebrate finishing the Rust course #fun !high
Added task 1.
```

### Hints

- Do the steps in order: fmt, then clippy, then docs. Commit after each step, so a reviewer
  (and future you) can follow what changed.
- `cargo clippy --fix --allow-dirty` can do much of the mechanical work. Review the diff afterwards.
- Doc tests that touch the filesystem should use a temp path, or be marked `no_run`.
- Common pedantic lints you'll see: `must_use_candidate` (add `#[must_use]` to pure functions
  returning values), `missing_errors_doc` (add `# Errors`), `doc_markdown` (put code in
  backticks), `module_name_repetitions` (often reasonable to allow).
- `#![warn(missing_docs)]` will flood you with warnings at first. That's your to-do list.

### Stretch goals (each of these adds dependencies, which is fine now)

- **clap**: replace the hand-written argument parsing with `#[derive(Parser)]`, including
  subcommands (`taskr add ...`, `taskr list --pending --tag home`). You get `--help` for free.
- **serde**: add a `--format json` option to storage using `serde_json`, keeping the old `|`
  format as the default. How much of `storage.rs` does it replace?
- **thiserror/anyhow**: replace your hand-written `Display`/`Error`/`From` impls with
  `thiserror`, and `Box<dyn Error>` in `main` with `anyhow::Result` plus `.context(...)`.
- **Due dates**: add `due:2026-12-01` to `add`, and an `overdue` filter (use `jiff` or `chrono`).
- **Publish**: push taskr to its own GitHub repo, with a CI workflow running fmt, clippy and tests.

---

## Submitting for review

1. Make sure everything passes:
   ```bash
   (cd exercises && cargo test -p lesson-15 && cargo clippy -p lesson-15 -- -D warnings)
   (cd taskr && cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test)
   ```
2. Commit on your `work-15` branch:
   ```bash
   git add -A && git commit -m "Lesson 15: polish, docs, release"
   ```
3. Ask Claude: **"Please review lesson 15"**. This final review will also look back over taskr as
   a whole.

**Congratulations, you've finished the course!** 🦀

---

## Further reading

- [The rustfmt book](https://rust-lang.github.io/rustfmt/) and [Clippy's lint list](https://rust-lang.github.io/rust-clippy/master/index.html)
- [The rustdoc book](https://doc.rust-lang.org/rustdoc/), especially "How to write documentation" and "Documentation tests"
- The Rust Book, ch. 14: [More about Cargo and Crates.io](https://doc.rust-lang.org/book/ch14-00-more-about-cargo.html)
- [The Cargo Book: Profiles](https://doc.rust-lang.org/cargo/reference/profiles.html)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/), a checklist for well-designed public APIs
- [Rust Design Patterns](https://rust-unofficial.github.io/patterns/)
- [This Week in Rust](https://this-week-in-rust.org), a weekly newsletter to keep learning
