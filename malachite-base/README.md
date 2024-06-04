- [crates.io](https://crates.io/crates/malachite-base)
- [docs.rs](https://docs.rs/malachite-base/latest/malachite_base/)

Rather than using this crate directly, use the
[`malachite`](https://crates.io/crates/malachite) meta-crate. It re-exports all of this crate's
public members.

In `malachite-base`'s doctests you will frequently see import paths beginning with
`malachite_base::`. When using the `malachite` crate, replace this part of the paths with
`malachite::`.

# malachite-base
This crate contains many utilities that are used by the
[`malachite-nz`](https://crates.io/crates/malachite-nz) and
[`malachite-q`](https://crates.io/crates/malachite-q) crates. These utilities include
- Traits that wrap functions from the standard library, like
  [`CheckedAdd`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CheckedAdd.html).
- Traits that give extra functionality to primitive types, like
  [`Gcd`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Gcd.html),
  [`FloorSqrt`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.FloorSqrt.html),
  and
  [`BitAccess`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.BitAccess.html).
- Iterator-producing functions that let you generate values for testing. Here's an example of
  an iterator that produces all pairs of
  [`u32`](https://doc.rust-lang.org/nightly/std/primitive.u32.html)s:
  ```
  use malachite_base::num::exhaustive::exhaustive_unsigneds;
  use malachite_base::tuples::exhaustive::exhaustive_pairs_from_single;

  let mut pairs = exhaustive_pairs_from_single(exhaustive_unsigneds::<u32>());
  assert_eq!(
      pairs.take(20).collect::<Vec<_>>(),
      &[
          (0, 0), (0, 1), (1, 0), (1, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 0), (2, 1),
          (3, 0), (3, 1), (2, 2), (2, 3), (3, 2), (3, 3), (0, 4), (0, 5), (1, 4), (1, 5)
      ]
  );
  ```
- The
  [`RoundingMode`](https://docs.rs/malachite-base/latest/malachite_base/rounding_modes/enum.RoundingMode.html)
  enum, which allows you to specify the rounding behavior of various functions.
- The
  [`NiceFloat`](https://docs.rs/malachite-base/latest/malachite_base/num/float/struct.NiceFloat.html) wrapper, which provides alternative implementations of
  [`Eq`](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html),
  [`Ord`](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html), and
  [`Display`](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html)
  for floating-point values which are in some ways nicer than the defaults.

# Demos and benchmarks
This crate comes with a `bin` target that can be used for running demos and benchmarks.
- Almost all of the public functions in this crate have an associated demo. Running a demo
  shows you a function's behavior on a large number of inputs. For example, to demo the
  [`mod_pow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPow.html#tymethod.mod_pow)
  function on [`u32`](https://doc.rust-lang.org/nightly/std/primitive.u32.html)s, you can use the
  following command:
  ```
  cargo run --features bin_build --release -- -l 10000 -m exhaustive -d demo_mod_pow_u32
  ```
  This command uses the `exhaustive` mode, which generates every possible input, generally
  starting with the simplest input and progressing to more complex ones. Another mode is
  `random`. The `-l` flag specifies how many inputs should be generated.
- You can use a similar command to run benchmarks. The following command benchmarks various
  GCD algorithms for [`u64`](https://doc.rust-lang.org/nightly/std/primitive.u64.html)s:
  ```text
  cargo run --features bin_build --release -- -l 1000000 -m random -b \
      benchmark_gcd_algorithms_u64 -o gcd-bench.gp
  ```
  This creates a file called gcd-bench.gp. You can use gnuplot to create an SVG from it like
  so:
  ```text
  gnuplot -e "set terminal svg; l \"gcd-bench.gp\"" > gcd-bench.svg
  ```

The list of available demos and benchmarks is not documented anywhere; you must find them by
browsing through
[`bin_util/demo_and_bench`](https://github.com/mhogrefe/malachite/tree/master/malachite-base/src/bin_util/demo_and_bench).

# Features
- `random`: This feature provides some functions for randomly generating values. It is off by
  default to avoid pulling in some extra dependencies.
- `test_build`: A large proportion of the code in this crate is only used for testing. For a
  typical user, building this code would result in an unnecessarily long compilation time and
  an unnecessarily large binary. Much of it is also used for testing
  [`malachite-nz`](https://crates.io/crates/malachite-nz) and
  [`malachite-q`](https://crates.io/crates/malachite-q), so it can't just be confined to the
  `tests` directory. My solution is to only build this code when the `test_build` feature is
  enabled. If you want to run unit tests, you must enable `test_build`. However, doctests don't
  require it, since they only test the public interface. Enabling this feature also enables
  `random`.
- `bin_build`: This feature is used to build the code for demos and benchmarks, which also
  takes a long time to build. Enabling this feature also enables `test_build` and `random`.

Malachite is developed by Mikhail Hogrefe. Thanks to b4D8, florian1345, konstin, Rowan Hart, YunWon Jeong, Park Joon-Kyu, Antonio Mamić, OliverNChalk, and shekohex for additional contributions.

Copyright © 2024 Mikhail Hogrefe
