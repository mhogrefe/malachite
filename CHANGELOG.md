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
- `Sin` and `SinAssign` (new traits in malachite-base) for `Float`, with the usual
  `sin_prec_round`, `sin_prec`, `sin_round`, and `_ref`/`_assign` variants: a port of `mpfr_sin`,
  which derives the sine from the cosine as $\pm\sqrt{1-\cos^2 x}$ after reducing arguments of
  magnitude 2 or more modulo $2\pi$. Inputs extremely close to a nonzero multiple of $\pi$ share
  the cosine's exact near-zero path, so the result is correct, and underflows correctly, even
  when the input agrees with the multiple to more than $2^{30}$ bits; the path is also taken as
  soon as the argument reduction detects such an input, where MPFR keeps raising its working
  precision instead.
- `sin_rational_prec_round` and `sin_rational_prec` (with `_ref` variants), the sine of a
  `Rational` as a `Float`, alongside the cosine versions. Small inputs are handled by the sine
  series in exact `Rational` arithmetic, so inputs too small to be `Float`s underflow correctly.
- `sin_with_period_prec_round`, `sin_with_period_prec`, and `sin_with_period_round` (with `_ref`
  and `_assign` variants), a port of `mpfr_sinu`: the sine of a `Float` measured in $u$ths of a
  turn. Multiples of a quarter of a turn are exact (a multiple of a half turn is a zero with the
  sign of the input, as IEEE 754-2019's `sinPi` specifies), as are the twelfths whose sine is
  $\pm1/2$, and thirds, sixths, eighths, and twentieths of a turn are computed from a single
  correctly rounded constant ($\sqrt3$, $\sqrt2$, or $\varphi$). Inputs within $2^{-2^{30}}$ of a
  half turn underflow correctly. `sin_with_period_rational_prec_round` and
  `sin_with_period_rational_prec` (with `_ref` variants) take a `Rational` instead, reaching the
  exact and closed-form cases directly, such as a twelfth or a twentieth of a turn.
  `primitive_float_sin_with_period` and `primitive_float_sin_with_period_rational` give the
  correctly rounded `f32` or `f64` results.
- `sin_pi_prec_round`, `sin_pi_prec`, and `sin_pi_round` (with `_ref` and `_assign` variants),
  `sin_pi_rational_prec_round` and `sin_pi_rational_prec` (with `_ref` variants), and
  `primitive_float_sin_pi` and `primitive_float_sin_pi_rational`: a port of `mpfr_sinpi`, the
  sine in half-turns, delegating to the `sin_with_period` family with a period of 2.
- `SinCos` and `SinCosAssign` (new traits in malachite-base) for `Float`, with the usual
  `sin_cos_prec_round`, `sin_cos_prec`, `sin_cos_round`, and `_ref`/`_assign` variants: a port of
  `mpfr_sin_cos`, computing the sine and cosine together with one argument reduction. The two
  results and their two ternary `Ordering`s are returned as a 4-tuple, and the `_assign` variants
  write the cosine to a second `&mut Float`. Inputs extremely close to a zero of either function
  take that function's exact near-zero path, so the results are correct, and underflow correctly,
  even when the input agrees with the zero to more than $2^{30}$ bits.
  `sin_cos_rational_prec_round` and `sin_cos_rational_prec` (with `_ref` variants) take a
  `Rational` instead, sharing the input rounding and, for inputs too large to be `Float`s, the
  reduction modulo $2\pi$ that dominates their cost. `primitive_float_sin_cos` and
  `primitive_float_sin_cos_rational` give the correctly rounded `f32` or `f64` pairs.
- `sin_cos_with_period_prec_round`, `sin_cos_with_period_prec`, and `sin_cos_with_period_round`
  (with `_ref` and `_assign` variants): the sine and cosine of a `Float` measured in $u$ths of a
  turn, together, with one argument reduction, one computation of $2\pi x/u$, and one `sin_cos`
  per iteration. MPFR has no such function. The results are those of the `sin_with_period` and
  `cos_with_period` families, including the exact quarter turns, the closed-form twelfths, sixths,
  and eighths, and the near-zero paths, so inputs within $2^{-2^{30}}$ of a multiple of a quarter
  turn underflow correctly. `sin_cos_with_period_rational_prec_round` and
  `sin_cos_with_period_rational_prec` (with `_ref` variants) take a `Rational` instead, and
  `primitive_float_sin_cos_with_period` and `primitive_float_sin_cos_with_period_rational` give
  the correctly rounded `f32` or `f64` pairs. `sin_cos_pi_prec_round`, `sin_cos_pi_prec`, and
  `sin_cos_pi_round` (with `_ref` and `_assign` variants), `sin_cos_pi_rational_prec_round` and
  `sin_cos_pi_rational_prec` (with `_ref` variants), and `primitive_float_sin_cos_pi` and
  `primitive_float_sin_cos_pi_rational` are the same in half-turns, delegating with a period of 2.
- `Tan` and `TanAssign` (new traits in malachite-base) for `Float`, with the usual
  `tan_prec_round`, `tan_prec`, `tan_round`, and `_ref`/`_assign` variants: a port of `mpfr_tan`,
  the sine and cosine together and their quotient in one Ziv loop. Unlike MPFR's, the result can
  overflow (an input within $2^{-2^{30}}$ of an odd multiple of $\pi/2$) or underflow (within that
  distance of a multiple of $\pi$); both are decided from exact brackets. `tan_rational_prec_round`
  and `tan_rational_prec` (with `_ref` variants) take a `Rational` instead, with a direct series
  bracket for tiny inputs, including those below the `Float` exponent range. `primitive_float_tan`
  and `primitive_float_tan_rational` give the correctly rounded `f32` or `f64` tangent.
- `tan_with_period_prec_round`, `tan_with_period_prec`, and `tan_with_period_round` (with `_ref`
  and `_assign` variants), a port of `mpfr_tanu`: the tangent of a `Float` measured in $u$ths of a
  turn. Multiples of a quarter turn are exact: a multiple of a half turn is a zero (reached from
  below, so that the function is odd), and an odd multiple of a quarter turn is a pole, returning
  an infinity; odd multiples of an eighth of a turn are $\pm1$, and thirds, sixths, and twelfths
  of a turn are computed from $\sqrt3$ or $\sqrt3/3$. Inputs within $2^{-2^{30}}$ of a multiple of
  a quarter turn overflow or underflow correctly. `tan_with_period_rational_prec_round` and
  `tan_with_period_rational_prec` (with `_ref` variants) take a `Rational` instead, reaching the
  exact and closed-form cases directly and needing no argument reduction beyond the exact one.
  `primitive_float_tan_with_period` and `primitive_float_tan_with_period_rational` give the
  correctly rounded `f32` or `f64` tangent; a primitive float is never merely close enough to a
  pole to overflow, but a `Rational` can be.
- `Atan` and `AtanAssign` (new traits in malachite-base) for `Float`, with the usual
  `atan_prec_round`, `atan_prec`, `atan_round`, and `_ref`/`_assign` variants: a port of
  `mpfr_atan`, the first of the inverse trigonometric functions. An input above 1 in magnitude is
  inverted and its arctangent taken from $\pi/2$; the argument is then halved through $\arctan x
  = 2\arctan((\sqrt{1+x^2}-1)/x)$ until it is below about $1/\sqrt{p}$, and split into binary
  chunks whose arctangents are summed by binary splitting of the series for $\arctan(x)/x$, with
  MPFR's table of the twenty most common small chunks for precisions up to 192 bits. The
  arctangent is bounded, so it never overflows; it underflows only for the smallest positive
  `Float` rounded toward zero, a case MPFR's small-input shortcut declines and a series bracket
  decides. `atan(\pm0.0)` is $\pm0.0$, `atan(\pm\infty)` is $\pm\pi/2$ rounded, and the
  function is odd. `primitive_float_atan` gives the correctly rounded `f32` or `f64` arctangent.
  `atan_rational_prec_round` and `atan_rational_prec` (with `_ref` variants) take a `Rational`
  instead. The general case rounds the input once and takes its `Float` arctangent, which is sound
  because the arctangent is 1-Lipschitz, so the input's half-ulp carries to the result unmagnified,
  and because the result is never much smaller than the input. The two ends of the exponent range,
  where the input itself is not a `Float`, are bracketed instead: below it the result is smaller
  than the input and so underflows, decided by the rounding mode alone; above it neither the input
  nor its reciprocal is a `Float`, and $\arctan x = \pi/2 - \arctan(1/x)$ is settled from a
  bracket on $\pi/2$. `primitive_float_atan_rational` gives the correctly rounded `f32` or `f64`
  arctangent of a `Rational`.
- `atan_with_period_prec_round`, `atan_with_period_prec`, `atan_with_period_round`, and
  `atan_with_period` (with `_ref` and `_assign` variants), a port of `mpfr_atanu`: the arctangent
  measured in $u$ths of a turn, $\arctan(x)u/(2\pi)$, which is the inverse of the convention
  `sin_with_period` uses for its input. An infinite input gives a quarter turn and an input of
  $\pm1$ an eighth of a turn, both exactly when `prec` is wide enough to hold them; a zero input,
  or $u = 0$, gives a zero with the sign of $x$, which keeps the function odd. Those are the only
  exact cases. The result never overflows, being under $u/4$, but it underflows for a tiny $x$
  together with a small $u$, where it is about $xu/(2\pi)$; MPFR's wider exponent range never sees
  that, so the quotient is formed with the numerator scaled up and the underflow decided by the
  rounding mode alone. `primitive_float_atan_with_period` gives the correctly rounded `f32` or
  `f64` arctangent in $u$ths of a turn.
- `atan_with_period_rational_prec_round` and `atan_with_period_rational_prec` (with `_ref`
  variants), which take a `Rational` instead. MPFR has no such function. The scaled Ziv loop is
  shared with the `Float` version, with the arctangent supplied by the `Rational` one, so the two
  ends of the exponent range are handled as they are there. The exception is an input below the
  bottom of the range, which is not a `Float` at all: its arctangent is its own leading term, so
  the quotient is formed from the input itself. That is not merely cheaper but necessary, since a
  large $u$ can lift the quotient back into the range, where the `Rational` arctangent's own
  underflow answer would be wrong. `primitive_float_atan_with_period_rational` gives the correctly
  rounded `f32` or `f64` arctangent of a `Rational` in $u$ths of a turn.
- `atan_pi_prec_round`, `atan_pi_prec`, `atan_pi_round`, and `atan_pi` (with `_ref` and `_assign`
  variants), together with `atan_pi_rational_prec_round` and `atan_pi_rational_prec` (with `_ref`
  variants): the arctangent measured in half-turns, $\arctan(x)/\pi$. This is IEEE 754's `atanPi`,
  which MPFR has no direct equivalent of, and it is `atan_with_period` with $u = 2$. Because a half
  and a quarter each need only one bit, the exact cases are exact at *every* precision, unlike the
  general periodic version: an infinite input gives $\pm1/2$, an input of $\pm1$ gives $\pm1/4$,
  and a zero input gives $\pm0.0$. Overflow is impossible, since the result is under $1/2$ in
  magnitude. `primitive_float_atan_pi` and `primitive_float_atan_pi_rational` give the correctly
  rounded `f32` or `f64` arctangent in half-turns.
- `Atan2` and `Atan2Assign` (new traits in malachite-base) for `Float`, with the usual
  `atan2_prec_round`, `atan2_prec`, `atan2_round`, and `_val_ref`/`_ref_val`/`_ref_ref`/`_assign`
  variants: a port of `mpfr_atan2`, the angle of the point $(x,y)$ measured from the positive
  $x$-axis. The twenty ISO C99 special cases are honored, with the sign of a zero argument choosing
  the quadrant: an infinite $y$ gives a quarter turn against a finite $x$, an eighth against
  $+\infty$, and three-eighths against $-\infty$; a zero $y$ gives $\pm0.0$ for a positive-signed
  $x$ and $\pm\pi$ for a negative-signed one. The zero results are the only exact cases. Overflow
  is impossible, since the result is at most $\pi$ in magnitude, but the result underflows for a
  positive $x$ with a tiny $|y/x|$. MPFR widens its exponent range for the whole computation, so
  its quotient $y/x$ is always representable; in Malachite's range it need not be, and a quotient
  beyond the top is taken from the limit $\pi/2$ instead. `primitive_float_atan2` gives the
  correctly rounded `f32` or `f64` angle.
- `atan2_rational_prec_round` and `atan2_rational_prec` (with `_ref` variants), which take
  `Rational` arguments. MPFR has no such function. A `Rational` has no NaN, no infinities, and no
  signed zeros, so the special cases collapse to two: a zero $y$ gives $0.0$ for a nonnegative $x$
  and $\pi$ for a negative one, and a zero $x$ gives $\pm\pi/2$ with the sign of $y$. The zero
  result is the only exact case. The quotient $y/x$ is formed exactly, so nothing corresponds to
  the `Float` case's division, its underflow, or its overflow beyond the exponent range.
  `primitive_float_atan2_rational` gives the correctly rounded `f32` or `f64` angle.
- `atan2_with_period_prec_round`, `atan2_with_period_prec`, and `atan2_with_period_round` (with
  `_val_ref`/`_ref_val`/`_ref_ref`/`_assign` variants): a port of `mpfr_atan2u`, the angle of the
  point $(x,y)$ measured in $u$ths of a turn. The quadrant diagonals are exact, at an eighth and
  three eighths of a turn, as are the axes, and an infinite $y$ against an infinite $x$ gives one
  or the other. Overflow is impossible, since the result is at most $u/2$; the result underflows
  for a positive $x$ with a tiny $|y/x|$ and a small $u$. Two deliberate divergences from MPFR:
  when $u$ is zero this returns a zero with the sign of $y$ throughout, where `mpfr_atan2u` returns
  $\pm1$ for a negative $x$ — contradicting its own definition, the formula it uses for that
  quadrant, and its own answers when $y$ is zero or infinite or $|y|=|x|$; and a quotient beyond
  the exponent range is answered from the turn fraction it approaches, where MPFR, whose widened
  range keeps the quotient representable, can instead spend an unbounded amount of time separating
  that fraction from the representable one beside it. `primitive_float_atan2_with_period` gives the
  correctly rounded `f32` or `f64` angle in $u$ths of a turn.
- `atan2_with_period_rational_prec_round` and `atan2_with_period_rational_prec` (with `_ref`
  variants), which take `Rational` arguments. MPFR has no such function. The quotient $y/x$ is
  exact, so for a positive $x$ the whole computation is the `Rational` arctangent in $u$ths of a
  turn, and only the negative-$x$ reflection needs a loop. The special cases are those a type
  without NaNs, infinities, or signed zeros can have: a zero $y$ gives $0.0$ for a nonnegative $x$
  and $u/2$ for a negative one, a zero $x$ gives $\pm u/4$ with the sign of $y$, and the quadrant
  diagonals give $\pm u/8$ and $\pm3u/8$. `primitive_float_atan2_with_period_rational` gives the
  correctly rounded `f32` or `f64` angle.
- `atan2_pi_prec_round`, `atan2_pi_prec`, and `atan2_pi_round` (with the usual variants), together
  with `atan2_pi_rational_prec_round` and `atan2_pi_rational_prec`: the angle measured in
  half-turns, $\operatorname{atan2}(y,x)/\pi$. This is IEEE 754's `atan2Pi` and a port of
  `mpfr_atan2pi`, and it is `atan2_with_period` with $u = 2$. The turn fractions are $1$, $1/2$,
  $1/4$ and $3/4$, all representable in two bits, so every special case is exact at every precision
  except $\pm3/4$, which needs two. `primitive_float_atan2_pi` and
  `primitive_float_atan2_pi_rational` give the correctly rounded `f32` or `f64` angle in half-turns.
- `Asin` and `AsinAssign` (new traits in malachite-base) for `Float`, with the usual
  `asin_prec_round`, `asin_prec`, `asin_round`, and `_ref`/`_assign` variants: a port of
  `mpfr_asin`, computed as $\arctan(x/\sqrt{1-x^2})$ at a working precision that covers the
  cancellation in $1-x^2$. The result is NaN for a NaN input, for either infinity, and for any
  $|x|>1$; $\pm0.0$ is exact; and $\pm1$ gives $\pm\pi/2$. Those are the only exact cases. Unlike
  the tangent family the arcsine can neither overflow nor underflow, since the result lies in
  $[-\pi/2,\pi/2]$ and $|\arcsin x|>|x|$, so a representable input always has a representable
  result. `primitive_float_asin` gives the correctly rounded `f32` or `f64` arcsine.
- `asin_rational_prec_round` and `asin_rational_prec` (with `_ref` variants), which take a
  `Rational`. MPFR has no such function. The identity is rearranged to
  $\arcsin x = \operatorname{sign}(x)\arctan(\sqrt{x^2/(1-x^2)})$, whose argument is an exact
  `Rational`: nothing cancels, so unlike the `Float` version the cost does not grow as $x$
  approaches $\pm1$, and the input never needs rounding — which matters because the arcsine is not
  1-Lipschitz, so the round-once approach `atan_rational` can afford would cost about half the
  cancelled bits here. Underflow, impossible for the `Float` arcsine, is reachable for a `Rational`
  below the exponent range, and is handled by the scaled path the periodic sine uses.
  `primitive_float_asin_rational` gives the correctly rounded `f32` or `f64` arcsine of a
  `Rational`.
- `asin_with_period_prec_round`, `asin_with_period_prec`, `asin_with_period_round`, and
  `asin_with_period` (with `_ref` and `_assign` variants), a port of `mpfr_asinu`: the arcsine
  measured in $u$ths of a turn, so that $u = 360$ gives degrees. The result is NaN wherever the
  plain arcsine is, even when $u = 0$; $\pm1$ gives $\pm u/4$, a quarter turn; and $\pm1/2$ gives
  $\pm u/12$, a twelfth, when $u$ is a multiple of 3. Two divergences from MPFR: at $u = 0$ MPFR
  returns $+0$ for every $x$, although its own $x = 0$ case keeps the sign so that the function
  stays odd, and Malachite keeps the sign throughout, as `mpfr_atanu` does; and the quotient is
  formed with the numerator scaled up, since $\arcsin(x)u/(2\pi)$ falls below the smallest
  positive `Float` for a tiny $x$ with a small $u$ — a regime MPFR's wider exponent range never
  reaches — with the underflowing result then decided by the rounding mode alone.
  `primitive_float_asin_with_period` gives the correctly rounded `f32` or `f64` angle.
- `asin_with_period_rational_prec_round` and `asin_with_period_rational_prec` (with `_ref`
  variants), which take a `Rational`. MPFR has no such function. The special cases match the
  `Float` version, except that a `Rational` has no signed zeros, so a zero input gives a positive
  zero. Underflow is reachable here for the same reason it is in `asin_rational`: a `Rational` may
  sit far below the bottom of the exponent range, where the arcsine is its own leading term, so the
  quotient is formed from the input itself — necessary rather than merely cheaper, since the
  `Rational` arcsine reports such an input as an underflow and a large $u$ can lift the quotient
  back into range. `primitive_float_asin_with_period_rational` gives the correctly rounded `f32` or
  `f64` angle.
- `asin_pi_prec_round`, `asin_pi_prec`, `asin_pi_round`, and `asin_pi` (with `_ref` and `_assign`
  variants), along with `asin_pi_rational_prec_round` and `asin_pi_rational_prec` (with `_ref`
  variants), a port of `mpfr_asinpi`: the arcsine measured in half-turns, which MPFR defines as
  `asinu` with $u = 2$ and Malachite delegates the same way. An input of $\pm1$ gives $\pm1/2$,
  exact at every precision since a half needs only one bit, and a zero input gives a zero; those
  are the only exact cases, and any $|x|>1$ (or, for a `Float`, NaN or either infinity) gives NaN.
  `primitive_float_asin_pi` and `primitive_float_asin_pi_rational` give the correctly rounded `f32`
  or `f64` angle in half-turns.
- `Acos` and `AcosAssign` (new traits in malachite-base) for `Float`, with the usual
  `acos_prec_round`, `acos_prec`, `acos_round`, and `_ref`/`_assign` variants: a port of
  `mpfr_acos`, computed as $\pi/2-\arctan(x/\sqrt{1-x^2})$ at a working precision that covers both
  the cancellation in that subtraction and the blow-up of the quotient. The result is NaN for a NaN
  input, for either infinity, and for any $|x|>1$; $\pm0.0$ gives $\pi/2$, $1$ gives $0.0$, and
  $-1$ gives $\pi$. The zero at $x=1$ is the only exact case — unlike the arcsine, a zero input is
  not one, since $\pi/2$ is never exactly representable. Overflow is not possible, since the result
  lies in $[0,\pi]$. `primitive_float_acos` gives the correctly rounded `f32` or `f64` arccosine.
- `acos_rational_prec_round` and `acos_rational_prec` (with `_ref` variants), which take a
  `Rational`. MPFR has no such function. The identity is rearranged to
  $\arccos x = \arctan(\sqrt{(1-x^2)/x^2})$, whose argument is an exact `Rational`: for a positive
  $x$ that is the whole answer, and nothing cancels anywhere, so unlike the `Float` version the cost
  does not grow as $x$ approaches 1. A negative $x$ is $\pi$ minus that, which loses a single bit at
  worst. Underflow, impossible for the `Float` arccosine, is reachable here, since a `Rational` may
  lie within $2^{-2^{31}}$ of 1; there $\arccos x$ is about $\sqrt{2(1-x)}$, and that form is used
  directly, which also avoids squaring an input that close to 1.
  `primitive_float_acos_rational` gives the correctly rounded `f32` or `f64` arccosine of a
  `Rational`.
- `acos_with_period_prec_round`, `acos_with_period_prec`, `acos_with_period_round`, and
  `acos_with_period` (with `_ref` and `_assign` variants), a port of `mpfr_acosu`: the arccosine
  measured in $u$ths of a turn, so that $u = 360$ gives degrees. The result is NaN wherever the
  plain arccosine is, even when $u = 0$; a zero input gives $u/4$, a quarter turn; $1$ gives $0.0$,
  following IEEE 754-2019's `acosPi`; $-1$ gives $u/2$; and $\pm1/2$ gives $u/6$ or $u/3$ when $u$
  is a multiple of 3. A zero period gives $+0.0$, since the arccosine is never negative. Like
  MPFR's, the implementation answers a tiny input from the neighbour of $u/4$ on the correct side,
  and like the other periodic inverse functions, the quotient is formed with the numerator scaled
  up so that an underflowing result is decided by the rounding mode alone.
  `primitive_float_acos_with_period` gives the correctly rounded `f32` or `f64` angle.
- `acos_with_period_rational_prec_round` and `acos_with_period_rational_prec` (with `_ref`
  variants), which take a `Rational`. MPFR has no such function. The special cases match the
  `Float` version. Underflow is reachable here for the same reason it is in `acos_rational`: a
  `Rational` may lie within $2^{-2^{31}}$ of 1, where $\arccos x$ falls below the smallest positive
  `Float`. That case is answered from $\sqrt{2(1-x)}$ directly, which is necessary rather than
  merely cheaper — the `Rational` arccosine reports such an input as an underflow, and a large $u$
  can lift the quotient back into range. `primitive_float_acos_with_period_rational` gives the
  correctly rounded `f32` or `f64` angle.
- `acos_pi_prec_round`, `acos_pi_prec`, `acos_pi_round`, and `acos_pi` (with `_ref` and `_assign`
  variants), along with `acos_pi_rational_prec_round` and `acos_pi_rational_prec` (with `_ref`
  variants), a port of `mpfr_acospi`: the arccosine measured in half-turns, which MPFR defines as
  `acosu` with $u = 2$ and Malachite delegates the same way. A zero input gives $1/2$, an input of
  1 gives $0.0$, and an input of $-1$ gives $1$; all three are exact at every precision, and they
  are the only exact cases. `primitive_float_acos_pi` and `primitive_float_acos_pi_rational` give
  the correctly rounded `f32` or `f64` angle in half-turns.
- `Asec` and `AsecAssign` (new traits in malachite-base) for `Float`, with the usual
  `asec_prec_round`, `asec_prec`, `asec_round`, and `_ref`/`_assign` variants. MPFR has no
  arcsecant. Rather than take $\arccos(1/x)$, which would round the reciprocal first and pay for it
  — the arccosine is not Lipschitz at 1, so an input near $\pm1$ would lose about half the bits of
  the reciprocal — the identity is used as $\operatorname{asec} x = \arctan(\sqrt{x^2-1})$ for a
  positive $x$, and $\pi$ minus that for a negative one. The subtraction $x^2-1$ is done at twice
  the input's precision, where it is exact, so unlike the arccosine the working precision does not
  grow as the input approaches $\pm1$. The result is NaN for a NaN input and for any $|x|<1$,
  including the zeros; $\pm\infty$ gives $\pi/2$, the value the secant grows toward; $1$ gives
  $0.0$, the only exact case; and $-1$ gives $\pi$. `primitive_float_asec` gives the correctly
  rounded `f32` or `f64` arcsecant.
- `asec_rational_prec_round` and `asec_rational_prec` (with `_ref` variants), which take a
  `Rational`. There $x^2-1$ is exact with no working precision to choose at all, so the identity
  $\operatorname{asec} x = \arctan(\sqrt{x^2-1})$ applies directly; a large $x$ skips the square
  altogether, its arcsecant being the arctangent of $|x|$ to within the working precision. Underflow,
  impossible for the `Float` arcsecant, is reachable here, since a `Rational` may lie within
  $2^{-2^{31}}$ of 1; there $\operatorname{asec} x$ is about $\sqrt{2(x-1)}$, and that form is used
  directly. `primitive_float_asec_rational` gives the correctly rounded `f32` or `f64` arcsecant of
  a `Rational`.
- `asec_with_period_prec_round`, `asec_with_period_prec`, `asec_with_period_round`, and
  `asec_with_period` (with `_ref` and `_assign` variants), the arcsecant measured in $u$ths of a
  turn, so that $u = 360$ gives degrees. Its exact cases are the arccosine's, seen through the
  reciprocal: NaN and every $|x|<1$ give NaN, even when $u = 0$; $\pm\infty$ gives $u/4$, a quarter
  turn, where the arccosine has a zero input; $1$ gives $0.0$; $-1$ gives $u/2$; and $\pm2$ give
  $u/6$ and $u/3$ when $u$ is a multiple of 3, where the arccosine has $\pm1/2$. A zero period gives
  $+0.0$, the arcsecant never being negative. A large $x$ is answered from the neighbour of $u/4$,
  as the arccosine answers a tiny one. `primitive_float_asec_with_period` gives the correctly
  rounded `f32` or `f64` angle.
- `asec_with_period_rational_prec_round` and `asec_with_period_rational_prec` (with `_ref`
  variants), which take a `Rational`. The exact cases are the `Float` version's, minus the
  infinities a `Rational` cannot be: every $|x|<1$ gives NaN, even when $u = 0$; a zero period gives
  $+0.0$; $1$ gives $0.0$; $-1$ gives $u/2$; and $\pm2$ give $u/6$ and $u/3$ when $u$ is a multiple
  of 3. As for the `Rational` arccosine, an input close enough to 1 makes the arcsecant itself
  underflow, so the quotient is formed from $\sqrt{2(x-1)}$ directly, a large $u$ being able to lift
  it back into the range. `primitive_float_asec_with_period_rational` gives the correctly rounded
  `f32` or `f64` angle of a `Rational`.
- `asec_pi_prec_round`, `asec_pi_prec`, `asec_pi_round`, and `asec_pi` (with `_ref` and `_assign`
  variants), along with `asec_pi_rational_prec_round` and `asec_pi_rational_prec` (with `_ref`
  variants): the arcsecant measured in half-turns, which is `asec_with_period` with $u = 2$ and
  Malachite delegates the same way. Either infinity gives $1/2$, an input of 1 gives $0.0$, and an
  input of $-1$ gives $1$; all three are exact at every precision, and they are the only exact
  cases. Unlike every other period, $\pm2$ are not exact ones here, a third and a two-thirds of a
  half-turn not being representable. NaN and any $|x|<1$, including the zeros, give NaN.
  `primitive_float_asec_pi` and `primitive_float_asec_pi_rational` give the correctly rounded `f32`
  or `f64` angle in half-turns. This closes the arcsecant, a function MPFR does not have.
- `Acsc` and `AcscAssign` (new traits in malachite-base) for `Float`, with the usual
  `acsc_prec_round`, `acsc_prec`, `acsc_round`, and `_ref`/`_assign` variants. MPFR has no
  arccosecant. Rather than take $\arcsin(1/x)$, which would round the reciprocal first and pay for
  it — the arcsine is not Lipschitz at 1, and $1/x$ lands there exactly when $x$ is near $\pm1$, so
  about half the bits of the reciprocal would be lost — the identity is used in the form
  $\operatorname{acsc} x = \arctan(1/\sqrt{x^2-1})$. The subtraction $x^2-1$ is done at twice the
  input's precision, where it is exact, and the reciprocal and the square root are taken together by
  one correctly rounded `reciprocal_sqrt`, so the working precision does not grow as the input
  approaches $\pm1$. The arccosecant is odd, so the sign is stripped and restored with the rounding
  mode reflected along with it. The result is NaN for a NaN input and for any $|x|<1$, including the
  zeros; $\pm\infty$ give $\pm0.0$, the only exact cases; $1$ gives $\pi/2$ and $-1$ gives
  $-\pi/2$. Neither overflow nor underflow is possible: $|\operatorname{acsc} x|\leq\pi/2$, and a
  [`Float`]'s bounded exponent keeps $1/|x|$ above twice the smallest positive `Float`.
  `primitive_float_acsc` gives the correctly rounded `f32` or `f64` arccosecant.
- `acsc_rational_prec_round` and `acsc_rational_prec` (with `_ref` variants), which take a
  `Rational`. There $x^2-1$ is exact with no working precision to choose at all; a large $|x|$ skips
  the square altogether, its arccosecant being the arctangent of the reciprocal of $|x|$ to within
  the working precision. Underflow, impossible for the `Float` arccosecant, is reachable here, a
  `Rational` having no exponent bound: $\operatorname{acsc} x$ is about $1/x$, so a large enough
  $|x|$ puts it below the smallest positive `Float`. `primitive_float_acsc_rational` gives the
  correctly rounded `f32` or `f64` arccosecant of a `Rational`.
- `acsc_with_period_prec_round`, `acsc_with_period_prec`, `acsc_with_period_round`, and
  `acsc_with_period` (with `_ref` and `_assign` variants), the arccosecant measured in $u$ths of a
  turn, so that $u = 360$ gives degrees. Its exact cases are the arcsine's, seen through the
  reciprocal: NaN and every $|x|<1$ give NaN, even when $u = 0$; $\pm\infty$ give $\pm0.0$; a
  zero period gives a zero with the sign of $x$, the function being odd; $\pm1$ give $\pm u/4$, a
  quarter turn; and $\pm2$ give $\pm u/12$ when $u$ is a multiple of 3, where the arcsine has
  $\pm1/2$. Unlike the arccosecant alone, this can underflow: for the largest `Float`s
  $1/|x|$ is only twice the smallest positive one, and a small $u$ carries the quotient below it.
  `primitive_float_acsc_with_period` gives the correctly rounded `f32` or `f64` angle.
- `acsc_with_period_rational_prec_round` and `acsc_with_period_rational_prec` (with `_ref`
  variants), which take a `Rational`. The exact cases are the `Float` version's, minus the
  infinities a `Rational` cannot be: every $|x|<1$ gives NaN, even when $u = 0$; a zero period gives
  a zero with the sign of $x$; $\pm1$ give $\pm u/4$; and $\pm2$ give $\pm u/12$ when $u$ is a
  multiple of 3. A `Rational` having no exponent bound, an $|x|$ large enough makes the arccosecant
  itself underflow, so the quotient is formed from the exact reciprocal directly, a large $u$ being
  able to lift it back into the range. `primitive_float_acsc_with_period_rational` gives the
  correctly rounded `f32` or `f64` angle of a `Rational`.
- `acsc_pi_prec_round`, `acsc_pi_prec`, `acsc_pi_round`, and `acsc_pi` (with `_ref` and `_assign`
  variants), along with `acsc_pi_rational_prec_round` and `acsc_pi_rational_prec` (with `_ref`
  variants): the arccosecant measured in half-turns, which is `acsc_with_period` with $u = 2$ and
  Malachite delegates the same way. Either infinity gives a zero of its sign and an input of $\pm1$
  gives $\pm1/2$; both are exact at every precision, and they are the only exact cases. Unlike
  every other period, $\pm2$ are not exact ones here, a sixth of a half-turn not being
  representable. NaN and any $|x|<1$, including the zeros, give NaN. `primitive_float_acsc_pi` and
  `primitive_float_acsc_pi_rational` give the correctly rounded `f32` or `f64` angle in half-turns.
  This closes the arccosecant, a function MPFR does not have.
- `Acot` and `AcotAssign` (new traits in malachite-base) for `Float`, with the usual
  `acot_prec_round`, `acot_prec`, `acot_round`, and `_ref`/`_assign` variants. MPFR has no
  arccotangent. This is the odd branch, $\operatorname{acot} x = \arctan(1/x)$, with range
  $(-\pi/2,\pi/2]$: the one that makes the arcsecant, arccosecant and arccotangent a uniform
  family of inverses of reciprocal arguments, that inverts `cot` on its own signed behaviour
  ($\cot(\pm0)=\pm\infty$ and so $\operatorname{acot}(\pm\infty)=\pm0$), and that Mathematica uses;
  the continuous branch $\pi/2-\arctan x$ with range $(0,\pi)$ is not provided. Unlike the other two
  inverses of reciprocals, nothing is lost to the reciprocal's rounding here, the arctangent being
  smooth everywhere; below 1 the reciprocal is not taken at all, the identity
  $\operatorname{acot} x = \pi/2 - \arctan x$ being used instead, which cannot cancel. NaN gives
  NaN; $\pm\infty$ give $\pm0.0$, the only exact cases; $\pm0.0$ give $\pm\pi/2$, the sign choosing
  the side of the jump; and $\pm1$ give $\pm\pi/4$. Neither overflow nor underflow is possible.
  `primitive_float_acot` gives the correctly rounded `f32` or `f64` arccotangent.
- `acot_rational_prec_round` and `acot_rational_prec` (with `_ref` variants), which take a
  `Rational`. Above 1 in magnitude the reciprocal is exact and the arctangent of it is the same real
  number, so the arctangent's own machinery decides everything, underflow at a huge $|x|$ included;
  below 1 the subtraction from $\pi/2$ is used. A `Rational` zero has no sign, so it gives $\pi/2$,
  the side the positive inputs approach. `primitive_float_acot_rational` gives the correctly rounded
  `f32` or `f64` arccotangent of a `Rational`.
- `Cot` and `CotAssign` (new traits in malachite-base) for `Float`, with the usual
  `cot_prec_round`, `cot_prec`, `cot_round`, and `_ref`/`_assign` variants: a port of `mpfr_cot`,
  MPFR's generic reciprocal template with the tangent. MPFR's tangent is itself a quotient of a
  sine and a cosine, so the cotangent is taken as $\cos x/\sin x$ directly, which saves the middle
  rounding and treats the two ends of the exponent range alike. Unlike the secant and the cosecant,
  the cotangent is not bounded away from zero, so it both overflows, within $2^{-2^{30}}$ of a
  multiple of $\pi$, and underflows, within $2^{-2^{30}}$ of an odd multiple of $\pi/2$; each end
  is decided from an exact bracket. MPFR's shortcut for a tiny input is kept and is load-bearing:
  there $\cot x$ is $1/x - x/3 + \ldots$, so rounding $1/x$ settles the result, except when $x$ is
  a power of 2 and $1/x$ is exact, where the true value lies one step short of it, toward zero.
  Without it the Ziv loop would face an exactly representable quotient that no working precision
  could certify. `cot(\pm0.0)` is $\pm\infty$, and the function is odd.
  `primitive_float_cot` gives the correctly rounded `f32` or `f64` cotangent, which neither the
  standard library nor `libm` provides; it overflows for a small enough input.
  `cot_rational_prec_round` and `cot_rational_prec` (with `_ref` variants) take a `Rational`
  instead, with a direct bracket for a tiny input, inverting the tangent's own series bracket; that
  also covers inputs below the `Float` exponent range, which no other path could round, and the
  powers of 2, whose reciprocals are exactly representable and which the Ziv loop could therefore
  never certify. `primitive_float_cot_rational` gives the correctly rounded `f32` or `f64`
  cotangent of a `Rational`.
- `cot_with_period_prec_round`, `cot_with_period_prec`, `cot_with_period_round`, and
  `cot_with_period` (with `_ref` and `_assign` variants), the cotangent of a `Float` measured in
  $u$ths of a turn. MPFR has no `cotu`; this is `cot` with the sine and cosine taken in turns, which
  reduces the argument exactly and so reaches the exact and closed-form cases the radian version
  cannot see. These are the tangent's, reciprocated: odd multiples of $1/8$ of a turn give exactly
  $\pm1$, odd multiples of $1/4$ give exactly $\pm0.0$, and thirds and sixths give
  $\pm\sqrt3/3$ while twelfths give $\pm\sqrt3$. Multiples of a half turn are the poles, where
  the sine is a zero carrying the sign of $x$ and the cosine is $\pm1$, so the infinity takes the
  sign of $x$ at an even multiple and the opposite at an odd one; keeping that identity is what
  makes the function odd. `primitive_float_cot_with_period` gives the correctly rounded `f32` or
  `f64` cotangent in $u$ths of a turn.
  `cot_with_period_rational_prec_round` and `cot_with_period_rational_prec` (with `_ref` variants)
  take a `Rational` instead, reaching the exact and closed-form cases directly and needing no
  argument reduction beyond the exact one. A `Rational` fraction of a turn can be small enough, or
  close enough to a multiple of a half turn, to overflow, and as close to an odd quarter turn to
  underflow; each end is decided from an exact bracket.
  `primitive_float_cot_with_period_rational` gives the correctly rounded `f32` or `f64` cotangent
  of a `Rational` fraction of a turn.
- `cot_pi_prec_round`, `cot_pi_prec`, `cot_pi_round`, and `cot_pi` (with `_ref` and `_assign`
  variants), the cotangent of a `Float` measured in half-turns, delegating to `cot_with_period`
  with a period of 2; MPFR has no `cotpi` to match its `sinpi` and `cospi`. Integers are poles and
  give $\pm\infty$, with the sign of $x$ at an even integer and the opposite at an odd one;
  half-integers give $\pm0.0$, odd multiples of $1/4$ give $\pm1$, odd multiples of $1/6$ give
  $\pm\sqrt3$, and multiples of $1/3$ that are not integers give $\pm\sqrt3/3$.
  `cot_pi_rational_prec_round` and `cot_pi_rational_prec` (with `_ref` variants) take a `Rational`
  instead, and `primitive_float_cot_pi` and `primitive_float_cot_pi_rational` give the correctly
  rounded `f32` or `f64` cotangent.
- `Csc` and `CscAssign` (new traits in malachite-base) for `Float`, with the usual
  `csc_prec_round`, `csc_prec`, `csc_round`, and `_ref`/`_assign` variants: a port of `mpfr_csc`,
  MPFR's generic reciprocal template with the sine. The cosecant never underflows, since its
  magnitude is at least 1, but unlike MPFR's it can overflow: within $2^{-2^{30}}$ of a multiple of
  $\pi$, and for any input whose reciprocal alone leaves the range. MPFR's shortcut for a tiny
  input is kept, where $\csc x$ is $1/x + x/6 + \ldots$ and rounding $1/x$ settles the result
  except when $x$ is a power of 2; without it the Ziv loop could never certify an exactly
  representable reciprocal. `csc(\pm0.0)` is $\pm\infty$, and the function is odd.
  `primitive_float_csc` gives the correctly rounded `f32` or `f64` cosecant, which overflows for a
  small enough input.
  `csc_rational_prec_round` and `csc_rational_prec` (with `_ref` variants) take a `Rational`
  instead, with a direct bracket for a tiny input, inverting a bracket on the sine; that also
  covers inputs below the `Float` exponent range, which no other path could round.
  `primitive_float_csc_rational` gives the correctly rounded `f32` or `f64` cosecant of a
  `Rational`.
- `csc_with_period_prec_round`, `csc_with_period_prec`, `csc_with_period_round`, and
  `csc_with_period` (with `_ref` and `_assign` variants), the cosecant of a `Float` measured in
  $u$ths of a turn. MPFR has no `cscu`; this is `csc` with the sine taken in turns, which reduces
  the argument exactly and so reaches the exact and closed-form cases the radian version cannot
  see: odd quarter turns give $\pm1$, odd twelfths give $\pm2$, eighths give $\pm\sqrt2$, thirds
  and sixths give $\pm2\sqrt3/3$, and twentieths give $\pm2\varphi$ or $\pm2(\varphi-1)$, where
  $\varphi$ is the golden ratio. Multiples of a half turn are poles, where the sine is a zero
  carrying the sign of the input and the cosecant is its reciprocal, an infinity with that sign;
  keeping that identity is what makes the function odd everywhere, at the cost of period $u$ at a
  pole alone. Unlike the secant's, this cosecant can overflow away from a pole, since a tiny angle
  has a huge cosecant; such a result is decided from an exact bracket on the sine.
  `primitive_float_csc_with_period` gives the correctly rounded `f32` or `f64` cosecant in $u$ths
  of a turn, which does overflow for a small enough angle.
  `csc_with_period_rational_prec_round` and `csc_with_period_rational_prec` (with `_ref` variants)
  take a `Rational` instead, reaching the exact and closed-form cases directly and needing no
  argument reduction beyond the exact one. The radian version's shortcut for a tiny input has no
  counterpart here: in turns the angle is never a `Float`, so by Niven's theorem the sine past the
  closed-form cases is irrational and its reciprocal is never exactly representable, which is what
  stalls the radian loop. A `Rational` fraction of a turn can be small enough, or close enough to a
  multiple of a half turn, that the sine falls below the `Float` exponent range; the bracket reads
  that as the overflow it is. `primitive_float_csc_with_period_rational` gives the correctly
  rounded `f32` or `f64` cosecant of a `Rational` fraction of a turn.
- `csc_pi_prec_round`, `csc_pi_prec`, `csc_pi_round`, and `csc_pi` (with `_ref` and `_assign`
  variants), the cosecant of a `Float` measured in half-turns, delegating to `csc_with_period` with
  a period of 2; MPFR has no `cscpi` to match its `sinpi` and `cospi`. Integers are poles and give
  $\pm\infty$ with the sign of $x$, half-integers give $\pm1$, odd multiples of $1/6$ give
  $\pm2$, odd multiples of $1/4$ give $\pm\sqrt2$, and multiples of $1/3$ that are not integers
  give $\pm2\sqrt3/3$. `csc_pi_rational_prec_round` and `csc_pi_rational_prec` (with `_ref`
  variants) take a `Rational` instead, and `primitive_float_csc_pi` and
  `primitive_float_csc_pi_rational` give the correctly rounded `f32` or `f64` cosecant.
- `Sec` and `SecAssign` (new traits in malachite-base) for `Float`, with the usual
  `sec_prec_round`, `sec_prec`, `sec_round`, and `_ref`/`_assign` variants: a port of `mpfr_sec`,
  which instantiates MPFR's generic reciprocal template with the cosine. The secant never
  underflows, since its magnitude is at least 1, but unlike MPFR's it can overflow, for an input
  within $2^{-2^{30}}$ of an odd multiple of $\pi/2$; such a result is decided from an exact
  bracket on the cosine. `primitive_float_sec` gives the correctly rounded `f32` or `f64` secant,
  which neither the standard library nor `libm` provides.
  `sec_rational_prec_round` and `sec_rational_prec` (with `_ref` variants) take a `Rational`
  instead, with a direct series bracket for a tiny input: there the cosine rounds toward zero to
  the `Float` just below 1, whose reciprocal ties back to 1 at every working precision, so the Ziv
  loop would not terminate without it. `primitive_float_sec_rational` gives the correctly rounded
  `f32` or `f64` secant of a `Rational`.
- `sec_with_period_prec_round`, `sec_with_period_prec`, `sec_with_period_round`, and
  `sec_with_period` (with `_ref` and `_assign` variants), the secant of a `Float` measured in $u$ths
  of a turn. MPFR has no `secu`; this is `sec` with the cosine taken in turns, which reduces the
  argument exactly and so reaches the exact and closed-form cases the radian version cannot see:
  even multiples of a half turn give $1$ and odd ones $-1$, thirds and sixths give $\pm2$, eighths
  give $\pm\sqrt2$, twelfths give $\pm2\sqrt3/3$, and fifths and tenths give $\pm2\varphi$ or
  $\pm2(\varphi-1)$, where $\varphi$ is the golden ratio. Odd multiples of a quarter turn are poles,
  where the cosine is $+0.0$ and the secant is its reciprocal, $\infty$; keeping that identity is
  what makes the function even everywhere. `primitive_float_sec_with_period` gives the correctly
  rounded `f32` or `f64` secant in $u$ths of a turn; like the tangent's, it can only reach an
  infinity at an exact pole, never through overflow.
  `sec_with_period_rational_prec_round` and `sec_with_period_rational_prec` (with `_ref` variants)
  take a `Rational` instead, reaching the exact and closed-form cases directly and needing no
  argument reduction beyond the exact one. `primitive_float_sec_with_period_rational` gives the
  correctly rounded `f32` or `f64` secant of a `Rational` fraction of a turn.
- `sec_pi_prec_round`, `sec_pi_prec`, `sec_pi_round`, and `sec_pi` (with `_ref` and `_assign`
  variants), the secant of a `Float` measured in half-turns, delegating to `sec_with_period` with a
  period of 2; MPFR has no `secpi` to match its `sinpi` and `cospi`. Even integers give $1$ and odd
  ones $-1$, half-integers are poles and give $\infty$, odd multiples of $1/4$ give $\pm\sqrt2$,
  and multiples of $1/3$ give $\pm2$. `sec_pi_rational_prec_round` and `sec_pi_rational_prec` (with
  `_ref` variants) take a `Rational` instead, and `primitive_float_sec_pi` and
  `primitive_float_sec_pi_rational` give the correctly rounded `f32` or `f64` secant.
- `tan_pi_prec_round`, `tan_pi_prec`, `tan_pi_round`, and `tan_pi` (with `_ref` and `_assign`
  variants), a port of `mpfr_tanpi`: the tangent of a `Float` measured in half-turns, delegating to
  `tan_with_period` with a period of 2. Integers give a signed zero, half-integers are poles and
  give an infinity, odd multiples of a quarter give $\pm1$, and thirds and sixths give $\pm\sqrt3$
  or $\pm\sqrt3/3$. `tan_pi_rational_prec_round` and `tan_pi_rational_prec` (with `_ref` variants)
  take a `Rational` instead, and `primitive_float_tan_pi` and `primitive_float_tan_pi_rational`
  give the correctly rounded `f32` or `f64` tangent.
- `sin_with_period`, `cos_with_period`, `tan_with_period`, `sin_cos_with_period`, `sin_pi`,
  `cos_pi`, and `sin_cos_pi` on `Float` (each with `_ref` and `_assign` variants), rounding to the
  precision of the input and to the nearest `Float`. This is the tier that `sin`, `cos`, `tan`, and
  `sin_cos` already had through their traits; a function that takes a period cannot go through one,
  since `Sin` and its siblings take no extra argument, so these are inherent methods.
- The Dottie number, the fixed point of the cosine, as `dottie_number_prec_round` and
  `dottie_number_prec` on `Float`, correctly rounded to any precision (Newton's method with a
  certified final bracket), and as a `DottieNumber` trait with constants for primitive floats.
- `sin`, `cos`, and `sin_cos` now use MPFR's asymptotically fast tier (`mpfr_sincos_fast`, binary
  splitting of the Taylor series over chunks of the reduced argument, combined by the angle-addition
  formulas) at and above a tuned precision threshold (25285 bits), as MPFR does at its
  `MPFR_SINCOS_THRESHOLD`, bringing their cost from $O(n^{3/2})$ to $O(n \log^3 n)$ word
  operations up to log factors. The tuner (`-g tune_sincos` in the `malachite-float` binary)
  shares its crossover machinery with `malachite-nz`'s, now in
  `malachite_base::test_util::bench::tune`.
- Fixed `cos_with_period_rational_prec_round` taking a working precision of billions of bits for a
  tiny negative input, and `cos_with_period_prec_round` doing the same for a `Float` just below a
  multiple of its period: the fraction of a turn is now reduced to $[-1/2, 1/2]$, where the
  small-input shortcut applies.
- `primitive_float_sin` and `primitive_float_sin_rational`, the correctly rounded sine of an `f32`
  or `f64`, or of a `Rational` as an `f32` or `f64`.
- `primitive_float_cos` and `primitive_float_cos_rational`, the correctly rounded cosine of an
  `f32` or `f64`, or of a `Rational` as an `f32` or `f64`, alongside the existing
  `primitive_float_exp` and `primitive_float_exp_rational`.
- Fixed `Float` remainders (`rem` and `ieee_remainder` families) by a divisor of more than
  $2^{30}$ bits with an odd mantissa, which overflowed to infinity: the integer remainder was
  rounded to a `Float` before the final shift brought it back into range. `cos` of an argument
  with a near-maximal exponent reduces modulo such a $2\pi$ and was affected.
- `cos_with_period_prec_round`, `cos_with_period_prec`, and `cos_with_period_round` (with `_ref` and `_assign` variants),
  a port of `mpfr_cosu`: the cosine of a `Float` measured in $u$ths of a turn, so that `u = 360`
  is degrees. Multiples of a quarter or a sixth of a turn are exact (an odd quarter turn is
  $+0.0$, as IEEE 754-2019's `cosPi` specifies), and eighths, twelfths, fifths, and tenths of a
  turn are computed from a single correctly rounded constant ($\sqrt2$, $\sqrt3$, or $\varphi$)
  rather than from $\pi$ and a cosine. Inputs within $2^{-2^{30}}$ of an odd quarter turn
  underflow correctly. `cos_with_period_rational_prec_round` and `cos_with_period_rational_prec`
  (with `_ref` variants) take a `Rational` instead, reaching the exact and closed-form cases
  directly, such as a third or an eighth of a turn. `primitive_float_cos_with_period` and
  `primitive_float_cos_with_period_rational` give the correctly rounded `f32` or `f64` results.
- `cos_pi_prec_round`, `cos_pi_prec`, and `cos_pi_round` (with `_ref` and `_assign` variants),
  `cos_pi_rational_prec_round` and `cos_pi_rational_prec` (with `_ref` variants), and
  `primitive_float_cos_pi` and `primitive_float_cos_pi_rational`: a port of `mpfr_cospi`, the
  cosine in half-turns, delegating to the `cos_with_period` family with a period of 2.
- `cos_rational_prec_round` and `cos_rational_prec` (with `_ref` variants), the correctly
  rounded cosine of a `Rational` as a `Float`, alongside the `exp_rational_*` family. Since
  cosine is not monotonic, the result is bracketed by a Lipschitz bound around the cosine of a
  `Float` approximation rather than by bracketing the input, with exact `Rational` handling near
  odd multiples of $\pi/2$ and for inputs too large to be `Float`s.
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
