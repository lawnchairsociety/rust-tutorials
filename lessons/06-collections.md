# Lesson 06 — Common Collections: Vec, String & HashMap

> Store many values, work with text properly, and look things up by key. Estimated time: 2–3 hours.

## What you'll learn

- `Vec<T>` in depth: creating, growing, reading safely, iterating, sorting
- Why `String` is a UTF-8 byte buffer, and why you can't write `s[0]`
- Working with characters, words and common string methods
- `HashMap<K, V>`: inserting, looking up, updating with the **entry API**
- `HashSet<T>` for "have I seen this before?" questions
- How ownership and borrowing rules apply to collections

You've already used `Vec` lightly since lesson 3. Now we'll look at it properly, and add the
other two collections you'll use all the time.

---

## 1. `Vec<T>` — a growable list

A `Vec<T>` stores values of **one type** `T`, next to each other on the heap.

```rust
fn main() {
    let mut numbers: Vec<i32> = Vec::new(); // empty; type annotation needed
    numbers.push(10);
    numbers.push(20);

    let mut letters = vec!['a', 'b', 'c']; // the vec! macro infers Vec<char>
    letters.pop();                          // removes and returns Some('c')

    println!("{numbers:?} {letters:?}");    // [10, 20] ['a', 'b']
    println!("len = {}", numbers.len());
}
```

### Reading elements: `[]` vs `.get()`

```rust
fn main() {
    let v = vec![1, 2, 3];

    let second = v[1];          // 2 — panics if the index is out of range!
    let maybe = v.get(10);      // None — never panics; returns Option<&i32>

    match v.get(1) {
        Some(x) => println!("second is {x}"),
        None => println!("no second element"),
    }
    println!("{second} {maybe:?}");
}
```

Use `[]` when an out-of-range index would be a **bug** in your program. Use `.get()` when
the index comes from outside (like user input) and might legitimately be wrong.

### Iterating

```rust
fn main() {
    let mut scores = vec![50, 72, 91];

    for s in &scores {          // borrow each element: s is &i32
        println!("{s}");
    }

    for s in &mut scores {      // mutable borrow: s is &mut i32
        *s += 5;                // `*` dereferences to change the value itself
    }

    for (i, s) in scores.iter().enumerate() { // index + element
        println!("#{i}: {s}");
    }

    for s in scores {           // consumes the Vec: s is i32, scores is gone afterwards
        println!("{s}");
    }
}
```

### Useful methods

| Method | What it does |
|---|---|
| `v.len()`, `v.is_empty()` | size checks |
| `v.contains(&x)` | linear search, returns `bool` |
| `v.insert(i, x)`, `v.remove(i)` | insert/remove at an index (shifts the rest) |
| `v.sort()`, `v.dedup()` | sort in place; remove *consecutive* duplicates |
| `v.first()`, `v.last()` | `Option<&T>` |
| `v.extend(other)` | append everything from another collection |
| `v.join(", ")` | for `Vec<String>`/`Vec<&str>`: glue into one `String` |

### Borrowing rules still apply

```rust
fn main() {
    let mut v = vec![1, 2, 3];
    let first = &v[0];   // immutable borrow of v...
    v.push(4);           // ...but push needs a mutable borrow
    println!("{first}");
}
```

```text
error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
```

Why is this an error even though we only touch element 0? Because `push` might need to
**reallocate** the whole buffer somewhere else in memory, which would leave `first`
pointing at freed memory. Rust stops that bug at compile time.

---

## 2. `String` — UTF-8 text

A `String` is a `Vec<u8>` of bytes that is guaranteed to be valid UTF-8. That one fact
explains almost everything surprising about strings in Rust.

```rust
fn main() {
    let mut s = String::from("crab");
    s.push(' ');                 // push one char
    s.push_str("🦀");            // push a &str
    let t = s.clone() + "!";     // + takes ownership of the left side
    let u = format!("{s} and {t}"); // format! never takes ownership — prefer it

    println!("{u}");
    println!("bytes: {}, chars: {}", s.len(), s.chars().count()); // 9, 6
}
```

`len()` is the number of **bytes**, not characters. The crab emoji takes 4 bytes.

### Why `s[0]` doesn't compile

```rust
let s = String::from("héllo");
let c = s[0]; // error[E0277]: the type `str` cannot be indexed by `{integer}`
```

What would `s[1]` even mean? Byte 1 is half of the `é`. Rust refuses to guess. Instead you
pick what you mean:

```rust
fn main() {
    let s = "héllo";
    let first_char = s.chars().next();      // Some('h') — characters
    let third_char = s.chars().nth(2);      // Some('l')
    let first_byte = s.as_bytes()[0];       // 104 — raw bytes
    let slice = &s[0..3];                   // "hé" — byte range; panics if it splits a char!
    println!("{first_char:?} {third_char:?} {first_byte} {slice}");
}
```

### Everyday string methods

```rust
fn main() {
    let line = "  add Buy milk #errand  ";
    let trimmed = line.trim();                       // "add Buy milk #errand"

    for word in trimmed.split_whitespace() {          // "add", "Buy", "milk", "#errand"
        if let Some(tag) = word.strip_prefix('#') {   // Some("errand") if it starts with '#'
            println!("tag: {}", tag.to_lowercase());
        } else {
            println!("word: {word}");
        }
    }

    println!("{}", trimmed.starts_with("add"));       // true
    println!("{}", trimmed.contains("milk"));         // true
    println!("{}", trimmed.replace("milk", "eggs"));  // a new String
    println!("{:?}", trimmed.split_once(' '));        // Some(("add", "Buy milk #errand"))
}
```

`char` has helpers too: `c.is_alphabetic()`, `c.is_ascii_digit()`, `c.is_whitespace()`,
`c.to_ascii_uppercase()`. And `c.to_uppercase()` gives you an *iterator*, because some
characters become several characters when uppercased (`'ß'` → `"SS"`).

### Building strings from pieces

To collect words back into one string, put them in a `Vec` and `join`:

```rust
fn main() {
    let mut title_words: Vec<&str> = Vec::new();
    for w in "Buy #errand milk".split_whitespace() {
        if !w.starts_with('#') {
            title_words.push(w);
        }
    }
    let title = title_words.join(" ");
    assert_eq!(title, "Buy milk");
}
```

---

## 3. `HashMap<K, V>` — lookup by key

```rust
use std::collections::HashMap; // not in the prelude — you must import it

fn main() {
    let mut stock: HashMap<String, u32> = HashMap::new();
    stock.insert(String::from("apples"), 3);
    stock.insert(String::from("pears"), 5);
    stock.insert(String::from("apples"), 7);   // overwrites: apples is now 7

    match stock.get("apples") {                // get takes a &str; returns Option<&u32>
        Some(n) => println!("{n} apples"),
        None => println!("no apples"),
    }

    if stock.contains_key("pears") {
        println!("we have pears");
    }

    stock.remove("pears");                     // returns Some(5)
    println!("{stock:?}");
}
```

### Ownership of keys and values

`insert` **moves** owned values into the map. After `map.insert(name, 1)`, the variable
`name` (a `String`) can no longer be used. Types that are `Copy`, like `u32`, are copied.

### The entry API — insert-or-update in one step

The classic "count things" pattern:

```rust
use std::collections::HashMap;

fn main() {
    let text = "the cat and the hat";
    let mut counts: HashMap<&str, usize> = HashMap::new();

    for word in text.split_whitespace() {
        let count = counts.entry(word).or_insert(0); // &mut usize, inserted as 0 if missing
        *count += 1;
    }

    println!("{:?}", counts.get("the")); // Some(2)
}
```

`entry(key)` looks up the key **once**. `.or_insert(default)` gives back a mutable reference
to the value, inserting `default` first if needed. Related: `.or_default()` inserts the
type's default value (`0`, an empty `Vec`, an empty `String`, and so on).

```rust
use std::collections::HashMap;

fn main() {
    // group words by their first letter
    let mut groups: HashMap<char, Vec<&str>> = HashMap::new();
    for w in ["apple", "avocado", "banana"] {
        let first = w.chars().next().unwrap();
        groups.entry(first).or_default().push(w);
    }
    println!("{groups:?}"); // {'a': ["apple", "avocado"], 'b': ["banana"]} (in some order)
}
```

### Iteration order is not defined

A `HashMap` iterates in an arbitrary order that can change from run to run. When you need
stable output, copy the entries into a `Vec` and sort it:

```rust
use std::collections::HashMap;

fn main() {
    let mut counts = HashMap::new();
    counts.insert("zebra", 1);
    counts.insert("ant", 4);

    let mut entries: Vec<(&&str, &i32)> = counts.iter().collect();
    entries.sort(); // tuples sort by first element, then second
    for (name, n) in entries {
        println!("{name}: {n}"); // ant: 4, then zebra: 1
    }
}
```

(`.collect()` builds a collection from an iterator. You'll study it properly in lesson 11.
For now, read `counts.iter().collect()` as "put all the entries into a Vec".)

> If you need keys kept in sorted order all the time, `std::collections::BTreeMap` has the
> same API as `HashMap` but iterates in key order.

### What can be a key?

Keys must implement the traits `Eq` and `Hash`. Strings, integers, `char` and `bool`
already do. For your **own** enums and structs, derive them:

```rust
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Color { Red, Green }

fn main() {
    let mut seen: HashMap<Color, u32> = HashMap::new();
    *seen.entry(Color::Red).or_insert(0) += 1;
    println!("{seen:?}");
}
```

---

## 4. `HashSet<T>` — unique values

A set is like a map with keys only. It's good for "is this already present?" checks.

```rust
use std::collections::HashSet;

fn main() {
    let mut seen = HashSet::new();
    for tag in ["home", "work", "home"] {
        if !seen.insert(tag) {          // insert returns false if it was already there
            println!("duplicate: {tag}");
        }
    }
    println!("{} unique", seen.len()); // 2
}
```

---

## Practice problems

All code problems live in `exercises/lesson-06/src/lib.rs`. Run them with:

```bash
cd exercises && cargo test -p lesson-06
```

1. **`unique_in_order`**: remove duplicates, keeping first occurrences in order.
2. **`byte_and_char_len`**: return the byte length and character count of a string.
3. **`capitalize_words`**: capitalize each word. Watch out for non-ASCII first letters like `é`.
4. **`word_counts`**: count case-insensitive words with a `HashMap` and the entry API.
5. **`most_common`**: find the most frequent word, breaking ties alphabetically.
6. **`group_by_length`**: build a `HashMap<usize, Vec<String>>`.
7. **`pig_latin`**: the classic Rust Book exercise (lowercase ASCII input).
8. **`median`**: sort, then pick the middle. Return `Option<f64>`.

Written questions are in `exercises/lesson-06/ANSWERS.md`:

9. Predict: what does `"naïve".len()` return, and why is it not 5?
10. Explain why `let first = &v[0]; v.push(4); println!("{first}");` is rejected, and give two different ways to fix it.
11. When would you use `v.get(i)` instead of `v[i]`? Give an example from `taskr`.
12. Rewrite this using the entry API, and explain what the original does wrong in terms of work done:
    ```rust
    if map.contains_key(&k) {
        let n = map.get(&k).unwrap() + 1;
        map.insert(k, n);
    } else {
        map.insert(k, 1);
    }
    ```

---

## Homework — taskr step 06: tags & stats

### Starting point

After lesson 05 your `taskr` has:

- `enum Priority { Low, Medium, High }`
- `struct Task { id, title, priority: Priority, done }` with `new`, `complete`, `summary`
- `struct TaskList { tasks: Vec<Task>, next_id: u32 }` with `new`, `add`, `complete`,
  `find`, `find_mut`, `remove`, `print`
- `enum Command { Add { title, priority }, Done(u32), Remove(u32), Show(u32), List, Help, Quit, Unknown(String) }`
- `parse_command(input: &str) -> Command` and `parse_priority(s: &str) -> Option<Priority>`

### Requirements

1. **Tags on tasks.** Add a field `tags: Vec<String>` to `Task`. Tags are stored
   **without** the leading `#` and **lowercased**. A task never holds the same tag twice.
2. **Parsing `add`.** Any word in an `add` command that starts with `#` is a tag. Any word
   that starts with `!` is a priority (`!low`, `!med`, `!high`), and it may now appear
   **anywhere** in the line. All other words, in order and joined by single spaces, form
   the title.
   - `add Buy milk #Errand !high #home` → title `"Buy milk"`, priority High, tags `["errand", "home"]`
   - A bare `#` (nothing after it) is ignored.
   - If the title ends up empty (`add #home`), return `Command::Unknown(...)` with the
     original input. Lesson 07 will give us proper errors.
   - `Command::Add` gains a `tags: Vec<String>` field, and `TaskList::add` takes the tags
     too.
3. **Show tags.** `list` and `show <id>` display tags after the title, for example
   `#2 [ ] Buy milk (High) #errand #home`. You decide the exact layout, but keep it consistent.
4. **`list #tag`.** Change `Command::List` to `List { tag: Option<String> }`. `list` shows
   everything. `list #home` shows only tasks tagged `home` (the match ignores case).
   If nothing matches, print a friendly message.
5. **`tags` command** (`Command::Tags`). Print every tag with the number of tasks that use
   it, **sorted alphabetically**. Build the counts with a `HashMap<String, usize>`.
6. **`stats` command** (`Command::Stats`). Print the total, done and pending counts, plus
   the number of tasks at each priority. Count priorities with a `HashMap<Priority, usize>`,
   which means adding `Eq` and `Hash` to `Priority`'s derives. Print priorities in the
   fixed order High, Medium, Low (not HashMap order) and show 0 for missing ones.
7. Update `help` to describe the new commands.

### Example session

```text
taskr> add Buy milk #errand #Home !high
Added task #1
taskr> add Fix bike #home
Added task #2
taskr> add Write report !low #work
Added task #3
taskr> done 2
Completed task #2
taskr> list #home
#1 [ ] Buy milk (High) #errand #home
#2 [x] Fix bike (Medium) #home
taskr> tags
errand  1
home    2
work    1
taskr> stats
3 tasks: 1 done, 2 pending
  High:   1
  Medium: 1
  Low:    1
taskr> add #oops
Unknown command: add #oops (type 'help')
```

### Hints

- Write a helper such as `fn parse_add_args(rest: &str) -> (String, Priority, Vec<String>)`
  that loops over `rest.split_whitespace()` and puts each word in the right bucket.
- `word.strip_prefix('#')` returns `Some(rest)` if the word starts with `#`.
- To avoid duplicate tags, check `tags.contains(&tag)` before pushing.
- For `tags`, after counting into the map, move the entries into a `Vec<(String, usize)>`
  and call `.sort()`.
- `HashMap<Priority, usize>` needs `use std::collections::HashMap;` at the top of the file.

### Stretch goals

- `untag <id> <tag>` and `tag <id> <tag>` commands.
- Show the top-3 most used tags in `stats`.
- Use a `BTreeMap` for the tag counts so you don't need the extra sort step.
  This replaces the `HashMap` in requirement 5.

---

## Submitting for review

1. Make sure `cd exercises && cargo test -p lesson-06` passes and that `taskr` builds
   (`cd taskr && cargo run`).
2. Commit on your `work-06` branch:
   ```bash
   git add -A && git commit -m "Lesson 06: collections, taskr tags & stats"
   ```
3. Ask Claude: **"Please review lesson 06"**. Claude will look at `exercises/lesson-06`,
   your `ANSWERS.md`, and the `taskr` diff against `work-05`.

## Further reading

- The Rust Book, ch. 8 — [Common Collections](https://doc.rust-lang.org/book/ch08-00-common-collections.html)
- Rust by Example — [Vectors](https://doc.rust-lang.org/rust-by-example/std/vec.html), [Strings](https://doc.rust-lang.org/rust-by-example/std/str.html), [HashMap](https://doc.rust-lang.org/rust-by-example/std/hash.html)
- std docs — [`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html), [`String`](https://doc.rust-lang.org/std/string/struct.String.html), [`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html), [`str`](https://doc.rust-lang.org/std/primitive.str.html)
