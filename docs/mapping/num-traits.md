---
layout: default
title: "Malachite for num Users: Traits"
permalink: /mapping/num-traits/
theme: jekyll-theme-slate
---

# Malachite for num Users: Traits

This page maps num's generic vocabulary, the
[num-traits](https://docs.rs/num-traits/latest/num_traits/) and
[num-integer](https://docs.rs/num-integer/latest/num_integer/) crates, with
[num-iter](https://docs.rs/num-iter/latest/num_iter/)'s range functions alongside, onto
Malachite's generic vocabulary, which lives in
[malachite-base](https://docs.rs/malachite-base/latest/malachite_base/). It covers num-traits
0.2.19, num-integer 0.1.46, and num-iter 0.1.46, and completes the num family begun by
[the integers](/mapping/num-integers/) and [rationals](/mapping/num-rationals/) pages, which
map the concrete types; this page is for code written against `T: Num` rather than against a
`BigUint`. The [mapping index](/mapping/) lists all the pages.

The two crates being mapped here are trait collections, so the rows of this page pair traits
with traits, and the mapping is between two generic *systems*: both num and Malachite implement
their vocabularies for the primitive integer and floating-point types as well as for their
bignum types, so in both worlds one generic function can span `u32` and a bignum. What a
porting reader translates is bounds and imports, and occasionally a method shape.

## Conventions {#conventions}

### Granularity

The deepest difference between the two systems is how much one trait means. num's vocabulary
is anchored by umbrella traits: `Num` bundles equality, the identities, the operators, and
string parsing; `PrimInt` and `Float` each carry dozens of methods. Malachite's vocabulary is
fine-grained: nearly every operation is its own trait, `Gcd`, `CheckedSub`,
`FloorSqrt`, and a bound is assembled from exactly the operations used. Porting a
`T: Num` bound therefore usually means writing several Malachite bounds, and porting a
Malachite bound into num terms usually means asking for more than is needed. The tables below
map each num trait to the Malachite traits that cover its members.

### Names collide, in both directions

Many correspondences are name-for-name: num's `CheckedAdd` and Malachite's
[`CheckedAdd`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CheckedAdd.html)
are different traits from different crates with the same job, and a `use` line decides which
is in scope. Two collisions deserve care rather than comfort. `num_traits::Float` is a trait
describing primitive floating-point types, while Malachite's
[`Float`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html) is
an arbitrary-precision type; the two have nothing to do with each other, and the float
section below sorts out what maps where. And num's `Integer` is a trait, from num-integer,
while Malachite's
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html) is
its signed bignum type; this page writes `num_integer::Integer` when it means the trait.

### Method shapes

num's operation traits generally take their arguments by reference and return owned results,
`fn checked_add(&self, v: &Self) -> Option<Self>`; Malachite's generally take `self` by value,
with `Output` associated types, reference implementations on `&T` where cloning matters, and
separate `*Assign` traits for in-place forms. The information carried is the same, and the
compiler polices the difference; where a shape difference changes what code can express, the
notes say so.

### What implements what

Each system implements its own vocabulary: num-traits for the primitives and, via num-bigint
and num-rational, for the bignum types; malachite-base for the primitives and, via
malachite-nz and malachite-q, for
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html),
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html),
[`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html),
and [`Float`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html).
Neither library implements the other's traits, so generic code chooses a vocabulary; the
concrete-type pages of this family are the dictionary for everything a bound reaches through
these traits.

### Categories

Each item falls into one of four categories:

| | meaning |
| :---: | --- |
| ✓ | A Malachite trait or function does the same thing. |
| ≈ | A Malachite trait or function serves the same purpose, but its specification differs. The notes say how. |
| — | No counterpart is needed; the notes say why. |
| ✗ | Malachite does not fully support this yet, but will in a future version. |

## Identities and umbrella traits {#identities-and-umbrella-traits}

| | num | Malachite |
| :---: | --- | --- |
| ✓ | `Zero`, `ConstZero` (`zero()`, `is_zero`, `set_zero`, `ZERO`) | [`Zero`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.Zero.html) |
| ✓ | `One`, `ConstOne` (`one()`, `is_one`, `set_one`, `ONE`) | [`One`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.One.html) |
| — | free functions `zero()`, `one()` | |
| ≈ | `Num` (`PartialEq + Zero + One + NumOps`, `from_str_radix`) | [`Zero`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.Zero.html), [`One`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.One.html), [`FromStringBase`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.FromStringBase.html) |
| — | `NumOps`, `NumRef`, `RefNum` | |
| — | `NumAssignOps`, `NumAssign`, `NumAssignRef` | |

**The identities.** The two systems agree on what and differ on how. num's `Zero` is a trait
with methods, `zero()` constructing the identity, `is_zero` testing it, `set_zero` assigning
it, refined since 0.2.19 by `ConstZero`'s constant; Malachite's
[`Zero`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.Zero.html)
has been a single associated constant from the start. The translations are one-liners:
`T::zero()` becomes `T::ZERO`, `x.is_zero()` becomes `x == T::ZERO` under a `PartialEq`
bound, `x.set_zero()` becomes `x = T::ZERO`, and the free functions `zero()` and `one()` are
the constants themselves. The constant family keeps going on the Malachite side:
[`Two`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.Two.html),
`NegativeOne`, and, for the types that have them, `OneHalf`, `Infinity`, and `NaN`, all
usable as generic bounds in the same way.

**`Num`.** The central umbrella, and a ≈ that unpacks rather than renames: a `T: Num` bound
says `PartialEq + Zero + One + NumOps` plus `from_str_radix`, so its Malachite translation
spells those out, `T: PartialEq + Zero + One + Add<Output = T> + Sub<Output = T>`
`+ Mul<Output = T> + Div<Output = T> + Rem<Output = T>`, with
[`FromStringBase`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.FromStringBase.html)
covering the parsing member for the primitives and the bignum integers; the rational
instantiation of that member is
[the base-string gap](/mapping/num-rationals/#strings-and-serde) noted on the rationals page.
The bound is longer, which is the granularity trade described under
[Conventions](#conventions): nothing is bundled, and nothing unused comes along.

**The operator bundles.** `NumOps` is shorthand for the five standard operator bounds,
`NumAssignOps` for their assign forms, and `NumRef`, `RefNum`, and `NumAssignRef` extend them
to borrowed operands. The standard library's operator traits are the shared vocabulary here,
and Malachite's types implement them in the same owned, borrowed, and assigning combinations,
so every pattern these shorthands describe holds; only the shorthand names themselves have no
counterparts, and a port writes the constituent bounds directly.

## Signs and bounds {#signs-and-bounds}

| | num | Malachite |
| :---: | --- | --- |
| ≈ | `Signed` (`abs`, `abs_sub`, `signum`, `is_positive`, `is_negative`) | [`Abs`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Abs.html), [`Sign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Sign.html), [`PrimitiveSigned`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/signeds/trait.PrimitiveSigned.html) |
| ≈ | `Unsigned` | [`PrimitiveUnsigned`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/unsigneds/trait.PrimitiveUnsigned.html) |
| — | free functions `abs`, `abs_sub`, `signum` | |
| ≈ | `Bounded`, `LowerBounded`, `UpperBounded` (`min_value`, `max_value`) | [`PrimitiveInt`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/integers/trait.PrimitiveInt.html), [`PrimitiveFloat`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/floats/trait.PrimitiveFloat.html) |
| — | free functions `clamp`, `clamp_min`, `clamp_max` | |

**`Signed` and `Unsigned`.** Each plays two roles. As method carriers, the `Signed` methods
were mapped with the concrete types, `abs` to
[`Abs`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Abs.html),
`signum` to
[`Sign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Sign.html),
the predicates to comparisons against zero, and `abs_sub` to a spelled-out positive
difference, on [the integers](/mapping/num-integers/#arithmetic-operators) and
[rationals](/mapping/num-rationals/#arithmetic) pages; the free functions are call-syntax
sugar over the same methods. Two relatives of `abs_sub` are worth knowing. Its formula, a
subtraction stopped at zero, is exactly
[`SaturatingSub`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.SaturatingSub.html)
on the unsigned types, where zero is the saturation point: `x.saturating_sub(y)` on a
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html) is
the positive difference with no comparison written. And
[`AbsDiff`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.AbsDiff.html),
`x.abs_diff(y)` computing $$|x - y|$$, is the different operation `abs_sub` is sometimes
mistaken for, and tells the two apart. As *markers*, the
two systems draw the line differently: num's
`Signed` spans the signed primitives, `BigInt`, and `BigRational` in one bound, while
Malachite's
[`PrimitiveSigned`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/signeds/trait.PrimitiveSigned.html)
and `PrimitiveUnsigned` are primitive-only umbrellas, richer in members but not spanning
the bignums. Code generic over signed types of both kinds assembles its bound from the
operations it uses, which is [the granularity trade](#conventions) again.

**The bounded traits.** Both systems attach bounds only where bounds exist: num implements
`Bounded` for the primitives and leaves the unbounded bignums out, and Malachite's bignums
likewise have no extremes to name. For generic code over primitives, `min_value()` and
`max_value()` become the
[`MIN` and `MAX` associated constants](https://docs.rs/malachite-base/latest/malachite_base/num/basic/integers/trait.PrimitiveInt.html)
that `PrimitiveInt` and `PrimitiveFloat` carry. The `LowerBounded`/`UpperBounded` split,
which exists for one-sidedly bounded types, reaches the same constants and has no separate
counterpart.

**The clamp functions.** `clamp`, `clamp_min`, and `clamp_max` predate and parallel the
standard library's
[`Ord::clamp`](https://doc.rust-lang.org/nightly/std/cmp/trait.Ord.html#method.clamp), which
covers every totally ordered type, Malachite's exact types included: `x.clamp(lo, hi)`. For
partially ordered types, num's versions accept floats; the same effect is two comparisons
wherever `Ord` is unavailable.

## Casts and bytes {#casts-and-bytes}

This is where the two systems differ most in philosophy. num has one conversion vocabulary,
`Option`-valued: a cast either produces the exact value or `None`, with the float cases
rounding. Malachite has seven, and the trait names the semantics:
[`TryFrom`](https://doc.rust-lang.org/nightly/std/convert/trait.TryFrom.html) is exact with
an `Err`,
[`ExactFrom`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.ExactFrom.html)
is exact with a panic,
[`WrappingFrom`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.WrappingFrom.html)
reduces modulo $$2^w$$,
[`SaturatingFrom`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.SaturatingFrom.html)
clamps to the target's range,
[`OverflowingFrom`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.OverflowingFrom.html)
wraps and reports whether it did,
[`ConvertibleFrom`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.ConvertibleFrom.html)
answers whether an exact conversion exists, and
[`RoundingFrom`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.RoundingFrom.html)
rounds in an explicit mode and reports the direction. A num call site maps to whichever names
what it meant.

| | num | Malachite |
| :---: | --- | --- |
| ≈ | `ToPrimitive` (`to_u8`..`to_u128`, `to_i*`, `to_f32`, `to_f64`) | [`TryFrom`](https://doc.rust-lang.org/nightly/std/convert/trait.TryFrom.html), [`RoundingFrom`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.RoundingFrom.html) |
| ≈ | `FromPrimitive` (`from_u8`.., `from_f32`, `from_f64`) | [`TryFrom`](https://doc.rust-lang.org/nightly/std/convert/trait.TryFrom.html), [`RoundingFrom`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.RoundingFrom.html) |
| ≈ | `NumCast`; free function `cast` | [`ExactFrom`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.ExactFrom.html), [`TryFrom`](https://doc.rust-lang.org/nightly/std/convert/trait.TryFrom.html) |
| ≈ | `AsPrimitive` (`as_`) | [`WrappingFrom`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.WrappingFrom.html) |
| — | `ToBytes`, `FromBytes`, `NumBytes` | |

**`ToPrimitive` and `FromPrimitive`.** The integer members are exact-or-`None`, num's docs
promising `None` "if the value cannot be represented", which is `TryFrom` with an `Err`, or
`ConvertibleFrom` when only the answer is wanted; the float members round, which is
`RoundingFrom` with the mode written down. The concrete behaviors on bignums, truncation
toward zero from floats and the one-ulp subtleties toward them, were mapped on
[the integers page](/mapping/num-integers/#conversion); here the point is the bound itself,
and a `T: ToPrimitive` generic becomes a bound on the specific conversions used.

**`NumCast` and `cast`.** An any-to-any conversion that routes through `ToPrimitive`, so
`cast::<T, U>(n)` is the same exact-or-`None` contract with type inference picking the
route. Malachite implements its conversion traits pairwise, so the translation is direct:
`U::try_from(n)` fallibly, or `U::exact_from(n)` when the caller would have unwrapped
anyway, panicking with intent rather than unwrapping an `Option`.

**`AsPrimitive`.** The generic spelling of Rust's `as` operator, with `as`'s own semantics:
wrapping between integers, saturating from floats since Rust 1.45, and its documentation
retains an example marked "UB" from the era before that. The integer-to-integer case is
exactly
[`WrappingFrom`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.WrappingFrom.html);
the float-involved cases are `RoundingFrom` and `SaturatingFrom` with the semantics named
instead of inherited from the operator.

**The byte traits.** `ToBytes` and `FromBytes` generalize the primitives' `to_le_bytes`
family behind a `Bytes` associated type, with `NumBytes` bounding it; num implements them for
the primitives. Malachite has no byte traits: concrete code reaches the standard inherent
methods on primitives, and the bignum byte conversions live with
[the digit functions](/mapping/num-integers/#conversion) of the integers page. Code generic
over byte representations keeps num's vocabulary or passes byte slices explicitly.

## Operator refinements {#operator-refinements}

The checked, overflowing, saturating, and wrapping families are where the two vocabularies
come closest to being one: most rows are the same trait name in two crates.

| | num | Malachite |
| :---: | --- | --- |
| ✓ | `CheckedAdd`, `CheckedSub`, `CheckedMul`, `CheckedDiv`, `CheckedRem`, `CheckedNeg` | [`CheckedAdd`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CheckedAdd.html) and kin |
| ≈ | `CheckedShl`, `CheckedShr` | [`ArithmeticCheckedShl`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ArithmeticCheckedShl.html), [`ArithmeticCheckedShr`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ArithmeticCheckedShr.html) |
| ✓ | `OverflowingAdd`, `OverflowingSub`, `OverflowingMul` | [`OverflowingAdd`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.OverflowingAdd.html) and kin |
| ✓ | `SaturatingAdd`, `SaturatingSub`, `SaturatingMul` | [`SaturatingAdd`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.SaturatingAdd.html) and kin |
| — | `Saturating` (legacy two-method umbrella) | |
| ✓ | `WrappingAdd`, `WrappingSub`, `WrappingMul`, `WrappingNeg` | [`WrappingAdd`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.WrappingAdd.html) and kin |
| ≈ | `WrappingShl`, `WrappingShr` | [`Shl`](https://doc.rust-lang.org/nightly/std/ops/trait.Shl.html), [`Shr`](https://doc.rust-lang.org/nightly/std/ops/trait.Shr.html) |
| ≈ | `Euclid`, `CheckedEuclid` | [`DivEuclidean`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivEuclidean.html) |
| ✓ | `Inv` | [`Reciprocal`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Reciprocal.html) |
| ✓ | `MulAdd` | [`AddMul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.AddMul.html) |
| ≈ | `MulAddAssign` | [`AddMulAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.AddMulAssign.html) |
| ✓ | `Pow` | [`Pow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Pow.html) |
| — | free functions `pow`, `checked_pow` | |

**The name-for-name families.** Checked, overflowing, saturating, and wrapping arithmetic
translate by changing the `use` line, with the
[method-shape difference](#conventions) from Conventions the only adjustment: num's forms
take references, Malachite's take values with `*Assign` companions. Malachite's families also
include `CheckedAbs`, `OverflowingNeg`, `SaturatingNeg`, and their relatives, extending the
grid beyond the common cases. The old `Saturating` umbrella, two methods from
before the granular traits existed, maps as its members do.

**The shifts.** A real semantic split hides under similar names, worth slowing down for.
num's shift refinements govern the *count*: `checked_shl` answers `None` exactly when the
count reaches the type's width, its doctest sending `checked_shl(&x, 16)` on a `u16` to
`None`, and `wrapping_shl` reduces the count modulo the width, sending 16 to a shift by zero.
Malachite's
[`ArithmeticCheckedShl`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ArithmeticCheckedShl.html)
governs the *value*: `None` when the shifted value cannot be represented, whatever the count.
The count-based behaviors are one-liners when ported exactly,
`(n < W).then(|| x << n)` and `x << (n % W)`, but a caller reaching for `checked_shl` often
wants the value-based question, and should choose deliberately.

**`Euclid` and `Inv`.** `div_euclid` is
[`DivEuclidean`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivEuclidean.html),
on the primitives as on
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html);
the remainder half belongs to the `ModEuclidean` companion
[planned on the integers page](/mapping/num-integers/#the-division-family), with the
mod-by-absolute-value spelling in the meantime, and `CheckedEuclid` adds the zero-divisor
guard. `Inv` is
[`Reciprocal`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Reciprocal.html),
on the primitive floats here and on
[`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html)
[on the rationals page](/mapping/num-rationals/#arithmetic).

**`MulAdd` and the argument order.** Both libraries have the ternary multiply-and-add; the
receivers differ, and the dictionary matters more here than anywhere on this page:
`x.mul_add(y, z)` computes $$xy + z$$, while `x.add_mul(y, z)` computes $$x + yz$$, so the
translation reorders, `x.mul_add(y, z)` becoming `z.add_mul(x, y)`, an exact match. The
assign forms do not reorder into each other: `mul_add_assign` mutates a multiplicand,
$$r \gets rx + c$$, the Horner step, while
[`add_mul_assign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.AddMulAssign.html)
mutates an accumulator, $$a \gets a + yz$$, the accumulation step; the Horner shape spells
`r = c.add_mul(r, &x)` by value, and
[`SubMul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.SubMul.html)
completes the family with $$x - yz$$. On the primitive floats, num's `mul_add` is the fused,
singly rounded operation; Malachite's fused `Float` arithmetic is
[a gap recorded on the MPFR page](/mapping/mpfr-floats/#arithmetic-functions).

**`Pow` and the free functions.** [`Pow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Pow.html)
keeps its name and its role, with the concrete exponent types mapped on the type pages. The
free `pow(base, exp)` and `checked_pow` are call-syntax over the trait, whose Malachite
spellings are the `Pow` and
[`CheckedPow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CheckedPow.html)
methods themselves.

## Floats {#floats}

The [name collision flagged under Conventions](#conventions) resolves here. num's float
traits describe the *primitive* floating-point types, `f32` and `f64`; Malachite's
[`Float`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html) is
an arbitrary-precision type with its own page-sized dictionary,
[the MPFR page](/mapping/mpfr-floats/). A port of `T: Float` code therefore chooses a
destination first: staying with primitive floats, the subject of these rows, or moving to
arbitrary precision, at which point the MPFR page is the map.

| | num | Malachite |
| :---: | --- | --- |
| ≈ | `FloatCore` | [`PrimitiveFloat`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/floats/trait.PrimitiveFloat.html) |
| ≈ | `Float` | [`PrimitiveFloat`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/floats/trait.PrimitiveFloat.html) |
| — | `FloatConst` | |
| — | `TotalOrder` | |
| — | `Real` | |

**`FloatCore` and `Float`.** num splits the primitive-float surface into a `no_std` core,
classification, rounding, and sign manipulation, and a `std`-dependent full trait that adds
the mathematical library, `sqrt` through `atan2`.
[`PrimitiveFloat`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/floats/trait.PrimitiveFloat.html)
covers the first role and goes deeper into the representation: classification and `to_bits`
alongside constants like `MANTISSA_WIDTH`, `MIN_POSITIVE_SUBNORMAL`, and
`LARGEST_ORDERED_REPRESENTATION` that num does not name, plus the integer-logarithm and
rounding supertraits. It does not carry the transcendental functions: generic code needing
`T: Float` for `sin` and `exp` keeps num's trait or calls the standard inherent methods
concretely, and code that wants those functions *correctly rounded* has outgrown `f64`,
which is what the
[`Float` type](/mapping/mpfr-floats/#transcendental-functions) is for.

**The rest.** `FloatConst` exists to make the standard `consts` modules generic; Malachite
has no counterpart trait, and its constant story at arbitrary precision, three dozen
constants to any precision, is
[on the MPFR page](/mapping/mpfr-floats/#transcendental-functions). `TotalOrder` predates the
standard library's
[`total_cmp`](https://doc.rust-lang.org/nightly/std/primitive.f64.html#method.total_cmp),
which now covers it; the arbitrary-precision total order is
[`ComparableFloat`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.ComparableFloat.html)'s.
`Real` abstracts `Float` for real-valued types generally; nothing binds to it here.

## Primitive integers {#primitive-integers}

| | num | Malachite |
| :---: | --- | --- |
| ≈ | `PrimInt` | [`PrimitiveInt`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/integers/trait.PrimitiveInt.html), [`PrimitiveUnsigned`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/unsigneds/trait.PrimitiveUnsigned.html), [`PrimitiveSigned`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/signeds/trait.PrimitiveSigned.html) |

**The umbrella meets the umbrella.** This is the one place both systems agree that a large
bound is the right shape for primitive integers, and the two match in kind. num's `PrimInt`
gathers the operators, shifts, casts, and the bit-counting and byte-order methods;
Malachite's
[`PrimitiveInt`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/integers/trait.PrimitiveInt.html)
gathers over a hundred supertraits, from the operator refinements of this page through
`ExtendedGcd`, `BinomialCoefficient`, and the Kronecker symbols, with `PrimitiveUnsigned`
and `PrimitiveSigned` refining by signedness. Shared members map name-for-name through the
fine-grained traits, `count_ones` to
[`CountOnes`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.CountOnes.html),
`leading_zeros` and `trailing_zeros` to `LeadingZeros` and `TrailingZeros`, `rotate_left` to
[`RotateLeft`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.RotateLeft.html),
`pow` to `Pow`, `from_str_radix` to `FromStringBase`; the byte-order members, `swap_bytes`
and the `to_be`/`to_le` family, have no trait spellings and stay with the standard inherent
methods.

## num-integer {#num-integer}

| | num | Malachite |
| :---: | --- | --- |
| ≈ | `num_integer::Integer` (as a bound) | [`DivMod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivMod.html), [`Gcd`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Gcd.html), [`Parity`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Parity.html), and kin |
| ≈ | `Roots` (as a bound) | [`FloorSqrt`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.FloorSqrt.html), [`FloorRoot`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.FloorRoot.html) |
| ✗ | `Average` (`average_floor`, `average_ceil`; free functions) | |
| — | `ExtendedGcd` (the struct) | |
| — | free functions `div_rem` through `gcd_lcm`, `sqrt`, `cbrt`, `nth_root` | |
| ✓ | `binomial (n, k)` | [`BinomialCoefficient`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.BinomialCoefficient.html) |
| ≈ | `multinomial (k: &[T])` | [`BinomialCoefficient`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.BinomialCoefficient.html) |
| — | `IterBinomial` | |

**The traits as bounds.** `num_integer::Integer`'s *methods* were mapped in detail on
[the integers page](/mapping/num-integers/#the-division-family), and `Roots`'s with
[the powers](/mapping/num-integers/#powers-and-roots), truncation subtlety included. As
*bounds*, both follow the granularity rule of this page: a function generic over
`T: num_integer::Integer` asks Malachite for the operations it actually uses, `DivMod` here,
`Gcd` and `Parity` there, and nothing more. The `ExtendedGcd` struct is num's packaging for
the Bézout triple that Malachite returns as a tuple, with the
[normalization advice](/mapping/num-integers/#gcd-and-parity) of the integers page attached;
the free functions are call syntax over the trait methods.

**`Average`.** The committed gap of this page: averaging functions,
$$\lfloor (a+b)/2 \rfloor$$ and its ceiling, overflow-proof, are planned for Malachite. Until
they land, the bignums have no overflow to dodge, `(&a + &b) >> 1` for the floor and
`(&a + &b + Natural::ONE) >> 1` for the ceiling, and the primitives use the classic
carry-free spellings, `(a & b) + ((a ^ b) >> 1)` for the floor and
`(a | b) - ((a ^ b) >> 1)` for the ceiling.

**The combinatorial functions.** `binomial(n, k)` is
[`BinomialCoefficient`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.BinomialCoefficient.html)'s
`T::binomial_coefficient(n, k)`, on the primitives and the bignums alike, with
`CheckedBinomialCoefficient` for the primitive cases that overflow. `multinomial` is a fold
of binomials on both sides of the boundary,
$$\binom{k_1 + \cdots + k_m}{k_m} \cdots \binom{k_1 + k_2}{k_2}$$, spelled with a running sum
and `binomial_coefficient`; `IterBinomial`, which walks a row of Pascal's triangle, is the
exact recurrence `b = b * (n - k) / (k + 1)` in an iterator's clothing, three lines with
[`DivExact`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivExact.html)
doing the division.

## num-iter {#num-iter}

| | num | Malachite |
| :---: | --- | --- |
| ✓ | `range (start, stop)` | [`exhaustive_natural_range`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/exhaustive/fn.exhaustive_natural_range.html), [`primitive_int_increasing_range`](https://docs.rs/malachite-base/latest/malachite_base/num/exhaustive/fn.primitive_int_increasing_range.html) |
| ✓ | `range_inclusive (start, stop)` | [`exhaustive_natural_inclusive_range`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/exhaustive/fn.exhaustive_natural_inclusive_range.html), [`primitive_int_increasing_inclusive_range`](https://docs.rs/malachite-base/latest/malachite_base/num/exhaustive/fn.primitive_int_increasing_inclusive_range.html) |
| ≈ | `range_from (start)` | [`successors`](https://doc.rust-lang.org/nightly/std/iter/fn.successors.html) |
| ≈ | `range_step`, `range_step_inclusive`, `range_step_from` | [`successors`](https://doc.rust-lang.org/nightly/std/iter/fn.successors.html) |

**The ranges.** num-iter exists because `a..b` requires the unstable `Step` machinery, which
bignum types cannot implement on stable Rust; `num_iter::range(a, b)` is the working spelling
of a `BigUint` range, and
[`exhaustive_natural_range`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/exhaustive/fn.exhaustive_natural_range.html)
is the same iterator over
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html)s,
half-open like num's, with the `_inclusive` pair matching and
`primitive_int_increasing_range` covering generic primitive code. The unbounded and stepped
variants compose with the standard library:
`successors(Some(start), |x| Some(x + Natural::ONE))` climbs without bound, the step versions
add a stride instead, and on primitives `(a..b).step_by(n)` needs no help. These sit inside a
larger exhaustive-generation module, increasing, decreasing, and
coverage-oriented orders over ranges and whole types, built for the same testing purposes as
the [random generators](/mapping/num-integers/#random-generation) of the integers page.

That completes the num family: the concrete types on
[the integers](/mapping/num-integers/) and [rationals](/mapping/num-rationals/) pages, and
the generic vocabulary here.
