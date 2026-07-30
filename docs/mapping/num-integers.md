---
layout: default
title: "Malachite for num Users: Integers"
permalink: /mapping/num-integers/
theme: jekyll-theme-slate
---

# Malachite for num Users: Integers

This page maps the types of the [num-bigint](https://docs.rs/num-bigint/latest/num_bigint/)
crate, `BigUint` and `BigInt`, onto their Malachite counterparts:
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html) and
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html), from
the `malachite-nz` crate. It covers num-bigint 0.4.8 as re-exported by the
[num](https://docs.rs/num/latest/num/) umbrella crate, together with the parts of
[num-integer](https://docs.rs/num-integer/latest/num_integer/) 0.1.46 and
[num-traits](https://docs.rs/num-traits/latest/num_traits/) 0.2.19 that these types implement,
since methods like `div_floor` and `checked_add` reach a num user through those traits. The
[mapping index](/mapping/) lists the whole family of pages.

Unlike the C libraries on the other pages, num has no manual whose organization a mapping can
follow: its API is the sum of a type's inherent methods and its trait implementations, spread
across three crates. The sections below are therefore organized by theme, and together they
cover the full public surface of the two types.

One asset exists here that no other page has. Because num and Malachite are both Rust,
migration has two routes. The drop-in route is
[malachite-bigint](https://crates.io/crates/malachite-bigint), a crate that reimplements the
num-bigint API on top of Malachite's arithmetic, so that most code keeps compiling with a
changed `use` line. The native route is this page: the same operations in Malachite's own
vocabulary, which is in places differently shaped. The drop-in route is the fast
way off the fence; the native route is where the rest of Malachite, from
[`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html) to
[`Float`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html),
becomes available. Measured performance comparisons between the libraries live on
[the performance page](/performance), not here.

## Conventions {#conventions}

### The types

The correspondence is one-to-one, and closer than on any other page of this family. `BigUint`
is an unsigned bignum and `BigInt` wraps one in a sign, `Sign` being `Minus`, `NoSign`, or
`Plus`; a
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html) is an
unsigned bignum and an
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html) is a
sign and a `Natural` magnitude. Both libraries maintain the same invariants without help, no
high zero digits and a zero that is never negative, so no canonicalization caveats appear
anywhere below. Both keep the magnitude in machine words sized to match the platform: num's
`BigUint` holds a `Vec` of 64-bit digits on 64-bit targets and 32-bit digits elsewhere, keyed
to the pointer width, while Malachite's limbs are 64-bit by default and 32-bit under the
`32_bit_limbs` feature. One representation difference is worth knowing: a `BigUint` stores its
digits on the heap whenever it is nonzero, while a `Natural` or `Integer` whose magnitude is
less than $$2^{64}$$ is stored inline with no allocation, as described
[on the GMP integers page](/mapping/gmp-integers/#allocation).

### Idiom parity

Most of the friction the C-library pages document does not arise here. Both libraries
implement the arithmetic operators for owned and borrowed operands in all combinations, so
`&a + &b` and `a + b` mean the same things on both sides; both implement the `*Assign`
operators; both follow the primitive types' conventions where they apply, truncating `/` and
`%` for signed division included. Values are created, not initialized; dropping frees; there
are no rounding modes or ternary values, these being exact types. The differences that remain
are vocabulary. num reaches much of its API through traits, `num_traits::One`,
`num_integer::Integer`, and so on, imported by their crate paths; Malachite's equivalents live
in [`malachite_base`](https://docs.rs/malachite-base/latest/malachite_base/)'s trait modules
and are re-exported through the umbrella
[`malachite`](https://docs.rs/malachite/latest/malachite/) crate. Where num names a method
after C tradition or its own history, Malachite tends to spell the operation out; the tables
give the dictionary.

### Features

The feature flags correspond directly. num-bigint's `serde` is Malachite's `serde`;
num-bigint's `rand` is Malachite's `random`, with different generator designs described under
Random below; both libraries work without `std`, num by disabling its default `std` feature
and Malachite by default. num-bigint's `arbitrary` and `quickcheck` integrations have no
Malachite counterparts, Malachite's testing story being its own
[generator machinery](https://docs.rs/malachite-base/latest/malachite_base/).

### Categories

Each item falls into one of four categories:

| | meaning |
| :---: | --- |
| ✓ | A Malachite function does the same thing. |
| ≈ | A Malachite function serves the same purpose, but its specification differs. The notes say how. |
| — | No counterpart is needed; the notes say why. |
| ✗ | Malachite does not fully support this yet, but will in a future version. |

## Construction and constants {#construction-and-constants}

| | num | Malachite |
| :---: | --- | --- |
| ✓ | `BigUint::new (digits: Vec<u32>) -> BigUint` | [`from_power_of_2_digits_asc`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html#method.from_power_of_2_digits_asc), [`from_limbs_asc`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html#method.from_limbs_asc) |
| ✓ | `BigUint::from_slice (slice: &[u32]) -> BigUint` | [`from_power_of_2_digits_asc`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html#method.from_power_of_2_digits_asc), [`from_limbs_asc`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html#method.from_limbs_asc) |
| ✓ | `BigUint::assign_from_slice (&mut self, slice: &[u32])` | [`from_power_of_2_digits_asc`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html#method.from_power_of_2_digits_asc) |
| ✓ | `BigUint::from_bytes_be (bytes: &[u8]) -> BigUint` | [`from_power_of_2_digits_desc`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html#method.from_power_of_2_digits_desc) |
| ✓ | `BigUint::from_bytes_le (bytes: &[u8]) -> BigUint` | [`from_power_of_2_digits_asc`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html#method.from_power_of_2_digits_asc) |
| ✓ | `BigUint::from_radix_be (buf: &[u8], radix: u32) -> Option<BigUint>` | [`from_digits_desc`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html#method.from_digits_desc) |
| ✓ | `BigUint::from_radix_le (buf: &[u8], radix: u32) -> Option<BigUint>` | [`from_digits_asc`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html#method.from_digits_asc) |
| ✓ | `BigInt::new (sign: Sign, digits: Vec<u32>) -> BigInt` | [`from_sign_and_abs`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html#method.from_sign_and_abs) |
| ✓ | `BigInt::from_biguint (sign: Sign, data: BigUint) -> BigInt` | [`from_sign_and_abs`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html#method.from_sign_and_abs) |
| ✓ | `BigInt::from_slice (sign: Sign, slice: &[u32]) -> BigInt` | [`from_sign_and_abs`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html#method.from_sign_and_abs) |
| ✓ | `BigInt::assign_from_slice (&mut self, sign: Sign, slice: &[u32])` | [`from_sign_and_abs`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html#method.from_sign_and_abs) |
| ✓ | `BigInt::from_bytes_be (sign: Sign, bytes: &[u8]) -> BigInt` | [`from_sign_and_abs`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html#method.from_sign_and_abs) |
| ✓ | `BigInt::from_bytes_le (sign: Sign, bytes: &[u8]) -> BigInt` | [`from_sign_and_abs`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html#method.from_sign_and_abs) |
| ≈ | `BigInt::from_signed_bytes_be (digits: &[u8]) -> BigInt` | [`from_twos_complement_limbs_desc`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html#method.from_twos_complement_limbs_desc) |
| ≈ | `BigInt::from_signed_bytes_le (digits: &[u8]) -> BigInt` | [`from_twos_complement_limbs_asc`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html#method.from_twos_complement_limbs_asc) |
| ✓ | `BigInt::from_radix_be (sign: Sign, buf: &[u8], radix: u32) -> Option<BigInt>` | [`from_digits_desc`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html#method.from_digits_desc) |
| ✓ | `BigInt::from_radix_le (sign: Sign, buf: &[u8], radix: u32) -> Option<BigInt>` | [`from_digits_asc`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html#method.from_digits_asc) |
| ✓ | `Default` (`BigUint`, `BigInt`) | [`Default`](https://doc.rust-lang.org/nightly/std/default/trait.Default.html) |
| ✓ | `Zero`, `ConstZero` (`zero()`, `ZERO`, `is_zero`) | [`Zero`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.Zero.html) |
| ✓ | `One`, `ConstOne` (`one()`, `ONE`, `is_one`) | [`One`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.One.html) |
| ✓ | `From<u8..u128, usize> for BigUint` | [`From`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html#impl-From%3Cu64%3E-for-Natural) |
| ✓ | `From<all primitive ints> for BigInt` | [`From`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html#impl-From%3Ci64%3E-for-Integer) |
| ≈ | `FromPrimitive for BigUint` (`from_u64`, `from_i64`, `from_f64`, ...) | [`TryFrom`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/conversion/primitive_int_from_natural/index.html), [`RoundingFrom`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/conversion/from_primitive_float/index.html) |
| ≈ | `FromPrimitive for BigInt` | [`TryFrom`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/conversion/from_primitive_float/index.html), [`RoundingFrom`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/conversion/from_primitive_float/index.html) |

**The word-digit constructors.** `BigUint::new`, `from_slice`, and `assign_from_slice` build
from "base $$2^{32}$$ digits... ordered least significant digit first", and num keeps that
`u32` interface on every platform, whatever its internal digit size. Malachite offers the same
construction two ways: `Natural::from_power_of_2_digits_asc(32, digits.iter().copied())`
matches num's interface digit for digit, and
[`from_limbs_asc`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html#method.from_limbs_asc)
builds from native limbs, 64 bits by default. `assign_from_slice` is plain reassignment,
without the buffer-reuse intent behind num's version. For `BigInt`, wrap the magnitude:
`Integer::from_sign_and_abs(sign, abs)` takes a `bool` where num takes a `Sign`, and the
three-valued `Sign` collapses cleanly, `Plus` and `Minus` becoming `true` and `false` and
`NoSign` taking care of itself, a zero magnitude producing zero whatever the sign argument
says.

**The byte constructors.** Big- and little-endian bytes are base-256 digits:
`from_power_of_2_digits_desc(8, ...)` and `from_power_of_2_digits_asc(8, ...)` respectively,
with an empty slice giving zero in both libraries. The signed pair is ≈ for a granularity
reason: `from_signed_bytes_be` reads two's complement bytes, while Malachite's
`from_twos_complement_limbs_asc` and `_desc` read two's complement *limbs*, so the bytes must
be repacked into limbs, sign-extending the top byte into a full word, before the same
semantics apply.

**The radix constructors.** `from_radix_be` and `from_radix_le` take digit *values*, one per
byte, in any radix from 2 to 256, and num's documentation demonstrates base 190;
[`from_digits_desc`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html#method.from_digits_desc)
and `from_digits_asc` are the same constructors with the 256 ceiling removed, the base being
any unsigned primitive or even a
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html).
Both libraries return `None` when a digit is not below the base.

**The constants.** `Default` is zero on both sides. num's `Zero` and `One`, with their 0.4.8
`ConstZero` and `ConstOne` refinements, map to Malachite's
[`Zero`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.Zero.html)
and `One`, whose `ZERO` and `ONE` are associated constants usable in `const` contexts from the
start; the `is_zero` and `is_one` predicates are `== Natural::ZERO`-style comparisons or the
`Zero`/`One` traits' own methods on the num side. Malachite adds `Two`, `NegativeOne` on
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html), and
`const`-buildable small values beyond the identities through `const_from`.

**The primitive conversions.** `From` covers every unsigned primitive into
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html) and
every primitive into
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html),
matching num's infallible impls. `FromPrimitive` is the Option-returning layer, and its ≈ is
shape rather than substance: `BigUint::from_i64(-1)` returns `None` where
`Natural::try_from(-1i64)` returns an `Err`, and `from_f64` accepts a float, discards its
fractional part, and returns `None` for NaN, infinities, and negatives, which in Malachite is
spelled with an explicit rounding, `Natural::rounding_from(f, Down)`, or exactly, with
`try_from`; the float cases that num answers with `None` are panics or `Err`s here, stated at
each site's documentation.

## Conversion {#conversion}

The outward direction, plus the traffic between the two types. Every digit and byte
deconstructor from the previous section has its inverse here, and the same correspondences
hold, with one recurring edge noted below: what zero deconstructs to.

| | num | Malachite |
| :---: | --- | --- |
| ✓ | `BigUint::to_bytes_be (&self) -> Vec<u8>` | [`to_power_of_2_digits_desc`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html#method.to_power_of_2_digits_desc) |
| ✓ | `BigUint::to_bytes_le (&self) -> Vec<u8>` | [`to_power_of_2_digits_asc`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html#method.to_power_of_2_digits_asc) |
| ✓ | `BigUint::to_radix_be (&self, radix: u32) -> Vec<u8>` | [`to_digits_desc`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html#method.to_digits_desc) |
| ✓ | `BigUint::to_radix_le (&self, radix: u32) -> Vec<u8>` | [`to_digits_asc`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html#method.to_digits_asc) |
| ✓ | `BigUint::to_u32_digits (&self) -> Vec<u32>` | [`to_power_of_2_digits_asc`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html#method.to_power_of_2_digits_asc) |
| ✓ | `BigUint::to_u64_digits (&self) -> Vec<u64>` | [`to_limbs_asc`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html#method.to_limbs_asc) |
| ✓ | `BigUint::iter_u32_digits (&self) -> U32Digits` | [`PowerOf2DigitIterable`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.PowerOf2DigitIterable.html) |
| ✓ | `BigUint::iter_u64_digits (&self) -> U64Digits` | [`limbs`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html#method.limbs) |
| ≈ | `ToPrimitive for BigUint` (`to_u64`, `to_i64`, `to_f64`, ...) | [`TryFrom`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/conversion/primitive_int_from_natural/index.html), [`RoundingFrom`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/conversion/primitive_float_from_natural/index.html) |
| ✓ | `TryFrom<&BigUint> for u8..u128, i8..i128` | [`TryFrom`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/conversion/primitive_int_from_natural/index.html) |
| ✓ | `ToBigInt for BigUint` (`to_bigint`) | [`From`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html#impl-From%3CNatural%3E-for-Integer) |
| ✓ | `BigInt::to_bytes_be (&self) -> (Sign, Vec<u8>)` | [`Sign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Sign.html), [`to_power_of_2_digits_desc`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html#method.to_power_of_2_digits_desc) |
| ✓ | `BigInt::to_bytes_le (&self) -> (Sign, Vec<u8>)` | [`Sign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Sign.html), [`to_power_of_2_digits_asc`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html#method.to_power_of_2_digits_asc) |
| ≈ | `BigInt::to_signed_bytes_be (&self) -> Vec<u8>` | [`to_twos_complement_limbs_desc`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html#method.to_twos_complement_limbs_desc) |
| ≈ | `BigInt::to_signed_bytes_le (&self) -> Vec<u8>` | [`to_twos_complement_limbs_asc`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html#method.to_twos_complement_limbs_asc) |
| ✓ | `BigInt::to_radix_be (&self, radix: u32) -> (Sign, Vec<u8>)` | [`Sign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Sign.html), [`to_digits_desc`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html#method.to_digits_desc) |
| ✓ | `BigInt::to_radix_le (&self, radix: u32) -> (Sign, Vec<u8>)` | [`Sign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Sign.html), [`to_digits_asc`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html#method.to_digits_asc) |
| ✓ | `BigInt::to_u32_digits (&self) -> (Sign, Vec<u32>)` | [`Sign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Sign.html), [`to_power_of_2_digits_asc`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html#method.to_power_of_2_digits_asc) |
| ✓ | `BigInt::to_u64_digits (&self) -> (Sign, Vec<u64>)` | [`Sign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Sign.html), [`to_limbs_asc`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html#method.to_limbs_asc) |
| ✓ | `BigInt::iter_u32_digits (&self) -> U32Digits` | [`PowerOf2DigitIterable`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.PowerOf2DigitIterable.html) |
| ✓ | `BigInt::iter_u64_digits (&self) -> U64Digits` | [`limbs`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html#method.limbs) |
| ≈ | `ToPrimitive for BigInt` | [`TryFrom`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/conversion/primitive_int_from_integer/index.html), [`RoundingFrom`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/conversion/primitive_float_from_integer/index.html) |
| ✓ | `TryFrom<&BigInt> for u8..u128, i8..i128` | [`TryFrom`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/conversion/primitive_int_from_integer/index.html) |
| ✓ | `TryFrom<BigInt> for BigUint` | [`TryFrom`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html#impl-TryFrom%3CInteger%3E-for-Natural) |
| ✓ | `BigInt::magnitude (&self) -> &BigUint` | [`unsigned_abs_ref`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html#method.unsigned_abs_ref) |
| ✓ | `BigInt::into_parts (self) -> (Sign, BigUint)` | [`Sign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Sign.html), [`UnsignedAbs`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.UnsignedAbs.html) |
| ✓ | `BigInt::sign (&self) -> Sign` | [`Sign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Sign.html) |
| ✓ | `ToBigUint for BigInt` (`to_biguint`) | [`TryFrom`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html#impl-TryFrom%3CInteger%3E-for-Natural) |
| ✓ | `From<BigUint> for BigInt` | [`From`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html#impl-From%3CNatural%3E-for-Integer) |

**Digits and bytes, outward.** The deconstructors mirror the constructors: bytes are base-256
digits through `to_power_of_2_digits_desc(8)` and `_asc(8)`, `to_radix_be` and `_le` are
`to_digits_desc(&radix)` and `_asc` with the base-256 ceiling again removed, `to_u32_digits`
is `to_power_of_2_digits_asc(32)`, and `to_u64_digits` is `to_limbs_asc`, with
[`limbs`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html#method.limbs)
and
[`PowerOf2DigitIterable`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.PowerOf2DigitIterable.html)'s
`power_of_2_digits(32)` playing the allocation-free roles of `iter_u64_digits` and
`iter_u32_digits`. The one recurring difference is zero: num deconstructs it to a single zero
element, `vec![0]`, while Malachite deconstructs it to nothing, an empty `Vec` or an empty
iterator, so round trips agree but the two libraries' outputs for zero differ by that element.
The `BigInt` versions return the num `Sign` alongside the magnitude's digits, which is
`x.sign()` paired with the same calls on `x.unsigned_abs_ref()`; the signed-bytes pair is the
[two's complement granularity ≈](#construction-and-constants) from the previous section, in
the other direction.

**`ToPrimitive`.** The integer targets are the same story as `FromPrimitive`, `Option` against
`Err`: `u64::try_from(&n)` answers what `n.to_u64()` answers. The float targets differ more
than shape. num's `to_f64` takes the value's 64 most significant bits and converts them, so an
input whose discarded lower bits sit against a rounding boundary can land one ulp from the
correctly rounded result; `f64::rounding_from(&n, Nearest)` rounds the full value correctly,
with the mode explicit and the direction reported. Values beyond `f64`'s range become
infinity in both libraries.

**The magnitude and the sign.** `BigInt::magnitude` borrows the magnitude, and
[`unsigned_abs_ref`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html#method.unsigned_abs_ref)
borrows the same thing, a `&Natural` at no cost; the owning decomposition `into_parts` is
`(x.sign(), x.unsigned_abs())` through the
[`UnsignedAbs`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.UnsignedAbs.html)
trait. `BigInt::sign` maps to Malachite's
[`Sign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Sign.html)
with a three-for-three dictionary: `Plus`, `NoSign`, and `Minus` are `Greater`, `Equal`, and
`Less`, an [`Ordering`](https://doc.rust-lang.org/nightly/std/cmp/enum.Ordering.html) against
zero. Malachite adds one more member here:
[`mutate_unsigned_abs`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html#method.mutate_unsigned_abs)
gives closure-mediated write access to the magnitude, restoring the sign invariant afterward,
where num offers no mutable path to `magnitude()`'s target.

**Between the types.** `BigInt::to_biguint` and `TryFrom<BigInt> for BigUint` both answer
`Natural::try_from(&x)`, failing exactly on negatives; `BigUint::to_bigint` and
`From<BigUint> for BigInt` are `Integer::from(n)`, infallible. num's fallible conversions
carry a
[`TryFromBigIntError`](https://docs.rs/num-bigint/latest/num_bigint/struct.TryFromBigIntError.html),
which for owned conversions returns the original value inside the error; Malachite's error
types are lighter, and the original is not consumed when the conversion borrows.

## Comparison {#comparison}

Both libraries derive or implement the standard comparison traits, and since these are exact
types the orders are total, so the table is short; what distinguishes the libraries here is
everything Malachite implements beyond it, described after the table.

| | num | Malachite |
| :---: | --- | --- |
| ✓ | `PartialEq`, `Eq` (`BigUint`, `BigInt`) | [`PartialEq`](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialEq.html), [`Eq`](https://doc.rust-lang.org/nightly/std/cmp/trait.Eq.html) |
| ✓ | `PartialOrd`, `Ord` (`BigUint`, `BigInt`) | [`PartialOrd`](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialOrd.html), [`Ord`](https://doc.rust-lang.org/nightly/std/cmp/trait.Ord.html) |
| ✓ | `Hash` (`BigUint`, `BigInt`) | [`Hash`](https://doc.rust-lang.org/nightly/std/hash/trait.Hash.html) |
| ✓ | `Signed::is_positive`, `is_negative` (`BigInt`) | [`PartialOrd`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/comparison/partial_cmp_primitive_int/index.html) |

**The standard traits.** Equal values compare and hash equally on both sides, `Ord` agrees
with the numeric order, and `x == y`, `x < y`, and `HashMap` keys behave identically. The
`Signed` predicates are spelled as comparisons against a primitive zero, `x > 0u32` and
`x < 0u32`, both false for zero as num's are; the rest of the `Signed` trait, `abs` and its
kin, appears with the operators below.

**Mixed comparisons and the magnitude order.** num compares its types against themselves;
comparing a
`BigUint` with a `u64` or a `BigInt` with a `BigUint` means converting first. Malachite
implements the mixed comparisons directly, in both argument orders: `PartialEq` and
`PartialOrd` against every primitive integer, against every primitive *float*, exactly and
without allocating, and between
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html) and
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html)
across the type boundary. Alongside the value order there is a magnitude order:
[`OrdAbs`](https://docs.rs/malachite-base/latest/malachite_base/num/comparison/traits/trait.OrdAbs.html),
[`PartialOrdAbs`](https://docs.rs/malachite-base/latest/malachite_base/num/comparison/traits/trait.PartialOrdAbs.html),
and
[`EqAbs`](https://docs.rs/malachite-base/latest/malachite_base/num/comparison/traits/trait.EqAbs.html)
compare `|x|` against `|y|` without materializing either absolute value, again across the
mixed operand types. In num, `a.magnitude().cmp(b.magnitude())` covers the same-type case.
The rest is native-route territory, the drop-in route keeping num's own API surface.

## Arithmetic operators {#arithmetic-operators}

| | num | Malachite |
| :---: | --- | --- |
| ✓ | `Add`, `AddAssign` (`BigUint`, `BigInt`) | [`Add`](https://doc.rust-lang.org/nightly/std/ops/trait.Add.html), [`AddAssign`](https://doc.rust-lang.org/nightly/std/ops/trait.AddAssign.html) |
| ✓ | `Sub`, `SubAssign` | [`Sub`](https://doc.rust-lang.org/nightly/std/ops/trait.Sub.html), [`SubAssign`](https://doc.rust-lang.org/nightly/std/ops/trait.SubAssign.html) |
| ✓ | `Mul`, `MulAssign` | [`Mul`](https://doc.rust-lang.org/nightly/std/ops/trait.Mul.html), [`MulAssign`](https://doc.rust-lang.org/nightly/std/ops/trait.MulAssign.html) |
| ✓ | `Div`, `DivAssign` | [`Div`](https://doc.rust-lang.org/nightly/std/ops/trait.Div.html), [`DivAssign`](https://doc.rust-lang.org/nightly/std/ops/trait.DivAssign.html) |
| ✓ | `Rem`, `RemAssign` | [`Rem`](https://doc.rust-lang.org/nightly/std/ops/trait.Rem.html), [`RemAssign`](https://doc.rust-lang.org/nightly/std/ops/trait.RemAssign.html) |
| ≈ | scalar operators (`BigUint ± u32/u64/u128`, `BigInt ± i32/i64/i128/u32/u64/u128`, both orders) | [`From`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html#impl-From%3Cu64%3E-for-Natural) |
| ✓ | `Neg` (`BigInt`) | [`Neg`](https://doc.rust-lang.org/nightly/std/ops/trait.Neg.html), [`NegAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.NegAssign.html) |
| — | `CheckedAdd`, `CheckedMul` (`BigUint`, `BigInt`), `CheckedSub` (`BigInt`) | |
| ✓ | `CheckedSub` (`BigUint`) | [`CheckedSub`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CheckedSub.html) |
| ≈ | `CheckedDiv` (`BigUint`, `BigInt`) | [`Div`](https://doc.rust-lang.org/nightly/std/ops/trait.Div.html) |
| ✓ | `Sum`, `Product` (`BigUint`, `BigInt`) | [`Sum`](https://doc.rust-lang.org/nightly/std/iter/trait.Sum.html), [`Product`](https://doc.rust-lang.org/nightly/std/iter/trait.Product.html) |
| ✓ | `Signed::abs` (`BigInt`) | [`Abs`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Abs.html), [`AbsAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.AbsAssign.html) |
| ≈ | `Signed::abs_sub` (`BigInt`) | [`Sub`](https://doc.rust-lang.org/nightly/std/ops/trait.Sub.html) |
| ✓ | `Signed::signum` (`BigInt`) | [`Sign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Sign.html) |

**The operators.** Full parity, combination by combination: both libraries implement the five
operators and their `*Assign` forms for owned and borrowed operands, division and remainder
truncate toward zero with the remainder taking the dividend's sign, dividing by zero panics,
subtracting a larger `BigUint` or
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html) from
a smaller one panics, and neither unsigned type has `Neg`. A num expression built from these
operators means the same thing after a find-and-replace of the type names.

**The scalar operators.** Here the comparison section's asymmetry runs the other way: num
implements the operators against 32-, 64-, and 128-bit primitives in both operand orders,
while Malachite keeps its operators between its own types and asks for a lift,
`x + Natural::from(5u32)`, exact and cheap for any word-sized value. So num has mixed
operators but not mixed comparisons, and Malachite mixed comparisons but not mixed operators;
the lift is the price of the native route in this direction.

**The checked family.** Only two of these operations can actually fail on bignums: unsigned
subtraction, where num's `checked_sub` and Malachite's
[`CheckedSub`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CheckedSub.html)
both answer `None` below zero, and division, where num's `checked_div` turns the zero-divisor
panic into `None` and Malachite spells the same guard directly,
`(y != 0u32).then(|| x / y)`. The rest, `checked_add`, `checked_mul`, and `checked_sub` on
`BigInt`, cannot fail and always return `Some`; num implements them so that generic code
written against the checked traits accepts bignums, a role that belongs to
[the traits page](/mapping/num-traits/#operator-refinements), and Malachite leaves them off
the types. In the other direction,
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html) has
a saturating story:
[`SaturatingSub`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.SaturatingSub.html)
and `SaturatingSubMul` clamp at zero instead of panicking or optioning.

**`Sum` and `Product`.** Both libraries let iterators of values or references fold into a
total, and `iter.sum::<Natural>()` replaces `iter.sum::<BigUint>()` unchanged.

**The `Signed` arithmetic.** `abs` is
[`Abs`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Abs.html)
with its assign form, and the
[`UnsignedAbs`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.UnsignedAbs.html)
variants from [Conversion](#conversion) cross into
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html) at
the same time. `abs_sub`, the positive difference $$\max(a - b, 0)$$, is a comparison and a
subtraction spelled out, `if a > b { a - b } else { Integer::ZERO }`. `signum` is a match on
[`Sign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Sign.html):
`Greater`, `Equal`, and `Less` become `1`, `0`, and `-1`. The division-family variants,
`div_floor` and its relatives from num-integer along with num-traits' `Euclid`, have a section
of their own next.

## The division family {#the-division-family}

num-integer's `Integer` trait and num-traits' `Euclid` supply the division variants beyond the
truncating operators, named by convention: `div_floor`, `div_ceil`, `div_euclid`. Malachite
names the same operations by their rounding mode or by the quantity returned; the dictionary
first, the additions after.

| | num | Malachite |
| :---: | --- | --- |
| ✓ | `Integer::div_floor (&self, other: &Self) -> Self` | [`DivRound`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivRound.html) |
| ✓ | `Integer::mod_floor (&self, other: &Self) -> Self` | [`Mod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Mod.html) |
| ✓ | `Integer::div_mod_floor (&self, other: &Self) -> (Self, Self)` | [`DivMod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivMod.html) |
| ✓ | `Integer::div_rem (&self, other: &Self) -> (Self, Self)` | [`DivRem`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivRem.html) |
| ✓ | `Integer::div_ceil (&self, other: &Self) -> Self` | [`DivRound`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivRound.html) |
| ✓ | `Integer::is_multiple_of (&self, other: &Self) -> bool` | [`DivisibleBy`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivisibleBy.html) |
| ≈ | `Integer::next_multiple_of (&self, other: &Self) -> Self` | [`RoundToMultiple`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.RoundToMultiple.html) |
| ≈ | `Integer::prev_multiple_of (&self, other: &Self) -> Self` | [`RoundToMultiple`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.RoundToMultiple.html) |
| ✓ | `Euclid::div_euclid (&self, v: &Self) -> Self` | [`DivEuclidean`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivEuclidean.html) |
| ✓ | `Euclid::rem_euclid (&self, v: &Self) -> Self` | [`ModEuclidean`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModEuclidean.html) |
| ✓ | `Euclid::div_rem_euclid (&self, v: &Self) -> (Self, Self)` | [`DivModEuclidean`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivModEuclidean.html) |
| ≈ | `CheckedEuclid::checked_div_euclid`, `checked_rem_euclid` | [`DivEuclidean`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivEuclidean.html) |

**The floor, ceiling, and truncating divisions.** `div_floor` is `x.div_round(&y, Floor)`,
`div_ceil` is `x.div_round(&y, Ceiling)`, and num's doctests are Malachite's semantics
exactly, `(-1).div_floor(&2)` giving $$-1$$ on both sides. `mod_floor` is `x.mod_op(&y)`, the
remainder with the divisor's sign; the simultaneous pair `div_mod_floor` is
[`DivMod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivMod.html);
and `div_rem`, the truncating pair whose remainder takes the dividend's sign, keeps its exact
name and meaning in Malachite's
[`DivRem`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivRem.html).
On `BigUint` and
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html) all
of these coincide with the plain operators. Malachite completes the matrix with
[`CeilingDivMod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CeilingDivMod.html)
and `CeilingMod`, the ceiling *pair* that num does not offer.

**Divisibility and multiples.** `is_multiple_of`, and its deprecated alias `divides`, is
[`DivisibleBy`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivisibleBy.html):
`x.divisible_by(&y)`. The multiple-rounding pair is ≈ for a directional subtlety worth
quoting: num's `next_multiple_of` moves in the direction of the *divisor's* sign, its doctest
showing `(-23).next_multiple_of(&-8)` landing at $$-24$$, below its argument. Malachite's
[`RoundToMultiple`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.RoundToMultiple.html)
takes the direction as an explicit
[`RoundingMode`](https://docs.rs/malachite-base/latest/malachite_base/rounding_modes/enum.RoundingMode.html),
so `next_multiple_of` is `round_to_multiple` with `Ceiling` for a positive divisor and
`Floor` for a negative one, and `prev_multiple_of` the reverse.

**Euclidean division.** The quotient-remainder convention with the remainder always in
$$[0, |d|)$$: `div_euclid` maps directly to
[`DivEuclidean`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivEuclidean.html)'s
`div_euclidean`, with its assign form. `rem_euclid` is the
[`ModEuclidean`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModEuclidean.html)
companion: `x.mod_euclidean(&y)`, whose result is nonnegative and therefore comes back as a
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html), with
[`ModEuclideanAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModEuclideanAssign.html)
as its in-place form. The provided `div_rem_euclid`, which computes both at once, is the
simultaneous pair
[`DivModEuclidean`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivModEuclidean.html).
On the
unsigned types, Euclidean, floor, and truncating division are all the same thing, which is how
num implements `Euclid for BigUint`. The `CheckedEuclid` pair adds the zero-divisor guard,
answered as `checked_div` was in
[the previous section](#arithmetic-operators): test the divisor, then divide.

**Beyond the dictionary.** Malachite's division constellation continues past num's:
[`DivRound`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivRound.html)
accepts every `RoundingMode`, including `Nearest` with ties to even and the panicking `Exact`;
[`DivExact`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivExact.html)
is the fast path for quotients known to be exact;
[`EqMod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.EqMod.html)
tests $$x \equiv y \pmod m$$ without forming either remainder; and the power-of-2
specializations,
[`DivisibleByPowerOf2`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivisibleByPowerOf2.html),
[`ModPowerOf2`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPowerOf2.html),
[`RoundToMultipleOfPowerOf2`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.RoundToMultipleOfPowerOf2.html),
and
[`EqModPowerOf2`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.EqModPowerOf2.html),
do their work with shifts and masks. None of these have num counterparts.

## Bits and logic {#bits-and-logic}

| | num | Malachite |
| :---: | --- | --- |
| ✓ | `BigUint::bits (&self) -> u64` | [`SignificantBits`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.SignificantBits.html) |
| ✓ | `BigUint::bit (&self, bit: u64) -> bool` | [`BitAccess`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.BitAccess.html) |
| ✓ | `BigUint::set_bit (&mut self, bit: u64, value: bool)` | [`BitAccess`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.BitAccess.html) |
| ✓ | `BigUint::count_ones (&self) -> u64` | [`CountOnes`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.CountOnes.html) |
| ✓ | `BigUint::trailing_zeros (&self) -> Option<u64>` | [`TrailingZeros`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.TrailingZeros.html) |
| ✓ | `BigUint::trailing_ones (&self) -> u64` | [`BitScan`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.BitScan.html) |
| ✓ | `BigInt::bits (&self) -> u64` | [`SignificantBits`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.SignificantBits.html) |
| ✓ | `BigInt::bit (&self, bit: u64) -> bool` | [`BitAccess`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.BitAccess.html) |
| ✓ | `BigInt::set_bit (&mut self, bit: u64, value: bool)` | [`BitAccess`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.BitAccess.html) |
| ✓ | `BigInt::trailing_zeros (&self) -> Option<u64>` | [`TrailingZeros`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.TrailingZeros.html) |
| ✓ | `BitAnd`, `BitOr`, `BitXor`, `*Assign` (`BigUint`) | [`BitAnd`](https://doc.rust-lang.org/nightly/std/ops/trait.BitAnd.html), [`BitOr`](https://doc.rust-lang.org/nightly/std/ops/trait.BitOr.html), [`BitXor`](https://doc.rust-lang.org/nightly/std/ops/trait.BitXor.html) |
| ✓ | `BitAnd`, `BitOr`, `BitXor`, `*Assign` (`BigInt`) | [`BitAnd`](https://doc.rust-lang.org/nightly/std/ops/trait.BitAnd.html), [`BitOr`](https://doc.rust-lang.org/nightly/std/ops/trait.BitOr.html), [`BitXor`](https://doc.rust-lang.org/nightly/std/ops/trait.BitXor.html) |
| ✓ | `Not` (`BigInt`) | [`Not`](https://doc.rust-lang.org/nightly/std/ops/trait.Not.html) |
| ≈ | `Shl`, `Shr`, `*Assign` (amounts: `u8`..`usize`, `i8`..`isize`) | [`Shl`](https://doc.rust-lang.org/nightly/std/ops/trait.Shl.html), [`Shr`](https://doc.rust-lang.org/nightly/std/ops/trait.Shr.html) |

**The bit queries.** `bits` is
[`significant_bits`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.SignificantBits.html),
zero for zero and, on the signed types, the magnitude's bit length in both libraries. `bit`
and `set_bit` are
[`BitAccess`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.BitAccess.html)'s
`get_bit` and `assign_bit`, with `set_bit`, `clear_bit`, and `flip_bit` as finer-grained
spellings, and the semantics on negative values agree exactly, num reading and writing "using
the two's complement for negative numbers" as
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html)
does. `count_ones` is
[`CountOnes`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.CountOnes.html);
`trailing_zeros` matches down to the return type, `Option<u64>` with `None` for zero; and
`trailing_ones` is
[`BitScan`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.BitScan.html)'s
`index_of_next_false_bit(0)`, the first clear bit being one past the trailing ones.

**The bitwise operators.** Present in all operand combinations on both sides, with `BigInt`
and `Integer` agreeing on two's complement semantics for negatives, and `!x = -x - 1` through
`Not` on the signed types. One implementation exists only on the Malachite side: `Not` for
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html),
crossing into `Integer` for the result, since the complement of an unsigned value is negative.

**The shifts.** Both libraries shift by any primitive integer type, and `>>` on the signed
types is arithmetic, rounding toward negative infinity, in both. The ≈ is what a *negative*
signed amount means, and it is a real behavioral divergence: num panics, "attempt to shift
left with negative", while Malachite reverses direction, `x << -3` meaning `x >> 3`, a
deliberate convention used throughout the library. Ported code that never shifts by a negative
amount is unaffected; code that relied on the panic as a guard will silently compute instead.

**Beyond the dictionary.** Malachite's logic module keeps going:
[`BitScan`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.BitScan.html)
finds the next set or clear bit from any starting index, not just index zero;
[`BitBlockAccess`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.BitBlockAccess.html)
reads and writes whole bit ranges;
[`BitIterable`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.BitIterable.html)
iterates over bits and
[`BitConvertible`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.BitConvertible.html)
converts to and from bit vectors;
[`HammingDistance`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.HammingDistance.html)
counts differing bits on `Natural`, with
[`CheckedHammingDistance`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.CheckedHammingDistance.html)
answering `None` for `Integer`s of opposite sign; and `Integer` gets `checked_count_ones` and
`checked_count_zeros`, defined exactly when the two's complement expansion makes them finite.

## Powers and roots {#powers-and-roots}

| | num | Malachite |
| :---: | --- | --- |
| ✓ | `pow (&self, exponent: u32)`; `Pow` (exponents `u8`..`u128`, `usize`, `BigUint`) | [`Pow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Pow.html), [`PowAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.PowAssign.html) |
| ✓ | `BigUint::modpow (&self, exponent: &Self, modulus: &Self) -> Self` | [`ModPow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPow.html) |
| ≈ | `BigInt::modpow (&self, exponent: &Self, modulus: &Self) -> Self` | [`Mod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Mod.html), [`ModPow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPow.html) |
| ✓ | `BigUint::modinv (&self, modulus: &Self) -> Option<Self>` | [`ModInverse`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModInverse.html) |
| ≈ | `BigInt::modinv (&self, modulus: &Self) -> Option<Self>` | [`Mod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Mod.html), [`ModInverse`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModInverse.html) |
| ✓ | `Roots::sqrt`, `cbrt`, `nth_root (n: u32)` (`BigUint`) | [`FloorSqrt`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.FloorSqrt.html), [`FloorRoot`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.FloorRoot.html) |
| ✓ | `Roots::sqrt` (`BigInt`) | [`FloorSqrt`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.FloorSqrt.html) |
| ≈ | `Roots::cbrt`, `nth_root` (`BigInt`) | [`FloorRoot`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.FloorRoot.html), [`CeilingRoot`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CeilingRoot.html) |

**Powers.** num spreads exponentiation across an inherent `pow` taking `u32` and `Pow`
implementations taking every unsigned primitive and even a `BigUint`; Malachite's
[`Pow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Pow.html)
takes a `u64`, with
[`PowAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.PowAssign.html)
in place. The bignum-exponent implementations are wider in type only: any power a machine
could ever finish computing has an exponent that fits comfortably in a `u64`, so
`u64::try_from(&e)` loses nothing but the out-of-memory outcome.

**Modular exponentiation and inversion.** `BigUint::modpow` is
[`ModPow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPow.html)'s
`x.mod_pow(&e, &m)`, and `BigUint::modinv` is
[`ModInverse`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModInverse.html)'s
`x.mod_inverse(&m)`, `Option` for `Option`, `None` exactly when no inverse exists. The
`BigInt` versions are ≈ because Malachite's modular arithmetic lives on
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html)
residues: num documents that its `modpow` "rounds like `mod_floor`, not like the `%`
operator", putting results in $$[0, m)$$ for a positive modulus, so the mapping reduces first
with `x.mod_op(&m)`, whose floor semantics are the same, and exponentiates the reduced
residue; a negative num modulus, with results in $$(m, 0]$$, follows from the same reduction
by $$|m|$$ and a shift by $$m$$. num's panics, on a negative exponent and on a zero modulus,
have no occasion to arise once the operands are residues by construction.

**Roots.** All roots agree on the unsigned types: num-integer's `sqrt`, `cbrt`, and
`nth_root` are floor roots, `floor_sqrt` and `floor_root(n)`. On the signed types the even
cases agree, both libraries panicking on an even root of a negative number, but odd roots of
negative values diverge: num *truncates* toward zero, its own doctest sending
`(i32::MIN + 1).nth_root(31)` to $$-1$$, while `floor_root` floors, sending the same value to
$$-2$$. num's behavior on a negative argument is therefore Malachite's
[`CeilingRoot`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CeilingRoot.html):
`if x >= 0 { x.floor_root(n) } else { x.ceiling_root(n) }` reproduces `nth_root` exactly.

**Beyond the dictionary.** Every root comes in three rounding flavors and an assign form:
`FloorSqrt`, `CeilingSqrt`, and
[`CheckedSqrt`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CheckedSqrt.html),
which answers `Some` only for perfect squares, with the same triple for general roots; and
[`SqrtRem`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.SqrtRem.html)
and `RootRem` return the root together with the remainder in one pass.
[`Square`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Square.html)
is the dedicated squaring num spells as `&x * &x`. The power-of-2 family,
[`PowerOf2`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.PowerOf2.html),
`IsPowerOf2`, and `NextPowerOf2`, and the integer logarithms, `floor_log_base_2`,
`ceiling_log_base_2`, and their general-base and checked relatives, are further additions. On
the modular side, Malachite continues into the
[`ModPowerOf2`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPowerOf2.html)
tower, arithmetic modulo $$2^k$$ with its own `mod_power_of_2_pow` and
`mod_power_of_2_inverse`, where masks and shifts replace division throughout.

## GCD and parity {#gcd-and-parity}

| | num | Malachite |
| :---: | --- | --- |
| ✓ | `Integer::gcd (&self, other: &Self) -> Self` (`BigUint`) | [`Gcd`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Gcd.html), [`GcdAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.GcdAssign.html) |
| ✓ | `Integer::lcm (&self, other: &Self) -> Self` (`BigUint`) | [`Lcm`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Lcm.html), [`LcmAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.LcmAssign.html) |
| ≈ | `Integer::gcd` (`BigInt`) | [`UnsignedAbs`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.UnsignedAbs.html), [`Gcd`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Gcd.html) |
| ≈ | `Integer::lcm` (`BigInt`) | [`UnsignedAbs`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.UnsignedAbs.html), [`Lcm`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Lcm.html) |
| ≈ | `Integer::gcd_lcm (&self, other: &Self) -> (Self, Self)` | [`Gcd`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Gcd.html), [`Lcm`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Lcm.html) |
| ≈ | `Integer::extended_gcd`, `extended_gcd_lcm` (`BigInt`) | [`ExtendedGcd`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ExtendedGcd.html) |
| ✓ | `Integer::is_even`, `is_odd` (`BigUint`, `BigInt`) | [`Parity`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Parity.html) |

**GCD and LCM.** On the unsigned types the mapping is direct, `x.gcd(&y)` on both sides, with
`gcd(0, 0) = 0` and the assign forms in place. Malachite keeps
[`Gcd`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Gcd.html)
and `Lcm` on
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html)
only, on the argument that a GCD is a property of magnitudes; num's `BigInt` versions, which
also return nonnegative results, are the same values through
`a.unsigned_abs().gcd(b.unsigned_abs())`, lifted back with `Integer::from` when the signed
type is wanted. `gcd_lcm` returns both at once, and one GCD suffices for the pair:
`let g = (&a).gcd(&b);` then `let l = (&a / &g) * &b;`, with the zero pair handled first.

**The extended GCD.** Both libraries compute the Bézout triple, `gcd` with cofactors `x` and
`y` satisfying $$\gcd = ax + by$$, num returning its `ExtendedGcd` struct and Malachite a
tuple. The cofactors are signed by nature, even for unsigned inputs, and the two libraries
respond differently: Malachite's
[`ExtendedGcd`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ExtendedGcd.html)
is implemented for `Natural` and
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html)
alike, with the cofactors always `Integer`s, while num's `BigUint` instantiation inherits a
default implementation whose intermediate subtractions require signed values, so the usable
form is `BigInt`'s. Each library documents its own normalization of the cofactor pair, so
ported code should rely on the Bézout identity rather than on particular cofactor values.
`extended_gcd_lcm` composes the triple with the one-GCD `lcm` spelling above.

**Parity.** `is_even` and `is_odd` are
[`Parity`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Parity.html)'s
`x.even()` and `x.odd()`, on both types.

**Beyond the dictionary.** Malachite adds
[`CoprimeWith`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CoprimeWith.html),
a dedicated coprimality test that can stop earlier than a full GCD, and the number-theoretic
symbols, [`LegendreSymbol`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.LegendreSymbol.html),
`JacobiSymbol`, and `KroneckerSymbol`, on both types. In the other direction, num-integer
defines an `Average` trait, but num-bigint does not implement it for the bignum types, so it
reappears with the generic machinery on
[the traits page](/mapping/num-traits/#num-integer); Malachite has no averaging functions
either, and a
`(&a + &b) >> 1` computes the floor average exactly, bignums having no overflow to dodge.

## Random generation {#random-generation}

Under num's `rand` feature, the `RandBigInt` trait extends any `rand::Rng` with bignum
generators, and adapter types plug the bignums into rand's distribution machinery. Malachite's
`random` feature is a self-contained framework instead: generators take a
[`Seed`](https://docs.rs/malachite-base/latest/malachite_base/random/struct.Seed.html) and
return infinite iterators, and the per-call `get_random_*` functions draw from a
`RandomPrimitiveInts` source built from one. The same uniform distributions are on both sides;
what differs is who owns the randomness.

| | num | Malachite |
| :---: | --- | --- |
| ✓ | `RandBigInt::gen_biguint (&mut self, bit_size: u64) -> BigUint` | [`get_random_natural_with_up_to_bits`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/random/fn.get_random_natural_with_up_to_bits.html) |
| ✓ | `RandBigInt::gen_bigint (&mut self, bit_size: u64) -> BigInt` | [`get_random_natural_with_up_to_bits`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/random/fn.get_random_natural_with_up_to_bits.html), [`from_sign_and_abs`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html#method.from_sign_and_abs) |
| ✓ | `RandBigInt::gen_biguint_below (&mut self, bound: &BigUint) -> BigUint` | [`get_random_natural_less_than`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/random/fn.get_random_natural_less_than.html) |
| ✓ | `RandBigInt::gen_biguint_range (&mut self, lbound: &BigUint, ubound: &BigUint) -> BigUint` | [`uniform_random_natural_range`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/random/fn.uniform_random_natural_range.html) |
| ✓ | `RandBigInt::gen_bigint_range (&mut self, lbound: &BigInt, ubound: &BigInt) -> BigInt` | [`get_uniform_random_integer_from_range`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/random/fn.get_uniform_random_integer_from_range.html) |
| — | `RandomBits` (a rand `Distribution`) | |
| — | `UniformBigUint`, `UniformBigInt` (rand `SampleUniform` backends) | |

**The generators.** `gen_biguint` draws uniformly among values of up to `bit_size` bits, which
is `get_random_natural_with_up_to_bits`; the exactly-`n`-bits refinement,
[`get_random_natural_with_bits`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/random/fn.get_random_natural_with_bits.html),
is there when the top bit must be set. `gen_bigint` attaches a random sign to a random
magnitude, which is the same draw wrapped in
[`from_sign_and_abs`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html#method.from_sign_and_abs)
with a coin flip. The bounded draws map directly: `gen_biguint_below` is
`get_random_natural_less_than`, and the half-open ranges are the `uniform_random_*_range`
generators, with
[`get_uniform_random_integer_from_range`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/random/fn.get_uniform_random_integer_from_range.html)
as the per-call form on
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html); on
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html) the
per-call range draw is `low + get_random_natural_less_than(&mut source, &(high - low))`. Empty
ranges panic on both sides.

**The adapters.** `RandomBits`, `UniformBigUint`, and `UniformBigInt` exist to plug the bignum
types into rand's `Distribution` and `gen_range` machinery. Malachite's framework has no
separate adapter layer: a seeded iterator like `uniform_random_natural_range` *is* the
distribution object, bound to its source, so these rows need no counterpart.

**Beyond the dictionary.** The generators described on
[the other pages](/mapping/flint-rationals/#random-number-generation) are all here too:
mean-parameterized streams like
[`random_naturals`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/random/fn.random_naturals.html),
the striped families whose long runs of ones and zeros probe corner cases, sign-restricted
`Integer` streams, and ranges unbounded above through `random_natural_range_to_infinity`.

## Strings, formatting, and serde {#strings-formatting-and-serde}

| | num | Malachite |
| :---: | --- | --- |
| ✓ | `Display`, `Debug` (`BigUint`, `BigInt`) | [`Display`](https://doc.rust-lang.org/nightly/std/fmt/trait.Display.html), [`Debug`](https://doc.rust-lang.org/nightly/std/fmt/trait.Debug.html) |
| ✓ | `Binary`, `Octal`, `LowerHex`, `UpperHex` | [`Binary`](https://doc.rust-lang.org/nightly/std/fmt/trait.Binary.html), [`Octal`](https://doc.rust-lang.org/nightly/std/fmt/trait.Octal.html), [`LowerHex`](https://doc.rust-lang.org/nightly/std/fmt/trait.LowerHex.html), [`UpperHex`](https://doc.rust-lang.org/nightly/std/fmt/trait.UpperHex.html) |
| ✓ | `to_str_radix (&self, radix: u32) -> String` | [`ToStringBase`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.ToStringBase.html) |
| ≈ | `FromStr` (error: `ParseBigIntError`) | [`FromStr`](https://doc.rust-lang.org/nightly/std/str/trait.FromStr.html) |
| ✓ | `Num::from_str_radix (s: &str, radix: u32) -> Result` | [`FromStringBase`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.FromStringBase.html) |
| ✓ | `parse_bytes (buf: &[u8], radix: u32) -> Option` | [`FromStringBase`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.FromStringBase.html) |
| ≈ | `Serialize`, `Deserialize` | [`Serialize`](https://docs.rs/serde/latest/serde/trait.Serialize.html), [`Deserialize`](https://docs.rs/serde/latest/serde/trait.Deserialize.html) |

**Formatting.** `Display` prints decimal, `Debug` prints exactly what `Display` prints, in
both libraries, and the radix formatting traits are present on both sides with the `#`
alternate prefixes. Beyond the formatter traits, `to_str_radix` handles bases 2 through 36 in
lowercase;
[`ToStringBase`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.ToStringBase.html)
covers the same range with `to_string_base`, and adds `to_string_base_upper`, upper-case
digits in any base rather than only the hexadecimal 16. Negative values print with a leading
`-` everywhere.

**Parsing.** `FromStr` parses base 10 on both sides, ≈ only in its error type: num returns a
`ParseBigIntError` describing the failure, Malachite's base-10 parse signals it with a unit
error. `from_str_radix` and its byte-slice twin `parse_bytes` are
[`FromStringBase`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.FromStringBase.html)'s
`from_string_base(base, s)`, `Option`-returning as `parse_bytes` is, accepting the same
case-insensitive digits.

**serde.** Both libraries serialize stably and portably, and in mutually incompatible forms.
num-bigint writes a `BigUint` as a sequence of base-$$2^{32}$$ digits, a format its source
pledges to keep independent of the internal representation; Malachite writes a hexadecimal
string, `"0x1f4"`, the same on every platform and `Limb` width. In a JSON snapshot the same
value is an array of numbers under num and a string under Malachite, so serialized data does
not migrate between the libraries directly; it crosses through any of the explicit forms from
[Conversion](#conversion), bytes, digits, or strings, re-serialized on the other side.

That closes the type-level mapping. What remains of the num-bigint experience is its generic
face, the num-traits and num-integer traits themselves, which
[the traits page](/mapping/num-traits/) takes up for code written against `T: Num` rather
than a concrete bignum.
