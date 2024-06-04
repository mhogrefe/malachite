- [crates.io](https://crates.io/crates/malachite-nz)
- [docs.rs](https://docs.rs/malachite-base/latest/malachite_nz/)

Rather than using this crate directly, use the
[`malachite`](https://crates.io/crates/malachite) meta-crate. It re-exports all of this crate's
public members.

In `malachite-nz`'s doctests you will frequently see import paths beginning with
`malachite_nz::`. When using the `malachite` crate, replace this part of the paths with
`malachite::`.

The import paths of the `Natural` and `Integer` types are shortened further, to
`malachite::Natural` and `malachite::Integer`.

# malachite-nz
This crate defines
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html)s
(non-negative integers) and
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html)s. Unlike
primitive integers ([`u32`](https://doc.rust-lang.org/nightly/std/primitive.u32.html),
[`i32`](https://doc.rust-lang.org/nightly/std/primitive.i32.html), and so on), these may be
arbitrarily large. The name of this crate refers to the mathematical symbols for natural numbers
and integers, ℕ and ℤ.
- There are many functions defined on
  [`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html)s and
  [`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html)s. These
  include
  - All the ones you'd expect, like addition, subtraction, multiplication, and integer division;
  - Implementations of
    [`DivRound`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivRound.html),
    which provides division that rounds according to a specified
    [`RoundingMode`](https://docs.rs/malachite-base/latest/malachite_base/rounding_modes/enum.RoundingMode.html);
  - Various mathematical functions, like implementations of
    [`FloorSqrt`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.FloorSqrt.html)
    and
    [`Gcd`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Gcd.html);
  - Modular arithmetic functions, like implementations of
    [`ModAdd`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModAdd.html)
    and
    [`ModPow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPow.html),
    and of traits for arithmetic modulo a power of 2, like
    [`ModPowerOf2Add`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPowerOf2Add.html)
    and
    [`ModPowerOf2Pow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPowerOf2Pow.html);
  - Various functions for logic and bit manipulation, like
    [`BitAnd`](https://doc.rust-lang.org/nightly/core/ops/trait.BitAnd.html) and
    [`BitAccess`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.BitAccess.html).
- The implementations of these functions use high-performance algorithms that work efficiently
  for large numbers. For example, multiplication uses the naive quadratic algorithm, or one of
  13 variants of
  [Toom-Cook multiplication](https://en.wikipedia.org/wiki/Toom%E2%80%93Cook_multiplication),
  or
  [Schönhage-Strassen (FFT) multiplication](https://en.wikipedia.org/wiki/Schonhage-Strassen_algorithm),
  depending on the input size.
- Small numbers are also handled efficiently. Any
  [`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html) smaller
  than 2<sup>64</sup> does not use any allocated memory, and working with such numbers is almost as
  fast as working with primitive integers. As a result, Malachite does not provide implementations
  for _e.g._ adding a
  [`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html)
  to a [`u64`](https://doc.rust-lang.org/nightly/std/primitive.u64.html), since the
  [`u64`](https://doc.rust-lang.org/nightly/std/primitive.u64.html) can be converted to a
  [`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html) very
  cheaply.
- Malachite handles memory intelligently. Consider the problem of adding a 1000-bit
  [`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html) and a
  500-bit
  [`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html). If we
  only have references to the
  [`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html)s, then
  we must allocate new memory for the result, and this is what the `&Natural + &Natural`
  implementation does. However, if we can take the first (larger)
  [`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html) by
  value, then we do not need to allocate any memory (except in the unlikely case of a carry): we
  can reuse the memory of the first
  [`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html) to
  store the result, and this is what the `Natural + &Natural` implementation does. On the other
  hand, if we can only take the second (smaller)
  [`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html) by
  value, then we only have 500 bits of memory available, which is not enough to store the sum. In
  this case, the [`Vec`](https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html) containing
  the smaller
  [`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html)'s data
  can be extended to hold 1000 bits, in hopes that this will be more efficient than allocating 1000
  bits in a completely new [`Vec`](https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html).
  Finally, if both
  [`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html)s are
  taken by value, then the `Natural + Natural` implementation chooses to reuse the memory of the
  larger one.

  Now consider what happens when evaluating the expression `&x + &y + &z`, where each [`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html) has
  _n_ bits. Malachite must allocate about _n_ bits for the result, but what about the intermediate
  sum `&x + &y`? Does Malachite need to allocate another _n_ bits for that, for a total of 2 _n_
  bits? No! Malachite first allocates _n_ bits for `&x + &y`, but then that partial sum is taken by
  _value_ using the `Natural + &Natural` implementation described above; so those _n_ bits are
  reused for the final sum.

# Limbs
Large [`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html)s
and [`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html)s
store their data as [`Vec`](https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html)s of some
primitive type. The elements of these
[`Vec`](https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html)s are called "limbs" in GMP
terminology, since they're large digits. By default, the type of a `Limb` is
[`u64`](https://doc.rust-lang.org/nightly/std/primitive.u64.html), but you can set it to
[`u32`](https://doc.rust-lang.org/nightly/std/primitive.u32.html) using the `32_bit_limbs` feature.

# Demos and benchmarks
This crate comes with a `bin` target that can be used for running demos and benchmarks.
- Almost all of the public functions in this crate have an associated demo. Running a demo shows
  you a function's behavior on a large number of inputs. For example, to demo the
  [`mod_pow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPow.html#tymethod.mod_pow)
  function on [`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html)s, you can use the following command:
  ```text
  cargo run --features bin_build --release -- -l 10000 -m exhaustive -d demo_natural_mod_pow
  ```
  This command uses the `exhaustive` mode, which generates every possible input, generally
  starting with the simplest input and progressing to more complex ones. Another mode is
  `random`. The `-l` flag specifies how many inputs should be generated.
- You can use a similar command to run benchmarks. The following command benchmarks various
  GCD algorithms for [`u64`](https://doc.rust-lang.org/nightly/std/primitive.u64.html)s:
  ```text
  cargo run --features bin_build --release -- -l 1000000 -m random -b \
      benchmark_natural_gcd_algorithms -o gcd-bench.gp
  ```
  or GCD implementations of other libraries:
  ```text
  cargo run --features bin_build --release -- -l 1000000 -m random -b \
      benchmark_natural_gcd_library_comparison -o gcd-bench.gp
  ```
  This creates a file called gcd-bench.gp. You can use gnuplot to create an SVG from it like
  so:
  ```text
  gnuplot -e "set terminal svg; l \"gcd-bench.gp\"" > gcd-bench.svg
  ```

The list of available demos and benchmarks is not documented anywhere; you must find them by
browsing through
[`bin_util/demo_and_bench`](https://github.com/mhogrefe/malachite/tree/master/malachite-nz/src/bin_util/demo_and_bench).

# Features
- `32_bit_limbs`: Sets the type of `Limb` to [`u32`](https://doc.rust-lang.org/nightly/std/primitive.u32.html) instead of the default, [`u64`](https://doc.rust-lang.org/nightly/std/primitive.u64.html).
- `random`: This feature provides some functions for randomly generating values. It is off by
  default to avoid pulling in some extra dependencies.
- `enable_serde`: Enables serialization and deserialization using [serde](`https://serde.rs/`).
- `test_build`: A large proportion of the code in this crate is only used for testing. For a
  typical user, building this code would result in an unnecessarily long compilation time and
  an unnecessarily large binary. Some of it is also used for testing
  [`malachite-q`](https://crates.io/crates/malachite-q), so it can't just be confined to the
  `tests` directory. My solution is to only build this code when the `test_build` feature is
  enabled. If you want to run unit tests, you must enable `test_build`. However, doctests don't
  require it, since they only test the public interface. Enabling this feature also enables
  `random`.
- `bin_build`: This feature is used to build the code for demos and benchmarks, which also
  takes a long time to build. Enabling this feature also enables `test_build` and `random`.

Malachite is developed by Mikhail Hogrefe. Thanks to b4D8, florian1345, konstin, Rowan Hart, YunWon Jeong, Park Joon-Kyu, Antonio Mamić, OliverNChalk, and shekohex for additional contributions.

Copyright © 2024 Mikhail Hogrefe
