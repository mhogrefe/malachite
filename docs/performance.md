---
layout: default
title: "Malachite Performance"
permalink: /performance/
theme: jekyll-theme-slate
---

# Performance
Here are some benchmarks comparing the Malachite to two other libraries:
[`num`](https://crates.io/crates/num), the de facto standard bignum library for Rust, and
[`rug`](https://crates.io/crates/rug), a library that calls GMP, a widely used, highly-optimized
bignum library written in C. The `num` version that is compared against is 0.4.0, and the `rug`
version is 1.16.0.

The general trend is that Malachite is faster than `num` due to better algorithms, and slower than
`rug`. My guess is that the better performance of `rug` is due partly to GMP's use of inline
assembly (Malachite has none, being written entirely in safe Rust), and possibly due to operations
on Rust's slices being a little slower than C's raw pointers.

The following is just a small sample of the benchmarks that are available in Malachite. For each
benchmark, I've included the command that you can use to run it yourself. You can specify the
output file using `-o benchfile.gp`, and then use [gnuplot](http://www.gnuplot.info/) to convert
the `.gp` to your format of choice. I like SVG:

`gnuplot -e "set terminal svg; l \"benchfile.gp\"" > benchfile.svg`
