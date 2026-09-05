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
- New `AbsSquared` and `AbsSquaredAssign` traits for computing the squared absolute value of a
  number, $|x|^2$, implemented for all numeric types. For real types this is the same as
  squaring; for `GaussianInteger` and `GaussianRational`, `abs_squared` is the sum of the
  squares of the real and imaginary parts (the norm), returned as an `Integer` or `Rational`
  respectively, and `abs_squared_assign` replaces the value with the purely real $|x|^2$
  embedded in the same type. Both traits are supertraits of `PrimitiveInt` and
  `PrimitiveFloat`.
- New `Conjugate` and `ConjugateAssign` traits for computing the complex conjugate of a number,
  implemented for all numeric types. A real number is its own conjugate, so for the real types
  these are the identity; the trivial implementations let generic code use conjugation
  uniformly. For `GaussianInteger` and `GaussianRational` the sign of the imaginary part is
  flipped. Both traits are supertraits of `PrimitiveInt` and `PrimitiveFloat`.
- New `IsGaussianInteger` and `IsReal` traits alongside `IsInteger`, implemented for all
  primitive types (and, in the other crates, all bignum types). For every type,
  `x.is_integer() == x.is_gaussian_integer() && x.is_real()`; for floating-point types, `NaN`
  and the infinities are neither real nor Gaussian integers.
- New `MulI`, `MulIAssign`, `DivI`, and `DivIAssign` traits for multiplying or dividing a number
  by $i$, the imaginary unit — quarter turns in the complex plane, which need no multiplication.
  Nothing in malachite-base implements them; `GaussianInteger` and `GaussianRational` do.
- New `IsUnit`, `CanonicalUnitIPow`, `CanonicalizeUnit`, and `CanonicalizeUnitAssign` traits,
  ports of FLINT's `fmpzi_is_unit`, `fmpzi_canonical_unit_i_pow`, and `fmpzi_canonicalise_unit`:
  a unit test, and canonicalization of a complex number under multiplication by $\pm 1$ and
  $\pm i$, choosing the associate whose argument lies in $(-\pi/4, \pi/4]$. They are
  implemented for all numeric types, so that generic code can normalize associates uniformly:
  for a real type the units are $\pm 1$ (1 alone for unsigned types, and every finite nonzero
  value for floats), the canonical form is the absolute value, and the power of $i$ is 2 for
  negative values and 0 otherwise. All four are supertraits of `PrimitiveInt` and
  `PrimitiveFloat`.
- `IsPowerOf2` is now implemented for the signed primitive integers (negative values are never
  powers of 2), and is a supertrait of `PrimitiveInt` rather than only of `PrimitiveUnsigned`.

### malachite-nz

- Multiplying a `Natural`, `Integer`, `Rational`, or `Float` by itself through aliased
  references (`&x * &x`) now routes to the squaring algorithm, which is faster, and adding a
  `Float` to itself through aliased references routes to a doubling shift. This extends an
  existing convention: several operations, such as `Integer` and `Natural` addition and
  `Natural`'s modular operations, already detect aliased operands and take shortcuts.
- A new `GaussianInteger` type, parallel to `Natural` and `Integer`: a pair of public `Integer`
  fields `real` and `imaginary`, always valid. So far it has the constants 0, 1, 2, -1, i, and
  -i; `Display` and `FromStr` (strict about term structure, permissive about degenerate
  coefficients like `"1i"` and `"0i"`); blanket `From` and `ImaginaryFrom` conversions from
  every type that converts to `Integer`; serde support; and exhaustive, random, and
  striped-random generators, wired into the demo, benchmark, and property-test machinery.
  `GaussianInteger` also implements `IsInteger`, `IsGaussianInteger`, and `IsReal` (and
  `Named`), and `Natural` and `Integer` implement the two new traits (trivially).
- The first arithmetic operations for the Gaussian types: `Neg` and `NegAssign` (negating both
  parts), `Conjugate` and `ConjugateAssign` (flipping the sign of the imaginary part), and
  componentwise addition and subtraction — `Add`, `Sub`, `AddAssign`, and `SubAssign`, in all
  the usual ownership variants — and multiplication (`Mul` and `MulAssign`) for
  `GaussianInteger` and `GaussianRational`. `GaussianInteger` multiplication uses FLINT's
  `fmpzi_mul` strategy: double-word arithmetic when all four parts fit in a signed word, a
  three-multiplication Karatsuba scheme for large balanced operands, and the fused
  `mul_add_mul`/`mul_sub_mul` kernels otherwise; `GaussianRational` multiplication uses the
  fused kernels. Both types also implement `Square` and `SquareAssign`; `GaussianInteger`
  squaring uses FLINT's `fmpzi_sqr` strategy, which prefers squarings over general
  multiplications and short-circuits purely real and purely imaginary values, and
  `GaussianRational` squaring uses the same $a^2 - b^2$, $2ab$ scheme, profiting from the fact
  that squaring a reduced fraction requires no GCD computations. Multiplying a Gaussian value
  by itself through aliased references routes to the squaring algorithm automatically. Both
  types also implement iterator `Sum` and `Product` (by value and by reference), mirroring
  their component types' strategies: `GaussianInteger` sums by accumulating with `+=` like
  `Integer`, `GaussianRational` sums in a balanced binary-tree order like `Rational`, which
  tends to keep intermediate denominators small, and both types multiply in a balanced
  binary-tree order, short-circuiting to zero when any factor is zero, like all four real
  bignum types.
- `OrdAbs` and `PartialOrdAbs` implementations for `GaussianInteger` (and, in malachite-q, for
  `GaussianRational`), comparing absolute values — distances from the origin. Componentwise and
  crosswise part comparisons decide most cases; the squared absolute values are only computed
  when both pairings strictly conflict.
- The full `PartialEq` matrix for the Gaussian types, completing the equality-operator
  convention that mixed-type comparisons get a full matrix: `GaussianInteger` can be compared
  with `Integer`, `Natural`, primitive integers, and primitive floats;
  `GaussianRational` (in malachite-q) with all of those plus `Rational` and `GaussianInteger`;
  and `Float` (in malachite-float) with both Gaussian types. All comparisons work in both
  directions. A Gaussian value equals a real one exactly when its imaginary part is zero and
  its real part is equal; no Gaussian value equals an infinity or NaN.
- The same matrix for `EqAbs`, testing whether absolute values — for complex numbers, distances
  from the origin — are equal, so that $3+4i$ is equal in absolute value to $5$. Comparisons
  against other Gaussian values delegate to the `OrdAbs` screens, and comparisons against real
  values only compute squared absolute values when both components are smaller in absolute
  value than the real operand, mirroring the `OrdAbs` strategy of computing squares only as a
  last resort. A non-integer float can never equal a Gaussian integer's absolute value (its odd
  mantissa squares to an odd numerator), and infinities and NaN are never equal in absolute
  value to anything.
- The same matrix for `PartialOrdAbs`, ordering by absolute value, so that $3+4i$ is greater in
  absolute value than $4$ and less than $6$. The screens are the ordering counterparts of the
  `EqAbs` ones: against a real value, a Gaussian value with two nonzero components is greater in
  absolute value unless both components are smaller in absolute value than the real operand,
  and only then are the squared absolute values compared; comparisons with a float square the
  float exactly (as an odd square times a power of two, or as a `Rational`) rather than
  rounding. NaN is incomparable to everything, and the infinities are greater in absolute value
  than every Gaussian value.
- `PowerOf2` and `IsPowerOf2` for the Gaussian types: `GaussianInteger::power_of_2(k)` and
  `GaussianRational::power_of_2(k)` (the latter also for negative `k`) produce purely real
  powers of 2, and `is_power_of_2` is true only for purely real, positive powers of 2 — $i$ and
  its multiples do not count. `Integer` also gains `IsPowerOf2`, which `Natural` and `Rational`
  already had; negative integers are never powers of 2 (and, in malachite-base, so does every
  signed primitive integer).
- `Shl` and `ShlAssign` for `GaussianInteger` by any unsigned primitive integer, shifting both
  parts (multiplying by a power of 2), in value, reference, and in-place variants. Signed shift
  amounts are deliberately not supported for `GaussianInteger`, since a negative amount would
  be a right shift and exact division by a power of 2 is not generally possible; in malachite-q,
  `GaussianRational` supports both unsigned and signed shift amounts, a negative amount dividing
  both parts exactly, and likewise `Shr` and `ShrAssign` by unsigned and signed amounts.
- `MulI`/`MulIAssign` and `DivI`/`DivIAssign` for both Gaussian types: multiplying by $i$ maps
  $a + bi$ to $-b + ai$ and dividing by $i$ maps it to $b - ai$, by swapping the parts and
  negating one of them.
- `Reciprocal` and `ReciprocalAssign` for `GaussianRational`: the conjugate divided by the squared
  absolute value, with purely real and purely imaginary values reducing to a single `Rational`
  reciprocal. Panics on zero, like `Rational`'s.
- `Div`, `DivAssign`, and `CheckedDiv` for `GaussianRational` in all the usual ownership
  variants. A purely real divisor divides both parts, a purely imaginary divisor does the same
  and then turns the result a quarter turn, and any other divisor multiplies by its reciprocal
  using the fused multiplication kernels. Division by zero panics; `checked_div` returns `None`.
- `IsUnit`, `CanonicalUnitIPow`, `CanonicalizeUnit`, and `CanonicalizeUnitAssign` for both
  Gaussian types, matching FLINT's choices tie for tie, and for `Natural`, `Integer` (and, in
  the other crates, `Rational` and `Float`), where canonical unit form is the absolute value.
  `GaussianInteger`'s units are $\pm 1$ and $\pm i$; `GaussianRational` is a field, so its
  units are the nonzero values.
- `SignificantBits` for both Gaussian types, summing the significant bits of the real and
  imaginary parts, and `GaussianInteger::max_significant_bits`, the larger of the two counts,
  which is FLINT's `fmpzi_bits` and the size measure its algorithm selection uses.
- `DivExact` and `DivExactAssign` for `GaussianInteger`, a port of FLINT's `fmpzi_divexact`: a
  purely real divisor divides both parts, a purely imaginary one does the same and turns the
  result a quarter turn, quotients below $2^{45}$ are recovered by rounding a double-precision
  evaluation of $x\bar{y}/N(y)$ (exact under the divisibility contract, with the operands scaled
  down above 500 bits), and larger quotients go through the exact conjugate-and-norm formula.
  Like the other `div_exact`s, an inexact division may panic or return a meaningless result.
- `DivRem` and `DivAssignRem` for `GaussianInteger`, a port of FLINT's `fmpzi_divrem`: the
  quotient is the exact quotient with each part rounded to the nearest integer, ties up, so the
  remainder satisfies $N(r) \leq N(y)/2$ (the Euclidean division of the Gaussian integers), and
  a dividend more than two bits smaller than the divisor short-cuts to quotient zero.
- The `/`, `/=`, `%`, and `%=` operators and `CheckedDiv` for `GaussianInteger`, with the same
  nearest-quotient rounding as `div_rem`; `/` skips computing the remainder.
- `GaussianInteger::remove_one_plus_i` and `remove_one_plus_i_assign`, a port of FLINT's
  `fmpzi_remove_one_plus_i`: they divide out the largest power of $1 + i$, the Gaussian prime
  above 2, by shifting out the common power of 2, fixing up the unit, and dividing once more by
  $1 + i$ when the parts share a 2-adic valuation, returning the exponent; zero stays zero with
  exponent 0.
- `Gcd` and `GcdAssign` for `GaussianInteger`, a port of FLINT's `fmpzi_gcd` without its lattice
  tier: once all four parts fit in 50 bits the Euclidean algorithm runs entirely in double
  precision, and until then it runs over an approximate nearest-quotient division. The result is
  in canonical unit form, so it is unique; $\gcd(0, 0) = 0$.
- `MulIPow` and `MulIPowAssign` traits in `malachite-base`, multiplication by $i^k$ for a `u64`
  exponent $k$ (only $k$ modulo 4 matters, and $i^{-k} = i^{3k}$), implemented for
  `GaussianInteger` and `GaussianRational` as a port of FLINT's `fmpzi_mul_i_pow_si`;
  `canonicalize_unit` is now defined through it.
- `Pow<u64>` and `PowAssign<u64>` for `GaussianInteger`, a port of FLINT's `fmpzi_pow_ui`: binary
  exponentiation over the fused squaring and multiplication, with purely real and purely
  imaginary bases reduced to an `Integer` power (times $i^n$ for the latter).
- `Pow<u64>`, `Pow<i64>`, and the matching `PowAssign`s for `GaussianRational`, structured like
  the `GaussianInteger` version; a negative exponent takes the reciprocal, and zero to a negative
  power panics, as for `Rational`.
- `ContentAndPrimitivePart`, `Content`, and `PrimitivePart` traits in `malachite-base`, for
  elements of vector spaces over the rationals with a distinguished integer lattice, implemented
  for `GaussianInteger` (content a `Natural`, the GCD of the parts) and `GaussianRational` (content
  a `Rational`, primitive part a `GaussianInteger` with coprime parts). `GaussianRational`'s power
  is computed through the split, so the intermediate values carry no denominators and there is one
  rational reduction per part at the end instead of several per squaring.
- `CheckedSqrt` for `GaussianInteger`, returning the principal square root (positive real part,
  or zero real part and non-negative imaginary part) of a perfect square and `None` otherwise. The
  root is read off the norm: $N = \sqrt{a^2 + b^2}$, then $x = \sqrt{(N + a) / 2}$ and
  $y = \pm \sqrt{(N - a) / 2}$ with the sign of $b$. `GaussianInteger::checked_sqrts` returns
  all the roots as a `Vec`: none, one for zero, or the principal root and its negative, in the
  canonical order of `ComparableGaussianInteger` (lexicographic by real part, then imaginary).
- `CheckedSqrt` and `checked_sqrts` for `GaussianRational` too, by clearing denominators: with
  $L$ the LCM of the denominators and $S = Lz$, $z$ is a square exactly when the Gaussian
  integer $SL$ is, and $\sqrt{z} = \sqrt{SL} / L$.
- `CheckedRoot<u64>` and `checked_roots` for `GaussianInteger`. A nonzero Gaussian integer has
  either no $n$th roots or exactly $\gcd(n, 4)$ of them; the principal one has argument in
  $(-\pi/g, \pi/g]$ for $g = \gcd(n, 4)$, which is the unique root for odd $n$, the
  `checked_sqrt` convention for $n \equiv 2 \pmod 4$, and the canonical unit form for
  $4 \mid n$. The odd part of the exponent is handled exactly through the norm and a Gaussian
  GCD, and the power of 2 by iterated square roots; no floating point is involved.
- `CheckedRoot<u64>` and `checked_roots` for `GaussianRational`, by clearing denominators: with
  $L$ the LCM of the denominators and $S = Lz$, any root $w$ has $Lw$ integral, so $Lw$ is the
  Gaussian integer root of $S L^{n-1}$.
- `ComparableGaussianInteger` and `ComparableGaussianIntegerRef`, wrappers around
  `GaussianInteger` (by value and by reference) that implement `Ord`, comparing
  lexicographically: first by real part, then by imaginary part. Since no total order on the
  complex numbers is compatible with arithmetic, `GaussianInteger` itself does not implement
  `Ord`; the wrappers provide a canonical order for sorting and for use as `BTreeMap` and
  `BTreeSet` keys, in the spirit of malachite-float's `ComparableFloat` and
  `ComparableFloatRef`.
- Conversions between `GaussianInteger` and the real types, completing the conversion matrix:
  `TryFrom` and `ConvertibleFrom` implementations for `Integer` (succeeding when the value is
  real), `Natural` (real and non-negative), all primitive integers (real and representable),
  and all primitive floats (real and exactly representable), plus `TryFrom` and
  `ConvertibleFrom` from primitive floats (finite integers), mirroring the corresponding
  `Rational` conversion families.

### malachite-q

- A new `GaussianRational` type, parallel to `GaussianInteger`: public `Rational` fields `real`
  and `imaginary`, always valid, with the same surface — constants, `Display` and `FromStr`
  (imaginary terms attach `i` to the numerator, as in `"i/2"` and `"2/3-5i/6"`), `From`
  conversions from every type that converts to `Rational` and componentwise conversions from
  `GaussianInteger`, a blanket `ImaginaryFrom`, serde support,
  and the full exhaustive/random/striped generator set with demo, benchmark, and property-test
  plumbing. `GaussianRational` also implements `IsInteger`, `IsGaussianInteger`, and `IsReal`
  (and `Named`), and `Rational` implements the two new traits.
- `ComparableGaussianRational` and `ComparableGaussianRationalRef`, wrappers around
  `GaussianRational` that implement `Ord` lexicographically (real part first, then imaginary
  part), mirroring malachite-nz's `ComparableGaussianInteger` wrappers: a canonical order for
  sorting and for `BTreeMap`/`BTreeSet` keys.
- Conversions between `GaussianRational` and the real types, completing the conversion matrix:
  `TryFrom` and `ConvertibleFrom` implementations for `Rational` (succeeding when the value is
  real), `GaussianInteger` (both parts integers), `Integer` (a real integer), `Natural` (a real
  non-negative integer), all primitive integers (real and representable), and all primitive
  floats (real and exactly representable), plus `TryFrom` and `ConvertibleFrom` from primitive
  floats (finite values) and from `GaussianInteger` (componentwise, added earlier in this
  cycle). `Rational` also gets `TryFrom` and `ConvertibleFrom` from `GaussianInteger`.

### malachite-float

- `Float` implements the new `IsGaussianInteger` and `IsReal` traits; a `Float` is real unless
  it is `NaN` or infinite.
- The first trigonometric function: `Cos` and `CosAssign` (new traits in malachite-base) for
  `Float`, with the usual `cos_prec_round`, `cos_prec`, `cos_round`, and `_ref`/`_assign`
  variants. Arguments of magnitude 4 or more are reduced modulo $2\pi$, so the cost grows with
  the input's exponent as well as with the precision. Inputs extremely close to an odd multiple
  of $\pi/2$ take a dedicated path that computes the distance to that multiple exactly, so the
  result is correct, and underflows correctly, even when the input agrees with the multiple to
  more than $2^{30}$ bits (a regime MPFR's wider exponent range never reaches).
- `primitive_float_cos`, the correctly rounded cosine of an `f32` or `f64`, alongside the
  existing `primitive_float_exp`.
- Conversions between `Float` and the Gaussian types: `TryFrom` and `ConvertibleFrom`
  implementations converting `GaussianInteger` and `GaussianRational` to `Float` (real and, for
  the rational case, dyadic; minimal precision) and `Float` to either Gaussian type (finite,
  and integral for `GaussianInteger`).

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
