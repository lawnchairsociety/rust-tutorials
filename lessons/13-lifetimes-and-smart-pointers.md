# Lesson 13 — Lifetimes, Trait Objects & Smart Pointers

> Learn how the compiler reasons about how long references live, how to treat different types uniformly at runtime with `dyn Trait`, and Rust's smart pointers `Box`, `Rc` and `RefCell`. Estimated time: 4–5 hours. This is the most conceptual lesson so far, so take it slowly.

## What you'll learn

- What lifetimes are and why the borrow checker needs them
- Lifetime annotation syntax: `'a` in functions, structs and `impl` blocks
- The three **lifetime elision** rules (why you rarely have to write lifetimes)
- `'static`
- Trait objects: `dyn Trait`, `&dyn Trait`, `Box<dyn Trait>`, and static vs dynamic dispatch
- `Box<T>` for heap allocation and recursive types
- `Rc<T>` for shared ownership, `RefCell<T>` for interior mutability, and `Rc<RefCell<T>>`
- The `Drop` trait

---

## 1. Lifetimes: why they exist

Every reference in Rust has a **lifetime**: the region of code during which it's valid. The
borrow checker's whole job is to make sure no reference outlives the value it points to:

```rust
fn main() {
    let r;
    {
        let x = 5;
        r = &x;          // ERROR: `x` does not live long enough
    }                    // x is dropped here...
    println!("{r}");     // ...but r would still point at it
}
```

Inside one function, the compiler works this out on its own. **Across function boundaries** it
needs help. Look at this function:

```rust
fn longer(a: &str, b: &str) -> &str {    // ERROR: missing lifetime specifier
    if a.len() >= b.len() { a } else { b }
}
```

The caller needs to know how long the returned reference stays valid. Does it borrow from `a`,
or from `b`? The compiler only looks at **signatures**, not bodies, when checking callers, so the
signature must say. Annotations do that:

```rust
fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}
```

Read it as: "for some lifetime `'a`, both inputs live at least as long as `'a`, and the output
is valid for `'a`." In practice `'a` becomes the **shorter** of the two input lifetimes.

**Key idea:** lifetime annotations don't change how long anything lives. They *describe*
relationships between references so the compiler can check every caller.

```rust
fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

fn main() {
    let a = String::from("a long string");
    {
        let b = String::from("short");
        let result = longer(&a, &b);
        println!("{result}");        // fine: used while both a and b are alive
    }
}
```

### Not every parameter needs to be tied

If the output only ever borrows from one parameter, tie only that one:

```rust
fn first_line<'a>(text: &'a str, _prefix: &str) -> &'a str {
    text.lines().next().unwrap_or("")
}
```

Now callers can drop `_prefix` immediately and keep using the result.

### Returning references to locals is impossible

```rust
fn make_greeting<'a>() -> &'a str {
    let s = String::from("hi");
    &s // ERROR: returns a reference to data owned by the current function
}
```

No annotation can fix this, because `s` is destroyed when the function returns. Return an
owned `String` instead.

---

## 2. Lifetime elision: why you rarely write them

You've written dozens of functions taking and returning references without annotations.
That works because the compiler applies three **elision rules**:

1. Each reference parameter gets its own lifetime: `fn f(a: &str, b: &str)` becomes `fn f<'a, 'b>(a: &'a str, b: &'b str)`.
2. If there is **exactly one** input lifetime, it's assigned to all output references:
   `fn first_word(s: &str) -> &str` becomes `fn first_word<'a>(s: &'a str) -> &'a str`.
3. If there are several input lifetimes but one of them is `&self` or `&mut self`, the
   lifetime of `self` is assigned to all outputs. That's why methods returning references to
   fields "just work".

If the rules don't determine every output lifetime, you get "missing lifetime specifier" and
must annotate. That's what happened with `longer` above.

This is why the `TaskList::filter(&self, pred) -> Vec<&Task>` method from lesson 11 needed no
annotations: by rule 3, the `&Task`s borrow from `self`.

---

## 3. Structs that hold references

A struct can store a reference, but then it must declare a lifetime parameter. That promises
the struct won't outlive the data it borrows:

```rust
struct Excerpt<'a> {
    part: &'a str,
}

impl<'a> Excerpt<'a> {
    fn new(text: &'a str) -> Self {
        let part = text.split('.').next().unwrap_or("");
        Excerpt { part }
    }

    // Elision rule 3: returns something borrowing from self
    fn part(&self) -> &str {
        self.part
    }

    // Explicit: the result borrows from the *original text*, not from self
    fn first_word(&self) -> &'a str {
        self.part.split_whitespace().next().unwrap_or("")
    }
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let word;
    {
        let ex = Excerpt::new(&novel);
        println!("{}", ex.part());
        word = ex.first_word();     // fine: borrows from `novel`, not from `ex`
    }
    println!("{word}");             // Call
}
```

`impl<'a> Excerpt<'a>` reads "for any lifetime `'a`, here are methods for `Excerpt<'a>`". It
works the same way as `impl<T> Wrapper<T>` for generics.

**When to use reference-holding structs:** short-lived "views" over data owned elsewhere, like
iterators, parsers, and reports. For long-lived data, store owned values (`String`, `Vec`)
instead. Fighting lifetimes on a long-lived struct is usually a sign it should own its data.

### `'static`

`'static` means "valid for the entire program". String literals are `&'static str` because
they're baked into the binary:

```rust
let s: &'static str = "I live forever";
```

You'll also see `T: 'static` bounds, for example on `thread::spawn` in lesson 14. That means
"`T` contains no non-static references". It does *not* mean the value lives forever.

> If the compiler suggests adding `'static` to fix an error, be suspicious. Usually the real fix
> is to return an owned value or restructure the borrows.

---

## 4. Trait objects: `dyn Trait`

Generics (lesson 9) give you **static dispatch**. `fn total<S: Shape>(shapes: &[S])` is
compiled separately for each concrete `S`, and every slice holds only *one* kind of shape.

What if you want a list holding *different* types that share a trait? Use a **trait object**:

```rust
trait Shape {
    fn area(&self) -> f64;
    fn name(&self) -> String { String::from("shape") } // default method
}

struct Circle { r: f64 }
struct Rect { w: f64, h: f64 }

impl Shape for Circle {
    fn area(&self) -> f64 { std::f64::consts::PI * self.r * self.r }
    fn name(&self) -> String { "circle".into() }
}
impl Shape for Rect {
    fn area(&self) -> f64 { self.w * self.h }
}

fn main() {
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { r: 1.0 }),
        Box::new(Rect { w: 2.0, h: 3.0 }),
    ];
    for s in &shapes {
        println!("{} {:.2}", s.name(), s.area());
    }
    let total: f64 = shapes.iter().map(|s| s.area()).sum();
    println!("total {total:.2}");
}
```

- `dyn Shape` is "some type implementing `Shape`, decided at runtime".
- Different types have different sizes, so a `dyn Shape` must always sit **behind a pointer**:
  `&dyn Shape`, `&mut dyn Shape`, or `Box<dyn Shape>`.
- The pointer is a **fat pointer**: data pointer + pointer to a *vtable* of method addresses.
  Calls go through the vtable. That's **dynamic dispatch**, slightly slower and not inlinable,
  but flexible.

### Choosing at runtime

The killer feature: the concrete type can depend on runtime input.

```rust
trait Greeter { fn greet(&self, name: &str) -> String; }
struct English;
struct Spanish;
impl Greeter for English { fn greet(&self, n: &str) -> String { format!("Hello, {n}!") } }
impl Greeter for Spanish { fn greet(&self, n: &str) -> String { format!("¡Hola, {n}!") } }

fn greeter_for(lang: &str) -> Option<Box<dyn Greeter>> {
    match lang {
        "en" => Some(Box::new(English)),
        "es" => Some(Box::new(Spanish)),
        _ => None,
    }
}

fn main() {
    for lang in ["en", "es", "fr"] {
        match greeter_for(lang) {
            Some(g) => println!("{}", g.greet("Ben")),
            None => println!("no greeter for {lang}"),
        }
    }
}
```

(`English` and `Spanish` are *unit structs* from lesson 4: types with no data, only behaviour.)

### Generics or trait objects?

| | Generics `T: Trait` / `impl Trait` | Trait objects `dyn Trait` |
|---|---|---|
| Dispatch | static (compile time), inlinable | dynamic (vtable) |
| Mixed types in one collection | no | **yes** |
| Type chosen at runtime | no | **yes** |
| Binary size | bigger (one copy per type) | smaller |

Default to generics. Reach for `dyn` when you need a heterogeneous collection, or a runtime
choice like a plugin, strategy, or exporter.

> Not every trait can be made into an object. Roughly, its methods can't be generic and can't
> return `Self`. The compiler will tell you if a trait isn't "dyn compatible".

---

## 5. `Box<T>`: values on the heap

`Box::new(x)` moves `x` to the heap and gives you an owning pointer. When the box is dropped,
so is the value. Three main uses:

1. **Trait objects**: `Box<dyn Trait>` (above).
2. **Recursive types**: a type can't contain itself directly (it would have infinite size),
   but it can contain a `Box` of itself, because a box has a fixed pointer size:

   ```rust
   #[derive(Debug)]
   enum List {
       Cons(i32, Box<List>),
       Nil,
   }

   fn sum(list: &List) -> i32 {
       match list {
           List::Cons(v, rest) => v + sum(rest),
           List::Nil => 0,
       }
   }

   fn main() {
       use List::{Cons, Nil};
       let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
       println!("{}", sum(&list)); // 6
   }
   ```
3. **Moving large values cheaply**: moving a box copies only the pointer.

`Box<T>` auto-derefs, so you can call `T`'s methods on it directly.

---

## 6. `Rc<T>`: shared ownership

Ownership normally has exactly one owner. Sometimes several parts of a program genuinely need
to *share* a value, with no single obvious owner. `Rc<T>` (reference counted) allows that:

```rust
use std::rc::Rc;

fn main() {
    let shared = Rc::new(String::from("config"));
    let a = Rc::clone(&shared);     // cheap: bumps a counter, doesn't copy the String
    let b = Rc::clone(&shared);
    println!("{} owners", Rc::strong_count(&shared)); // 3
    drop(a);
    println!("{} owners", Rc::strong_count(&shared)); // 2
    println!("{b}");                // the String is freed when the last Rc is dropped
}
```

- `Rc::clone(&x)` is the idiomatic spelling. It makes clear this is a cheap pointer clone, not a deep copy.
- `Rc` gives **shared, read-only** access. You can't get `&mut T` out of it (it might be aliased!).
- `Rc` is single-threaded. Its multi-threaded sibling is `Arc` (lesson 14).

## 7. `RefCell<T>`: interior mutability

`RefCell<T>` moves the borrowing rules from compile time to **runtime**. You can mutate the
inner value through a shared `&RefCell<T>`:

```rust
use std::cell::RefCell;

fn main() {
    let cell = RefCell::new(vec![1, 2]);
    cell.borrow_mut().push(3);               // &mut access via &cell
    println!("{:?}", cell.borrow());          // [1, 2, 3]

    let r1 = cell.borrow();
    // let w = cell.borrow_mut();            // would PANIC at runtime: already borrowed
    println!("len {}", r1.len());
}
```

The rules are the same as ever (many readers *or* one writer). Breaking them causes a
**panic**, not a compile error. Keep `borrow()`/`borrow_mut()` guards short-lived.

### `Rc<RefCell<T>>`: shared *and* mutable

Combine them to get several owners that can all mutate the same value:

```rust
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Account { balance: i64 }

fn main() {
    let acct = Rc::new(RefCell::new(Account { balance: 100 }));
    let alice = Rc::clone(&acct);
    let bob = Rc::clone(&acct);

    alice.borrow_mut().balance -= 30;
    bob.borrow_mut().balance += 5;
    println!("{:?}", acct.borrow()); // Account { balance: 75 }
}
```

This is useful for graphs, GUI state, caches, and observers. Don't reach for it by default,
though: plain ownership plus `&mut` is simpler and checked at compile time. In `taskr`, you
won't need `Rc<RefCell<>>`. The exercises give you practice instead.

> ⚠️ Two `Rc`s pointing at each other never reach a count of zero, so you leak memory.
> `std::rc::Weak` breaks such cycles. See the Rust Book ch. 15.6 if you're curious.

---

## 8. `Drop`: cleanup code

Implement `Drop` to run code when a value goes out of scope. This is how `File` closes itself,
`Box` frees memory, and `MutexGuard` unlocks (lesson 14):

```rust
struct Noisy(&'static str);

impl Drop for Noisy {
    fn drop(&mut self) {
        println!("dropping {}", self.0);
    }
}

fn main() {
    let _a = Noisy("a");
    let b = Noisy("b");
    drop(b);                  // explicit early drop with std::mem::drop
    let _c = Noisy("c");
    println!("end of main");
}   // locals dropped in REVERSE declaration order: c, then a
// Output: dropping b / end of main / dropping c / dropping a
```

You can't call `x.drop()` yourself. Use `drop(x)`, which takes ownership so the value can't be
used afterwards.

---

## Practice problems

All problems live in `exercises/lesson-13/`. Run them with:

```bash
cd exercises && cargo test -p lesson-13
```

**Part A: Lifetimes**
1. `longer`: annotated function returning one of two inputs.
2. `first_word`: rely on elision.
3. `Highlighter<'a>`: a struct borrowing text. `words_containing` must return `Vec<&'a str>`, so
   the result outlives the highlighter (the test checks this!). Also implement `highlight`.

**Part B: Trait objects**
4. Implement `Shape` for `Circle` and `Square`, then `make_shape` (runtime choice →
   `Box<dyn Shape>`) and `total_area`.

**Part C: Box**
5. `eval`: recursively evaluate a boxed expression tree.

**Part D: Rc, RefCell, Drop**
6. `Logger`: clones share one `Rc<RefCell<Vec<String>>>`.
7. `Drop for Noisy`: log drops, and check the drop order.

Written questions in `exercises/lesson-13/ANSWERS.md`:

- **Q1** elision rules
- **Q2** fix a dangling-reference compile error
- **Q3** generics vs `dyn`
- **Q4** `Rc` and `RefCell`
- **Q5** why `Report<'a>` blocks mutation (answer after doing the homework)

---

## Homework — taskr step 13

### Starting point

After lesson 12, `taskr`:

- is a library (`task`, `task_list`, `command`, `error`, `storage`) + binary
- saves to `tasks.db` (or `--file <path>`) after every mutating command and loads on startup
- supports one-shot mode (`taskr add Buy milk #home`) and the REPL
- has `main` returning `Result<(), Box<dyn Error>>`
- supports `add`, `done`, `remove`, `show`, `edit`, `list` (with filters), `search`, `clear`,
  `tags`, `stats`, `sort`, `help`, `quit`

### Requirements

#### (a) `Report<'a>`: a borrowing view

1. Create `src/report.rs` with:
   ```rust
   pub struct Report<'a> {
       tasks: Vec<&'a Task>,
   }
   ```
   and methods:
   - `pub fn new(tasks: Vec<&'a Task>) -> Self`
   - `pub fn total(&self) -> usize`, `pub fn done(&self) -> usize`, `pub fn pending(&self) -> usize`
   - `pub fn by_priority(&self, p: Priority) -> usize`
   - `pub fn percent_done(&self) -> f64` (0.0 for an empty report)
   - `pub fn summary(&self) -> String`, the multi-line text your `stats` command prints
2. Add a convenience method `TaskList::report(&self) -> Report<'_>` (all tasks).
   `'_` is the "anonymous lifetime". It says "this borrows from `self`" without naming it.
3. `stats` now uses `Report`. **Bonus:** `stats` accepts the same filters as `list`
   (`stats #work`, `stats pending`), built from `TaskList::filter(...)`.
4. Tests for `Report`, including an empty report and `percent_done`.

#### (b) Exporters: trait objects

5. Create `src/export.rs` with:
   ```rust
   pub trait Exporter {
       fn name(&self) -> &str;                       // e.g. "markdown"
       fn extension(&self) -> &str { "txt" }         // default method; override it
       fn export(&self, tasks: &[&Task]) -> String;
   }

   pub struct MarkdownExporter;
   pub struct CsvExporter;

   pub fn exporter_for(format: &str) -> Option<Box<dyn Exporter>>;
   ```
6. `MarkdownExporter` produces a GitHub checkbox list:
   ```markdown
   # Tasks

   - [ ] Buy milk (high) #errand #home
   - [x] Write report (medium) #work
   ```
7. `CsvExporter` produces a header row and one row per task. Titles are always double-quoted,
   with embedded quotes doubled (`"` → `""`), which is standard CSV escaping:
   ```text
   id,title,priority,done,tags
   1,"Buy milk",high,false,errand;home
   2,"Say ""hi""",medium,true,
   ```
8. `exporter_for("md")` and `exporter_for("markdown")` return Markdown, `"csv"` returns CSV, and
   anything else returns `None`.
9. New command `export <format> <path>`. It writes all tasks with the chosen exporter
   (`fs::write`) and prints `Exported 3 tasks to out.md (markdown).` An unknown format is a
   `ParseError` (or a friendly error) listing the valid formats. `export` doesn't modify the list.
10. Tests for both exporters (exact expected strings) and for `exporter_for`.

#### (c) Undo

11. `#[derive(Clone)]` on `TaskList` (and whatever it contains).
12. Keep a history of snapshots in the REPL: `history: Vec<TaskList>`. Before executing any
    **mutating** command, push `list.clone()`. Keep at most 20 (drop the oldest with `remove(0)`,
    or use a `VecDeque` if you're feeling fancy).
13. New command `undo`. It restores the most recent snapshot (`history.pop()`), saves, and prints
    `Undone.`, or `Nothing to undo.` if the history is empty. History only lives for one REPL
    session. In one-shot mode `undo` can just report there's nothing to undo.

### Example session

```text
taskr> add Buy milk #errand !high
Added task 1.
taskr> add Say "hi" to Sam
Added task 2.
taskr> done 2
Completed task 2.
taskr> stats
Total: 2  Done: 1 (50%)  Pending: 1
High: 1  Medium: 1  Low: 0
taskr> export md tasks.md
Exported 2 tasks to tasks.md (markdown).
taskr> export pdf x.pdf
Error: unknown export format 'pdf' (expected md or csv)
taskr> remove 1
Removed task 1.
taskr> undo
Undone.
taskr> list
  1. [ ] (High) Buy milk #errand
  2. [x] (Medium) Say "hi" to Sam
```

### Hints

- When you call `list.report()` and then try `list.add(...)` while the report is still in use,
  you'll get "cannot borrow `list` as mutable because it is also borrowed as immutable". That's
  the lifetime `'a` protecting you from a report pointing at tasks that may have moved. Finish
  using the report first. (This is ANSWERS.md Q5.)
- To pass all tasks to an exporter as `&[&Task]`, try `let all: Vec<&Task> = list.filter(|_| true);`
  and then `exporter.export(&all)`.
- `Box<dyn Exporter>` lets you call `.name()` and `.export(...)` directly. Auto-deref does the rest.
- Unit structs (`pub struct CsvExporter;`) have no fields. Create one with just `CsvExporter`.
- For undo, a snapshot is a whole `TaskList`, which is simple and good enough for a todo list.
  The more memory-efficient approach would be storing inverse operations (the "command
  pattern"). That's a stretch goal.

### Stretch goals

- A third exporter, e.g. `JsonExporter` written by hand (no crates), with proper string escaping.
- `redo`: when you undo, push the current state onto a `redo` stack. Any new mutating command clears it.
- `export` respects filters: `export md out.md pending #work`.
- Write a generic `fn export_with(exporter: &dyn Exporter, ...)`, and compare it with `fn export_with<E: Exporter>(exporter: &E, ...)`.

---

## Submitting for review

1. Make sure everything passes:
   ```bash
   (cd exercises && cargo test -p lesson-13)
   (cd taskr && cargo test)
   ```
2. Commit on your `work-13` branch:
   ```bash
   git add -A && git commit -m "Lesson 13: lifetimes, trait objects, undo"
   ```
3. Ask Claude: **"Please review lesson 13"**.

---

## Further reading

- The Rust Book, ch. 10.3: [Validating References with Lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
- The Rust Book, ch. 18.2: [Using Trait Objects That Allow for Values of Different Types](https://doc.rust-lang.org/book/ch18-02-trait-objects.html)
- The Rust Book, ch. 15: [Smart Pointers](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html) (`Box`, `Deref`, `Drop`, `Rc`, `RefCell`, reference cycles)
- Rust by Example: [Lifetimes](https://doc.rust-lang.org/rust-by-example/scope/lifetime.html)
- [Common Rust Lifetime Misconceptions](https://github.com/pretzelhammer/rust-blog/blob/master/posts/common-rust-lifetime-misconceptions.md), an excellent deeper read once the basics click
