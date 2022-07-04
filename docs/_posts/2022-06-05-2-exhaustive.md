---
layout: post
title: Iterators that generate everything, part 1
author: Mikhail Hogrefe
---

## Iterators

If you've ever used a modern programming language, you're probably familiar with iterators. For example,
if you've got a `Vec<u64>` called `xs` in Rust, you can write this:
```rust
for x in &xs {
    println!("{}", x);
}
```
This code takes `xs` and produces an iterator, a thing that knows how to spit out elements. Iterators
are very flexible: you can filter them, map them, zip them, and collect them into concrete
collections like `Vec`s or sets. In Haskell, the basic aggregate type, `List`, _is_ an iterator.

Iterators are also very general. An iterator doesn't have to be backed by anything in memory. You can
easily write this:
```rust
for x: u16 in 0..10 {
    println!("{}", x);
}
```
or even
```rust
for x: u16 in 0.. {
    println!("{}", x);
}
```

There's something compelling about an iterator that generates EVERY `u16`. If you were testing a
function that takes a `u16`, you could test it on every possible input! If we were dealing with
with `u64`s instead, you'd want to cut it off after some number of values using `take`.

# Some simple examples

The `malachite_base` crate provides lots of exhaustive iterators. Let me show you:
```rust
for b in exhaustive_bools() {
    println!("{}", b);
}
```
```
false
true
```
I mentioned `0..` earlier. Malachite provides an iterator that does the same thing but more
explicitly:
```rust
for u in exhaustive_unsigneds::<u16>().take(10) {
    println!("{}", u);
}
```
```
0
1
2
3
4
5
6
7
8
9
```
Ok, this isn't very interesting so far. How about signed integers?
```rust
for u in exhaustive_signeds::<i16>().take(10) {
    println!("{}", u);
}
```
```
0
1
-1
2
-2
3
-3
4
-4
5
```
The philosophy behind exhaustive iterators in Malachite is that simpler values should come first.
That's why the `i16`s are sorted by absolute value rather than by their usual order. If you _do_ want
to start at -65536 and go up from there, you can use
`primitive_int_increasing_inclusive_range(i16::MIN, i16::MAX)` instead.

By the way, there are lots of other iterators I'm skipping over, like `exhaustive_nonzero_integers`. You can find them by going
[here](https://docs.rs/malachite-base/latest/malachite_base/all.html) and Ctrl-F'ing "exhaustive".

How about `char`s?
```rust
for c in exhaustive_chars().take(10) {
    println!("{}", c);
}
```
```
a
b
c
d
e
f
g
h
i
j
```
Again, these are not in their usual order. Thanks to various historical circumstances, the `char`s
that are usually ordered first are mostly unprintable useless characters like "vertical tab" and
"unit separator". `exhaustive_chars` pushes these to the back of the line; see its
[documentation](https://docs.rs/malachite-base/latest/malachite_base/chars/exhaustive/fn.exhaustive_chars.html)
for details.

## Combining iterators
We've touched on most of Rust's primitive types, except for floats; those are more
complicated and I'll discuss them later. But now, let's see how to create iterators for
composite types, like `Option<i32>`:
```rust
for ox in exhaustive_options(exhaustive_signeds::<i32>()).take(10) {
    println!("{:?}", ox);
}
```
```
None
Some(0)
Some(1)
Some(-1)
Some(2)
Some(-2)
Some(3)
Some(-3)
Some(4)
Some(-4)
```
Reasonable.

Next, I want to talk about `Union`s. These don't exist in the standard library, so I've defined them in
Malachite. (Since variadic generics aren't a thing in Rust yet, what I've _actually_
defined are `Union2`s and a macro to define unions of higher arity.)

A union (specifically a tagged union, also called a variant or a sum type) is essentially a generic enum. For
example, a value of type `Union2<u16, char>` might be `Union2::A(12)` or `Union2::B('d')`.
It's usually better to use a purpose-built enum than a union, but it's very handy to have
exhaustive iterators for unions. If you want to create an exhaustive iterator for a 2-variant enum,
you can just do `exhaustive_union2s(...).map(|u| match u { ... })`.

Actually, before `exhaustive_union2s`, let me show you `lex_union2s`:
```rust
for b_or_u in lex_union2s(exhaustive_bools(), 1..=3) {
    println!("{}", b_or_u);
}
```
```
A(false)
A(true)
B(1)
B(2)
B(3)
```
It's very simple; it just produces all the values of the first variant, then all the values of
the second variant. I've called it `lex` because it returns its elements in lexicographic, or
"dictionary", order (with respect to the order of the input iterators' elements). It's only
suitable when the first iterator is short. You don't want to use
`lex_union2s(exhaustive_unsigneds::<u64>(), ...)` because you'll be waiting forever
before you see the second variant. In general, `exhaustive_union2s` is the better choice:
```rust
for u_or_b in exhaustive_union2s(
    exhaustive_unsigneds::<u64>(),
    exhaustive_bools()
).take(10) {
    println!("{}", u_or_b);
}
```
```
A(0),
B(false),
A(1),
B(true),
A(2),
A(3),
A(4),
A(5),
A(6),
A(7),
```
and now the `B` variant gets to see the light of day. `exhaustive_union3s`, etc., work in the same
way, selecting between their variants in a round-robin fashion.

# Intermission

We haven't gotten to the really interesting bits, which have to do with generating
tuples (and after that, lists, sets, and floats). I'll talk about those in future posts.

[Part 2: Generating tuples](/2022/07/03/exhaustive)
