This crate defines `Natural`s (non-negative integers) and `Integer`s. Unlike primitive integers
(`u32`, `i32`, and so on), these may be arbitrarily large. The name of this crate refers to the
mathematical symbols for natural numbers and integers, $\N$ and $\Z$.
- There are many functions defined on `Natural`s and `Integer`s. These include
  - All the ones you'd expect, like addition, subtraction, multiplication, and integer
    division;
  - Implementations of `DivRound`, which provides division that rounds according to a specified
    `RoundingMode`;
  - Various mathematical functions, like implementations of `FloorSqrt` and `Gcd`;
  - Modular arithmetic functions, like implementations of `ModAdd` and `ModPow`, and of traits for
    arithmetic modulo a power of 2, like `ModPowerOf2Add` and `ModPowerOf2Pow`;
  - Various functions for logic and bit manipulation, like `BitAnd` and `BitAccess`.
- The implementations of these functions use high-performance algorithms that work efficiently
  for large numbers. For example, multiplication uses the naive quadratic algorithm, or one of
  13 variants of
  [Toom-Cook multiplication](https://en.wikipedia.org/wiki/Toom%E2%80%93Cook_multiplication),
  or
  [Schönhage-Strassen (FFT) multiplication](https://en.wikipedia.org/wiki/C3%B6nhage%E2%80%93Strassen_algorithm),
  depending on the input size.
- Small numbers are also handled efficiently. Any `Natural` smaller than $2^{64}$ does not use any
  allocated memory, and working with such numbers is almost as fast as working with primitive
  integers. As a result, Malachite does not provide implementations for _e.g._ adding a `Natural`
  to a `u64`, since the `u64` can be converted to a `Natural` very cheaply.
- Malachite handles memory intelligently. Consider the problem of adding a 1000-bit `Natural` and a
  500-bit `Natural`. If we only have references to the `Natural`s, then we must allocate new memory
  for the result, and this is what the `&Natural + &Natural` implementation does. However, if we
  can take the first (larger) `Natural` by value, then we do not need to allocate any memory
  (except in the unlikely case of a carry): we can reuse the memory of the first `Natural` to store
  the result, and this is what the `Natural + &Natural` implementation does. On the other hand, if
  we can only take the second (smaller) `Natural` by value, then we only have 500 bits of memory
  available, which is not enough to store the sum. In this case, the [`Vec`] containing the smaller
  `Natural`'s data can be extended to hold 1000 bits, in hopes that this will be more efficient
  than allocating 1000 bits in a completely new [`Vec`]. Finally, if both `Natural`s are taken by
  value, then the `Natural + Natural` implementation chooses to reuse the memory of the larger one.

  Now consider what happens when evaluating the expression `&x + &y + &z`, where each `Natural` has
  $n$ bits. Malachite must allocate $n$ bits for the result, but what about the intermediate sum
  `&x + &y`? Does Malachite need to allocate another $n$ bits for that? No! Malachite first
  allocates $n$ bits for `&x + &y`, but then that partial sum is taken by _value_ using the
  `Natural + &Natural` implementation described above; so those $n$ bits are reused for the final
  sum.

# Limbs
Large `Natural`s and `Integer`s store their data as `Vec`s of some primitive type. The elements of
these [`Vec`]s are called "limbs" in GMP terminology, since they're large digits. By default, the
type of a `Limb` is `u64`, but you can set it to `u32` using the `32_bit_limbs` feature.

# Demos and benchmarks
This crate comes with a `bin` target that can be used for running demos and benchmarks.
- Almost all of the public functions in this crate have an associated demo. Running a demo shows
  you a function's behavior on a large number of inputs. For example, to demo the `mod_pow`
  function on `Natural`s, you can use the following command:
  ```text
  cargo run --features bin_build --release -- -l 10000 -m exhaustive -d demo_natural_mod_pow
  ```
  This command uses the `exhaustive` mode, which generates every possible input, generally
  starting with the simplest input and progressing to more complex ones. Another mode is
  `random`. The `-l` flag specifies how many inputs should be generated.
- You can use a similar command to run benchmarks. The following command benchmarks various
  GCD algorithms for `u64`s:
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
browsing through `bin_util/demo_and_bench`.

# Features
- `32_bit_limbs`: Sets the type of `Limb` to `u32` instead of the default, `u64`.
- `test_build`: A large proportion of the code in this crate is only used for testing. For a
  typical user, building this code would result in an unnecessarily long compilation time and
  an unnecessarily large binary. Some of it is also used for testing `malachite-q`, so it can't
  just be confined to the `tests` directory. My solution is to only build this code when the
  `test_build` feature is enabled. If you want to run unit tests, you must enable `test_build`.
  However, doctests don't require it, since they only test the public interface.
- `bin_build`: This feature is used to build the code for demos and benchmarks, which also
  takes a long time to build. Enabling this feature also enables `test_build`.