# Learn Rust: a 15-lesson course

This course teaches Rust from first principles. Each lesson has a short practice set and a homework step. All of the homework goes into one project, **`taskr`**, a command-line task tracker. You start it in Lesson 1 and keep adding to it. By Lesson 15 it saves tasks to disk, supports tags and filters, exports files, has undo, saves in a background thread, and comes with a test suite.

## What's in each lesson

Every lesson lives in `lessons/NN-*.md` and has these parts:

| Section | What it is |
|---|---|
| **What you'll learn** | The goals for the lesson |
| **Concept sections** | Explanations with runnable examples, plus the compiler errors you're likely to hit |
| **Practice problems** | Small, separate problems in `exercises/lesson-NN/`. Some are functions to implement (run the tests to check them) and some are written questions in `ANSWERS.md` |
| **Homework: taskr step NN** | The next feature for `taskr`, building on the previous lesson's homework |
| **Submitting for review** | How to hand your work to Claude for review |
| **Further reading** | Matching chapters in *The Rust Programming Language* and other references |

## Repository layout

```
README.md                  ← you are here
lessons/                   ← one markdown file per lesson (added branch by branch)
exercises/
  Cargo.toml               ← a Cargo workspace; every lesson-NN crate joins it automatically
  lesson-NN/
    src/lib.rs             ← practice problems: replace each todo!() so the tests pass
    ANSWERS.md             ← write your answers to the conceptual questions here
taskr/                     ← YOUR homework project (you create it in Lesson 1)
```

## How the branches work

Each lesson is a git branch. Each lesson branch builds on the one before it, so `lesson-05` contains lessons 1–5.

```
main ── lesson-01 ── lesson-02 ── lesson-03 ── … ── lesson-15     (course material, read-only)
            │            │            │
            ▼            ▼            ▼
         work-01 ──▶ work-02 ──▶ work-03 ──▶ …                    (your work)
```

You do your work on your own **`work-NN`** branches. Each one starts from your previous work branch, and you merge in the next lesson's material. That way your `taskr` code carries forward from lesson to lesson, and your solution for every lesson stays on its own branch.

The lesson branches only add new files under `lessons/` and `exercises/lesson-NN/`. You only edit `taskr/` and the exercise files, so the merges never conflict.

## Commands

### First-time setup

Install Rust if you don't have it yet (https://rustup.rs), then:

```bash
cd ~/Workspaces/rust-tutorials
git switch -c work-01 lesson-01      # create your Lesson 1 work branch
```

### While working on a lesson

```bash
# read the lesson
less lessons/01-getting-started.md   # or open it in your editor / markdown viewer

# run the practice problems for lesson NN (from the exercises/ folder)
cd exercises
cargo test -p lesson-01
cd ..

# build and run your homework
cd taskr
cargo run
cd ..
```

### Finishing a lesson and asking for a review

```bash
git add -A
git commit -m "Lesson 01 complete"
```

Then ask Claude: **"Please review lesson 01."** The review covers your practice solutions, your `ANSWERS.md`, and your `taskr` changes. If the review suggests fixes, commit them on the same branch.

### Moving on to the next lesson

Run this from your finished `work-NN` branch. The example goes from Lesson 1 to Lesson 2:

```bash
git switch -c work-02                # new branch that starts from your work-01
git merge --no-edit lesson-02        # bring in the Lesson 2 material
```

General form: `git switch -c work-<NEXT>` then `git merge --no-edit lesson-<NEXT>`.

### Other handy commands

```bash
git branch                           # list branches (yours and the lessons)
git switch work-03                   # go back to an earlier piece of your work
git diff work-02 work-03 -- taskr    # see exactly what you changed in taskr during lesson 3
git log --oneline --graph --all      # picture of the whole history
```

## Lessons

| # | Lesson | Topics | taskr homework |
|---|---|---|---|
| 01 | [Getting Started](https://github.com/lawnchairsociety/rust-tutorials/blob/lesson-01/lessons/01-getting-started.md) | cargo, variables, mutability, scalar & compound types, `println!`, constants, shadowing | Create `taskr`; print a banner and hardcoded tasks |
| 02 | [Functions & Control Flow](https://github.com/lawnchairsociety/rust-tutorials/blob/lesson-02/lessons/02-functions-and-control-flow.md) | functions, expressions vs statements, `if`, loops, ranges, basic `match`, reading stdin | Interactive prompt with `list` / `help` / `quit` |
| 03 | [Ownership & Borrowing](https://github.com/lawnchairsociety/rust-tutorials/blob/lesson-03/lessons/03-ownership-and-borrowing.md) | ownership, moves, `Clone`/`Copy`, references, borrowing rules, `String` vs `&str`, slices | `add` and `done` commands backed by a `Vec` |
| 04 | [Structs & Methods](https://github.com/lawnchairsociety/rust-tutorials/blob/lesson-04/lessons/04-structs-and-methods.md) | structs, tuple/unit structs, `impl`, methods, associated functions, `derive` | `Task` and `TaskList` structs with stable IDs |
| 05 | [Enums & Pattern Matching](https://github.com/lawnchairsociety/rust-tutorials/blob/lesson-05/lessons/05-enums-and-pattern-matching.md) | enums with data, `match`, guards, `if let`, `let else`, `Option` | `Priority` and `Command` enums; `remove` / `show` |
| 06 | [Common Collections](https://github.com/lawnchairsociety/rust-tutorials/blob/lesson-06/lessons/06-collections.md) | `Vec`, `String` & UTF-8, `HashMap`, entry API | `#tags`, `tags`, `list #tag`, `stats` |
| 07 | [Error Handling](https://github.com/lawnchairsociety/rust-tutorials/blob/lesson-07/lessons/07-error-handling.md) | `panic!`, `Result`, `?`, custom error types, `Display` | Typed `ParseError` / `TaskError`; never panic on input |
| 08 | [Packages, Crates & Modules](https://github.com/lawnchairsociety/rust-tutorials/blob/lesson-08/lessons/08-modules-and-crates.md) | `lib.rs` + `main.rs`, `mod`, `pub`, `use`, file modules, dependencies | Split `taskr` into a library plus a thin binary |
| 09 | [Traits & Generics](https://github.com/lawnchairsociety/rust-tutorials/blob/lesson-09/lessons/09-traits-and-generics.md) | generics, traits, bounds, `impl Trait`, std traits (`Display`, `FromStr`, `Ord`, `Default`, `From`) | `Display`/`FromStr` impls, `sort` command |
| 10 | [Testing](https://github.com/lawnchairsociety/rust-tutorials/blob/lesson-10/lessons/10-testing.md) | unit, integration & doc tests, `should_panic`, TDD | 20+ tests; write the `edit` command test-first |
| 11 | [Closures & Iterators](https://github.com/lawnchairsociety/rust-tutorials/blob/lesson-11/lessons/11-closures-and-iterators.md) | closures, `Fn` traits, iterator adapters & consumers | Combinable `list` filters, `search`, `clear` |
| 12 | [Files & the Command Line](https://github.com/lawnchairsociety/rust-tutorials/blob/lesson-12/lessons/12-files-and-cli.md) | `std::fs`, buffered I/O, `env::args`, `Box<dyn Error>` | Save/load tasks to a file; `--file`; one-shot mode |
| 13 | [Lifetimes, Trait Objects & Smart Pointers](https://github.com/lawnchairsociety/rust-tutorials/blob/lesson-13/lessons/13-lifetimes-and-smart-pointers.md) | lifetime annotations, `dyn Trait`, `Box`, `Rc`, `RefCell` | `Report<'a>`, Markdown/CSV `export`, `undo` |
| 14 | [Concurrency](https://github.com/lawnchairsociety/rust-tutorials/blob/lesson-14/lessons/14-concurrency.md) | threads, channels, `Arc`, `Mutex`, scoped threads | Background saver thread with clean shutdown |
| 15 | [Polish & Next Steps](https://github.com/lawnchairsociety/rust-tutorials/blob/lesson-15/lessons/15-polish-and-next-steps.md) | rustfmt, clippy, docs, release builds, the crate ecosystem | Lint-clean, documented, installable `taskr` |


## Tips

- **Read the compiler errors.** Rust's error messages are unusually helpful and often include the fix. Most lessons show you the errors you're likely to see.
- **Expect warnings in the practice crates.** Until you implement a function, the compiler warns about unused variables. Those warnings go away as you fill in each `todo!()`.
- **Run one test at a time** with `cargo test -p lesson-03 longest_word`. Cargo only runs tests whose names contain the filter text.
- **Keep the official book open:** https://doc.rust-lang.org/book/. The "Further reading" section in each lesson points to the matching chapter.
