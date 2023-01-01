---
layout: default
title: "Malachite Performance"
permalink: /performance/
theme: jekyll-theme-slate
---

# Performance
Here are some benchmarks comparing the Malachite to two other libraries:
- [`num`](https://crates.io/crates/num), the de facto standard bignum library for Rust.
- [`rug`](https://crates.io/crates/rug), a library that calls GMP, a widely used, highly-optimized
bignum library written in C.

The `num` version that is compared against is 0.4.0, and the `rug` version is 1.16.0.

The general trend is that Malachite is faster than `num` due to better algorithms, and slower than
`rug`. My guess is that the better performance of `rug` is due partly to GMP's use of inline
assembly (Malachite has none, being written entirely in safe Rust), and possibly due to operations
on Rust's slices being a little slower than C's raw pointers.

The following is just a small sample of the benchmarks that are available in Malachite. For each
benchmark, I've included the command that you can use to run it yourself. You can specify the
output file using `-o benchfile.gp`, and then use [gnuplot](http://www.gnuplot.info/) to convert
the `.gp` to your format of choice. I like SVG:

`gnuplot -e "set terminal svg; l \"benchfile.gp\"" > benchfile.svg`

## Rational addition
`cargo run --features bin_build --release -- -l 100000 -m random -b benchmark_rational_add_library_comparison -c "mean_bits_n 256"`
<p align="center">
  <img width="650" src="/assets/benchmarks/2022-06-04-q-add.svg" alt="Rational addition">
</p>

The most significant operations involved in adding two rational numbers (fractions) are GCD
computation and division.

For GCD computation, `num` uses the
[binary GCD algorithm](https://en.wikipedia.org/wiki/Binary_GCD_algorithm), a quadratic algorithm.
Malachite follows GMP in using
[Lehmer's GCD algorithm](https://en.wikipedia.org/wiki/Lehmer%27s_GCD_algorithm), which takes
advantage of fast multiplication algorithms to achieve $$O(n (\log n)^2 \log \log n)$$ time.

For division, `num` uses Knuth's
[Algorithm D](https://ridiculousfish.com/blog/posts/labor-of-division-episode-iv.html), which is
also quadratic. Malachite, again following GMP, uses several division algorithms depending on the
input size. For the largest inputs, it uses a kind of
[Barrett reduction](https://en.wikipedia.org/wiki/Barrett_reduction), which takes
$$O(n \log n \log \log n)$$ time.

## Converting a Natural to a string
`cargo run --features bin_build --release -- -l 100000 -m random -b benchmark_natural_to_string_library_comparison -c "mean_bits_n 1024"`
<p align="center">
  <img width="650" src="/assets/benchmarks/2022-06-04-n-to_string.svg" alt="Natural to string">
</p>

When converting a natural number to a string, `num` seems to use an $$O(n^{3/2})$$ algorithm.
Malachite uses a divide-and-conquer algorithm that takes $$O(n (\log n)^2 \log \log n)$$ time.

## Natural multiplication
`cargo run --features bin_build --release -- -l 20000 -m random -b benchmark_natural_mul_library_comparison -c "mean_bits_n 16384"`
<p align="center">
  <img width="650" src="/assets/benchmarks/2022-06-04-n-mul.svg" alt="Natural multiplication">
</p>

For multiplying two natural numbers, `num` uses a basecase quadratic algorithm for small inputs,
then [Toom-22](https://en.wikipedia.org/wiki/Toom%E2%80%93Cook_multiplication) (Karatsuba)
multiplication for larger inputs, and finally Toom-33 for the largest ones. This means that
multiplication takes $$O(n^{\log_3 5}) \approx O(n^{1.465})$$ time.

Malachite also uses a basecase quadratic algorithm, then 13 variants of Toom-Cook multiplication,
and finally [Schönhage-Strassen (FFT) multiplication](https://en.wikipedia.org/wiki/Schonhage-Strassen_algorithm) for the largest inputs, achieving $$O(n \log n \log \log n)$$
time.

Given all of this machinery, it's a little disappointing that Malachite isn't much faster at
multiplying than `num` is, in practice. I have a few improvements in mind that should boost
Malachite's multiplication performance further.

For numbers of up to 1000 bits, all three libraries are about equally fast:

`cargo run --features bin_build --release -- -l 100000 -m random -b benchmark_natural_mul_library_comparison -c "mean_bits_n 64"`
<p align="center">
  <img width="650" src="/assets/benchmarks/2022-06-04-n-mul-small.svg" alt="Natural multiplication">
</p>

## Natural addition
`cargo run --features bin_build --release -- -l 100000 -m random -b benchmark_natural_add_library_comparison -c "mean_bits_n 1024"`
<p align="center">
  <img width="650" src="/assets/benchmarks/2022-06-04-n-add.svg" alt="Natural addition">
</p>

Addition of natural numbers is fast for all three libraries, being a straightforward and
linear-time affair. Interestingly, `rug` is the slowest of the bunch. I find it hard to
believe that GMP is slower than `num` or Malachite, so maybe there's some overhead
associated with FFI.

Copyright © 2023 Mikhail Hogrefe
