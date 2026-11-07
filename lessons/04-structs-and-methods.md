# Lesson 04 — Structs & Methods

> Group related data into your own types and give them behavior.
> Estimated time: 2 hours.

## What you'll learn

- Defining and instantiating **structs**
- Field init shorthand and struct update syntax
- **Tuple structs** and **unit structs**
- `impl` blocks: **methods** (`&self`, `&mut self`, `self`) and **associated functions**
- Deriving `Debug`, `Clone`, and `PartialEq`, and printing with `{:?}` / `{:#?}`
- How ownership and borrowing apply to struct fields

---

## 1. Defining a struct

A tuple like `(String, u8, bool)` works, but `task.2` doesn't say much. A **struct** names
each field:

```rust
struct Task {
    title: String,
    priority: u8,
    done: bool,
}

fn main() {
    let task = Task {
        title: String::from("Buy milk"),
        priority: 1,
        done: false,
    };
    println!("{} (P{}) done={}", task.title, task.priority, task.done);
}
```

- Struct names are `UpperCamelCase`; fields are `snake_case`.
- You must provide **every** field when creating one (there are no implicit defaults).
- The struct **owns** its fields. When `task` is dropped, its `title` `String` is dropped
  too.

### Mutability

Mutability belongs to the *binding*, not to individual fields:

```rust
struct Counter {
    count: u32,
}

fn main() {
    let mut c = Counter { count: 0 };   // the whole struct is mutable
    c.count += 1;
    println!("{}", c.count);
}
```

### Field init shorthand

When a variable has the same name as a field, you can skip the `field: field` repetition:

```rust
struct Task {
    title: String,
    priority: u8,
    done: bool,
}

fn new_task(title: &str, priority: u8) -> Task {
    Task {
        title: title.to_string(),
        priority,          // same as `priority: priority`
        done: false,
    }
}

fn main() {
    let t = new_task("Call mom", 3);
    println!("{} {}", t.title, t.priority);
}
```

### Struct update syntax

Create a new struct from an existing one, overriding some fields:

```rust
#[derive(Debug)]
struct Config {
    verbose: bool,
    color: bool,
    width: u32,
}

fn main() {
    let base = Config { verbose: false, color: true, width: 80 };
    let wide = Config { width: 120, ..base };   // remaining fields from `base`
    println!("{wide:?}");
}
```

Careful: `..base` *moves* any non-`Copy` fields (like a `String`) out of `base`, after which
those fields of `base` can't be used.

---

## 2. Tuple structs and unit structs

**Tuple structs** have a name but unnamed fields. They're handy for giving a distinct type
to a simple value:

```rust
struct Point(i32, i32);
struct Meters(f64);

fn main() {
    let p = Point(3, 4);
    let Point(x, y) = p;            // destructure
    let d = Meters(5.5);
    println!("{x},{y} and {}", d.0); // access by position
}
```

`Meters(5.0)` and `f64` are different types — you can't accidentally pass seconds where
meters are expected. This is called the **newtype pattern**.

**Unit structs** have no fields at all. They're useful later with traits (lesson 09/13):

```rust
struct MarkdownFormat;

fn main() {
    let _f = MarkdownFormat;
}
```

---

## 3. Printing structs: `#[derive(Debug)]`

`println!("{}", task)` doesn't work — Rust doesn't know how you want a `Task` displayed.
For a quick developer view, **derive** the `Debug` trait:

```rust
#[derive(Debug)]
struct Task {
    title: String,
    priority: u8,
    done: bool,
}

fn main() {
    let t = Task { title: "Buy milk".to_string(), priority: 1, done: false };
    println!("{t:?}");
    // Task { title: "Buy milk", priority: 1, done: false }
    println!("{t:#?}");
    // Task {
    //     title: "Buy milk",
    //     priority: 1,
    //     done: false,
    // }
    dbg!(&t); // prints file:line, the expression, and its value to stderr
}
```

`#[derive(...)]` asks the compiler to write standard trait implementations for you. Other
common derives you'll use now:

| Derive | Gives you |
|--------|-----------|
| `Debug` | `{:?}` formatting |
| `Clone` | `.clone()` — a deep copy |
| `PartialEq` | `==` and `!=` (field by field) |
| `Copy` | implicit copies (only if all fields are `Copy`; requires `Clone` too) |

Traits are covered properly in lesson 09; for now, think of them as "capabilities a type
has".

```rust
#[derive(Debug, Clone, PartialEq)]
struct Point { x: i32, y: i32 }

fn main() {
    let a = Point { x: 1, y: 2 };
    let b = a.clone();
    println!("{}", a == b); // true
}
```

---

## 4. Methods with `impl`

Functions associated with a type go in an `impl` block:

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // An *associated function* (no `self`) — often used as a constructor.
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    fn square(size: u32) -> Self {   // `Self` = the type this impl is for
        Self { width: size, height: size }
    }

    // A *method*: first parameter is `self` in some form.
    fn area(&self) -> u32 {          // borrows immutably
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn scale(&mut self, factor: u32) { // borrows mutably
        self.width *= factor;
        self.height *= factor;
    }

    fn into_square(self) -> Rectangle { // takes ownership; the old value is consumed
        let side = self.width.max(self.height);
        Rectangle::square(side)
    }
}

fn main() {
    let mut r = Rectangle::new(3, 4);   // associated functions use `Type::name`
    println!("area = {}", r.area());    // methods use `value.name()`
    r.scale(2);
    println!("{r:?}");
    let small = Rectangle::square(2);
    println!("can hold? {}", r.can_hold(&small));
    let sq = r.into_square();
    // println!("{r:?}");               // ❌ r was moved into into_square
    println!("{sq:?}");
}
```

### Which `self` should I use?

| Receiver | Meaning | Use when... |
|----------|---------|-------------|
| `&self` | borrow immutably | you only read fields (most methods) |
| `&mut self` | borrow mutably | you modify fields |
| `self` | take ownership | you transform the value into something else or consume it |

Rust automatically adds `&`, `&mut`, or `*` when calling methods (`r.area()` is really
`Rectangle::area(&r)`), so you rarely have to think about it at the call site.

### Getters

It's common to keep fields private (lesson 08) and expose read-only *getter* methods:

```rust
struct Task {
    title: String,
}

impl Task {
    fn title(&self) -> &str {   // return a borrowed view, not a clone
        &self.title
    }
}

fn main() {
    let t = Task { title: "Read".to_string() };
    println!("{}", t.title());
}
```

You can have multiple `impl` blocks for the same type; that's occasionally useful for
organization.

---

## 5. Structs containing collections

A struct can own other structs and collections. This pattern — a "manager" struct that owns
a `Vec` of items — is exactly what taskr needs:

```rust
#[derive(Debug)]
struct Book {
    id: u32,
    title: String,
    checked_out: bool,
}

struct Library {
    books: Vec<Book>,
    next_id: u32,
}

impl Library {
    fn new() -> Self {
        Library { books: Vec::new(), next_id: 1 }
    }

    fn add(&mut self, title: &str) -> u32 {
        let id = self.next_id;
        self.books.push(Book { id, title: title.to_string(), checked_out: false });
        self.next_id += 1;
        id
    }

    fn check_out(&mut self, id: u32) -> bool {
        for book in &mut self.books {      // book: &mut Book
            if book.id == id && !book.checked_out {
                book.checked_out = true;
                return true;
            }
        }
        false
    }

    fn print(&self) {
        for book in &self.books {          // book: &Book
            let mark = if book.checked_out { "out" } else { "in" };
            println!("#{} {} [{mark}]", book.id, book.title);
        }
    }
}

fn main() {
    let mut lib = Library::new();
    let id = lib.add("The Rust Book");
    lib.add("Rust by Example");
    println!("checked out #{id}: {}", lib.check_out(id));
    lib.print();
}
```

Notice:
- `next_id` gives each book a **stable id** that doesn't change when other books are
  added or removed (unlike a position in the list).
- `for book in &mut self.books` iterates with mutable references so we can modify books in
  place.

---

## Practice problems

All in `exercises/lesson-04/src/lib.rs` (the struct definitions are provided; implement the
methods). Run: `cargo test -p lesson-04`.

1. **`Rectangle`**: `new(width, height)`, `square(size)`, `area(&self)`, `perimeter(&self)`,
   `can_hold(&self, other: &Rectangle)` (strictly larger in both dimensions), and
   `is_square(&self)`.
2. **`Point(i32, i32)`** (tuple struct): `origin()`, `manhattan_distance(&self, other: &Point) -> u32`
   (`|x1-x2| + |y1-y2|`; see `i32::abs_diff`), and `translate(&mut self, dx, dy)`.
3. **`Counter`**: `new()` starts at 0, `increment(&mut self)`, `increment_by(&mut self, n)`,
   `get(&self) -> u32`, `reset(&mut self)`.
4. **`BankAccount { owner: String, balance: u64 }`**: `new(owner: &str)` with zero balance,
   `deposit(&mut self, amount: u64)`, `withdraw(&mut self, amount: u64) -> bool` (fails and
   changes nothing if funds are insufficient), `owner(&self) -> &str`,
   `balance(&self) -> u64`.
5. **`Inventory { items: Vec<Item> }`** with `Item { name: String, quantity: u32 }`:
   `new()`, `add(&mut self, name: &str, quantity: u32)` (if an item with that name exists,
   increase its quantity instead of adding a duplicate), `quantity_of(&self, name: &str) -> u32`
   (0 if missing), `total_items(&self) -> u32`.

Written questions (`exercises/lesson-04/ANSWERS.md`):

- **Q1.** What's the difference between `fn area(&self)`, `fn scale(&mut self)`, and
  `fn into_square(self)`? Give an example of when you'd use each.
- **Q2.** Why does this fail, and how would you fix it?
  ```rust
  struct User { name: String }
  let u = User { name: String::from("Ann") };
  u.name.push_str("ie");
  ```
- **Q3.** What does `#[derive(Debug, PartialEq)]` do? Why can't you `#[derive(Copy)]` on a
  struct that has a `String` field?
- **Q4.** Why is a stable `id` (from a `next_id` counter) better than a list position for
  identifying records that can be deleted?

---

## Homework — taskr step 04

### Starting point

From lesson 03: tasks are a `Vec<(String, u8, bool)>`; free functions `print_tasks`,
`add_task`, `mark_done`, `format_task`; commands `add <title>`, `done <n>` (1-based
position), `list`, `help`, `quit`.

### Requirements

1. Define a `Task` struct and derive `Debug, Clone, PartialEq`:
   ```rust
   struct Task {
       id: u32,
       title: String,
       priority: u8,
       done: bool,
   }
   ```
   with methods:
   - `fn new(id: u32, title: &str) -> Task` — pending, priority `2`.
   - `fn complete(&mut self)`
   - `fn summary(&self) -> String` — e.g. `#3 [ ] (P2) Call mom` (this replaces
     `format_task`).
2. Define a `TaskList` struct:
   ```rust
   struct TaskList {
       tasks: Vec<Task>,
       next_id: u32,
   }
   ```
   with methods:
   - `fn new() -> TaskList` — empty, `next_id` starts at `1`.
   - `fn add(&mut self, title: &str) -> u32` — creates a task with the next id and returns
     that id.
   - `fn complete(&mut self, id: u32) -> bool` — `false` if no task has that id.
   - `fn print(&self)` — each task's `summary()`, then the `N tasks, M done` line (or
     `No tasks.` when empty).
3. Tasks are now identified by **id**, not position: `done 3` means "the task with id 3".
4. `main` creates a `TaskList`, adds the three starting tasks with `add`, completes task
   #1 with `complete(1)`, then runs the REPL. (All three now have the default priority 2;
   priorities become meaningful in lesson 05.)
5. Delete the old free functions and the tuple-based code. `print_help` can stay a free
   function.
6. Messages: `Added task #4: Buy eggs`, `Completed task #2.`, `No task with id 9.`

### Example session

```console
=== taskr v0.1.0 ===
Type 'help' for a list of commands.
taskr> list
#1 [x] (P2) Buy milk
#2 [ ] (P2) Write report
#3 [ ] (P2) Call mom
3 tasks, 1 done
taskr> add Buy eggs
Added task #4: Buy eggs
taskr> done 2
Completed task #2.
taskr> done 9
No task with id 9.
taskr> list
#1 [x] (P2) Buy milk
#2 [x] (P2) Write report
#3 [ ] (P2) Call mom
#4 [ ] (P2) Buy eggs
4 tasks, 2 done
taskr> quit
Bye!
```

### Hints

- `done <n>` should now parse to a `u32` instead of `usize`: `rest.parse::<u32>()`.
- In `TaskList::complete`, loop with `for task in &mut self.tasks` and call
  `task.complete()` when the id matches.
- Count done tasks inside `print` with a simple loop and counter.
- If you get "cannot borrow `*self` as mutable because it is also borrowed as immutable",
  look for a loop over `&self.tasks` that tries to modify `self` inside.

### Stretch goals

- Add `fn get_mut(&mut self, id: u32)` — hmm, what should it return when there's no such
  task? Think about it; lesson 05 gives the answer (`Option`).
- Add `fn pending_count(&self) -> usize` and `fn done_count(&self) -> usize` and use them
  in `print`.
- Add a `priority <id> <1-3>` command implemented as a `TaskList` method.

---

## Submitting for review

1. `cargo test -p lesson-04` passes; `ANSWERS.md` filled in; `taskr` works.
2. Commit on `work-04`: `git add -A && git commit -m "Lesson 04 complete"`.
3. Ask Claude: **"Please review lesson 04"**.

---

## Further reading

- The Rust Book, ch. 5 — [Using Structs to Structure Related Data](https://doc.rust-lang.org/book/ch05-00-structs.html)
- Rust by Example — [Structures](https://doc.rust-lang.org/rust-by-example/custom_types/structs.html), [Methods](https://doc.rust-lang.org/rust-by-example/fn/methods.html)
- std docs — [`derive` attribute](https://doc.rust-lang.org/reference/attributes/derive.html)
