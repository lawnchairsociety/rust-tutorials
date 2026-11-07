# Lesson 06 — Written answers

Write your answers below each question. Claude will review them with your code.

## 9. What does `"naïve".len()` return, and why is it not 5?

Answer:

## 10. Why is this rejected? Give two different ways to fix it.

```rust
let mut v = vec![1, 2, 3];
let first = &v[0];
v.push(4);
println!("{first}");
```

Answer:

## 11. When would you use `v.get(i)` instead of `v[i]`? Give an example from `taskr`.

Answer:

## 12. Rewrite with the entry API, and explain what the original does wastefully.

```rust
if map.contains_key(&k) {
    let n = map.get(&k).unwrap() + 1;
    map.insert(k, n);
} else {
    map.insert(k, 1);
}
```

Answer:
