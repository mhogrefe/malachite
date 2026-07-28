---
layout: default
title: "Malachite for GMP Users: Rationals"
permalink: /mapping/gmp-rationals/
theme: jekyll-theme-slate
---

# Malachite for GMP Users: Rationals

This page maps the functions of GMP's rational number type, `mpq_t`, onto
[`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html), from
the `malachite-q` crate (and re-exported at the top level of the umbrella crate
[`malachite`](https://docs.rs/malachite/latest/malachite/)). It follows the organization of the
[Rational Number Functions](https://gmplib.org/manual/Rational-Number-Functions) chapter of the
GMP manual, and is a companion to
[Malachite for GMP Users: Integers](/mapping/gmp-integers/); the [mapping index](/mapping/) lists
both, along with the pages for other libraries as they are written. The [Conventions](/mapping/gmp-integers/#conventions) described there, including the
[Allocation](/mapping/gmp-integers/#allocation) discussion, apply here unchanged.

One fact shapes the whole chapter. A GMP rational is only canonical, in lowest terms with a
positive denominator, if you keep it that way: the arithmetic and comparison functions assume
canonical operands, several assignment functions can leave a value non-canonical, and the manual
is dotted with reminders to call `mpq_canonicalize` before going on. A
[`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html) is
canonical by construction: it is a sign together with a numerator and denominator that are kept
coprime, with the denominator always positive. Every constructor reduces its input on the way in,
every operation produces a reduced result, and there is no way to build or observe a
non-canonical value. `mpq_canonicalize` therefore has no counterpart, and none of the manual's
canonicalization reminders carry over.

Each function falls into one of four categories:

| | meaning |
| :---: | --- |
| ✓ | A Malachite function does the same thing. |
| ≈ | A Malachite function serves the same purpose, but its specification differs. The notes say how. |
| — | No counterpart is needed, either because Rust handles it for you or because it is outside Malachite's scope. The notes say which. |
| ✗ | Malachite does not fully support this yet, but will in a future version. |

## [Initialization and Assignment](https://gmplib.org/manual/Initializing-Rationals) {#initialization-and-assignment}

| | GMP | Malachite |
| :---: | --- | --- |
| — | `void mpq_canonicalize (mpq_t op)` | |
| ✓ | `void mpq_init (mpq_t x)` | [`Rational::ZERO`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.Zero.html) |
| — | `void mpq_inits (mpq_t x, ...)` | |
| — | `void mpq_clear (mpq_t x)` | |
| — | `void mpq_clears (mpq_t x, ...)` | |
| ✓ | `void mpq_set (mpq_t rop, const mpq_t op)` | [`Clone`](https://doc.rust-lang.org/nightly/std/clone/trait.Clone.html) |
| ✓ | `void mpq_set_z (mpq_t rop, const mpz_t op)` | [`From`](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/from_integer/index.html) |
| ✓ | `void mpq_set_ui (mpq_t rop, unsigned long int op1, unsigned long int op2)` | [`from_unsigneds`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.from_unsigneds) |
| ✓ | `void mpq_set_si (mpq_t rop, signed long int op1, unsigned long int op2)` | [`from_signeds`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.from_signeds) |
| ✗ | `int mpq_set_str (mpq_t rop, const char *str, int base)` | [`FromStr`](https://doc.rust-lang.org/nightly/std/str/trait.FromStr.html) |
| — | `void mpq_swap (mpq_t rop1, mpq_t rop2)` | |

**`mpq_canonicalize`.** Nothing to call: as described above, a
[`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html) is
always canonical. Wherever the manual attaches a canonicalization reminder to a function, to
`mpq_set_ui`, `mpq_set_si`, and `mpq_set_str` here and to `mpq_set_num` and `mpq_set_den` later,
the Malachite counterpart reduces its inputs itself.

**`mpq_init`.** `let x = Rational::ZERO;` holds exactly the value `mpq_init` produces, 0/1. `Rational` also implements
[`Default`](https://doc.rust-lang.org/nightly/std/default/trait.Default.html), which returns
zero.

**`mpq_inits`, `mpq_clear`, `mpq_clears`.** As with the integers, declaration replaces
initialization and going out of scope replaces clearing; see the fuller discussion under
[Initializing Integers](/mapping/gmp-integers/#initializing-integers).

**`mpq_set`.** `x = y.clone()`, or `x.clone_from(&y)` if `x` already holds a value whose
allocations are worth reusing. If you do not need the original afterwards, `x = y` moves it and
copies nothing.

**`mpq_set_z`.** `Rational::from(z)` for an
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html) or a
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html), by
value or by reference; the by-value form reuses the integer's storage as the numerator. The
primitive integer types convert the same way, so `Rational::from(3u32)` covers what GMP would
spell `mpq_set_ui(q, 3, 1)`.

**`mpq_set_ui`, `mpq_set_si`.** `Rational::from_unsigneds(n, d)` and
`Rational::from_signeds(n, d)`, for any primitive integer width. The GMP caveat that the result
"may have common factors" and must be canonicalized before use does not exist here:
`Rational::from_unsigneds(4, 6)` is 2/3. A zero denominator panics at the construction site,
where GMP would divide by zero at some later use. For numerators and denominators that are
already big integers there are
[`from_naturals`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.from_naturals)
and
[`from_integers`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.from_integers),
each with a `_ref` variant that borrows instead of consuming.

**`mpq_set_str`.** Base 10 works through
[`FromStr`](https://doc.rust-lang.org/nightly/std/str/trait.FromStr.html):
`"22/7".parse::<Rational>()`, accepting an integer like `"41"` or a fraction like `"41/152"`,
just as GMP does. The numerator and denominator need not be in lowest terms; the parse reduces
them. A zero denominator is an `Err` at parse time, where GMP accepts the string and the zero
surfaces as a division error later. The sign may only appear at the front of the string. The
other bases are the gap: GMP accepts 2 through 62, and `Rational` does not yet have the
`from_string_base` that
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html) and
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html) provide
for bases 2 through 36.

**`mpq_swap`.** [`std::mem::swap`](https://doc.rust-lang.org/nightly/std/mem/fn.swap.html) swaps
any two values of the same type, exchanging the representations without copying any limbs.

## [Conversion Functions](https://gmplib.org/manual/Rational-Conversions) {#conversion-functions}

A short section, and a simpler one than its integer counterpart: since every finite `double` is
a rational number, the conversions between `double` and
[`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html) are
exact in one direction and rounded in the other, never lossy in both.

| | GMP | Malachite |
| :---: | --- | --- |
| ≈ | `double mpq_get_d (const mpq_t op)` | [`RoundingFrom`](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/primitive_float_from_rational/index.html), [`TryFrom`](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/primitive_float_from_rational/index.html) |
| ✓ | `void mpq_set_d (mpq_t rop, double op)` | [`TryFrom`](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/from_primitive_float/index.html) |
| ✓ | `void mpq_set_f (mpq_t rop, const mpf_t op)` | [`TryFrom`](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/rational_from_float/index.html) |
| ✗ | `char * mpq_get_str (char *str, int base, const mpq_t op)` | [`Display`](https://doc.rust-lang.org/nightly/std/fmt/trait.Display.html) |

**`mpq_get_d`.** GMP truncates toward zero, so `f64::rounding_from(&q, Down)` reproduces it, and
also returns an [`Ordering`](https://doc.rust-lang.org/nightly/std/cmp/enum.Ordering.html) saying
which way the rounding went; pass `Nearest` instead for round-to-nearest.
`f64::try_from(&q)` succeeds only when the value is exactly representable, and
[`ConvertibleFrom`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.ConvertibleFrom.html)
asks that question without performing the conversion. At the large end, the IEEE-following
overflow behavior described under
[`mpz_get_d`](/mapping/gmp-integers/#conversion-functions) applies unchanged. The small end is new,
since a rational can also be too small for a `double`: there, truncation toward zero yields
zero.

**`mpq_set_d`.** `Rational::try_from(d)`. The conversion is exact, as in GMP, and exact means
literal: `Rational::try_from(0.1f64)` is 3602879701896397/36028797018963968, the value the
`double` holds, not 1/10. If what you want is the simplest rational that rounds back to the
given `double`, that is
[`try_from_float_simplest`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.try_from_float_simplest),
which does return 1/10 for `0.1f64`. The conversion fails, rather than being undefined, for an
infinity or a NaN.

**`mpq_set_f`.** `Rational::try_from(f)` for a
[`Float`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html), also
exact, also failing for an infinity or a NaN. As explained under
[Assigning Integers](/mapping/gmp-integers/#assigning-integers), Malachite's
[`Float`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html) follows
MPFR rather than GMP's `mpf_t`, so this row is the analogue rather than the equivalent of
`mpq_set_f`.

**`mpq_get_str`.** For base 10,
[`Display`](https://doc.rust-lang.org/nightly/std/fmt/trait.Display.html) produces exactly GMP's
format: `"num/den"`, or just `"num"` when the denominator is 1, so `q.to_string()` and
`format!("{q}")` are direct replacements, returning a
[`String`](https://doc.rust-lang.org/nightly/std/string/struct.String.html) with no buffer to
size and no allocation to free. Other bases are missing, as with `mpq_set_str` above: `Rational`
has no `to_string_base` yet. The base only affects how the two integers are printed: `22/7` in
base 16 is `"16/7"`. For a representation where the base matters mathematically,
[`digits`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.digits)
and
[`to_digits`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.to_digits)
produce the positional expansion of a rational in any base, with the eventually-repeating
fractional part represented exactly, and
[`ToSci`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.ToSci.html)
renders a rounded decimal form; GMP has no counterpart to either.

## [Arithmetic Functions](https://gmplib.org/manual/Rational-Arithmetic) {#arithmetic-functions}

The operators do all of this section. Where GMP writes a result into an `rop` you supplied,
Malachite returns it; the `*Assign` traits cover the in-place case; and every operator has
borrowing forms, so `&x + &y` works without cloning either side. GMP's preconditions for this
section also take care of themselves: these functions require canonical operands and promise
canonical results, which in Malachite is the invariant, maintained without being asked.

| | GMP | Malachite |
| :---: | --- | --- |
| ✓ | `void mpq_add (mpq_t sum, const mpq_t addend1, const mpq_t addend2)` | [`Add`](https://doc.rust-lang.org/nightly/std/ops/trait.Add.html), [`AddAssign`](https://doc.rust-lang.org/nightly/std/ops/trait.AddAssign.html) |
| ✓ | `void mpq_sub (mpq_t difference, const mpq_t minuend, const mpq_t subtrahend)` | [`Sub`](https://doc.rust-lang.org/nightly/std/ops/trait.Sub.html), [`SubAssign`](https://doc.rust-lang.org/nightly/std/ops/trait.SubAssign.html) |
| ✓ | `void mpq_mul (mpq_t product, const mpq_t multiplier, const mpq_t multiplicand)` | [`Mul`](https://doc.rust-lang.org/nightly/std/ops/trait.Mul.html), [`MulAssign`](https://doc.rust-lang.org/nightly/std/ops/trait.MulAssign.html) |
| ✓ | `void mpq_mul_2exp (mpq_t rop, const mpq_t op1, mp_bitcnt_t op2)` | [`Shl`](https://doc.rust-lang.org/nightly/std/ops/trait.Shl.html), [`ShlAssign`](https://doc.rust-lang.org/nightly/std/ops/trait.ShlAssign.html) |
| ✓ | `void mpq_div (mpq_t quotient, const mpq_t dividend, const mpq_t divisor)` | [`Div`](https://doc.rust-lang.org/nightly/std/ops/trait.Div.html), [`DivAssign`](https://doc.rust-lang.org/nightly/std/ops/trait.DivAssign.html) |
| ✓ | `void mpq_div_2exp (mpq_t rop, const mpq_t op1, mp_bitcnt_t op2)` | [`Shr`](https://doc.rust-lang.org/nightly/std/ops/trait.Shr.html), [`ShrAssign`](https://doc.rust-lang.org/nightly/std/ops/trait.ShrAssign.html) |
| ✓ | `void mpq_neg (mpq_t negated_operand, const mpq_t operand)` | [`Neg`](https://doc.rust-lang.org/nightly/std/ops/trait.Neg.html), [`NegAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.NegAssign.html) |
| ✓ | `void mpq_abs (mpq_t rop, const mpq_t op)` | [`Abs`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Abs.html), [`AbsAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.AbsAssign.html) |
| ✓ | `void mpq_inv (mpq_t inverted_number, const mpq_t number)` | [`Reciprocal`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Reciprocal.html), [`ReciprocalAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ReciprocalAssign.html) |

**`mpq_mul_2exp`, `mpq_div_2exp`.** `q << k` and `q >> k`. As described under
[Conventions](/mapping/gmp-integers/#conventions), the shift count may be any primitive integer type,
signed or unsigned, with a negative count reversing the direction. One habit from the integer
types can be dropped here: `Natural` and `Integer` floor when shifted right, but a `Rational`
has nowhere to lose bits to, so both directions are exact, just as GMP's `mpq_div_2exp` is.

**`mpq_div`.** `x / y`, with `x /= y` and `x /= &y` for the in-place forms. GMP declares
division by zero undefined; Malachite panics, so the failure is at the division site, not
wherever the undefined behavior eventually surfaces.

**`mpq_neg`, `mpq_abs`.** `-x` and `x.abs()`, with
[`NegAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.NegAssign.html)
and
[`AbsAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.AbsAssign.html)
mutating in place. Both are sign adjustments that touch no limbs.

**`mpq_inv`.** `x.reciprocal()`, or `x.reciprocal_assign()` in place. In both libraries this is
an exchange of numerator and denominator, constant time, with canonical form preserved for free.
The zero case differs in the usual way: GMP's manual warns that inverting a zero "will divide by
zero", while `Rational::reciprocal` panics with the message "Cannot take reciprocal of zero".

## [Comparison Functions](https://gmplib.org/manual/Comparing-Rationals) {#comparison-functions}

| | GMP | Malachite |
| :---: | --- | --- |
| ✓ | `int mpq_cmp (const mpq_t op1, const mpq_t op2)` | [`Ord`](https://doc.rust-lang.org/nightly/std/cmp/trait.Ord.html) |
| ✓ | `int mpq_cmp_z (const mpq_t op1, const mpz_t op2)` | [`PartialOrd`](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialOrd.html) |
| ✓ | `int mpq_cmp_ui (const mpq_t op1, unsigned long int num2, unsigned long int den2)` | [`PartialOrd`](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialOrd.html) |
| ✓ | `int mpq_cmp_si (const mpq_t op1, long int num2, unsigned long int den2)` | [`PartialOrd`](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialOrd.html) |
| ✓ | `int mpq_sgn (const mpq_t op)` | [`Sign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Sign.html) |
| ✓ | `int mpq_equal (const mpq_t op1, const mpq_t op2)` | [`PartialEq`](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialEq.html) |

As on the integer page, comparison is where Malachite provides every combination of types: a
[`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html)
compares against another `Rational`, an
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html), a
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html), any
primitive integer, or any primitive float, in either order. Three discussions there carry over
unchanged: [what the comparison returns](/mapping/gmp-integers/#comparison-functions), an
[`Ordering`](https://doc.rust-lang.org/nightly/std/cmp/enum.Ordering.html) rather than a
sign-carrying `int`, for [`Sign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Sign.html)
too; why single-type rows say `Ord` while mixed-type rows say `PartialOrd`; and the warning
about macros that evaluate their arguments more than once, which covers `mpq_cmp_ui`,
`mpq_cmp_si`, and `mpq_sgn`.

**`mpq_cmp_ui`, `mpq_cmp_si`.** These compare against a fraction, not just an integer. When the
denominator is 1, the mixed comparison is direct: `q < 5u32`. For a general fraction, build it:
`q < Rational::from_unsigneds(3u32, 4u32)`. GMP notes that `num2` and `den2` may have common
factors; `from_unsigneds` removes them first, with a word-sized gcd, where GMP instead
cross-multiplies without reducing.

**Comparing against a float.** GMP's rational chapter has no `mpq_cmp_d`; a C program compares
an `mpq_t` with a `double` by converting one to the other's type first, choosing which end loses
precision. Malachite's `PartialOrd<f64>` for `Rational` is exact, since every finite `double` is
a rational, and a NaN yields
[`None`](https://doc.rust-lang.org/nightly/std/option/enum.Option.html) rather than an undefined
result.

**`mpq_equal`.** GMP provides `mpq_equal` as a separate entry point because it is "much faster"
than `mpq_cmp`: two canonical rationals are equal exactly when their numerators and denominators
match componentwise, no cross-multiplication required, and canonical form is what makes that
shortcut sound. In Malachite the shortcut is the only path: `==` is the derived structural
equality on sign, numerator, and denominator, and the ordering work belongs to
[`Ord`](https://doc.rust-lang.org/nightly/std/cmp/trait.Ord.html) alone. The same invariant makes the derived
[`Hash`](https://doc.rust-lang.org/nightly/std/hash/trait.Hash.html) agree with `==`, so a
`Rational` works as a hash-map key with no extra care, something an `mpq_t` needs a
canonicalization discipline to offer.

The absolute-value comparison family described on the integer page,
[`OrdAbs`](https://docs.rs/malachite-base/latest/malachite_base/num/comparison/traits/trait.OrdAbs.html),
[`PartialOrdAbs`](https://docs.rs/malachite-base/latest/malachite_base/num/comparison/traits/trait.PartialOrdAbs.html),
and
[`EqAbs`](https://docs.rs/malachite-base/latest/malachite_base/num/comparison/traits/trait.EqAbs.html),
extends to `Rational` as well; GMP's rational chapter has no `cmpabs` functions to map onto it.

## [Applying Integer Functions to Rationals](https://gmplib.org/manual/Applying-Integer-Functions) {#applying-integer-functions-to-rationals}

This is the section where the two representations part ways. An `mpq_t` is a pair of `mpz_t`s,
a signed numerator and a positive denominator, and the section's premise is direct access to
those halves so that any integer function can be applied to them, with the manual cautioning
that "care should be taken" to leave the value canonical afterwards. A
[`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html) is a
sign and two [`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html)s, so
the sign is not part of the numerator, and mutable access is mediated to keep canonical form
intact. Reading is as direct as in GMP; writing trades raw references for closures that clean up
after themselves.

| | GMP | Malachite |
| :---: | --- | --- |
| ≈ | `mpz_ptr mpq_numref (const mpq_t op)` | [`numerator_ref`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.numerator_ref), [`mutate_numerator`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.mutate_numerator) |
| ≈ | `mpz_ptr mpq_denref (const mpq_t op)` | [`denominator_ref`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.denominator_ref), [`mutate_denominator`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.mutate_denominator) |
| ≈ | `void mpq_get_num (mpz_t numerator, const mpq_t rational)` | [`to_numerator`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.to_numerator), [`into_numerator`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.into_numerator) |
| ✓ | `void mpq_get_den (mpz_t denominator, const mpq_t rational)` | [`to_denominator`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.to_denominator), [`into_denominator`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.into_denominator) |
| ≈ | `void mpq_set_num (mpq_t rational, const mpz_t numerator)` | [`mutate_numerator`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.mutate_numerator), [`from_sign_and_naturals`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.from_sign_and_naturals) |
| ✓ | `void mpq_set_den (mpq_t rational, const mpz_t denominator)` | [`mutate_denominator`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.mutate_denominator) |

**Where the sign went.** Every `≈` in this section is the same difference: GMP's numerator
carries the sign, and Malachite's accessors deal in magnitudes, with the sign asked for
separately, by comparing with zero or through
[`Sign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Sign.html).
When you need GMP's signed numerator as a value,
`Integer::from_sign_and_abs(q >= 0, q.to_numerator())` builds it, and
[`from_sign_and_abs_ref`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html#method.from_sign_and_abs_ref)
borrows the magnitude rather than consuming it.

**`mpq_numref`, `mpq_denref`, reading.**
[`numerator_ref`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.numerator_ref)
and
[`denominator_ref`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.denominator_ref)
return `&Natural` at no cost, and any
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html) method
applies: `q.denominator_ref().is_power_of_2()`, `q.numerator_ref().significant_bits()`, and so
on, which is the section's premise, unchanged.

**`mpq_numref`, `mpq_denref`, writing; `mpq_set_num`, `mpq_set_den`.** Writing through GMP's
references, or with `mpq_set_num` and `mpq_set_den`, leaves the caller responsible for
canonicalizing. Malachite mediates the same access through closures:
`q.mutate_numerator(|n| ...)` and `q.mutate_denominator(|d| ...)` hand the closure a
`&mut Natural`, and when it returns, the value is reduced to lowest terms, a numerator that
became zero resets the sign to nonnegative, and a denominator that became zero panics. The
manual's "care should be taken" paragraph becomes machinery. For coordinated changes to both
parts there is
[`mutate_numerator_and_denominator`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.mutate_numerator_and_denominator),
which reduces once at the end. A replacement that changes the sign is the one thing a closure
over a magnitude cannot express; for that, rebuild with
`Rational::from_sign_and_naturals(sign, num, den)`, using
[`into_numerator_and_denominator`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.into_numerator_and_denominator)
to recover the current parts without copying them.

**`mpq_get_num`, `mpq_get_den`.** GMP copies the half out; `to_numerator` and `to_denominator`
do the same. Rust adds a third access mode alongside borrowing and copying: `into_numerator` and
`into_denominator` consume the
[`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html) and
move the half out without copying any limbs. `mpq_get_den` maps exactly, since denominators are
positive in both libraries; `mpq_get_num` differs by the sign, as above.

## [Input and Output Functions](https://gmplib.org/manual/I_002fO-of-Rationals) {#input-and-output-functions}

| | GMP | Malachite |
| :---: | --- | --- |
| ✗ | `size_t mpq_out_str (FILE *stream, int base, const mpq_t op)` | [`Display`](https://doc.rust-lang.org/nightly/std/fmt/trait.Display.html) |
| ✗ | `size_t mpq_inp_str (mpq_t rop, FILE *stream, int base)` | [`FromStr`](https://doc.rust-lang.org/nightly/std/str/trait.FromStr.html) |

As on [the integer page](/mapping/gmp-integers/#input-and-output-functions), there is no `FILE *` half to
port: writing is `write!(stream, "{q}")` for any
[`Write`](https://doc.rust-lang.org/nightly/std/io/trait.Write.html), reading means pulling a
token out of your reader and handing it to
[`FromStr`](https://doc.rust-lang.org/nightly/std/str/trait.FromStr.html), and nothing in
Malachite needs to know about streams. What keeps the rows at ✗ is the base: these are
`mpq_get_str` and `mpq_set_str` with a stream attached, and they inherit the gap described under
[Conversion Functions](#conversion-functions), along with the absence of `mpq_inp_str`'s base-0
prefix detection.

GMP's rational chapter has no raw-format I/O, but the serde support described on the integer page
covers `Rational` too: with the `enable_serde` feature it implements
[`Serialize`](https://docs.rs/serde/latest/serde/trait.Serialize.html) and
[`Deserialize`](https://docs.rs/serde/latest/serde/trait.Deserialize.html), encoding a value as a
small struct holding the sign and the two halves in the hexadecimal-string form used for
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html).
Deserialization validates canonical form, rejecting a zero denominator, a negative zero, and a
fraction not in lowest terms. So the invariant this page opened with holds across serialization
as well: code downstream of a `Deserialize` never sees a non-canonical
[`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html).
