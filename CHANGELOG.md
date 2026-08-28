# Changelog

All Malachite crates are versioned in lockstep and released together, so this single file covers
the whole workspace. Within each release, entries are grouped by crate. Entries are added to the
Unreleased section as work lands; at release time the section is stamped with the version and
date. The 0.10.0 section was reconstructed retroactively; releases before 0.10.0 are only
documented by git history.

## 0.12.0 (unreleased)

### malachite-base

- New traits for complex types downstream (nothing in malachite-base implements them): the
  constant traits `I` and `NegativeI`, and the conversion traits `ImaginaryFrom` and
  `ImaginaryInto`.

### malachite-nz

- A new `GaussianInteger` type, parallel to `Natural` and `Integer`: a pair of public `Integer`
  fields `real` and `imaginary`, always valid. So far it has the constants 0, 1, 2, -1, i, and
  -i; `Display` and `FromStr` (strict about term structure, permissive about degenerate
  coefficients like `"1i"` and `"0i"`); blanket `From` and `ImaginaryFrom` conversions from
  every type that converts to `Integer`; serde support; and exhaustive, random, and
  striped-random generators, wired into the demo, benchmark, and property-test machinery.

### Documentation

- The `FromStr` docs for `Natural`, `Integer`, and `Rational` now mention the accepted leading
  `'+'` (and, for `Rational`, the `'+'` allowed on the denominator), which the parsers had
  always accepted.

## 0.11.0 — 2026-08-27

The main themes of this release are a large batch of number-theoretic functions (CRT, modular
division and square roots, rational reconstruction, and a family of combinatorial sequences),
broad new MPFR coverage for `Float` (correctly rounded sums, products, and fused operations;
remainders and rounding functions; bit-exact random samplers; and the constants that complete
the MPFR constants section), ten transition-mapping pages on the website documenting how
Malachite corresponds to GMP, MPFR, FLINT, and num, and a substantial upgrade of the num-bigint
compatibility crate.

### Breaking and behavioral changes

- `Float::increment` and `Float::decrement` are now precision-preserving neighbor steps,
  matching IEEE `nextUp`/`nextDown`, MPFR's `mpfr_nextabove`/`nextbelow`, and Rust's
  `f64::next_up`/`next_down`. Previously they were full-ulp steps that could change a value's
  precision at binade boundaries and collapsed precision-1 powers of 2 to zero. If the old
  behavior is needed, write `x ± x.ulp()`.
- Dividing zero by zero now panics, as the documentation always claimed. Previously an
  equal-operands fast path made `Natural`/`Integer` `div_mod`, `div_rem`, and `div_exact` return
  a quotient of 1 when both operands were zero.
- Formatting a negative `Integer` (or a signed primitive through `BaseFmtWrapper`) with the `+`
  flag, a fill/alignment specifier, or a plain width now follows the standard library's rules.
  Previously `{:+}` printed a stray plus after the minus sign and any width forced zero-padding,
  ignoring fill and alignment. Zero-padded forms like `{:08}` are unchanged.
- In malachite-bigint, `Roots for BigInt` now truncates toward zero on negative inputs instead
  of flooring, and `modinv` returns `None` instead of panicking when the value is a multiple of
  the modulus — both matching num-bigint.

### malachite-base

- New arithmetic traits with primitive implementations, also implemented by the bignum types
  where noted below: `Average` (floor and ceiling midpoints, implemented everywhere),
  `Compound`/`CompoundAssign`, `RisingFactorial`, `MulAddMul`/`MulSubMul` (fused
  `x * y ± z * w`), and the comparison family `PartialOrdDouble`/`PartialOrdAbsDouble`/
  `OrdDouble` (compare a number against twice another without computing the double — the shape
  of a round-to-nearest decision).
- New named constants for primitive floats, with corresponding traits: Catalan's constant and
  Euler's constant.
- GMP-style formatting: `gmp_format!` and friends, with `%Z`, `%Q`, and `%R` conversions
  rendering `Integer`, `Rational`, and `Float` values, plus GMP-compatible string conversions
  to back them.
- Balanced-tree folding for iterator `Sum` and `Product` (`balanced_fold`), improving both
  accuracy and speed of long reductions.

### malachite-nz

- Number theory: the Chinese remainder theorem (`multi_crt` and balanced variants), modular
  division (`ModDiv`, `mod_div_list`), modular square roots (`ModSqrt`), Bell numbers (single
  and vector forms), Landau's function, rising factorials, and completed Fibonacci and Lucas
  sequences with improved subfactorials. A Kronecker symbol edge case was also fixed.
- Fused operations: `mul_shr_round` (a fused `(x * y) >> k` with rounding, via a Mulders short
  product) and `MulAddMul`/`MulSubMul` for `Natural` and `Integer`, and `AddMul`/`SubMul` are
  now faster than their unfused equivalents.
- Performance: division and modular arithmetic with precomputed inverses (modular
  multiplication improved by roughly 20% in the precomputed paths) and fused shift-add limb
  kernels.
- Fixed: the 0/0 division contract and negative-number formatting flags listed above, and an
  unsoundness in the `mpfr_can_round_raw` port with 32-bit limbs (a latent bug in MPFR itself
  on 32-bit-limb builds): a carry absorbed by a truncated limb was misread as a binade change,
  letting `Float::can_round` claim an undecidable rounding was decided.

### malachite-q

- `Rational` GCD and extended GCD (in the lattice sense), rational reconstruction (recovering
  p/q from its residue mod m), Dedekind sums, harmonic numbers, and height functions
  (`to_height`, `into_height`, `height_significant_bits`).
- `simplest_rational_in_interval` now uses FLINT's algorithm, and the related
  denominators-in-interval functions were redesigned around a mediant heap, making some of them
  hundreds of times faster.
- `AddMul` and `SubMul` implementations, sequence utilities, and GMP-style string conversions.

### malachite-float

- Correctly rounded aggregates: `Sum`, `Product`, and dot products (ports of `mpfr_sum` and
  `mpfr_dot`, without the latter's abort on extreme exponents), `add_mul`/`sub_mul` (fused
  multiply-add rounded once), `mul_add_mul`/`mul_sub_mul` (`mpfr_fmma`/`fmms`), and
  `Float`-valued factorials.
- More MPFR coverage: `hypot`, `compound` (with an upstream MPFR rounding bug found and
  corrected in the port), `positive_difference` (`mpfr_dim`), `min`/`max`, remainders (`rem`,
  IEEE remainder, and quotient-bit variants), the round-to-integer family including
  `fractional_part` and integer/fraction decomposition, `can_round`, and `subnormalize`
  (enabling faithful emulation of IEEE formats such as quad precision).
- Mixed `Float`/`Rational` variants throughout (fused operations, remainders, `min`/`max`,
  `positive_difference`), treating the `Rational` operand exactly.
- New constants, correctly rounded to any precision: Euler's constant γ (Brent-McMillan),
  Catalan's constant (Adamchik's formula), and the digit-defined Liouville, Champernowne, and
  Copeland-Erdős constants. This completes the MPFR constants section of the mapping.
- Random generation matching MPFR bit for bit: uniform floats in the unit interval and the
  other MPFR samplers, plus a new suite of random and exhaustive `Float` generators for
  testing.
- `ToStringBase` and additional string-conversion functions.
- More correctly rounded `f32`/`f64` functions in the `primitive_float_*` family, computed
  exactly via `Float` and rounded once, including sums, products, and dot products of slices.
- The `increment`/`decrement` semantics change listed above.

### malachite-bigint

- The behavioral fixes for num-bigint parity listed above (`Roots`, `modinv`).
- New optional features matching num-bigint 0.4.8 exactly: `serde` (identical wire format,
  cross-deserializable with num-bigint), `rand` (`RandBigInt`, `RandomBits`,
  `UniformBigInt`/`UniformBigUint`, producing bit-identical value streams from identically
  seeded RNGs), `arbitrary`, and `quickcheck`.
- Completed API surface: `DoubleEndedIterator` for `U32Digits`, `Mul` for `Sign`, and overrides
  of `num_integer::Integer`'s default methods (`div_mod_floor` in one division, `div_ceil`,
  `gcd_lcm`, `extended_gcd_lcm`, `next`/`prev_multiple_of`, `dec`/`inc`), with
  `Euclid`/`CheckedEuclid` forwarded to Malachite's Euclidean-division operations.
- Removed a hex-parsing workaround for a malachite bug that no longer exists.

### Documentation and website

- Ten transition-mapping pages documenting the correspondence between Malachite and other
  libraries, function by function: GMP integers and rationals; FLINT integers, integers mod n,
  rationals, and arithmetic functions; MPFR floats; and num integers, rationals, and traits.
- A documentation audit across the workspace: complexity annotations verified and standardized
  (conventions recorded in `DOC-CONVENTIONS.md`), plus refreshed front-page examples.

## 0.10.0 — 2026-07-26

Reconstructed retroactively. The two big themes were `Float` elementary functions — the
exponential and power families, correctly rounded at any precision — and a complete rewrite of
`Float`-string interconversion.

### Breaking and behavioral changes

- `Float`'s `Display`, `Debug`, and the other string conversions were rewritten on a port of
  MPFR's `get_str`. Output became correctly rounded scientific decimal at every exponent (the
  old implementation bailed out above `|exponent| > 10000`), with MPFR's precision-dependent
  digit counts, so many outputs differ textually from 0.9.2.
- `RationalSequence` in malachite-base was renamed to `FoerSequence` (a sequence that is Finite
  Or Eventually Repeating), freeing the old name from the misreading that it had something to
  do with `Rational`.
- The internal module layouts of malachite-float and malachite-q were reorganized; code that
  named deep module paths directly may have needed import adjustments.

### Highlights

- The `Float` exponential family: `exp`, `exp_x_minus_1`, `power_of_2`, `power_of_10`, and
  their `_x_minus_1` companions, with `Rational`-argument versions and `Float`-valued outputs
  for primitive inputs.
- The `Float` power family: `Float` raised to `Float`, signed and unsigned integer powers, IEEE
  `powr`, and roots — `sqrt`, `cbrt`, and nth roots (`root_u`, `root_s`) — including
  `Float`-valued square roots, cube roots, and logarithms of unsigned integers.
- String-to-`Float` parsing (a port of `set_str`, bases 2 through 62), `to_sci_string`, and
  serde support for `Float`, alongside the `get_str` rewrite above.
- New constants: e, the cube root of 2, Gelfond's constant, the Gelfond-Schneider constant, and
  Ramanujan's constant, with the corresponding primitive-float constant traits in
  malachite-base and the correctly-rounded-`f32`/`f64` emulation machinery
  (`primitive_float_*`) in malachite-float.
- Performance: string conversion with precomputed inverses and a round of division and
  conversion threshold tuning in malachite-nz.
