This crate contains many utilities that are used by the `malachite-nz` and `malachite-q`
crates. These utilities include
- Traits that wrap functions from the standard library, like `CheckedAdd`.
- Traits that give extra functionality to primitive types, like `Gcd`, `FloorSqrt`, and
  `BitAccess`.
- Iterator-producing functions that let you generate values for testing. Here's an example of
  an iterator that produces all pairs of `u32`s:
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
- The `RoundingMode` enum, which allows you to specify the rounding behavior of various functions.
- The `NiceFloat` wrapper, which provides alternative implementations of `Eq`, `Ord`, and `Display`
  for floating-point values which are in some ways nicer than the defaults.

# Demos and benchmarks
This crate comes with a `bin` target that can be used for running demos and benchmarks.
- Almost all of the public functions in this crate have an associated demo. Running a demo
  shows you a function's behavior on a large number of inputs. For example, to demo the
  `mod_pow` function on `u32`s, you can use the following command:
  ```
  cargo run --features bin_build --release -- -l 10000 -m exhaustive -d demo_mod_pow_u32
  ```
  This command uses the `exhaustive` mode, which generates every possible input, generally
  starting with the simplest input and progressing to more complex ones. Another mode is
  `random`. The `-l` flag specifies how many inputs should be generated.
- You can use a similar command to run benchmarks. The following command benchmarks various
  GCD algorithms for `u64`s:
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
browsing through `bin_util/demo_and_bench`.

# Features
- `test_build`: A large proportion of the code in this crate is only used for testing. For a
  typical user, building this code would result in an unnecessarily long compilation time and
  an unnecessarily large binary. Much of it is also used for testing `malachite-nz` and
  `malachite-q`, so it can't just be confined to the `tests` directory. My solution is to only
  build this code when the `test_build` feature is enabled. If you want to run unit tests, you
  must enable `test_build`. However, doctests don't require it, since they only test the public
  interface.
- `bin_build`: This feature is used to build the code for demos and benchmarks, which also
  takes a long time to build. Enabling this feature also enables `test_build`.
