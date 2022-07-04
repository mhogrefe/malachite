---
layout: post
title: Iterators that generate everything, part 2
author: Mikhail Hogrefe
---

## Introduction

In [Part 1](/2022/06/05/2-exhaustive) of this series, I described how Malachite provides iterators that generate all values of a type, or all values satisfying some condition. Now I'm going to show you how to generate tuples.

It's not immediately obvious how to do this. One thing you might try is generating the tuples in lexicographic order. Malachite lets you do this:

```rust
for p in lex_pairs_from_single(exhaustive_unsigneds::<u64>()).take(10) {
    println!("{}", p);
}
```
```
(0, 0)
(0, 1)
(0, 2)
(0, 3)
(0, 4)
(0, 5)
(0, 6)
(0, 7)
(0, 8)
(0, 9)
```

(The "from_single" in [`lex_pairs_from_single`](https://docs.rs/malachite-base/latest/malachite_base/tuples/exhaustive/fn.lex_pairs_from_single.html) indicates that both elements of the output pairs come from the same iterator. There's also a [`lex_pairs`](https://docs.rs/malachite-base/latest/malachite_base/tuples/exhaustive/fn.lex_pairs.html) function that accepts two iterators.)

As you can see, lexicographic generation only makes sense if the input iterator has a small number of elements, like `exhaustive_bools`. It doesn't work so well for `exhaustive_unsigneds::<u64>`, since you'd have to wait forever to get any pair that doesn't start with 0. But Malachite does provide [`exhaustive_pairs`](https://docs.rs/malachite-base/latest/malachite_base/tuples/exhaustive/fn.exhaustive_pairs.html) and [`exhaustive_pairs_from_single`](https://docs.rs/malachite-base/latest/malachite_base/tuples/exhaustive/fn.exhaustive_pairs_from_single.html), that work well for any input iterators. How do these functions work?

## First attempt: the Cantor pairing function

For simplicity, let's first consider `exhaustive_pairs_from_single(exhaustive_naturals())`. To think about generating every pair of `Natural`s, it helps to visualize a path through an infinite grid. One such path is this:

<p align="center">
  <img width="400" src="/assets/exhaustive-part-2/cantor-grid.svg" alt="The path defined by the Cantor pairing function">
</p>

You can think of it as first generating (0, 0), whose sum is 0; then (1, 0) and (0, 1), whose sum is 1; then (2, 0), (1, 1), and (0, 2), whose sum is 2; and so on. This path describes a bijection between the indices 0, 1, 2, ..., and all pairs of natural numbers. The opposite direction of this bijection, that takes pairs to indices, is known as Cantor's pairing function.

This bijection has some nice properties; for example, the function from pairs to indices is a polynomial in the elements of the pair: the pair $$(x, y)$$ corresponds index $$\tfrac{1}{2}(x + y)(x + y + 1)$$. We can also look at the two functions from sequence index to the first and second elements of the pairs. These functions look like this:

<p align="center">
  <img width="650" src="/assets/exhaustive-part-2/cantor-graph.svg" alt="The inverses of the Cantor pairing function">
</p>

Although the functions jump around a lot (they have to, since they must evaluate to every natural number infinitely many times), there's a sense in which they are nicely balanced. Both have a growth rate of $$O(\sqrt n)$$, and if you take evaluate them at a random large input, they're generally about the same size:

<p align="center">
  <img height="200" src="/assets/exhaustive-part-2/cantor-large.svg" alt="The Cantor unpairing of powers of 3">
</p>

However, it isn't obvious how generalize the Cantor pairing function to triples in a balanced way. The standard way to create a tripling function from a pairing function is $$g(x, y, z) = f(x, f(y, z))$$, but this is no longer balanced. If $$f$$ is the Cantor pairing function, then the three outputs of $$g^{-1}$$ grow as $$O(\sqrt n)$$, $$O(\sqrt[4] n)$$, and $$O(\sqrt[4]n)$$. Here's what the corresponding table looks like:

<p align="center">
  <img height="200" src="/assets/exhaustive-part-2/cantor-triples-large.svg" alt="An unbalanced Cantor untripling of powers of 3">
</p>

## More balance and flexibility with bit interleaving

Instead of the Cantor pairing function, Malachite uses bit interleaving. This idea is not new; it has been described by [Steven Pigeon](https://hbfs.wordpress.com/2011/09/27/pairing-functions/) and others.

To generate the pairs corresponding to index 0, 1, 2, and so on, first write the indices in binary, with infinitely many leading zeros:

<p align="center">
  <img height="200" src="/assets/exhaustive-part-2/interleave-bits-1.svg" alt="First step in un-interleaving bits">
</p>

And then distribute the bits of each index into the two slots of the corresponding pair. The even-indexed bits (counting from the right) end up in the second slot, and the odd-indexed bits in the first:

<p align="center">
  <img height="200" src="/assets/exhaustive-part-2/interleave-bits-2.svg" alt="Final steps in un-interleaving bits">
</p>

I've color-coded the bits according to which slot they end up in. Malachite keeps track of this using an explicit bit map that looks like $$[1, 0, 1, 0, 1, 0, ldots]$, indicating that bit 0 goes to slot 1, bit 1 goes to slot 0, bit 2 goes to slot 1, and so on.

This is how this method walks through the grid of pairs of natural numbers:

<p align="center">
  <img width="400" src="/assets/exhaustive-part-2/interleave-grid.svg" alt="The path defined by the bit-interleaving pairing function">
</p>

This path is called a [Z-order curve](https://en.wikipedia.org/wiki/Z-order_curve), although with this choice of coordinates it looks like it's made up of N's rather than Z's.

And here's what the functions from the index to the first and second elements of the pairs look like:

<p align="center">
  <img width="650" src="/assets/exhaustive-part-2/interleave-pairs_graph.svg" alt="The inverses of the bit interleaving pairing function">
</p>

Like the inverses of the Cantor pairing function, these functions are balanced in the sense that both have growth rate $$O(\sqrt n)$$, although here the first element lags noticeably behind the second. And here's a table showing them evaluated at large indices:

<p align="center">
  <img height="200" src="/assets/exhaustive-part-2/interleave-large.svg" alt="Bit un-interleaving of powers of 3">
</p>

The advantage that this approach has over the Cantor pairing approach is flexibility. We can generate triples in a balanced way simply by changing the bit map to $$[2, 1, 0, 2, 1, 0, 2, 1, 0, \ldots]$$:

<p align="center">
  <img height="400" src="/assets/exhaustive-part-2/interleave-triples-bits.svg" alt="un-interleaving bits to form triples">
</p>

Here are the three functions from index to output. They each grow as $$O(\sqrt[3]n)$$.

<p align="center">
  <img width="650" src="/assets/exhaustive-part-2/interleave-triples-graph.svg" alt="The inverses of the bit interleaving tripling function">
</p>

Here they are evaluated at large indices:

<p align="center">
  <img height="200" src="/assets/exhaustive-part-2/interleave-triples-large.svg" alt="Bit un-interleaving of powers of 3 to form triples">
</p>

This generalizes straightforwardly to $$n$$-tuples for any $$n$$. Notice that the first $$2^{kn}$$ tuples are all the tuples with elements less than $$2^k$$.

## Customized balancing

By changing the bit map further, we can deliberately unbalance the tuples. For example, if we want to produce pairs where the second element of the pair grows as $$O(\sqrt[3]n)$$ and the first element as $$O(n^{2/3})$$, we can use the bit map $$[1, 0, 0, 1, 0, 0, 1, 0, 0, \ldots]$$. If we want the the first element to be exponentially larger than the second, we can use $$[0, 1, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, \ldots]$$, where there is a 1 at the $$2^i$$th position and 0 elsewhere.

To actually generate the pair sequences in the two preceding examples, you can use

```rust
exhaustive_pairs_custom_output(
    exhaustive_naturals(),
    exhaustive_naturals(),
    BitDistributorOutputType::normal(2),
    BitDistributorOutputType::normal(1),
)
```
and
```rust
exhaustive_pairs_custom_output(
    exhaustive_naturals(),
    exhaustive_naturals(),
    BitDistributorOutputType::normal(1),
    BitDistributorOutputType::tiny(),
)
```
respectively. See the documentation of [exhaustive_pairs_custom_output](https://docs.rs/malachite-base/0.2.4/malachite_base/tuples/exhaustive/fn.exhaustive_pairs_custom_output.html) and [BitDistributorOutputType](https://docs.rs/malachite-base/latest/malachite_base/iterators/bit_distributor/struct.BitDistributorOutputType.html) for more details

## Generating tuples of other iterators

So far I've only talked about generating tuples of `Natural`s. What about tuples of elements from some other iterator `xs`? If `xs` is infinitely long, then the process is straightforward. Instead of generating, say, (8, 3), generate the pair consisting of the 8th and 3rd elements of `xs` (indexing from 0, of course). Internally, Malachite uses a structure called [`IteratorCache`](https://docs.rs/malachite-base/latest/malachite_base/iterators/iterator_cache/struct.IteratorCache.html), which stores the values produced by `xs` into a `Vec` for easy access.

For example, here's how to generate all pairs of the characters `'a'` to `'z'` (in this case, there are additional complications due to `xs` being finite, but we'll address those in a moment):

```rust
for p in exhaustive_pairs_from_single('a'..='z').take(10) {
    println!("{}", p);
}
```
```
(a, a),
(a, b),
(b, a),
(b, b),
(a, c),
(a, d),
(b, c),
(b, d),
(c, a),
(c, b)
```

## Dealing with finite iterators

Finite iterators are a bit of a problem. You can't generate the (8th, 3rd) pair if the input iterator has no 8th element. To take an extreme case, consider `exhaustive_pairs(exhaustive_naturals(), exhaustive_bools())`. There are only two bools, so any index pair $$(i, j)$$ where $$j \geq 2$$ can't be used for indexing. Here's our grid of indices again, with valid indices green and invalid indices red:

<p align="center">
  <img width="400" src="/assets/exhaustive-part-2/interleave-grid-filtered.svg" alt="Valid indices when one iterator has length 2">
</p>

My original solution was just to skip over the invalid indices, with a bit of extra logic to determine when both input iterators are finished. This was very slow: out of the first $$n$$ pairs, only about $$2 \sqrt n$$ are valid, and the situation gets worse for $$n$$-tuples with larger $$n$$. So what does `exhaustive_pairs` do instead?

Again, we can leverage the bit map! This scenario is actually what led me to make the bit map an explicit object in memory. What `exhaustive_pairs` does is notice when one of its iterators has completed, and then adjusts the bit map on the fly. In this particular case, it realizes that since `exhaustive_bools` produces two elements, it only needs a single bit for indexing; and then bit map is modified from $$[1, 0, 1, 0, 1, 0, 1, 0, \ldots]$$, to $$[1, 0, 0, 0, 0, 0, 0, \ldots]$$. In general, modifying the bit map on the fly is dangerous because you might end up repeating some output that you've already produced, but `exhaustive_pairs` only modifies the part of the map that hasn't been used yet.

(In this particular case, the best thing to do would be to use `lex_pairs(exhaustive_naturals(), exhaustive_bools())`, which would produce (0, false), (0, true), (1, false), (1, true), and so on. But in general, when you call `exhaustive_pairs(xs, ys)` you might not know ahead of time whether ys is short, long, or infinite.)

In the next part, I will discuss how Malachite generates all `Vec`s containing elements from some iterator.
