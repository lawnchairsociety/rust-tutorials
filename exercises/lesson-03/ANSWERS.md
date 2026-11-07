# Lesson 03 — Written answers

For each snippet: does it compile? If not, explain *why* in terms of the ownership/borrowing
rules, and show a fix.

## Q1.

```rust
let a = String::from("x");
let b = a;
println!("{a}");
```

Answer:

## Q2.

```rust
let mut s = String::from("x");
let r1 = &mut s;
let r2 = &mut s;
r1.push('y');
r2.push('z');
```

Answer:

## Q3.

```rust
let n = 5;
let m = n;
println!("{n} {m}");
```

Answer:

## Q4.

```rust
fn longest_name(names: Vec<String>) -> usize { /* ... */ 0 }
let names = vec![String::from("Ann")];
let n = longest_name(names);
println!("{} names, longest is {n}", names.len());
```

Also: what would be a better signature for `longest_name`?

Answer:

## Q5. Why can't you have a `&mut` reference and a `&` reference to the same value at the same time?

Answer:
