---
layout: default
title: "Malachite for GMP Users"
permalink: /gmp-mapping/
theme: jekyll-theme-slate
---

# Malachite for GMP Users

This page maps the functions of [GMP](https://gmplib.org/manual/) onto their Malachite
counterparts, section by section, following the organization of the GMP manual. It is meant to be
read in two directions: if you are porting C code, look up the `mpz_` function you are using and
find what to write instead; if you are wondering what Malachite is still missing, look for the
rows marked ✗, which are the ones it is committed to filling in.

## Conventions {#conventions}

GMP's integers, the `mpz_t`, correspond to two Malachite types rather than one.
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html) is the
direct analogue, and
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html) is the
nonnegative half, which is often what you want: it makes "this cannot be negative" a
property the compiler checks rather than a comment. Where a GMP function makes sense for both, both
are listed.

One structural difference accounts for many of the rows below. GMP
gives most operations a family of variants, one per operand type: `mpz_add` and `mpz_add_ui`,
`mpz_mul`, `mpz_mul_ui` and `mpz_mul_si`. Malachite generally does not, because converting a
primitive integer to a
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html) or an
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html) costs
almost nothing (a small value is stored inline, with no allocation), so `x + Natural::from(1u32)`
does the work of `mpz_add_ui` without a separate function needing to exist. Each `_ui` and `_si`
variant still gets its own row below, so you can find it, but the answer is usually the same as its
base function's.

There are three exceptions, where mixed-type operations do exist:

- **Comparison**, where every combination is provided, since asking whether a
  [`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html) equals
  `5u8` is common enough to be worth the convenience.
- **Shifting**, where the right-hand side of `<<` and `>>` may be any primitive integer, signed or
  unsigned. A negative shift count reverses the direction.
- **Operations with no lossless conversion between the operand types**, most visibly between
  [`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html) and
  [`Float`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html). Not
  every rational is a float, so a mixed operation cannot be written as a conversion followed by an
  ordinary one, and the mixed forms exist in their own right.

Everything on this page is reachable through the umbrella crate
[`malachite`](https://docs.rs/malachite/latest/malachite/), which re-exports `Natural`, `Integer`, and
`Rational` at its top level and `Float` behind its `floats` feature. The individual crates also
work as direct dependencies: `malachite-base` holds the traits, `malachite-nz` holds `Natural`
and `Integer`, `malachite-q` holds `Rational`, and `malachite-float` holds `Float`.

Each function falls into one of four categories:

| | meaning |
| :---: | --- |
| ✓ | A Malachite function does the same thing. |
| ≈ | A Malachite function serves the same purpose, but its specification differs. The notes say how. |
| — | No counterpart is needed, either because Rust handles it for you or because it is outside Malachite's scope. The notes say which. |
| ✗ | Malachite does not fully support this yet, but will in a future version. |

## [Initializing Integers](https://gmplib.org/manual/Initializing-Integers) {#initializing-integers}

Rust has no separate initialization step: a variable holds a fully-formed value from the moment it
exists, and its memory is released when it goes out of scope. Most of this section therefore has
nothing to correspond to, which is the point: these are the bugs (use before `mpz_init`, forgotten
`mpz_clear`, double `mpz_clear`) that you stop being able to write.

| | GMP | Malachite |
| :---: | --- | --- |
| ✓ | `void mpz_init (mpz_t x)` | [`Natural::ZERO`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.Zero.html), [`Integer::ZERO`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.Zero.html) |
| — | `void mpz_inits (mpz_t x, ...)` | |
| — | `void mpz_init2 (mpz_t x, mp_bitcnt_t n)` | |
| — | `void mpz_clear (mpz_t x)` | |
| — | `void mpz_clears (mpz_t x, ...)` | |
| — | `void mpz_realloc2 (mpz_t x, mp_bitcnt_t n)` | |

**`mpz_init`.** Malachite's zeros are constants, so `mpz_t x; mpz_init(x);` becomes
`let x = Natural::ZERO;`. `Natural` and `Integer` also implement
[`Default`](https://doc.rust-lang.org/nightly/std/default/trait.Default.html), which returns zero,
so `Natural::default()` works too and `#[derive(Default)]` does the right thing for a struct
containing one.

**`mpz_inits`.** A variadic convenience for initializing several variables at once. In Rust you
declare several variables.

**`mpz_init2`.** A capacity hint rather than a capability: GMP's own manual notes that calling it
"is never necessary", since reallocation is handled automatically, and the same is true of
Malachite. A capacity hint would also sit at a lower level than `Natural` is meant to
occupy, and the idea does not stop at integers: a `Rational` would then need separate hints for
its numerator and its denominator. If you do need that much control, build
a [`Vec`](https://doc.rust-lang.org/nightly/std/vec/struct.Vec.html) with
[`Vec::with_capacity`](https://doc.rust-lang.org/nightly/std/vec/struct.Vec.html#method.with_capacity)
and hand it over with
[`Natural::from_owned_limbs_asc`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html#method.from_owned_limbs_asc),
which takes ownership of the buffer rather than copying it.

**`mpz_clear`, `mpz_clears`.** Memory is freed when the value is dropped, which happens
automatically at the end of its scope. There is nothing to call and nothing to forget.

**`mpz_realloc2`.** Resizing an allocation in place is a C concern.
[`Vec`](https://doc.rust-lang.org/nightly/std/vec/struct.Vec.html) grows on its own, and Malachite
does not expose the allocation. `mpz_realloc2` also sets the value to 0 if it no longer fits;
nothing in Malachite corresponds to that.

## [Assigning Integers](https://gmplib.org/manual/Assigning-Integers) {#assigning-integers}

GMP has one assignment function per source type. Malachite instead has conversion traits, so the
Rust name is the same in every case and the source type is what varies. Two differences run through
the whole section. First, an assignment in Rust is just `=`, so where GMP writes into an existing
`rop` you write `let x = ...` or `x = ...`. Second, GMP's conversions from `double`, `mpq_t`, and
`mpf_t` silently truncate; Malachite will not throw away information without being told to, so
those become either
[`RoundingFrom`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.RoundingFrom.html),
which takes a
[`RoundingMode`](https://docs.rs/malachite-base/latest/malachite_base/rounding_modes/enum.RoundingMode.html)
and returns how it rounded, or
[`TryFrom`](https://doc.rust-lang.org/nightly/std/convert/trait.TryFrom.html), which fails rather
than rounds. Passing `Down` reproduces GMP exactly.

| | GMP | Malachite |
| :---: | --- | --- |
| ✓ | `void mpz_set (mpz_t rop, const mpz_t op)` | [`Clone`](https://doc.rust-lang.org/nightly/std/clone/trait.Clone.html) |
| ✓ | `void mpz_set_ui (mpz_t rop, unsigned long int op)` | [`From`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/conversion/from_primitive_int/index.html) |
| ✓ | `void mpz_set_si (mpz_t rop, signed long int op)` | [`From`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/conversion/from_primitive_int/index.html), [`TryFrom`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/conversion/from_primitive_int/index.html) |
| ≈ | `void mpz_set_d (mpz_t rop, double op)` | [`RoundingFrom`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/conversion/from_primitive_float/index.html), [`TryFrom`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/conversion/from_primitive_float/index.html) |
| ≈ | `void mpz_set_q (mpz_t rop, const mpq_t op)` | [`RoundingFrom`](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/integer_from_rational/index.html), [`TryFrom`](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/integer_from_rational/index.html) |
| ≈ | `void mpz_set_f (mpz_t rop, const mpf_t op)` | [`RoundingFrom`](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/integer_from_float/index.html), [`TryFrom`](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/integer_from_float/index.html) |
| ✗ | `int mpz_set_str (mpz_t rop, const char *str, int base)` | [`FromStringBase`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.FromStringBase.html), [`FromStr`](https://doc.rust-lang.org/nightly/std/str/trait.FromStr.html) |
| — | `void mpz_swap (mpz_t rop1, mpz_t rop2)` | |

**`mpz_set`.** `x = y.clone()`, or `x.clone_from(&y)` if `x` already holds a value whose allocation
is worth reusing. If you do not need the original afterwards, `x = y` moves it and copies nothing.

**`mpz_set_ui`, `mpz_set_si`.** Every primitive integer type converts, not just `unsigned long` and
`signed long`: `Natural::from(123u8)` and `Integer::from(-5i128)` both work. Converting a signed
value to a `Natural` is the one case that can fail, so it is
[`TryFrom`](https://doc.rust-lang.org/nightly/std/convert/trait.TryFrom.html) rather than `From`:
a negative number is not a `Natural`, and the compiler makes you say what should happen.

**`mpz_set_d`.** `Integer::rounding_from(2.7f64, Down)` gives `(2, Less)`: the value, and an
[`Ordering`](https://doc.rust-lang.org/nightly/std/cmp/enum.Ordering.html) saying it rounded down.
`Integer::try_from(2.7f64)` fails instead, and succeeds only for a float that is already an
integer. Neither accepts an infinity or a NaN, where GMP leaves the result unspecified: `try_from`
returns an error and `rounding_from` panics.

**`mpz_set_q`, `mpz_set_f`.** The same two traits, with
[`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html) and
[`Float`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html) as the
source. These live in `malachite-q` and `malachite-float` respectively, so a program that only
uses integers need not depend on rationals or floats.

Malachite's [`Float`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html)
follows [MPFR](https://www.mpfr.org/) rather than GMP's `mpf_t`: it has a precision attached to
each (finite, nonzero) value, correctly rounds every operation, and includes NaN and the infinities. That is the
combination GMP itself recommends — its manual says that the `mpf` functions are "*not* intended as
a smooth extension to IEEE P754 arithmetic", and that "new projects should consider using the GMP
extension library MPFR instead". So the closest GMP analogue of a Malachite
[`Float`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html) is an
`mpfr_t`, not an `mpf_t`, and the MPFR functions will be mapped separately.

**`mpz_set_str`.** `Natural::from_string_base(base, s)` and `Integer::from_string_base(base, s)`
return an [`Option`](https://doc.rust-lang.org/nightly/std/option/enum.Option.html), which is
`None` exactly when GMP would return −1, so the failure is impossible to ignore. Base 10 has the
shorthand [`FromStr`](https://doc.rust-lang.org/nightly/std/str/trait.FromStr.html), letting you
write `"1234".parse::<Integer>()`.

Malachite currently accepts bases 2 through 36, where GMP goes up to 62 by giving upper- and
lower-case letters different values; it will accept the same range in a future version. It already
does so for [`Float`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html),
whose string conversions follow MPFR and reach base 62, so the limit is an inconsistency within
Malachite as much as a difference from GMP. Malachite also has no equivalent of GMP's base 0, which
infers the base from a `0x`, `0b`, or `0` prefix.

**`mpz_swap`.** [`std::mem::swap`](https://doc.rust-lang.org/nightly/std/mem/fn.swap.html) swaps
any two values of the same type, and for a `Natural` or an `Integer` it is the same pointer
exchange GMP performs.

## [Combined Initialization and Assignment](https://gmplib.org/manual/Simultaneous-Integer-Init-_0026-Assign) {#combined-initialization-and-assignment}

GMP offers these as a convenience: initialize the output and store a value in it in one call. In
Rust that is not a convenience but the only thing you can do, since a binding holds a value from
the moment it exists. So this section maps directly onto the previous one: `mpz_init_set_ui` and
`mpz_set_ui` are both `Natural::from`, and it is GMP's separate `mpz_set_*` step that has no
analogue, rather than these.

| | GMP | Malachite |
| :---: | --- | --- |
| ✓ | `void mpz_init_set (mpz_t rop, const mpz_t op)` | [`Clone`](https://doc.rust-lang.org/nightly/std/clone/trait.Clone.html) |
| ✓ | `void mpz_init_set_ui (mpz_t rop, unsigned long int op)` | [`From`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/conversion/from_primitive_int/index.html) |
| ✓ | `void mpz_init_set_si (mpz_t rop, signed long int op)` | [`From`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/conversion/from_primitive_int/index.html), [`TryFrom`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/conversion/from_primitive_int/index.html) |
| ≈ | `void mpz_init_set_d (mpz_t rop, double op)` | [`RoundingFrom`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/conversion/from_primitive_float/index.html), [`TryFrom`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/conversion/from_primitive_float/index.html) |
| ✗ | `int mpz_init_set_str (mpz_t rop, const char *str, int base)` | [`FromStringBase`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.FromStringBase.html), [`FromStr`](https://doc.rust-lang.org/nightly/std/str/trait.FromStr.html) |

Each row behaves as its `mpz_set_*` counterpart in
[Assigning Integers](#assigning-integers) does, including the explicit rounding that `mpz_init_set_d`
needs and the base range that `mpz_init_set_str` is still missing. GMP has no `mpz_init_set_q` or
`mpz_init_set_f`; Malachite draws no such distinction, so converting from a
[`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html) or a
[`Float`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html) looks the
same here as it does there.

One C hazard disappears. `mpz_init_set_str` initializes `rop` even when the string is invalid, so a
caller who checks the return value must still call `mpz_clear`. `Natural::from_string_base` returns
[`None`](https://doc.rust-lang.org/nightly/std/option/enum.Option.html) instead, and there is no
half-built object to remember to clean up.

## [Conversion Functions](https://gmplib.org/manual/Converting-Integers) {#conversion-functions}

Going the other way, out of a big integer and into a machine type, is where a value can fail to
fit. GMP's getters answer that by returning something anyway (the low bits, or a system-dependent
result) and offering separate `mpz_fits_*_p` predicates to ask beforehand. Malachite instead has a
family of traits, one per policy, so the question is answered by which one you call:
[`TryFrom`](https://doc.rust-lang.org/nightly/std/convert/trait.TryFrom.html) fails,
[`WrappingFrom`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.WrappingFrom.html)
keeps the low bits,
[`SaturatingFrom`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.SaturatingFrom.html)
clamps,
[`OverflowingFrom`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.OverflowingFrom.html)
wraps and tells you it did, and
[`ConvertibleFrom`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.ConvertibleFrom.html)
is the `mpz_fits_*_p` predicate.

| | GMP | Malachite |
| :---: | --- | --- |
| ≈ | `unsigned long int mpz_get_ui (const mpz_t op)` | [`TryFrom`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/conversion/primitive_int_from_natural/index.html), [`WrappingFrom`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/conversion/primitive_int_from_natural/index.html), [`SaturatingFrom`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/conversion/primitive_int_from_natural/index.html) |
| ≈ | `signed long int mpz_get_si (const mpz_t op)` | [`TryFrom`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/conversion/primitive_int_from_integer/index.html), [`WrappingFrom`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/conversion/primitive_int_from_integer/index.html), [`SaturatingFrom`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/conversion/primitive_int_from_integer/index.html) |
| ≈ | `double mpz_get_d (const mpz_t op)` | [`RoundingFrom`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/conversion/primitive_float_from_natural/index.html), [`TryFrom`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/conversion/primitive_float_from_natural/index.html) |
| ≈ | `double mpz_get_d_2exp (signed long int *exp, const mpz_t op)` | [`SciMantissaAndExponent`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.SciMantissaAndExponent.html) |
| ✗ | `char * mpz_get_str (char *str, int base, const mpz_t op)` | [`ToStringBase`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.ToStringBase.html), [`Display`](https://doc.rust-lang.org/nightly/std/fmt/trait.Display.html) |

**`mpz_get_ui`.** For a
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html),
`u64::wrapping_from(&x)` is exactly `mpz_get_ui`: the low 64 bits, whatever the size. For an
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html) the two
disagree. GMP discards the sign and returns the low bits of
the absolute value, so `mpz_get_ui` of −1 is 1; `u64::wrapping_from` reduces modulo `2^64` in the
usual two's-complement way, so it gives `u64::MAX`. If you want GMP's answer, take the absolute
value first. Most of the time what you want is `u64::try_from(&x)`, which returns an error
rather than a plausible wrong number.

**`mpz_get_si`.** The same family. GMP returns "the least significant part of op, with the same
sign as op" when the value does not fit, and its own manual warns that the result "is probably not
very useful"; Malachite has no equivalent of that rule, and you choose the policy explicitly.

**`mpz_get_d`.** `f64::rounding_from(&x, Down)` truncates toward zero, as GMP does, and also
returns an [`Ordering`](https://doc.rust-lang.org/nightly/std/cmp/enum.Ordering.html) saying which
way it went. `f64::try_from(&x)` succeeds only when the value is exactly representable. Where GMP's
behavior on a too-large value is system-dependent (infinity, or an overflow trap), Malachite
follows IEEE 754, as MPFR does. The value is rounded as though the exponent range were unbounded,
and only if the result then exceeds `f64::MAX` does overflow occur, in which case `Nearest` yields
infinity and `Down` yields `f64::MAX`. So values a little above `f64::MAX` are still finite: the
first one that becomes infinite under `Nearest` is halfway to the next power of two, `2^1024 -
2^970`, since that is where the unbounded rounding would give `2^1024`.

**`mpz_get_d_2exp`.** [`SciMantissaAndExponent`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.SciMantissaAndExponent.html)
splits a number the same way but normalizes differently: Malachite's mantissa lies in `[1, 2)`,
GMP's in `[0.5, 1)`. For 1000, Malachite gives `(1.953125, 9)` and GMP gives `(0.9765625, 10)`:
the same number, with the binary point one place over. Halve the mantissa and add one to the
exponent to go from Malachite's form to GMP's.

**`mpz_get_str`.** `x.to_string_base(base)` for a lower-case result and `x.to_string_base_upper(base)`
for an upper-case one, with [`Display`](https://doc.rust-lang.org/nightly/std/fmt/trait.Display.html)
and the `{:b}`, `{:o}`, and `{:x}` formats covering the common bases. Malachite returns a
[`String`](https://doc.rust-lang.org/nightly/std/string/struct.String.html), so there is no buffer
to size with `mpz_sizeinbase` and no allocation to free.

As with [`mpz_set_str`](#assigning-integers), the base range is narrower than GMP's for now:
Malachite accepts 2 through 36 and will accept 2 through 62 in a future version, matching
[`Float`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html), whose
string conversions follow MPFR and already reach base 62. GMP additionally accepts negative bases
from −2 to −36, which select upper-case digits; Malachite spells that as a separate method rather
than as a sign on the base.

## [Arithmetic Functions](https://gmplib.org/manual/Integer-Arithmetic) {#arithmetic-functions}

The operators do most of this section. Where GMP writes a result into an `rop` you supplied,
Malachite returns it, and the `*Assign` traits cover the in-place case where GMP's `rop` is also an
input. Every operator also has borrowing forms, so `&x + &y` works without cloning either side.

| | GMP | Malachite |
| :---: | --- | --- |
| ✓ | `void mpz_add (mpz_t rop, const mpz_t op1, const mpz_t op2)` | [`Add`](https://doc.rust-lang.org/nightly/std/ops/trait.Add.html), [`AddAssign`](https://doc.rust-lang.org/nightly/std/ops/trait.AddAssign.html) |
| ✓ | `void mpz_add_ui (mpz_t rop, const mpz_t op1, unsigned long int op2)` | [`Add`](https://doc.rust-lang.org/nightly/std/ops/trait.Add.html), [`AddAssign`](https://doc.rust-lang.org/nightly/std/ops/trait.AddAssign.html) |
| ≈ | `void mpz_sub (mpz_t rop, const mpz_t op1, const mpz_t op2)` | [`Sub`](https://doc.rust-lang.org/nightly/std/ops/trait.Sub.html), [`CheckedSub`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CheckedSub.html), [`SaturatingSub`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.SaturatingSub.html) |
| ≈ | `void mpz_sub_ui (mpz_t rop, const mpz_t op1, unsigned long int op2)` | [`Sub`](https://doc.rust-lang.org/nightly/std/ops/trait.Sub.html), [`CheckedSub`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CheckedSub.html) |
| ≈ | `void mpz_ui_sub (mpz_t rop, unsigned long int op1, const mpz_t op2)` | [`Sub`](https://doc.rust-lang.org/nightly/std/ops/trait.Sub.html), [`CheckedSub`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CheckedSub.html) |
| ✓ | `void mpz_mul (mpz_t rop, const mpz_t op1, const mpz_t op2)` | [`Mul`](https://doc.rust-lang.org/nightly/std/ops/trait.Mul.html), [`MulAssign`](https://doc.rust-lang.org/nightly/std/ops/trait.MulAssign.html) |
| ✓ | `void mpz_mul_si (mpz_t rop, const mpz_t op1, long int op2)` | [`Mul`](https://doc.rust-lang.org/nightly/std/ops/trait.Mul.html), [`MulAssign`](https://doc.rust-lang.org/nightly/std/ops/trait.MulAssign.html) |
| ✓ | `void mpz_mul_ui (mpz_t rop, const mpz_t op1, unsigned long int op2)` | [`Mul`](https://doc.rust-lang.org/nightly/std/ops/trait.Mul.html), [`MulAssign`](https://doc.rust-lang.org/nightly/std/ops/trait.MulAssign.html) |
| ✓ | `void mpz_addmul (mpz_t rop, const mpz_t op1, const mpz_t op2)` | [`AddMul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.AddMul.html), [`AddMulAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.AddMulAssign.html) |
| ✓ | `void mpz_addmul_ui (mpz_t rop, const mpz_t op1, unsigned long int op2)` | [`AddMul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.AddMul.html), [`AddMulAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.AddMulAssign.html) |
| ≈ | `void mpz_submul (mpz_t rop, const mpz_t op1, const mpz_t op2)` | [`SubMul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.SubMul.html), [`SubMulAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.SubMulAssign.html) |
| ≈ | `void mpz_submul_ui (mpz_t rop, const mpz_t op1, unsigned long int op2)` | [`SubMul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.SubMul.html), [`SubMulAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.SubMulAssign.html) |
| ✓ | `void mpz_mul_2exp (mpz_t rop, const mpz_t op1, mp_bitcnt_t op2)` | [`Shl`](https://doc.rust-lang.org/nightly/std/ops/trait.Shl.html), [`ShlAssign`](https://doc.rust-lang.org/nightly/std/ops/trait.ShlAssign.html) |
| ✓ | `void mpz_neg (mpz_t rop, const mpz_t op)` | [`Neg`](https://doc.rust-lang.org/nightly/std/ops/trait.Neg.html) |
| ✓ | `void mpz_abs (mpz_t rop, const mpz_t op)` | [`Abs`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Abs.html), [`UnsignedAbs`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.UnsignedAbs.html) |

**Addition and multiplication.** `x + y`, `x * y`, `x += y`, `x *= y`. The `_ui` and `_si` rows are
the same operators, reached by converting the primitive first, as described under
[Conventions](#conventions); `x + Natural::from(2u32)` is `mpz_add_ui`.

**Subtraction.** For an
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html), subtraction behaves as GMP's does. For a
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html) the result may not be a
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html) at all, so `x - y` panics when `y` is greater than `x`, and
[`CheckedSub`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CheckedSub.html) returns
[`None`](https://doc.rust-lang.org/nightly/std/option/enum.Option.html) instead: the borrow that
would go unnoticed in C becomes a decision you have to make. If you want the difference
regardless of order there is [`AbsDiff`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.AbsDiff.html), and if you want it clamped at
zero, [`SaturatingSub`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.SaturatingSub.html).

`mpz_ui_sub` exists in GMP because the operands are not interchangeable and neither is an `mpz_t`
on the left. In Rust the same `-` handles both sides, so `Natural::from(5u32) - x` is the whole of
it; the same panic-or-`CheckedSub` choice applies, and this direction is the one likelier to go
negative.

**`mpz_addmul`, `mpz_submul`.** [`AddMul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.AddMul.html) and
[`SubMul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.SubMul.html) compute `x + y * z` and `x - y * z` in one step, as GMP does,
saving the temporary. GMP's versions accumulate into `rop`, which is both an input and
the output, so the direct translation of `mpz_addmul(r, a, b)` is `r.add_mul_assign(a, b)` rather
than the by-value form. [`SubMul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.SubMul.html) on a
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html) carries the same restriction as subtraction, with
[`CheckedSubMul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CheckedSubMul.html) and
[`SaturatingSubMul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.SaturatingSubMul.html) alongside it.

**`mpz_mul_2exp`.** `x << n`. Unlike GMP's `mp_bitcnt_t`, the shift amount may be any primitive
integer type, signed or unsigned, and a negative count shifts the other way: `x << -3` is
`x >> 3`. The corresponding right shift is `mpz_fdiv_q_2exp` or `mpz_tdiv_q_2exp`, in
[Division Functions](https://gmplib.org/manual/Integer-Division), since for a negative number the
two disagree about rounding.

**`mpz_neg`, `mpz_abs`.** `-x` and `x.abs()`. Negating a
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html) gives an [`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html), since that is the only type that can hold the
answer, and [`UnsignedAbs`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.UnsignedAbs.html) goes the other way, turning an
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html) into the [`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html) that is its magnitude, a conversion GMP cannot
express because it has only the one type.

## [Division Functions](https://gmplib.org/manual/Integer-Division) {#division-functions}

This is GMP's largest integer section, and almost all of its size comes from one thing: rounding.
Every division comes in three flavors (`cdiv` rounds the quotient toward +∞, `fdiv` toward −∞,
`tdiv` toward zero), and each flavor is then spelled out for quotient, remainder, both, the `_ui`
operand, and powers of two. Malachite does not multiply functions by rounding mode; it takes the
mode as an argument. [`DivRound`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivRound.html) with a
[`RoundingMode`](https://docs.rs/malachite-base/latest/malachite_base/rounding_modes/enum.RoundingMode.html) covers all three at once, along with `Nearest` and `Exact`, which GMP has no
functions for.

There are still named traits for the common pairings, because asking for a quotient and its
remainder together is worth a single call: [`DivMod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivMod.html) is `fdiv`,
[`DivRem`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivRem.html) is `tdiv`, and [`CeilingDivMod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CeilingDivMod.html) is
`cdiv`. Rust's own `/` and `%` truncate, so they are `tdiv` as well.

### Ceiling: quotient toward +∞

| | GMP | Malachite |
| :---: | --- | --- |
| ✓ | `void mpz_cdiv_q (mpz_t q, const mpz_t n, const mpz_t d)` | [`DivRound`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivRound.html), [`CeilingDivMod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CeilingDivMod.html) |
| ✓ | `void mpz_cdiv_r (mpz_t r, const mpz_t n, const mpz_t d)` | [`CeilingMod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CeilingMod.html) |
| ✓ | `void mpz_cdiv_qr (mpz_t q, mpz_t r, const mpz_t n, const mpz_t d)` | [`CeilingDivMod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CeilingDivMod.html) |
| ✓ | `unsigned long int mpz_cdiv_q_ui (mpz_t q, const mpz_t n, unsigned long int d)` | [`DivRound`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivRound.html), [`CeilingDivMod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CeilingDivMod.html) |
| ✓ | `unsigned long int mpz_cdiv_r_ui (mpz_t r, const mpz_t n, unsigned long int d)` | [`CeilingMod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CeilingMod.html) |
| ✓ | `unsigned long int mpz_cdiv_qr_ui (mpz_t q, mpz_t r, const mpz_t n, unsigned long int d)` | [`CeilingDivMod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CeilingDivMod.html) |
| ✓ | `unsigned long int mpz_cdiv_ui (const mpz_t n, unsigned long int d)` | [`CeilingMod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CeilingMod.html) |
| ✓ | `void mpz_cdiv_q_2exp (mpz_t q, const mpz_t n, mp_bitcnt_t b)` | [`ShrRound`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ShrRound.html) |
| ✓ | `void mpz_cdiv_r_2exp (mpz_t r, const mpz_t n, mp_bitcnt_t b)` | [`CeilingModPowerOf2`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CeilingModPowerOf2.html) |

### Floor: quotient toward −∞

| | GMP | Malachite |
| :---: | --- | --- |
| ✓ | `void mpz_fdiv_q (mpz_t q, const mpz_t n, const mpz_t d)` | [`DivRound`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivRound.html), [`DivMod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivMod.html) |
| ✓ | `void mpz_fdiv_r (mpz_t r, const mpz_t n, const mpz_t d)` | [`Mod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Mod.html) |
| ✓ | `void mpz_fdiv_qr (mpz_t q, mpz_t r, const mpz_t n, const mpz_t d)` | [`DivMod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivMod.html) |
| ✓ | `unsigned long int mpz_fdiv_q_ui (mpz_t q, const mpz_t n, unsigned long int d)` | [`DivRound`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivRound.html), [`DivMod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivMod.html) |
| ✓ | `unsigned long int mpz_fdiv_r_ui (mpz_t r, const mpz_t n, unsigned long int d)` | [`Mod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Mod.html) |
| ✓ | `unsigned long int mpz_fdiv_qr_ui (mpz_t q, mpz_t r, const mpz_t n, unsigned long int d)` | [`DivMod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivMod.html) |
| ✓ | `unsigned long int mpz_fdiv_ui (const mpz_t n, unsigned long int d)` | [`Mod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Mod.html) |
| ✓ | `void mpz_fdiv_q_2exp (mpz_t q, const mpz_t n, mp_bitcnt_t b)` | [`Shr`](https://doc.rust-lang.org/nightly/std/ops/trait.Shr.html), [`ShrRound`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ShrRound.html) |
| ✓ | `void mpz_fdiv_r_2exp (mpz_t r, const mpz_t n, mp_bitcnt_t b)` | [`ModPowerOf2`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPowerOf2.html) |

### Truncate: quotient toward zero

| | GMP | Malachite |
| :---: | --- | --- |
| ✓ | `void mpz_tdiv_q (mpz_t q, const mpz_t n, const mpz_t d)` | [`Div`](https://doc.rust-lang.org/nightly/std/ops/trait.Div.html), [`DivRound`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivRound.html), [`DivRem`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivRem.html) |
| ✓ | `void mpz_tdiv_r (mpz_t r, const mpz_t n, const mpz_t d)` | [`Rem`](https://doc.rust-lang.org/nightly/std/ops/trait.Rem.html) |
| ✓ | `void mpz_tdiv_qr (mpz_t q, mpz_t r, const mpz_t n, const mpz_t d)` | [`DivRem`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivRem.html) |
| ✓ | `unsigned long int mpz_tdiv_q_ui (mpz_t q, const mpz_t n, unsigned long int d)` | [`Div`](https://doc.rust-lang.org/nightly/std/ops/trait.Div.html), [`DivRound`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivRound.html), [`DivRem`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivRem.html) |
| ✓ | `unsigned long int mpz_tdiv_r_ui (mpz_t r, const mpz_t n, unsigned long int d)` | [`Rem`](https://doc.rust-lang.org/nightly/std/ops/trait.Rem.html) |
| ✓ | `unsigned long int mpz_tdiv_qr_ui (mpz_t q, mpz_t r, const mpz_t n, unsigned long int d)` | [`DivRem`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivRem.html) |
| ✓ | `unsigned long int mpz_tdiv_ui (const mpz_t n, unsigned long int d)` | [`Rem`](https://doc.rust-lang.org/nightly/std/ops/trait.Rem.html) |
| ✓ | `void mpz_tdiv_q_2exp (mpz_t q, const mpz_t n, mp_bitcnt_t b)` | [`ShrRound`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ShrRound.html) |
| ✓ | `void mpz_tdiv_r_2exp (mpz_t r, const mpz_t n, mp_bitcnt_t b)` | [`RemPowerOf2`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.RemPowerOf2.html) |

### The rest

| | GMP | Malachite |
| :---: | --- | --- |
| ✓ | `void mpz_mod (mpz_t r, const mpz_t n, const mpz_t d)` | [`DivEuclidean`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivEuclidean.html) |
| ✓ | `unsigned long int mpz_mod_ui (mpz_t r, const mpz_t n, unsigned long int d)` | [`DivEuclidean`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivEuclidean.html) |
| ✓ | `void mpz_divexact (mpz_t q, const mpz_t n, const mpz_t d)` | [`DivExact`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivExact.html) |
| ✓ | `void mpz_divexact_ui (mpz_t q, const mpz_t n, unsigned long d)` | [`DivExact`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivExact.html) |
| ✓ | `int mpz_divisible_p (const mpz_t n, const mpz_t d)` | [`DivisibleBy`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivisibleBy.html) |
| ✓ | `int mpz_divisible_ui_p (const mpz_t n, unsigned long int d)` | [`DivisibleBy`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivisibleBy.html) |
| ✓ | `int mpz_divisible_2exp_p (const mpz_t n, mp_bitcnt_t b)` | [`DivisibleByPowerOf2`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivisibleByPowerOf2.html) |
| ✓ | `int mpz_congruent_p (const mpz_t n, const mpz_t c, const mpz_t d)` | [`EqMod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.EqMod.html) |
| ✓ | `int mpz_congruent_ui_p (const mpz_t n, unsigned long int c, unsigned long int d)` | [`EqMod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.EqMod.html) |
| ✓ | `int mpz_congruent_2exp_p (const mpz_t n, const mpz_t c, mp_bitcnt_t b)` | [`EqModPowerOf2`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.EqModPowerOf2.html) |

**Choosing a rounding.** The three conventions, shown as (quotient, remainder). No single example
separates all three: when the quotient is positive, truncation agrees with `fdiv`, and when it is
negative, with `cdiv`.

| | 7 and 3 | −7 and 3 |
| --- | ---: | ---: |
| `cdiv`, [`CeilingDivMod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CeilingDivMod.html) | (3, −2) | (−2, −1) |
| `fdiv`, [`DivMod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivMod.html) | (2, 1) | (−3, 2) |
| `tdiv`, [`DivRem`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivRem.html), `/` and `%` | (2, 1) | (−2, −1) |

[`DivRound`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivRound.html) gives the quotient alone under any mode, and returns an
[`Ordering`](https://doc.rust-lang.org/nightly/std/cmp/enum.Ordering.html) saying which way it
rounded. `Exact` panics if the division is inexact, which is a checked alternative to
`mpz_divexact`.

**`mpz_mod`.** GMP's `mpz_mod` ignores the divisor's sign and always returns a non-negative
remainder, which is neither `fdiv` nor `tdiv` when the divisor is negative. Malachite spells that
[`DivEuclidean`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivEuclidean.html), whose remainder is non-negative by construction;
for an [`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html) it
is a [`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html), so the type says so. Take care not to reach for
[`Mod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Mod.html) instead: that one gives the remainder the sign of the divisor, so with
−7 and −3, `mpz_mod` and `div_euclidean` give 2 while `mod_op` gives −1.

**`mpz_divexact`.** [`DivExact`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivExact.html) has the same contract: much faster than
ordinary division, and correct only when the division is exact. If it is not, the result is
unspecified; depending on the sizes of the operands, you may get a panic or a meaningless answer, just as with `mpz_divexact`. Only use it when you already know
the division comes out even. If you would rather have that checked, `div_round` with `Exact` panics
whenever the division is inexact, and [`DivisibleBy`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivisibleBy.html) asks the question
without dividing.

**`mpz_congruent_p`.** [`EqMod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.EqMod.html) asks whether two values are congruent modulo
a third. For an
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html) the
modulus is a [`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html): a negative modulus means the same as its absolute value, and the
type removes the question.

**The `_2exp` functions.** Dividing by a power of two is a shift. `>>` truncates for an
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html), so
[`ShrRound`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ShrRound.html), which takes a
[`RoundingMode`](https://docs.rs/malachite-base/latest/malachite_base/rounding_modes/enum.RoundingMode.html) , covers all three of `cdiv_q_2exp`, `fdiv_q_2exp`, and `tdiv_q_2exp`. The
matching remainders are [`ModPowerOf2`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPowerOf2.html),
[`CeilingModPowerOf2`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CeilingModPowerOf2.html), and
[`RemPowerOf2`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.RemPowerOf2.html), which are faster than the general versions because
they only need to look at the low bits.

## [Exponentiation Functions](https://gmplib.org/manual/Integer-Exponentiation) {#exponentiation-functions}

| | GMP | Malachite |
| :---: | --- | --- |
| ≈ | `void mpz_powm (mpz_t rop, const mpz_t base, const mpz_t exp, const mpz_t mod)` | [`ModPow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPow.html) |
| ≈ | `void mpz_powm_ui (mpz_t rop, const mpz_t base, unsigned long int exp, const mpz_t mod)` | [`ModPow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPow.html) |
| — | `void mpz_powm_sec (mpz_t rop, const mpz_t base, const mpz_t exp, const mpz_t mod)` | |
| ✓ | `void mpz_pow_ui (mpz_t rop, const mpz_t base, unsigned long int exp)` | [`Pow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Pow.html) |
| ✓ | `void mpz_ui_pow_ui (mpz_t rop, unsigned long int base, unsigned long int exp)` | [`Pow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Pow.html) |

**`mpz_pow_ui`, `mpz_ui_pow_ui`.** `x.pow(n)`, with `n` a `u64`. Malachite agrees with GMP that
0<sup>0</sup> is 1. The second function exists in GMP because its base is an `unsigned long` rather
than an `mpz_t`; in Malachite it is the same
[`Pow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Pow.html) after a conversion, as described under [Conventions](#conventions).
There is also [`PowerOf2`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.PowerOf2.html) for the common case of $$2^n$$, which is
much cheaper than the general power.

**`mpz_powm`, `mpz_powm_ui`.** [`ModPow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPow.html) computes the same thing, with
three differences.

The base must already be reduced modulo $$m$$ (that is, less than $$m$$), where GMP accepts any base
and reduces for you. This is checked: `mod_pow` panics if the base is greater than or equal to the
modulus, so it will not quietly give you a wrong answer, but you do have to reduce first.

It is defined on [`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html) rather than on [`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html), which follows from the first
difference: a base reduced modulo $$m$$ is already non-negative, so the residues that modular
arithmetic works with are naturally [`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html)s. Reduce a negative base into that range and
you are back on the mapped path.

Negative exponents are not accepted, where GMP supports them when the inverse of the base exists.
Malachite separates the two steps: take [`ModInverse`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModInverse.html) first, which
returns [`None`](https://doc.rust-lang.org/nightly/std/option/enum.Option.html) when no inverse
exists rather than raising a division-by-zero error, and then raise that to the positive exponent.

**`mpz_powm_sec`.** Not something Malachite intends to provide. GMP's version is constant-time:
"designed to take the same time and have the same cache access patterns for any two same-size
arguments", for use "where resilience to side-channel attacks is desired". Malachite makes no
timing guarantees anywhere and is not written to be used in that setting, so an equivalent would be
a promise it could not keep. If you need side-channel resistance, keep using GMP for that step, or
a library built for it.

## [Root Extraction Functions](https://gmplib.org/manual/Integer-Roots) {#root-extraction-functions}

| | GMP | Malachite |
| :---: | --- | --- |
| ≈ | `int mpz_root (mpz_t rop, const mpz_t op, unsigned long int n)` | [`FloorRoot`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.FloorRoot.html), [`CeilingRoot`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CeilingRoot.html), [`CheckedRoot`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CheckedRoot.html) |
| ✗ | `void mpz_rootrem (mpz_t root, mpz_t rem, const mpz_t u, unsigned long int n)` | [`RootRem`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.RootRem.html) |
| ✓ | `void mpz_sqrt (mpz_t rop, const mpz_t op)` | [`FloorSqrt`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.FloorSqrt.html) |
| ✓ | `void mpz_sqrtrem (mpz_t rop1, mpz_t rop2, const mpz_t op)` | [`SqrtRem`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.SqrtRem.html) |
| ✗ | `int mpz_perfect_power_p (const mpz_t op)` | [`IsPower`](https://docs.rs/malachite-base/latest/malachite_base/num/factorization/traits/trait.IsPower.html) |
| ✓ | `int mpz_perfect_square_p (const mpz_t op)` | [`IsSquare`](https://docs.rs/malachite-base/latest/malachite_base/num/factorization/traits/trait.IsSquare.html) |

**Truncation versus flooring.** GMP sets
`rop` to "the truncated integer part" of the root, and truncation rounds toward zero, so on a
negative operand GMP's answer is the *ceiling*: `mpz_root` of −30 with $$n = 3$$ is −3, not −4.
Malachite names the rounding instead of leaving it implicit, so
[`FloorRoot`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.FloorRoot.html) of −30 is −4. If you are porting code that takes
roots of negative numbers, GMP's `mpz_root` is Malachite's
[`CeilingRoot`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CeilingRoot.html); on non-negative operands the two agree and
[`FloorRoot`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.FloorRoot.html) is the one you want.

Both directions are available: alongside `floor_root` and `ceiling_root` there are
[`FloorSqrt`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.FloorSqrt.html) and [`CeilingSqrt`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CeilingSqrt.html), and each
has an `Assign` variant that avoids the extra allocation.

**`mpz_root`.** GMP packs two results into one call: it writes the root and returns a flag saying
whether the root was exact. Malachite splits them. If the flag is what you care about, use
[`CheckedRoot`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CheckedRoot.html), which returns
[`Some`](https://doc.rust-lang.org/nightly/std/option/enum.Option.html) of the root when the
operand is a perfect `n`th power and
[`None`](https://doc.rust-lang.org/nightly/std/option/enum.Option.html) when it is not, so the
exactness test and the root cannot get out of sync. If you want the truncated root and the
exactness, use [`RootRem`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.RootRem.html) and check whether the remainder is zero, which
is what GMP's flag amounts to.

Like GMP, Malachite rejects a zeroth root and an even root of a negative number; GMP raises a
division-by-zero or a domain error, and Malachite panics, with the condition written into the
documentation of each method.

**`mpz_rootrem`, `mpz_sqrtrem`.** [`RootRem`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.RootRem.html) and
[`SqrtRem`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.SqrtRem.html) return the root and the remainder as a pair, so GMP's warning
that `mpz_sqrtrem`'s two outputs must not be the same variable has nothing to correspond to. Both
are defined on [`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html) only. For `mpz_sqrtrem` that matches GMP's domain,
since a negative operand has no square root. `mpz_rootrem` does accept a negative `u` together
with an odd `n`, again truncating, and a future version of Malachite will extend
[`RootRem`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.RootRem.html) to [`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html) to cover it.

**`mpz_perfect_square_p`, `mpz_perfect_power_p`.** [`IsSquare`](https://docs.rs/malachite-base/latest/malachite_base/num/factorization/traits/trait.IsSquare.html) and
[`IsPower`](https://docs.rs/malachite-base/latest/malachite_base/num/factorization/traits/trait.IsPower.html) agree with GMP's conventions, including that 0 and 1 count as
both perfect squares and perfect powers. Both are [`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html)-only. For squares that costs
nothing, because no negative number is a square. For powers it does: GMP accepts a negative
operand, where an odd perfect power such as −8 counts. A future version of Malachite will
implement [`IsPower`](https://docs.rs/malachite-base/latest/malachite_base/num/factorization/traits/trait.IsPower.html) for [`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html) so that this case is covered too.

[`ExpressAsPower`](https://docs.rs/malachite-base/latest/malachite_base/num/factorization/traits/trait.ExpressAsPower.html) goes a step further
than the predicate: where `mpz_perfect_power_p` reports only that some $$a^b$$ exists,
`express_as_power` on 64 returns `Some((2, 6))`, and on a number that is not a perfect power
it returns [`None`](https://doc.rust-lang.org/nightly/std/option/enum.Option.html).

## [Number Theoretic Functions](https://gmplib.org/manual/Number-Theoretic-Functions) {#number-theoretic-functions}

### Greatest common divisors and inverses

| | GMP | Malachite |
| :---: | --- | --- |
| ✓ | `void mpz_gcd (mpz_t rop, const mpz_t op1, const mpz_t op2)` | [`Gcd`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Gcd.html) |
| ✓ | `unsigned long int mpz_gcd_ui (mpz_t rop, const mpz_t op1, unsigned long int op2)` | [`Gcd`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Gcd.html) |
| ✓ | `void mpz_gcdext (mpz_t g, mpz_t s, mpz_t t, const mpz_t a, const mpz_t b)` | [`ExtendedGcd`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ExtendedGcd.html) |
| ✓ | `void mpz_lcm (mpz_t rop, const mpz_t op1, const mpz_t op2)` | [`Lcm`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Lcm.html) |
| ✓ | `void mpz_lcm_ui (mpz_t rop, const mpz_t op1, unsigned long op2)` | [`Lcm`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Lcm.html) |
| ✓ | `int mpz_invert (mpz_t rop, const mpz_t op1, const mpz_t op2)` | [`ModInverse`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModInverse.html) |

**`mpz_gcd`, `mpz_lcm`.** [`Gcd`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Gcd.html) and [`Lcm`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Lcm.html) are defined
on [`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html). GMP accepts negative operands but documents that the result "is always
positive even if one or both input operands are negative", so nothing is lost: take
`unsigned_abs` of each [`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html) first, and the answer is the same one GMP would give.
Both traits have `Assign` variants, and there is also [`CheckedLcm`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CheckedLcm.html)
for the primitive types, where the least common multiple can overflow.

`mpz_gcd_ui` returns the gcd as an `unsigned long`, and returns 0 if the answer does not fit,
which is also what it returns for a genuine gcd of 0. Malachite returns a [`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html), which is never too small to hold the answer, so there is nothing to
check.

**`mpz_gcdext`.** [`ExtendedGcd`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ExtendedGcd.html) returns the triple
`(gcd, s, t)` rather than writing through three out-parameters, and it is implemented for both
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html) and [`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html). GMP pins down `s` and `t` uniquely by choosing them so
that `abs(s) < abs(b) / (2 g)` and `abs(t) < abs(a) / (2 g)`; Malachite makes the same choice, so
ported code gets identical cofactors, including in the sign and zero cases.

**`mpz_invert`.** [`ModInverse`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModInverse.html) returns
[`Option`](https://doc.rust-lang.org/nightly/std/option/enum.Option.html), so the "does an inverse exist" flag that GMP returns separately is carried by
the [`None`](https://doc.rust-lang.org/nightly/std/option/enum.Option.html) case and cannot be read out of step with the value. There is also
[`ModPowerOf2Inverse`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPowerOf2Inverse.html) for a power-of-2 modulus, which is
faster than the general routine.

### Jacobi, Legendre, and Kronecker symbols

| | GMP | Malachite |
| :---: | --- | --- |
| ≈ | `int mpz_jacobi (const mpz_t a, const mpz_t b)` | [`JacobiSymbol`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.JacobiSymbol.html) |
| ≈ | `int mpz_legendre (const mpz_t a, const mpz_t p)` | [`LegendreSymbol`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.LegendreSymbol.html) |
| ✓ | `int mpz_kronecker (const mpz_t a, const mpz_t b)` | [`KroneckerSymbol`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.KroneckerSymbol.html) |
| ✓ | `int mpz_kronecker_si (const mpz_t a, long b)` | [`KroneckerSymbol`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.KroneckerSymbol.html) |
| ✓ | `int mpz_kronecker_ui (const mpz_t a, unsigned long b)` | [`KroneckerSymbol`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.KroneckerSymbol.html) |
| ✓ | `int mpz_si_kronecker (long a, const mpz_t b)` | [`KroneckerSymbol`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.KroneckerSymbol.html) |
| ✓ | `int mpz_ui_kronecker (unsigned long a, const mpz_t b)` | [`KroneckerSymbol`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.KroneckerSymbol.html) |

All three symbols exist for [`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html) and [`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html), and all return an `i8` of
−1, 0, or 1. GMP's five Kronecker entries differ only in which argument is a machine integer, so
they collapse into the single [`KroneckerSymbol`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.KroneckerSymbol.html) described under
[Conventions](#conventions).

Negative numerators work the same way in both libraries, so `mpz_jacobi` of −7 over 5 and
`Integer::from(-7).jacobi_symbol(Integer::from(5))` agree. Jacobi and Legendre are marked ≈ for
the denominator instead. GMP asks only that it be odd and will accept a negative one; Malachite
requires it to be odd *and positive*, which is the classical definition of the Jacobi symbol.
Extending the denominator past that is what the Kronecker symbol is for, and Malachite keeps
the two distinct.

So if you are porting code that can pass a negative `b` to `mpz_jacobi` or `mpz_legendre`, reach
for [`KroneckerSymbol`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.KroneckerSymbol.html). It is total (no panics for any inputs)
and returns exactly what GMP returns, because the Kronecker symbol agrees with the Jacobi
symbol whenever the denominator is odd, which is the only case GMP defines `mpz_jacobi` for.

An even denominator panics in Malachite, which is the domain GMP declares the symbol undefined on
and leaves to the caller.

### Factorials and binomial coefficients

| | GMP | Malachite |
| :---: | --- | --- |
| ✓ | `void mpz_fac_ui (mpz_t rop, unsigned long int n)` | [`Factorial`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Factorial.html) |
| ✓ | `void mpz_2fac_ui (mpz_t rop, unsigned long int n)` | [`DoubleFactorial`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DoubleFactorial.html) |
| ✓ | `void mpz_mfac_uiui (mpz_t rop, unsigned long int n, unsigned long int m)` | [`Multifactorial`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Multifactorial.html) |
| ✓ | `void mpz_primorial_ui (mpz_t rop, unsigned long int n)` | [`Primorial`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Primorial.html) |
| ✓ | `void mpz_bin_ui (mpz_t rop, const mpz_t n, unsigned long int k)` | [`BinomialCoefficient`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.BinomialCoefficient.html) |
| ✓ | `void mpz_bin_uiui (mpz_t rop, unsigned long int n, unsigned long int k)` | [`BinomialCoefficient`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.BinomialCoefficient.html) |

These map across directly. All are associated functions taking `u64` arguments, so
`mpz_mfac_uiui(rop, n, m)` is `Natural::multifactorial(n, m)` in the same argument order, and
`mpz_primorial_ui` is `Natural::primorial(n)`, the product of the primes up to and including `n`.

[`BinomialCoefficient`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.BinomialCoefficient.html) is implemented for
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html) as well as [`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html), and it handles a negative `n` the way
`mpz_bin_ui` does, via the identity $$\binom{-n}{k} = (-1)^k \binom{n+k-1}{k}$$.

Malachite adds a few relatives GMP has no equivalent for:
[`Subfactorial`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Subfactorial.html), the number of derangements of `n` elements;
`product_of_first_n_primes` on [`Primorial`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Primorial.html), which counts primes rather
than bounding them; and [`CoprimeWith`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CoprimeWith.html), which answers whether the gcd
is 1 without building it. For the primitive integer types each of these has a `Checked` variant
returning [`None`](https://doc.rust-lang.org/nightly/std/option/enum.Option.html) on overflow, which for the fast-growing functions here is easy to reach.

### Primes and factors

| | GMP | Malachite |
| :---: | --- | --- |
| ✗ | `int mpz_probab_prime_p (const mpz_t n, int reps)` | |
| ✗ | `void mpz_nextprime (mpz_t rop, const mpz_t op)` | |
| ✗ | `int mpz_prevprime (mpz_t rop, const mpz_t op)` | |
| ✗ | `mp_bitcnt_t mpz_remove (mpz_t rop, const mpz_t op, const mpz_t f)` | |
| ✗ | `void mpz_fib_ui (mpz_t fn, unsigned long int n)` | |
| ✗ | `void mpz_fib2_ui (mpz_t fn, mpz_t fnsub1, unsigned long int n)` | |
| ✗ | `void mpz_lucnum_ui (mpz_t ln, unsigned long int n)` | |
| ✗ | `void mpz_lucnum2_ui (mpz_t ln, mpz_t lnsub1, unsigned long int n)` | |

This is where the gaps are, and they fall into three groups.

**Primality.** Malachite has [`IsPrime`](https://docs.rs/malachite-base/latest/malachite_base/num/factorization/traits/trait.IsPrime.html) and
[`Factor`](https://docs.rs/malachite-base/latest/malachite_base/num/factorization/traits/trait.Factor.html) for the primitive integer types, but not yet for
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html), so `mpz_probab_prime_p` has no counterpart at that size. A future version will
supply one. What does exist for [`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html) today is
[`Primes`](https://docs.rs/malachite-base/latest/malachite_base/num/factorization/traits/trait.Primes.html), which produces iterators: `Natural::primes()` for all of them,
and `primes_less_than` and `primes_less_than_or_equal_to` for a bounded run. Those iterators are
the closest thing to `mpz_nextprime` and `mpz_prevprime` right now, but they are a different shape
from "the next prime after this specific large number", so both are marked as gaps rather than as
approximations.

Probabilistic primality testing is planned too, but it is likely to be shaped after FLINT's API
rather than GMP's. FLINT parameterizes its probable-prime tests by the base to test against, as in
`fmpz_is_strong_probabprime(n, a)`, and otherwise fixes the algorithm and takes no parameter at
all, as in its Baillie-PSW and Lucas tests. So there may end up being no direct analogue of GMP's
`reps` count. GMP's three-way return, which distinguishes "definitely prime" from "probably
prime", is also likely to be split across separate functions that each answer yes or no.

**`mpz_remove`.** Removing every occurrence of a factor and reporting how many were removed has no
public counterpart. The machinery is present internally and will be exposed in a future version.

**Fibonacci and Lucas numbers.** Not implemented yet; they are planned for the next release. GMP's
two-output forms, `mpz_fib2_ui` and `mpz_lucnum2_ui`, exist because computing F[n] produces
F[n−1] along the way, so the pair is nearly free; expect the Malachite versions to return tuples
rather than write through out-parameters, as [`ExtendedGcd`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ExtendedGcd.html) and
[`RootRem`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.RootRem.html) do.

## [Comparison Functions](https://gmplib.org/manual/Integer-Comparisons) {#comparison-functions}

| | GMP | Malachite |
| :---: | --- | --- |
| ✓ | `int mpz_cmp (const mpz_t op1, const mpz_t op2)` | [`Ord`](https://doc.rust-lang.org/nightly/std/cmp/trait.Ord.html) |
| ✓ | `int mpz_cmp_d (const mpz_t op1, double op2)` | [`PartialOrd`](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialOrd.html) |
| ✓ | `int mpz_cmp_si (const mpz_t op1, signed long int op2)` | [`PartialOrd`](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialOrd.html) |
| ✓ | `int mpz_cmp_ui (const mpz_t op1, unsigned long int op2)` | [`PartialOrd`](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialOrd.html) |
| ✓ | `int mpz_cmpabs (const mpz_t op1, const mpz_t op2)` | [`OrdAbs`](https://docs.rs/malachite-base/latest/malachite_base/num/comparison/traits/trait.OrdAbs.html) |
| ✓ | `int mpz_cmpabs_d (const mpz_t op1, double op2)` | [`PartialOrdAbs`](https://docs.rs/malachite-base/latest/malachite_base/num/comparison/traits/trait.PartialOrdAbs.html) |
| ✓ | `int mpz_cmpabs_ui (const mpz_t op1, unsigned long int op2)` | [`PartialOrdAbs`](https://docs.rs/malachite-base/latest/malachite_base/num/comparison/traits/trait.PartialOrdAbs.html) |
| ✓ | `int mpz_sgn (const mpz_t op)` | [`Sign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Sign.html) |

Comparison is the one place where Malachite deliberately provides every combination of types,
as noted under [Conventions](#conventions), so GMP's `_d`, `_si`, and `_ui` suffixes all collapse
into the same trait. `Integer::from(-5) < 7u64` works, and so does comparing an
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html) with a [`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html), a `u8`, or an `f32`, in either order.

**What the comparison returns.** GMP returns an `int` whose sign carries the answer and whose
magnitude is unspecified. Rust's comparisons
return [`Ordering`](https://doc.rust-lang.org/nightly/std/cmp/enum.Ordering.html), a three-variant enum you can match
exhaustively, and the compiler will tell you if you miss a case.
[`Sign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Sign.html) returns the same [`Ordering`](https://doc.rust-lang.org/nightly/std/cmp/enum.Ordering.html) rather
than −1, 0, or 1, so `mpz_sgn` fits the same pattern.

**Comparing against a `double`.** The only behavioral difference in this section is here. GMP
says `mpz_cmp_d` and `mpz_cmpabs_d` "can be called with an infinity, but results are undefined
for a NaN", since an `int` has no value that means "unordered".
Malachite's float comparisons return [`Option`](https://doc.rust-lang.org/nightly/std/option/enum.Option.html) of an
[`Ordering`](https://doc.rust-lang.org/nightly/std/cmp/enum.Ordering.html), so a NaN gives [`None`](https://doc.rust-lang.org/nightly/std/option/enum.Option.html)
and there is no undefined case to avoid. Every
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html) is less than positive infinity and greater than negative infinity.

**Why some rows say `Ord` and others say `PartialOrd`.** This follows from Rust rather than from
anything about numbers. [`Ord`](https://doc.rust-lang.org/nightly/std/cmp/trait.Ord.html) relates a type only to itself: its
method is `fn cmp(&self, other: &Self)`, with no room for a second type. Every mixed-type
comparison therefore goes through [`PartialOrd`](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialOrd.html), whatever the other
type is. Comparing an [`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html) with a [`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html) or a `u64` always succeeds and
never yields [`None`](https://doc.rust-lang.org/nightly/std/option/enum.Option.html), but it is still
[`PartialOrd`](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialOrd.html); floats are the case where
[`None`](https://doc.rust-lang.org/nightly/std/option/enum.Option.html) is reachable. Malachite's absolute-value traits
are layered the same way, with [`OrdAbs`](https://docs.rs/malachite-base/latest/malachite_base/num/comparison/traits/trait.OrdAbs.html) for a single type and
[`PartialOrdAbs`](https://docs.rs/malachite-base/latest/malachite_base/num/comparison/traits/trait.PartialOrdAbs.html) for everything else, which is why `mpz_cmpabs` and
`mpz_cmpabs_ui` land on different rows above.

**No macros.** GMP documents `mpz_cmp_ui`, `mpz_cmp_si`, and `mpz_sgn` as macros that evaluate
their arguments more than once, so an argument with a side effect is evaluated repeatedly. These
are ordinary methods in Malachite, so the question does not arise.

Malachite also has [`EqAbs`](https://docs.rs/malachite-base/latest/malachite_base/num/comparison/traits/trait.EqAbs.html) for an absolute-value equality test, whose GMP
spelling is `mpz_cmpabs(a, b) == 0`.

## [Logical and Bit Manipulation Functions](https://gmplib.org/manual/Integer-Logic-and-Bit-Fiddling) {#logical-and-bit-manipulation-functions}

| | GMP | Malachite |
| :---: | --- | --- |
| ✓ | `void mpz_and (mpz_t rop, const mpz_t op1, const mpz_t op2)` | [`BitAnd`](https://doc.rust-lang.org/nightly/std/ops/trait.BitAnd.html) |
| ✓ | `void mpz_ior (mpz_t rop, const mpz_t op1, const mpz_t op2)` | [`BitOr`](https://doc.rust-lang.org/nightly/std/ops/trait.BitOr.html) |
| ✓ | `void mpz_xor (mpz_t rop, const mpz_t op1, const mpz_t op2)` | [`BitXor`](https://doc.rust-lang.org/nightly/std/ops/trait.BitXor.html) |
| ✓ | `void mpz_com (mpz_t rop, const mpz_t op)` | [`Not`](https://doc.rust-lang.org/nightly/std/ops/trait.Not.html) |
| ✓ | `mp_bitcnt_t mpz_popcount (const mpz_t op)` | [`CountOnes`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.CountOnes.html) |
| ✓ | `mp_bitcnt_t mpz_hamdist (const mpz_t op1, const mpz_t op2)` | [`HammingDistance`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.HammingDistance.html), [`CheckedHammingDistance`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.CheckedHammingDistance.html) |
| ✓ | `mp_bitcnt_t mpz_scan0 (const mpz_t op, mp_bitcnt_t starting_bit)` | [`BitScan`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.BitScan.html) |
| ✓ | `mp_bitcnt_t mpz_scan1 (const mpz_t op, mp_bitcnt_t starting_bit)` | [`BitScan`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.BitScan.html) |
| ✓ | `void mpz_setbit (mpz_t rop, mp_bitcnt_t bit_index)` | [`BitAccess`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.BitAccess.html) |
| ✓ | `void mpz_clrbit (mpz_t rop, mp_bitcnt_t bit_index)` | [`BitAccess`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.BitAccess.html) |
| ✓ | `void mpz_combit (mpz_t rop, mp_bitcnt_t bit_index)` | [`BitAccess`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.BitAccess.html) |
| ✓ | `int mpz_tstbit (const mpz_t op, mp_bitcnt_t bit_index)` | [`BitAccess`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.BitAccess.html) |

**Negative numbers work exactly as they do in GMP.** Nothing needs to change when porting
bit-twiddling code. GMP notes that its functions
"behave as if two's complement arithmetic were used (although sign-magnitude is the actual
implementation)", and Malachite makes the same choice: an [`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html) is stored as a sign
and a magnitude, but every bitwise operation treats it as an infinite two's-complement bit string,
sign-extended forever. So `-1` has a `true` bit at every index, `!x` is `-x - 1`, and
`Integer::from(-12) & Integer::from(-10)` is `-12`, all matching `mpz_com` and `mpz_and` term for
term.

The four bit-access operations line up one to one:
[`BitAccess`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.BitAccess.html) provides `get_bit` for `mpz_tstbit`, `set_bit` for
`mpz_setbit`, `clear_bit` for `mpz_clrbit`, and `flip_bit` for `mpz_combit`, all indexed from the
least significant bit as GMP does. `assign_bit` is a small addition, setting a bit to a `bool` you
supply rather than to a fixed value.

**GMP's "infinite" sentinel becomes [`None`](https://doc.rust-lang.org/nightly/std/option/enum.Option.html).** GMP has four situations where the honest
answer is "there is no such number", and in each it returns the largest possible `mp_bitcnt_t`:
the population count of a negative number, the Hamming distance between operands of opposite
signs, and `mpz_scan0` or `mpz_scan1` finding no such bit. That sentinel is
indistinguishable from a real answer except by testing for it explicitly. Malachite returns [`Option`](https://doc.rust-lang.org/nightly/std/option/enum.Option.html) in precisely those four cases, so
`Integer::from(-1).index_of_next_false_bit(0)` and `Integer::from(0).index_of_next_true_bit(0)`
are both [`None`](https://doc.rust-lang.org/nightly/std/option/enum.Option.html), and so is
`Integer::from(12).checked_hamming_distance(&Integer::from(-10))`.

Which of the two forms you get is decided by the type. A
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html) has no negative case to worry about, so `count_ones` and `hamming_distance`
return a plain `u64`. An [`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html) can be negative, so it offers `checked_count_ones` and
[`CheckedHammingDistance`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.CheckedHammingDistance.html) returning
[`Option`](https://doc.rust-lang.org/nightly/std/option/enum.Option.html) instead.

**Malachite has several traits with no `mpz_` counterpart.**
[`BitBlockAccess`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.BitBlockAccess.html) reads or writes a whole range of bits at once
rather than one at a time, [`BitIterable`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.BitIterable.html) yields a
double-ended iterator over the bits, [`BitConvertible`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.BitConvertible.html) converts to
and from a sequence of bits, and [`LowMask`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.LowMask.html) builds a run of `n` ones.
[`SignificantBits`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.SignificantBits.html),
[`TrailingZeros`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.TrailingZeros.html), [`LeadingZeros`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.LeadingZeros.html),
and [`CountZeros`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.CountZeros.html) round out the set. In GMP you would reach for
`mpz_sizeinbase` or a `mpz_scan1` call for some of these and write the rest by hand.

## [Input and Output Functions](https://gmplib.org/manual/I_002fO-of-Integers) {#input-and-output-functions}

| | GMP | Malachite |
| :---: | --- | --- |
| ✗ | `size_t mpz_out_str (FILE *stream, int base, const mpz_t op)` | [`ToStringBase`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.ToStringBase.html), [`Display`](https://doc.rust-lang.org/nightly/std/fmt/trait.Display.html) |
| ✗ | `size_t mpz_inp_str (mpz_t rop, FILE *stream, int base)` | [`FromStringBase`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.FromStringBase.html), [`FromStr`](https://doc.rust-lang.org/nightly/std/str/trait.FromStr.html) |
| ≈ | `size_t mpz_out_raw (FILE *stream, const mpz_t op)` | [`Serialize`](https://docs.rs/serde/latest/serde/trait.Serialize.html) |
| ≈ | `size_t mpz_inp_raw (mpz_t rop, FILE *stream)` | [`Deserialize`](https://docs.rs/serde/latest/serde/trait.Deserialize.html) |

**There is no `FILE *` half to port.** Rust keeps formatting and I/O separate, so writing a number
to a stream is `write!(stream, "{x}")` for any [`Write`](https://doc.rust-lang.org/nightly/std/io/trait.Write.html), and reading
one means pulling a token out of your reader and handing it to
[`FromStr`](https://doc.rust-lang.org/nightly/std/str/trait.FromStr.html). Nothing in Malachite needs to know about streams, and
you are not restricted to `stdio` the way `mpz_out_str` is.

What keeps the first two rows at ✗ is the base, not the stream. `mpz_out_str` and `mpz_inp_str`
are `mpz_get_str` and `mpz_set_str` with a stream attached, so they inherit the same gap:
Malachite accepts bases 2 through 36 today and will accept 2 through 62 in a future version. See
[Conversion Functions](#conversion-functions) for the details, including GMP's negative bases and
its base-0 prefix detection.

**`mpz_out_raw` and `mpz_inp_raw` map onto [serde](https://serde.rs/).** Enable the `enable_serde`
feature and [`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html) and [`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html) implement
[`Serialize`](https://docs.rs/serde/latest/serde/trait.Serialize.html) and [`Deserialize`](https://docs.rs/serde/latest/serde/trait.Deserialize.html), as do
`Rational` and `Float`. The encoding is not fixed: the same type goes to JSON, to a compact binary format such as
`bincode` or CBOR, or to anything else with a serde implementation, and it composes with whatever
struct the number is embedded in.

The two are marked ≈ rather than ✓ because the encodings differ. GMP writes "4 bytes of size
information, and that many bytes of limbs" in big-endian order. Malachite writes a hexadecimal
string: a 104-bit `Natural` becomes `"0x18ee90ff6c373e0ee4e3f0ad2"`, and an
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html) carries its sign outside the prefix, as `"-0x…"`. Two consequences follow. The
result is larger: through `bincode`, that number takes 35 bytes where GMP's raw format takes 17,
roughly a factor of two.

The second consequence: a hexadecimal string has no byte order and no word size. GMP specifies
big-endian ordering to keep its raw format portable across architectures, and its manual notes a
compatibility provision from when 32-bit and 64-bit systems diverged; the string form has nothing
to specify, so a [`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html) written by a build
using 32-bit limbs is read back by a build using 64-bit limbs with nothing to arrange. It is also
human-readable, which helps when the serialized data itself is what you are debugging.

## [Random Number Functions](https://gmplib.org/manual/Integer-Random-Numbers) {#random-number-functions}

| | GMP | Malachite |
| :---: | --- | --- |
| ✓ | `void mpz_urandomb (mpz_t rop, gmp_randstate_t state, mp_bitcnt_t n)` | [`get_random_natural_with_up_to_bits`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/random/fn.get_random_natural_with_up_to_bits.html), [`random_naturals`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/random/fn.random_naturals.html) |
| ✓ | `void mpz_urandomm (mpz_t rop, gmp_randstate_t state, const mpz_t n)` | [`get_random_natural_less_than`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/random/fn.get_random_natural_less_than.html), [`random_naturals_less_than`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/random/fn.random_naturals_less_than.html) |
| ✓ | `void mpz_rrandomb (mpz_t rop, gmp_randstate_t state, mp_bitcnt_t n)` | [`get_striped_random_natural_with_bits`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/random/fn.get_striped_random_natural_with_bits.html), [`striped_random_naturals`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/random/fn.striped_random_naturals.html) |
| — | `void mpz_random (mpz_t rop, mp_size_t max_size)` | |
| — | `void mpz_random2 (mpz_t rop, mp_size_t max_size)` | |

Random generation lives behind the `random` feature, in the
[`natural::random`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/random/index.html) and [`integer::random`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/random/index.html) modules. The last two rows are
marked obsolete by GMP's manual, which directs you to `mpz_urandomb` and `mpz_rrandomb`, the rows
above them.

**There is no `gmp_randstate_t` to look after.** Every GMP function here takes a state that you
must initialize with one of the `gmp_randinit` functions and release with `gmp_randclear`, and
that pairing is yours to get right. Malachite's iterator forms take a [`Seed`](https://docs.rs/malachite-base/latest/malachite_base/random/struct.Seed.html) and return an
infinite iterator, so there is no initialize-and-release pair to balance and nothing to leak. The
single-value `get_*` forms do take a bit source by mutable reference
([`StripedBitSource`](https://docs.rs/malachite-base/latest/malachite_base/num/random/striped/struct.StripedBitSource.html) for the striped ones),
which is the same idea as GMP's state, with the compiler tracking its lifetime.

**Iterators rather than one call per number.** GMP fills one `mpz_t` per call. Malachite's primary
form is a stream: [`random_naturals`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/random/fn.random_naturals.html) is an infinite iterator that
composes with `take`, `filter`, `zip`, and the rest of the iterator machinery. The `get_*`
functions in the table are for when you want a single value.

**Striped generation is a parameter, not a separate function.** GMP has one function for
producing "long strings of zeros and ones", with no parameter controlling how long. Malachite
makes the mean run length an explicit argument to [`StripedBitSource`](https://docs.rs/malachite-base/latest/malachite_base/num/random/striped/struct.StripedBitSource.html):

```text
uniform 40-bit:                  1110010001101100010001011110111101110001
striped 40-bit, mean run 4:      1111000000000111100000000110011010000000
striped 40-bit, mean run 16:     1000000000000111111111111111111111110000
```

Striping also extends across the generator shapes. Alongside [`striped_random_naturals`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/random/fn.striped_random_naturals.html) there are
striped versions for ranges, inclusive ranges, and ranges to infinity, and on the
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html) side for positive, negative, and nonzero values. GMP's `mpz_rrandomb`
corresponds to one entry in that grid.

**Sizes drawn from a distribution.** `mpz_urandomb` fixes the bit length, so every number it
produces is the same size. Malachite's stream generators instead take a *mean* bit length and draw sizes
from a geometric distribution, so one stream spans many magnitudes:

```text
random_naturals(seed, 32, 1) -> 20431208470830262, 2777240, 114, 12184833305054,
                                1121025855008623490210, 13478874522577592
```

That distribution suits property testing, which is what Malachite itself uses these generators
for. Fixing the size is still available through
[`get_random_natural_with_up_to_bits`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/random/fn.get_random_natural_with_up_to_bits.html) when you
need GMP's behavior exactly.

## [Import and Export](https://gmplib.org/manual/Integer-Import-and-Export) {#import-and-export}

| | GMP | Malachite |
| :---: | --- | --- |
| ≈ | `void mpz_import (mpz_t rop, size_t count, int order, size_t size, int endian, size_t nails, const void *op)` | [`PowerOf2Digits`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.PowerOf2Digits.html), [`from_limbs_asc`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html#method.from_limbs_asc) |
| ≈ | `void * mpz_export (void *rop, size_t *countp, int order, size_t size, int endian, size_t nails, const mpz_t op)` | [`PowerOf2Digits`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.PowerOf2Digits.html), [`to_limbs_asc`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html#method.to_limbs_asc) |

These are ≈ because GMP packs the whole job into one function with six parameters, while
Malachite spreads it over a family of methods. Nothing is missing; the pieces are arranged
differently.
Taking GMP's parameters one at a time:

**`order`** becomes the method name. Every conversion comes in `_asc` and `_desc` forms
([`to_limbs_asc`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html#method.to_limbs_asc) and [`to_limbs_desc`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html#method.to_limbs_desc),
`to_power_of_2_digits_asc` and `to_power_of_2_digits_desc`), so GMP's `order` values of −1 and 1
are two calls rather than an argument.

**`size`** becomes the digit type together with the base. For GMP's native word size, use
[`to_limbs_asc`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html#method.to_limbs_asc), which returns a `Vec<Limb>`. For any other width,
[`PowerOf2Digits`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.PowerOf2Digits.html) is implemented for `u8` through `u64`, so
`n.to_power_of_2_digits_asc(8)` collected as a `Vec<u8>` is GMP's `size = 1`, and
`n.to_power_of_2_digits_asc(32)` as a `Vec<u32>` is `size = 4`.

**`nails`** has a direct analogue. GMP's nails let you skip
the top bits of each word; in Malachite you get the same effect by choosing a `log_base` smaller
than the digit type's width. `n.to_power_of_2_digits_asc(7)` as a `Vec<u8>` is exactly GMP's
"8-bit words with one nail bit": every digit is below `0x80`, and
`from_power_of_2_digits_asc(7, …)` reads it back.

**`endian`** has nothing to correspond to. Malachite produces typed integers rather than a byte buffer,
so byte order within a word never comes up. If you need a particular byte order, take the digits
and call `to_be_bytes` or `to_le_bytes` on each.

**`count` and `countp`** disappear into the [`Vec`](https://doc.rust-lang.org/nightly/std/vec/struct.Vec.html),
which knows its own length, and GMP's convention where a `NULL` `rop` means "allocate for me" is
what always happens.

**The sign** matches by construction. GMP says "the sign of `op` is ignored, just the absolute
value is exported", and on import "there is no sign taken from the data, `rop` will simply be a
positive integer". Malachite's methods live on [`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html), where there is no sign to
ignore; starting from an [`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html) you take `unsigned_abs` first, which makes the same
choice explicit.

**One caveat.** [`to_limbs_asc`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html#method.to_limbs_asc) returns `Vec<Limb>`, and
`Limb` is `u32` or `u64` depending on whether the crate was built with the `32_bit_limbs` feature.
That makes limbs the right choice when you are feeding another part of the same program and the
wrong one when the bytes will outlive the build that wrote them. For anything persisted or sent
over a wire, name the width explicitly with
[`PowerOf2Digits`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.PowerOf2Digits.html) instead.

Finally, [`Digits`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.Digits.html) works in an arbitrary base, not just powers of two, so
`to_digits_asc(&10u8)` gives decimal digits directly. GMP's pair handles binary word data only;
for anything else you would go through `mpz_get_str`.

## [Miscellaneous Functions](https://gmplib.org/manual/Miscellaneous-Integer-Functions) {#miscellaneous-functions}

| | GMP | Malachite |
| :---: | --- | --- |
| ✓ | `int mpz_fits_ulong_p (const mpz_t op)` | [`ConvertibleFrom`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.ConvertibleFrom.html) |
| ✓ | `int mpz_fits_slong_p (const mpz_t op)` | [`ConvertibleFrom`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.ConvertibleFrom.html) |
| ✓ | `int mpz_fits_uint_p (const mpz_t op)` | [`ConvertibleFrom`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.ConvertibleFrom.html) |
| ✓ | `int mpz_fits_sint_p (const mpz_t op)` | [`ConvertibleFrom`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.ConvertibleFrom.html) |
| ✓ | `int mpz_fits_ushort_p (const mpz_t op)` | [`ConvertibleFrom`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.ConvertibleFrom.html) |
| ✓ | `int mpz_fits_sshort_p (const mpz_t op)` | [`ConvertibleFrom`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.ConvertibleFrom.html) |
| ✓ | `int mpz_odd_p (const mpz_t op)` | [`Parity`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Parity.html) |
| ✓ | `int mpz_even_p (const mpz_t op)` | [`Parity`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Parity.html) |
| ≈ | `size_t mpz_sizeinbase (const mpz_t op, int base)` | [`SignificantBits`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.SignificantBits.html), [`FloorLogBase`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.FloorLogBase.html) |

**The six `fits` functions collapse into one trait.**
[`ConvertibleFrom`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.ConvertibleFrom.html) answers the same question for any target
type, so `u32::convertible_from(&n)` replaces `mpz_fits_uint_p` and `i16::convertible_from(&n)`
replaces `mpz_fits_sshort_p`. Any primitive integer type can be the target, including `u128`,
`i128`, `usize`, and `isize`, which have no `mpz_fits` counterparts. If you want the value rather
than the verdict, [`TryFrom`](https://doc.rust-lang.org/nightly/std/convert/trait.TryFrom.html) does both at once.

**`mpz_odd_p` and `mpz_even_p`** are [`Parity`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Parity.html)'s `odd` and `even`. GMP
notes that these are macros that "evaluate their argument more than once"; the Malachite versions
are ordinary methods, as with the comparison macros above.

**`mpz_sizeinbase` is the one ≈, and the difference is exactness.** GMP is explicit that "the
result will be either exact or 1 too big", and only promises exactness when the base is a power of
two. That is a deliberate trade: allowing an overshoot lets GMP answer from the bit count alone,
which is what you want when sizing a buffer before a call to `mpz_get_str`. For example, `mpz_sizeinbase(999, 10)` returns 4, though 999 has three digits; in base 10 every
value just below a power of ten is reported one too large.

Malachite has no single function with those semantics, but it has exact answers. For base 2,
[`SignificantBits`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.SignificantBits.html) gives the bit count directly. For any other
base, `n.floor_log_base(&base) + 1` is the exact number of digits, and
[`Digits`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.Digits.html) returns the digits themselves if you need them. If you need
GMP's cheap upper bound rather than the true count, derive it from
[`SignificantBits`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.SignificantBits.html) and keep the estimate explicit.

Watch the zero case when porting. GMP special-cases it, so `mpz_sizeinbase` of 0 is 1 in every
base, on the reasoning that writing zero still takes one character. Malachite does not special-case
it: `Natural::ZERO.significant_bits()` is 0, and `floor_log_base` panics on 0, since the logarithm
of zero is undefined. Whichever convention you want, make it explicit at the call site.

## [Special Functions](https://gmplib.org/manual/Integer-Special-Functions) {#special-functions}

| | GMP | Malachite |
| :---: | --- | --- |
| — | `void mpz_array_init (mpz_t integer_array, mp_size_t array_size, mp_size_t fixed_num_bits)` | |
| — | `void * _mpz_realloc (mpz_t integer, mp_size_t new_alloc)` | |
| ✓ | `mp_limb_t mpz_getlimbn (const mpz_t op, mp_size_t n)` | [`limbs`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html#method.limbs) |
| ✓ | `size_t mpz_size (const mpz_t op)` | [`limb_count`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html#method.limb_count) |
| ✓ | `const mp_limb_t * mpz_limbs_read (const mpz_t x)` | [`limbs`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html#method.limbs) |
| — | `mp_limb_t * mpz_limbs_write (mpz_t x, mp_size_t n)` | |
| — | `mp_limb_t * mpz_limbs_modify (mpz_t x, mp_size_t n)` | |
| — | `void mpz_limbs_finish (mpz_t x, mp_size_t s)` | |
| — | `mpz_srcptr mpz_roinit_n (mpz_t x, const mp_limb_t *xp, mp_size_t xs)` | |
| — | `mpz_t MPZ_ROINIT_N (mp_limb_t *xp, mp_size_t xs)` | |

**Reading limbs maps directly.** [`limb_count`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html#method.limb_count) is `mpz_size`, and
[`limbs`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html#method.limbs) returns a borrowed view that is both an iterator and indexable, so
`n.limbs()[i]` is `mpz_getlimbn(op, i)` and the view as a whole is `mpz_limbs_read`. The edge
cases agree too: the limb count of zero is 0, and indexing at or past the end returns 0, matching
what GMP promises for an out-of-range `mpz_getlimbn`.

**The writing side has no counterpart.** `mpz_limbs_write`, `mpz_limbs_modify`,
`mpz_limbs_finish`, and `mpz_roinit_n` let you drop to GMP's `mpn_` layer, work on the limb array
in place, and hand the result back without a copy, with `mpz_limbs_finish` leaving normalization
to the caller.

Malachite does not expose the equivalent. Handing out a
mutable limb array would mean publishing the normalization invariant as part of the public API,
which freezes the internal representation. Malachite's representation is also not the one the
signatures assume: a [`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html) smaller than 2<sup>64</sup> is stored inline, with no
allocated limb array to point at, so `mpz_limbs_write` would have nothing to return in the common
case.
Reading works because a view can be synthesized; writing would not. Along the same lines,
`mpz_array_init` is marked obsolete by GMP's manual, and the allocation control offered by
`_mpz_realloc` is handled by the growth strategy of the underlying
[`Vec`](https://doc.rust-lang.org/nightly/std/vec/struct.Vec.html).

Malachite is developed by Mikhail Hogrefe. Thanks to 43615, b4D8, Romain Billot, Maxim Biryukov, coolreader18, Dasaav-dsv, Duncan Freeman, florian1345, konstin, Rowan Hart, YunWon Jeong, Park Joon-Kyu, Antonio Mamić, OliverNChalk, Kevin Phoenix, probablykasper, shekohex, skycloudd, John Vandenberg, Brandon Weeks, and Will Youmans for additional contributions.

Copyright © 2026 Mikhail Hogrefe
