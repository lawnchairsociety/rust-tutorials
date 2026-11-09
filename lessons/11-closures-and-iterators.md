# Lesson 11 — Closures & Iterators

> Functions you can store in variables, plus Rust's lazy iterator pipelines: the tools that make Rust code short *and* fast. Estimated time: 3–4 hours.

## What you'll learn

- How to write closures and how they **capture** their environment
- The three closure traits: `Fn`, `FnMut`, `FnOnce`, and what `move` does
- How to take closures as parameters (`impl Fn(...)`) and return them
- The `Iterator` trait and why iterators are **lazy**
- `iter()` vs `iter_mut()` vs `into_iter()`
- The most useful adapters (`map`, `filter`, `enumerate`, `zip`, `take`, `skip`, `rev`, `flat_map`, ...)
- The most useful consumers (`collect`, `sum`, `count`, `fold`, `any`, `all`, `find`, `position`, `max_by_key`, ...)
- Sorting with closures (`sort_by`, `sort_by_key`) and filtering in place with `retain`

---

## 1. Closures: anonymous functions that capture

A **closure** is a function without a name that you can store in a variable or pass around.
It is written with pipes around its parameters:

```rust
fn main() {
    let add_one = |x: i32| -> i32 { x + 1 };   // fully annotated
    let add_two = |x| x + 2;                   // types inferred, body is one expression

    println!("{}", add_one(1)); // 2
    println!("{}", add_two(1)); // 3
}
```

Compare with a normal function:

```rust
fn add_one_fn(x: i32) -> i32 { x + 1 }
```

Type annotations are optional on closures because they're usually used locally and the
compiler can infer them from how they're called. Once inferred, the types are fixed:

```rust
let identity = |x| x;
let s = identity(String::from("hi"));
let n = identity(5); // ERROR: expected `String`, found integer
```

### Capturing the environment

The big difference from `fn`: closures can use variables from the scope they're defined in.

```rust
fn main() {
    let threshold = 10;
    let is_big = |n: i32| n > threshold; // `threshold` is captured
    println!("{}", is_big(15)); // true
}
```

A regular `fn` can't do this; it would say ``can't capture dynamic environment in a fn item``.

Closures capture in the **least demanding** way that works:

| The closure body...                  | captures by...       |
|--------------------------------------|----------------------|
| only reads the variable              | shared reference `&T`  |
| modifies the variable                | mutable reference `&mut T` |
| moves the value (e.g. `drop(x)`, returns it) | value (move)        |

The normal borrowing rules from lesson 3 still apply. While a closure holds a `&mut`
borrow, you can't use the variable anywhere else:

```rust
fn main() {
    let mut count = 0;
    let mut inc = || count += 1; // mutably borrows `count`
    inc();
    inc();
    // println!("{count}");  // ERROR here if `inc` is used again below
    inc();
    println!("{count}"); // OK: `inc` isn't used after this line, so the borrow is over → 3
}
```

Note that `inc` itself must be declared `mut` because calling it changes its captured state.

### `move` closures

Writing `move` before the pipes makes the closure **take ownership** of everything it captures,
even if it would only need a reference:

```rust
fn main() {
    let name = String::from("taskr");
    let greet = move || println!("hello, {name}");
    greet();
    // println!("{name}"); // ERROR: `name` was moved into the closure
}
```

You need `move` whenever the closure might **outlive** the variables it uses: returning a
closure from a function, or handing it to another thread (lesson 14).

---

## 2. `Fn`, `FnMut`, `FnOnce`

Every closure automatically implements one or more of these traits, depending on what its body
does with captured values:

- **`FnOnce`**: can be called at least once. *All* closures implement this. A closure that
  moves a captured value out of itself implements *only* `FnOnce`, because after the first call
  the value is gone.
- **`FnMut`**: can be called many times and may **mutate** captured state.
- **`Fn`**: can be called many times, even concurrently, and doesn't mutate captured state.

They nest: every `Fn` is also `FnMut`, and every `FnMut` is also `FnOnce`.

When you **accept** a closure, ask for the *least* you need. That lets callers pass the widest
range of closures:

```rust
// Calls f once → accept FnOnce (anything works)
fn run_once(f: impl FnOnce() -> String) -> String {
    f()
}

// Calls f repeatedly and lets it mutate state → FnMut
fn repeat(n: usize, mut f: impl FnMut()) {
    for _ in 0..n {
        f();
    }
}

// Calls f repeatedly, read-only → Fn
fn apply_twice(f: impl Fn(i32) -> i32, x: i32) -> i32 {
    f(f(x))
}

fn main() {
    let s = String::from("owned");
    println!("{}", run_once(move || s));       // moves s out: FnOnce only

    let mut total = 0;
    repeat(3, || total += 10);                 // mutates: FnMut
    println!("{total}");                       // 30

    let offset = 5;
    println!("{}", apply_twice(|x| x + offset, 0)); // reads: Fn → 10
}
```

`impl Fn(i32) -> i32` in argument position means "some type that implements this trait",
the same `impl Trait` syntax you met in lesson 9. You may also see the generic form:

```rust
fn apply_twice<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 { f(f(x)) }
// or
fn apply_twice_where<F>(f: F, x: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(f(x))
}
```

### Returning closures

Every closure has its own unique, unnameable type, so you return one with `impl Fn...`:

```rust
fn make_multiplier(factor: i32) -> impl Fn(i32) -> i32 {
    move |x| x * factor // `move` needed: `factor` dies when this function returns
}

fn main() {
    let triple = make_multiplier(3);
    println!("{}", triple(7)); // 21
}
```

### Functions work too

Anywhere a closure is accepted, you can pass a named function (or method path) instead:

```rust
fn is_even(n: &i32) -> bool { n % 2 == 0 }

fn main() {
    let v = vec![1, 2, 3, 4];
    let evens: Vec<&i32> = v.iter().filter(|n| is_even(n)).collect();
    let words: Vec<String> = vec!["a", "b"].into_iter().map(String::from).collect();
    println!("{evens:?} {words:?}");
}
```

---

## 3. The `Iterator` trait

An iterator is anything that produces a sequence of values, one at a time. The whole trait
boils down to one required method:

```rust
pub trait Iterator {
    type Item;                                  // the type of value produced
    fn next(&mut self) -> Option<Self::Item>;   // Some(value) or None when finished
    // ...plus ~75 provided methods (map, filter, sum, ...) built on top of `next`
}
```

`type Item;` is an **associated type**: each iterator says what it yields.

You've been using iterators all along: a `for` loop is sugar for calling `next()` until `None`:

```rust
fn main() {
    let v = vec![10, 20, 30];
    let mut it = v.iter();
    println!("{:?}", it.next()); // Some(10)
    println!("{:?}", it.next()); // Some(20)
    println!("{:?}", it.next()); // Some(30)
    println!("{:?}", it.next()); // None
}
```

### Writing your own iterator

```rust
struct Countdown {
    n: u32,
}

impl Iterator for Countdown {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.n == 0 {
            None
        } else {
            self.n -= 1;
            Some(self.n + 1)
        }
    }
}

fn main() {
    for x in (Countdown { n: 3 }) {
        print!("{x} "); // 3 2 1
    }
    println!();
    // Because Countdown implements Iterator, we get all the adapters for free:
    let total: u32 = Countdown { n: 4 }.map(|x| x * 10).sum();
    println!("{total}"); // 100
}
```

### Three ways to iterate a collection

| Method          | Yields        | Collection afterwards |
|-----------------|---------------|-----------------------|
| `v.iter()`      | `&T`          | still usable          |
| `v.iter_mut()`  | `&mut T`      | still usable (modified) |
| `v.into_iter()` | `T` (owned)   | **moved / consumed**  |

`for x in &v` is the same as `v.iter()`, `for x in &mut v` is `v.iter_mut()`, and `for x in v`
is `v.into_iter()`.

```rust
fn main() {
    let mut names = vec![String::from("ann"), String::from("bob")];

    for n in names.iter_mut() {
        n.push('!');                 // n: &mut String
    }
    let lengths: Vec<usize> = names.iter().map(|n| n.len()).collect(); // n: &String
    let owned: Vec<String> = names.into_iter().collect();               // n: String
    // names is gone now
    println!("{lengths:?} {owned:?}");
}
```

---

## 4. Laziness: adapters vs consumers

Iterator methods come in two kinds:

- **Adapters** take an iterator and return a *new iterator*: `map`, `filter`, `enumerate`, ...
  They do **nothing** until something pulls values through.
- **Consumers** pull values through and produce a final result: `collect`, `sum`, `count`, `for` loops, ...

```rust
fn main() {
    let v = vec![1, 2, 3];
    v.iter().map(|x| x * 2); // WARNING: unused `Map` that must be used — iterators are lazy
}
```

Laziness is what makes long chains efficient. Rust fuses the whole chain into one loop with no
intermediate vectors, and it often compiles down to the same machine code as a hand-written
loop. (This is what Rust people mean by "zero-cost abstractions".)

### The adapters you'll use most

```rust
fn main() {
    let v = vec![3, 1, 4, 1, 5, 9, 2, 6];

    let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
    let odds: Vec<&i32> = v.iter().filter(|x| *x % 2 == 1).collect();
    let first_three: Vec<&i32> = v.iter().take(3).collect();
    let after_two: Vec<&i32> = v.iter().skip(2).collect();
    let backwards: Vec<&i32> = v.iter().rev().collect();

    for (i, x) in v.iter().enumerate() {
        if i < 2 { println!("index {i} = {x}"); }
    }

    let names = ["ann", "bob"];
    let ages = [31, 42];
    let pairs: Vec<(&str, i32)> = names.iter().copied().zip(ages.iter().copied()).collect();

    let sentences = ["hello world", "rust is fun"];
    let words: Vec<&str> = sentences.iter().flat_map(|s| s.split_whitespace()).collect();

    println!("{doubled:?}\n{odds:?}\n{first_three:?}\n{after_two:?}\n{backwards:?}");
    println!("{pairs:?}\n{words:?}");
}
```

A note on `filter(|x| ...)`: `v.iter()` yields `&i32`, and `filter` hands its closure a
*reference to the item*, so `x` is `&&i32`. You can dereference (`**x`), rely on auto-deref for
methods, or destructure in the pattern: `filter(|&&x| x % 2 == 1)`. Also handy:
`.copied()` / `.cloned()` turn an iterator of `&T` into one of `T`.

### The consumers you'll use most

```rust
fn main() {
    let v = vec![3, 1, 4, 1, 5];

    let total: i32 = v.iter().sum();                    // 14
    let how_many = v.iter().filter(|&&x| x > 2).count(); // 3
    let product = v.iter().fold(1, |acc, x| acc * x);   // 60
    let any_big = v.iter().any(|&x| x > 4);             // true
    let all_pos = v.iter().all(|&x| x > 0);             // true
    let first_even = v.iter().find(|&&x| x % 2 == 0);   // Some(&4)
    let where_five = v.iter().position(|&x| x == 5);    // Some(4)
    let biggest = v.iter().max();                       // Some(&5)

    let words = ["kiwi", "banana", "fig"];
    let longest = words.iter().max_by_key(|w| w.len()); // Some(&"banana")
    let shortest = words.iter().min_by_key(|w| w.len()); // Some(&"fig")

    println!("{total} {how_many} {product} {any_big} {all_pos}");
    println!("{first_even:?} {where_five:?} {biggest:?} {longest:?} {shortest:?}");
}
```

### `collect` is magic (sort of)

`collect` can build *any* collection that implements `FromIterator`. You tell it which one with
a type annotation or the "turbofish" `::<>`:

```rust
use std::collections::{HashMap, HashSet};

fn main() {
    let words = ["b", "a", "b", "c"];
    let v: Vec<&str> = words.iter().copied().collect();
    let set: HashSet<&str> = words.iter().copied().collect();
    let joined: String = words.iter().copied().collect(); // "babc"
    let counts = words.iter().fold(HashMap::new(), |mut m, w| {
        *m.entry(*w).or_insert(0) += 1;
        m
    });
    let lens = words.iter().map(|w| w.len()).collect::<Vec<_>>();
    println!("{v:?} {} {joined} {counts:?} {lens:?}", set.len());

    // Collecting Results: stops at the first Err!
    let parsed: Result<Vec<i32>, _> = ["1", "2", "x"].iter().map(|s| s.parse::<i32>()).collect();
    println!("{parsed:?}"); // Err(ParseIntError { kind: InvalidDigit })
}
```

That last trick (`Iterator<Item = Result<T, E>>` → `Result<Vec<T>, E>`) is extremely useful
for parsing, and you'll use it in lesson 12.

---

## 5. Sorting and in-place filtering

Sorting methods live on slices/vectors and take closures:

```rust
#[derive(Debug)]
struct Item { name: String, price: u32 }

fn main() {
    let mut items = vec![
        Item { name: "pear".into(), price: 3 },
        Item { name: "apple".into(), price: 3 },
        Item { name: "melon".into(), price: 7 },
    ];

    items.sort_by_key(|i| i.price);                         // ascending by price
    items.sort_by(|a, b| b.price.cmp(&a.price));            // descending by price
    items.sort_by(|a, b| b.price.cmp(&a.price).then_with(|| a.name.cmp(&b.name)));
    // ^ by price descending, ties broken by name ascending

    items.retain(|i| i.price < 5);                          // keep only cheap items, in place
    println!("{items:#?}");
}
```

- `sort_by_key(|x| key)`: sorts by a key you extract. The key must implement `Ord`.
- `sort_by(|a, b| ordering)`: you return a `std::cmp::Ordering`. `Ordering::then` and
  `then_with` chain tie-breakers. `.reverse()` flips an ordering.
- `sort` / `sort_by*` are **stable**: equal elements keep their original order.
- `retain(|x| keep?)` removes every element for which the closure returns `false`.

To sort by a `String` key without cloning, prefer `sort_by(|a, b| a.name.cmp(&b.name))`.
`sort_by_key(|i| i.name.clone())` works, but it allocates.

---

## 6. Loop or iterator?

Both are fine Rust. Iterators shine when you're *transforming or summarising* data:

```rust
// Loop version
let mut total = 0;
for t in &tasks {
    if !t.done {
        total += 1;
    }
}

// Iterator version, which says *what* rather than *how*
let total = tasks.iter().filter(|t| !t.done).count();
```

Plain `for` loops are still the clearest choice when each step does side effects such as
printing, or when you need `break`/`continue` with complex logic. Don't contort code just to
avoid a loop.

---

## Practice problems

All problems live in `exercises/lesson-11/`. Run them with:

```bash
cd exercises && cargo test -p lesson-11
```

**Challenge rule:** solve problems 1–9 *without* `for`/`while` loops.

1. `sum_of_even_squares`: `filter`, `map`, `sum`.
2. `long_words_upper`: `filter` + `map` + `collect` into `Vec<String>`.
3. `make_adder`: return a closure (`impl Fn`, `move`).
4. `count_true`: accept an `FnMut` and call it repeatedly. Ranges are iterators too!
5. `first_index_above`: `position`.
6. `top_scorer`: `max_by_key`, but watch out for the tie-breaking rule in the doc comment.
7. `number_lines`: `enumerate`, `map`, `format!`, and joining strings.
8. `drop_negatives_and_double`: `retain` + `iter_mut`.
9. `common_words`: build two sets and intersect them. (`BTreeSet` is a `HashSet` that keeps
   its items sorted, which is handy here.)

Written questions are in `exercises/lesson-11/ANSWERS.md`:

- **Q1** predict the output (laziness)
- **Q2** fix a compile error (capturing + lifetime of a closure)
- **Q3** classify closures as `Fn` / `FnMut` / `FnOnce`
- **Q4** `iter` vs `iter_mut` vs `into_iter`

---

## Homework — taskr step 11

### Starting point

After lesson 10, `taskr` is a library + binary:

- `src/task.rs`: `Task` (id, title, priority, done, tags) and `Priority` (`Low < Medium < High`,
  implements `Display`, `FromStr`, `Hash`, `Eq`, `Ord`)
- `src/task_list.rs`: `TaskList` (`Default`, `add`, `complete`, `remove`, `find`, ... returning `Result<_, TaskError>`)
- `src/command.rs`: `Command` with `FromStr`, plus `parse_priority`, `parse_id`
- `src/error.rs`: `ParseError`, `TaskError` with `Display`
- `src/main.rs`: the REPL
- Commands: `add`, `done`, `remove`, `show`, `edit`, `list [#tag]`, `tags`, `stats`, `sort priority|id`, `help`, `quit`
- 20+ unit, integration, and doc tests, all passing

Your code probably has a lot of `for` loops with `if`s and counters inside. This lesson turns them
into iterator pipelines and uses closures to build a much more flexible `list` command.

### Requirements

1. **Iterator refactor.** Rewrite loop-based logic to use iterators wherever it makes the code
   clearer. At minimum:
   - `stats`: totals and per-priority counts using `filter(...).count()` (or `fold`)
   - `tags`: gather all tags with `flat_map`, count them into a `HashMap` (or a `BTreeMap`, if
     you did lesson 06's stretch goal), then sort
   - tag/priority parsing in `command.rs`: e.g. `split_whitespace().filter(...)`
   - any "find task by id" code: `iter().find(...)` / `iter_mut().find(...)` / `position(...)`

2. **Generic filter method.** Add to `TaskList`:
   ```rust
   pub fn filter(&self, pred: impl Fn(&Task) -> bool) -> Vec<&Task>
   ```
   It returns references to every task for which `pred` returns `true`, in current order.
   (You don't need lifetime annotations here. The compiler's *elision rules* work out that the
   returned references borrow from `self`. Lesson 13 explains why.)

3. **Combinable `list` filters.** Replace `Command::List { tag: Option<String> }` with a filter
   description, for example:
   ```rust
   pub enum Status { Pending, Done }

   pub struct ListFilter {
       pub status: Option<Status>,
       pub tag: Option<String>,
       pub priority: Option<Priority>,
   }
   ```
   and `Command::List(ListFilter)`. Supported forms, in any order and combination:
   - `list`: everything
   - `list pending` / `list done`
   - `list #home`
   - `list !high` (also `!med`, `!low`)
   - `list pending #home !high`: all conditions must match (logical AND)

   An unknown word (e.g. `list banana`) is a `ParseError`. Implement it with your `filter` method
   and a single closure that checks all three conditions.

4. **`search <text>`.** Case-insensitive substring search on titles. `search MILK` finds
   "Buy milk". Missing text → `ParseError::MissingArgument`. Print matching tasks, or
   `No tasks match "<text>".`

5. **`clear`.** Remove all completed tasks using `Vec::retain`, and print how many were removed.
   Add a `TaskList::clear_done(&mut self) -> usize` method.

6. **Tests.** Add unit tests for: `filter` with a couple of different predicates, `clear_done`,
   parsing every new `list` form (including an error case), and `search` parsing.
   Keep every existing test passing.

7. Update `help` to describe the new commands.

### Example session

```text
taskr> add Buy milk #errand #home !high
Added task 1.
taskr> add Write report #work
Added task 2.
taskr> add Water plants #home !low
Added task 3.
taskr> done 3
Completed task 3.
taskr> list pending #home
  1. [ ] (High) Buy milk #errand #home
taskr> list done
  3. [x] (Low) Water plants #home
taskr> list banana
Error: unknown list filter 'banana' (expected pending, done, #tag or !priority)
taskr> search REPORT
  2. [ ] (Medium) Write report #work
taskr> search xyz
No tasks match "xyz".
taskr> clear
Removed 1 completed task.
taskr> stats
Total: 2  Done: 0  Pending: 2
High: 1  Medium: 1  Low: 0
```

(Your exact output format can differ. Keep whatever style you established earlier.)

### Hints

- A closure can capture `&filter` and check everything in one place:
  ```rust
  let matches = list.filter(|t| {
      filter.status.as_ref().is_none_or(|s| /* compare with t.done */)
          && filter.tag.as_ref().is_none_or(|tag| t.tags.contains(tag))
          && filter.priority.is_none_or(|p| t.priority == p)
  });
  ```
  `Option::is_none_or(f)` returns `true` if the option is `None`, otherwise it returns `f(value)`.
  That's exactly the "no condition = match everything" semantics you want.
- For case-insensitive search, lowercase both sides once: `let q = text.to_lowercase();`
  then `t.title.to_lowercase().contains(&q)`.
- `clear_done` can compute `before - after` lengths around `retain`.
- If you're borrowing a `Vec<&Task>` from the list, you can't mutate the list until you're done with
  that `Vec`. The borrow checker will tell you if you try.

### Stretch goals

- `list` sorted output: `list !high sort` or similar. Reuse your `sort` logic with `sort_by`.
  This makes `sort` a valid word, an exception to the unknown-word `ParseError` in requirement 3.
- Add `TaskList::iter(&self)` returning `impl Iterator<Item = &Task>`, and use it in your
  Display/printing code. To go further, implement the `Iterator` trait yourself on a custom
  struct.
- Support OR within a category: `list #home #work` = tasks with *either* tag.
  This changes `tag` in requirement 3 to a list, such as `Vec<String>`. Different categories
  still combine with AND.

---

## Submitting for review

1. Make sure everything passes:
   ```bash
   (cd exercises && cargo test -p lesson-11)
   (cd taskr && cargo test)
   ```
2. Commit on your `work-11` branch:
   ```bash
   git add -A && git commit -m "Lesson 11: closures & iterators"
   ```
3. Ask Claude: **"Please review lesson 11"**. Claude will look at `exercises/lesson-11`,
   `ANSWERS.md`, and the `taskr` diff against `work-10`.

---

## Further reading

- The Rust Book, ch. 13: [Functional Language Features: Iterators and Closures](https://doc.rust-lang.org/book/ch13-00-functional-features.html)
- Rust by Example: [Closures](https://doc.rust-lang.org/rust-by-example/fn/closures.html) and [Iterator::any / find](https://doc.rust-lang.org/rust-by-example/fn/closures/closure_examples.html)
- std docs: [`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html) (skim the list of provided methods; it's a treasure chest)
- std docs: [`slice::sort_by`](https://doc.rust-lang.org/std/primitive.slice.html#method.sort_by), [`Vec::retain`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.retain)
