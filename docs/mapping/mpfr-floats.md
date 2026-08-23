---
layout: default
title: "Malachite for MPFR Users: Floats"
permalink: /mapping/mpfr-floats/
theme: jekyll-theme-slate
---

# Malachite for MPFR Users: Floats

This page maps the functions of MPFR's floating-point type, `mpfr_t`, onto their Malachite
counterpart:
[`Float`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html), from
the `malachite-float` crate. It follows the organization of the
[MPFR Interface chapter](https://www.mpfr.org/mpfr-current/mpfr.html#MPFR-Interface) of the MPFR
manual, as of MPFR 4.2.2, and the [mapping index](/mapping/) lists the whole family of pages.
The word-type and aliasing conventions of
[the GMP integers page](/mapping/gmp-integers/#conventions) apply here as well.

This page has a different character from the others. GMP and FLINT map onto types that Malachite
designed independently; `Float` was designed to follow MPFR. Its documentation states the
contract outright: `Float` uses "the same representation used by MPFR, and all Malachite
functions produce exactly the same result as their counterparts in MPFR, unless otherwise
noted", and much of its arithmetic is ported directly from MPFR's. The consequence for the
mapping is that agreement here is usually exact, down to the rounding of the last bit and the
sign of a zero, and the notes can spend their time on the exceptions. The other consequence is
that this is the page with the most ✗ rows: `Float` is younger than its model, and the rows
mark, function by function, what remains to be built.

## Conventions {#conventions}

### The representation

An `mpfr_t` and a
[`Float`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html) are
the same mathematical object: a radix-2 floating-point number with a sign, an
arbitrary-precision normalized significand, and a bounded exponent, alongside the three special
values shared with IEEE 754, signed zero, signed infinity, and NaN. The agreement extends to the
fine print. "MPFR has a single kind of NaN and does not have subnormals", and both statements
are true of `Float` as well: one NaN with no payload, and gradual underflow only by explicit
request, which for Malachite means never. Both libraries also place the radix point before the
first digit of the significand, so that, as MPFR's manual notes, "the exponent values in MPFR
and in IEEE 754 differ by 1"; the number 1 has exponent 1 in both MPFR and Malachite. `mpfr_t`
is a one-element array, the same pass-by-reference device as
[every `_t` on these pages](/mapping/flint-integers/#conventions).

Two structural differences are worth knowing from the start. First, in MPFR every variable
carries a precision, including one holding NaN, an infinity, or a zero; in Malachite, only
finite nonzero `Float`s have a precision, significand, or exponent. The rows touching
`mpfr_get_prec` and `mpfr_set_prec` say what this means in practice. Second, the component
types differ in fixity: `mpfr_prec_t` and `mpfr_exp_t` are configuration- and
platform-dependent, while a `Float`'s precision is always a `u64` and its exponent always an
`i32`. A `Float` whose precision is 64 bits or less is stored inline, with no allocated memory,
the same small-value economy as
[the other Malachite types](/mapping/gmp-integers/#allocation).

### One function, a family of spellings

An MPFR function reads its output precision from a variable the caller prepared: `mpfr_log(rop,
op, rnd)` rounds into whatever precision `rop` was given. Malachite returns its results, so the
precision travels as an argument, and each MPFR function corresponds to a family of spellings
built from one base name. The exact counterpart of `mpfr_log` is
[`ln_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.ln_prec_round):
`op.ln_prec_round(prec, rm)` returns the rounded result and the ternary value. From there the
family abbreviates: `ln_prec(prec)` rounds to nearest, `ln_round(rm)` keeps the input's
precision, and the plain trait spelling
[`op.ln()`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Ln.html)
does both. Each has a `_ref` form that borrows instead of consuming, and an `_assign` form that
mutates in place and returns the bare ternary value. The tables below list each row's base name
once and leave the family implied; operations that Rust spells through standard traits, like
the arithmetic operators, are listed as traits, as on the other pages.

### The ternary value

Nearly every MPFR function returns an `int` about which the manual says: "If the ternary value
is zero, it means that the value stored in the destination variable is the exact result of the
corresponding mathematical function", and a positive or negative value means the stored result
is greater or lower than the exact one. This is precisely an
[`Ordering`](https://doc.rust-lang.org/nightly/std/cmp/enum.Ordering.html), and Malachite's
families return one: `Equal` for exact, `Greater` and `Less` for the two inexact directions.
Both libraries are correctly rounding, behaving "as if it computed the result with an infinite
precision, then rounded it to the precision of this variable", and both regard input variables
"as exact (in particular, their precision does not affect the result)".

### Rounding modes

| MPFR | Malachite |
| --- | --- |
| `MPFR_RNDN` (to nearest, ties to even) | [`Nearest`](https://docs.rs/malachite-base/latest/malachite_base/rounding_modes/enum.RoundingMode.html) |
| `MPFR_RNDZ` (toward zero) | [`Down`](https://docs.rs/malachite-base/latest/malachite_base/rounding_modes/enum.RoundingMode.html) |
| `MPFR_RNDU` (toward $$+\infty$$) | [`Ceiling`](https://docs.rs/malachite-base/latest/malachite_base/rounding_modes/enum.RoundingMode.html) |
| `MPFR_RNDD` (toward $$-\infty$$) | [`Floor`](https://docs.rs/malachite-base/latest/malachite_base/rounding_modes/enum.RoundingMode.html) |
| `MPFR_RNDA` (away from zero) | [`Up`](https://docs.rs/malachite-base/latest/malachite_base/rounding_modes/enum.RoundingMode.html) |
| `MPFR_RNDF` (faithful) | |

The five deterministic modes correspond exactly, ties-to-even rule included; only the names'
points of view differ, MPFR describing the direction on the number line and Malachite the
effect on the absolute value. `MPFR_RNDF`, faithful rounding, which MPFR describes as
"currently experimental", has no counterpart. Malachite adds a sixth mode in the other
direction:
[`Exact`](https://docs.rs/malachite-base/latest/malachite_base/rounding_modes/enum.RoundingMode.html)
panics if any rounding would occur, turning what an MPFR caller expresses by checking the
ternary value into a mode of its own.

### Precision

Precision belongs to each value in both libraries, and the floor is the same: "MPFR_PREC_MIN is
equal to 1", and Malachite's `prec` arguments accept any nonzero `u64`, panicking on zero. At
the top there is no practical difference; MPFR's manual warns against approaching
`MPFR_PREC_MAX`, and in both libraries the working limit is memory. What Malachite does not
have is `mpfr_set_default_prec`'s global default precision: there is no implicit precision
anywhere, and every function either takes `prec` explicitly or inherits the precision of its
input. The rows for the default-precision functions are under
[Initialization Functions](#initialization-functions).

### The exponent range

MPFR's default exponent range is $$[-(2^{30}-1), 2^{30}-1]$$, and Malachite's constants
[`Float::MIN_EXPONENT`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#associatedconstant.MIN_EXPONENT)
and
[`Float::MAX_EXPONENT`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#associatedconstant.MAX_EXPONENT)
are those same two numbers. The difference is that MPFR's range is a mutable global, movable
with `mpfr_set_emin` and `mpfr_set_emax` out to roughly $$\pm(2^{62}-1)$$, while Malachite's is
fixed. Against a default-configured MPFR the two libraries overflow to infinity and underflow
to zero at exactly the same boundaries, with the same ternary values; a program that widens
MPFR's range computes values that a `Float` cannot represent, and has no counterpart here. The
functions that manage the range, and the flags raised at its edges, are treated together under
[Exception Related Functions](#exception-related-functions).

### Global state

The exponent range is one instance of a general pattern. MPFR keeps several pieces of global
(in recent versions, thread-local) state: the default precision, the default rounding mode, the
exponent range, the sticky exception flags, and internal caches for constants like π, freed
with `mpfr_free_cache`. Malachite keeps none: precision and rounding mode are arguments,
exactness is a return value, the range is fixed, and constants are computed at the requested
precision on demand. Sections whose subject is that state, chiefly
[Exception Related Functions](#exception-related-functions), are where this design difference
is settled row by row.

### Equality, and equality

`Float`'s `==` behaves like MPFR's `mpfr_equal_p`: NaN is unequal to everything including
itself, positive and negative zero are equal, and precision plays no part. When identity rather
than numeric equality is wanted, Malachite wraps the value in
[`ComparableFloat`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.ComparableFloat.html),
under which NaN equals NaN, the zeros separate, and precision counts; this pairs with MPFR's
`mpfr_total_order_p` in the [Comparison Functions](#comparison-functions) section, where the
differences between the two total orders are spelled out.

### Categories

Each function falls into one of four categories:

| | meaning |
| :---: | --- |
| ✓ | A Malachite function does the same thing. |
| ≈ | A Malachite function serves the same purpose, but its specification differs. The notes say how. |
| — | No counterpart is needed, either because Rust handles it for you or because it is outside Malachite's scope. The notes say which. |
| ✗ | Malachite does not fully support this yet, but will in a future version. |

## [Initialization Functions](https://www.mpfr.org/mpfr-current/mpfr.html#Initialization-Functions) {#initialization-functions}

The manual opens this section with a sentence that has no translation: "An `mpfr_t` object must
be initialized before storing the first value in it." A `Float` begins life holding a value, so
most of these rows tell the familiar story from
[the other pages](/mapping/gmp-integers/#initializing-integers), with one new warning about a
name that means different things in the two libraries.

| | MPFR | Malachite |
| :---: | --- | --- |
| ≈ | `void mpfr_init2 (mpfr_t x, mpfr_prec_t prec)` | [`Float::NAN`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.NaN.html) |
| — | `void mpfr_inits2 (mpfr_prec_t prec, mpfr_t x, ...)` | |
| — | `void mpfr_clear (mpfr_t x)` | |
| — | `void mpfr_clears (mpfr_t x, ...)` | |
| ✓ | `void mpfr_init (mpfr_t x)` | [`Float::NAN`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.NaN.html) |
| — | `void mpfr_inits (mpfr_t x, ...)` | |
| — | `MPFR_DECL_INIT (name, prec)` | |
| — | `void mpfr_set_default_prec (mpfr_prec_t prec)` | |
| — | `mpfr_prec_t mpfr_get_default_prec (void)` | |
| — | `void mpfr_set_prec (mpfr_t x, mpfr_prec_t prec)` | |
| ≈ | `mpfr_prec_t mpfr_get_prec (mpfr_t x)` | [`get_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.get_prec) |

**`mpfr_init`, `mpfr_init2`, and the clearing family.** `let x = Float::NAN;` produces the
value that `mpfr_init` produces, and dropping replaces `mpfr_clear`, as
[usual](/mapping/gmp-integers/#initializing-integers). The ≈ on `mpfr_init2` is its first half:
"set its precision to be exactly `prec` bits and its value to NaN" assigns a precision to the
NaN, and a Malachite NaN [carries no precision](#conventions), nor is there significand storage
to size in advance, since every result manages its own. The variadic `mpfr_inits2`, `mpfr_clears`,
and `mpfr_inits`, and the stack-allocating `MPFR_DECL_INIT` macro, are C conveniences with
nothing left to abbreviate in a `let`.

**`mpfr_set_default_prec`, `mpfr_get_default_prec`.** The global default precision, 53 bits
initially and thread-local in thread-safe builds, is a piece of state
[Malachite does not have](#conventions): precision is always explicit, either an argument or
the precision of an input. MPFR's own manual describes the hazard of the global, warning that
"some other libraries might change the default precision and not restore it. Thus it is safer
to use `mpfr_init2`"; in Malachite there is no default to change, and nothing to restore.

**`mpfr_set_prec`.** A storage-management operation: reset the precision "and set its value to
NaN. The previous value stored in `x` is lost", equivalent to clear-then-init but reusing the
allocation. Nothing corresponds, because destinations are not prepared in advance; spelled
literally it is `x = Float::NAN`. The warning is the name:
[`Float::set_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.set_prec)
is not this function. Malachite's `set_prec` keeps the value, rounding it to the new precision
and returning the ternary value, which is the behavior of MPFR's `mpfr_prec_round`, treated
with the other precision-changing functions under
[Rounding-Related Functions](#rounding-related-functions).

**`mpfr_get_prec`.** [`get_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.get_prec)
returns "the number of bits used to store its significand" as an
[`Option`](https://doc.rust-lang.org/nightly/std/option/enum.Option.html): `Some` for a finite
nonzero value, and `None` for a NaN, an infinity, or a zero, where `mpfr_get_prec` reports the
precision the variable carries. This is the
[structural difference](#conventions) from the top of the page paying its first visit to a
table.

## [Assignment Functions](https://www.mpfr.org/mpfr-current/mpfr.html#Assignment-Functions) {#assignment-functions}

"These functions assign new values to already initialized floats." In Malachite there is no
initialized-but-empty state to fill, so assignment and construction are the same act, and every
`mpfr_set_*` maps to a constructor: `from_TYPE_prec_round(op, prec, rm)` returns the rounded
[`Float`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html) and
the ternary value, with the usual family of spellings around it, including plain `From`
conversions that are exact. The generic constructors accept every width of primitive, so the
`_ui`/`_uj` and `_si`/`_sj` distinctions collapse.

| | MPFR | Malachite |
| :---: | --- | --- |
| ✓ | `int mpfr_set (mpfr_t rop, mpfr_t op, mpfr_rnd_t rnd)` | [`from_float_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.from_float_prec_round), [`Clone`](https://doc.rust-lang.org/nightly/std/clone/trait.Clone.html) |
| ✓ | `int mpfr_set_ui (mpfr_t rop, unsigned long int op, mpfr_rnd_t rnd)` | [`from_unsigned_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.from_unsigned_prec_round) |
| ✓ | `int mpfr_set_si (mpfr_t rop, long int op, mpfr_rnd_t rnd)` | [`from_signed_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.from_signed_prec_round) |
| ✓ | `int mpfr_set_uj (mpfr_t rop, uintmax_t op, mpfr_rnd_t rnd)` | [`from_unsigned_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.from_unsigned_prec_round) |
| ✓ | `int mpfr_set_sj (mpfr_t rop, intmax_t op, mpfr_rnd_t rnd)` | [`from_signed_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.from_signed_prec_round) |
| ✓ | `int mpfr_set_flt (mpfr_t rop, float op, mpfr_rnd_t rnd)` | [`from_primitive_float_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.from_primitive_float_prec_round) |
| ✓ | `int mpfr_set_d (mpfr_t rop, double op, mpfr_rnd_t rnd)` | [`from_primitive_float_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.from_primitive_float_prec_round) |
| — | `int mpfr_set_ld (mpfr_t rop, long double op, mpfr_rnd_t rnd)` | |
| — | `int mpfr_set_float128 (mpfr_t rop, mpfr_float128 op, mpfr_rnd_t rnd)` | |
| — | `int mpfr_set_decimal64 (mpfr_t rop, _Decimal64 op, mpfr_rnd_t rnd)` | |
| — | `int mpfr_set_decimal128 (mpfr_t rop, _Decimal128 op, mpfr_rnd_t rnd)` | |
| ✓ | `int mpfr_set_z (mpfr_t rop, mpz_t op, mpfr_rnd_t rnd)` | [`from_integer_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.from_integer_prec_round) |
| ✓ | `int mpfr_set_q (mpfr_t rop, mpq_t op, mpfr_rnd_t rnd)` | [`from_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.from_rational_prec_round) |
| — | `int mpfr_set_f (mpfr_t rop, mpf_t op, mpfr_rnd_t rnd)` | |
| ✓ | `int mpfr_set_ui_2exp (mpfr_t rop, unsigned long int op, mpfr_exp_t e, mpfr_rnd_t rnd)` | [`IntegerMantissaAndExponent`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.IntegerMantissaAndExponent.html) |
| ✓ | `int mpfr_set_si_2exp (mpfr_t rop, long int op, mpfr_exp_t e, mpfr_rnd_t rnd)` | [`IntegerMantissaAndExponent`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.IntegerMantissaAndExponent.html) |
| ✓ | `int mpfr_set_uj_2exp (mpfr_t rop, uintmax_t op, intmax_t e, mpfr_rnd_t rnd)` | [`IntegerMantissaAndExponent`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.IntegerMantissaAndExponent.html) |
| ✓ | `int mpfr_set_sj_2exp (mpfr_t rop, intmax_t op, intmax_t e, mpfr_rnd_t rnd)` | [`IntegerMantissaAndExponent`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.IntegerMantissaAndExponent.html) |
| ✓ | `int mpfr_set_z_2exp (mpfr_t rop, mpz_t op, mpfr_exp_t e, mpfr_rnd_t rnd)` | [`IntegerMantissaAndExponent`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.IntegerMantissaAndExponent.html) |
| ✓ | `int mpfr_set_str (mpfr_t rop, const char *s, int base, mpfr_rnd_t rnd)` | [`set_str`](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/string/strtofr/fn.set_str.html) |
| ✓ | `int mpfr_strtofr (mpfr_t rop, const char *nptr, char **endptr, int base, mpfr_rnd_t rnd)` | [`strtofr`](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/string/strtofr/fn.strtofr.html) |
| ✓ | `void mpfr_set_nan (mpfr_t x)` | [`Float::NAN`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.NaN.html) |
| ✓ | `void mpfr_set_inf (mpfr_t x, int sign)` | [`Float::INFINITY`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.Infinity.html), [`Float::NEGATIVE_INFINITY`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.NegativeInfinity.html) |
| ✓ | `void mpfr_set_zero (mpfr_t x, int sign)` | [`Float::ZERO`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.Zero.html), [`Float::NEGATIVE_ZERO`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.NegativeZero.html) |
| — | `void mpfr_swap (mpfr_t x, mpfr_t y)` | |

**The `mpfr_set` family.** Each source type has its constructor family, and the semantics carry
over to the letter: the input is "regarded as exact", the result is rounded once to `prec`, and
a zero input becomes $$+0$$ "regardless of the rounding mode" in both libraries. For `mpfr_set`
itself, `Float::from_float_prec_round(op, prec, rm)` is the precision-changing copy, and a copy
that keeps the precision is `Clone`; MPFR's remark that the sign of a NaN is propagated has
nothing to attach to here, there being only one NaN. The integer constructors are generic, so
`u64` covers `mpfr_set_uj` and `u128` goes beyond it, and `from_natural_prec_round` exists
alongside `from_integer_prec_round` for
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html)s. The
manual's advice about decimal constants transfers unchanged: just as it recommends `mpfr_set_str`
over `mpfr_set_d`, so that a constant is not "first converted into a reduced-precision (e.g.,
53-bit) binary" value, `Float::from(0.1f64)` inherits the `double`'s rounding error, while
parsing `"0.1"` at the target precision gives the correctly rounded constant.

**The types Rust does not have.** `long double`, `mpfr_float128`, `_Decimal64`, and
`_Decimal128` have no Rust counterparts, so their four rows are outside Malachite's scope; MPFR
itself treats most of them as optional, built only under configure options. `mpfr_set_f` is the
[GMP boundary](/mapping/flint-integers/#conversion) again, in `mpf_t` form; the whole of GMP's
float type is taken up under [Compatibility With MPF](#compatibility-with-mpf) at the end of
this page.

**The `_2exp` constructors.** $$op \cdot 2^e$$ is assembled exactly by
`Float::from_integer_mantissa_and_exponent(m, e)`, from the
[`IntegerMantissaAndExponent`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.IntegerMantissaAndExponent.html)
trait, with a word-sized `op` lifted by `Natural::from` and a negative one handled by negating
the result; [`set_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.set_prec_round)
then supplies the rounding half of the MPFR semantics.

**`mpfr_set_str`, `mpfr_strtofr`.** The parsing engine is a port of MPFR's, and the two entry
points keep MPFR's names.
[`strtofr`](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/string/strtofr/fn.strtofr.html)`(s, base, prec, rm)`
reads the longest valid prefix and returns the value, the ternary value, and the number of
bytes consumed, which is `endptr` as a count; when nothing parses, both libraries return zero,
MPFR's doc saying "for consistency with `strtod`". The full grammar carries over: bases 2
through 62 with base 0 detecting `0b` and `0x` prefixes, `e`/`p`/`@` exponent prefixes, and the
`nan`, `inf`, and `@nan@`-style specials with their base-16 cutoff.
[`set_str`](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/string/strtofr/fn.set_str.html)
is the whole-string version, and its return conventions differ in Malachite's usual direction:
MPFR's `mpfr_set_str` returns `0` or `-1` and directs "users interested in the ternary value"
to `mpfr_strtofr`, while Malachite's returns
[`Option`](https://doc.rust-lang.org/nightly/std/option/enum.Option.html)`<(Float, Ordering)>`,
keeping the ternary value in the success case. These two functions speak MPFR's grammar;
Malachite's own string conversions, [`FromStr`](https://doc.rust-lang.org/nightly/std/str/trait.FromStr.html)
and the `from_sci_string` family, speak Malachite's, and the grammars differ in corners such as
the spelling of specials in high bases.

**`mpfr_set_nan`, `mpfr_set_inf`, `mpfr_set_zero`.** The five constants: `Float::NAN`, and the
signed pairs `Float::INFINITY`/`Float::NEGATIVE_INFINITY` and
`Float::ZERO`/`Float::NEGATIVE_ZERO`, the `sign` argument becoming a choice of constant. The
manual leaves `mpfr_set_nan`'s sign bit unspecified, which the single NaN settles by having
none.

**`mpfr_swap`.** [`std::mem::swap`](https://doc.rust-lang.org/nightly/std/mem/fn.swap.html),
which exchanges the values "without rounding" as `mpfr_swap` does; precisions travel with the
values, and the warning about `MPFR_DECL_INIT`-allocated variables has no analogue, there being
no allocation methods to mix.

## [Combined Initialization and Assignment Functions](https://www.mpfr.org/mpfr-current/mpfr.html#Combined-Initialization-and-Assignment-Functions) {#combined-initialization-and-assignment-functions}

As on [the GMP pages](/mapping/gmp-integers/#combined-initialization-and-assignment), combining
initialization with assignment is not a convenience in Rust but the only thing a binding can
do, so this section maps onto [the previous one](#assignment-functions). The first eight
entries are macros, and what they add to `mpfr_set_*`, taking the precision "from the active
default precision", is the global that Malachite does not have; the precision is passed
explicitly instead.

| | MPFR | Malachite |
| :---: | --- | --- |
| ✓ | `int mpfr_init_set (mpfr_t rop, mpfr_t op, mpfr_rnd_t rnd)` | [`from_float_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.from_float_prec_round), [`Clone`](https://doc.rust-lang.org/nightly/std/clone/trait.Clone.html) |
| ✓ | `int mpfr_init_set_ui (mpfr_t rop, unsigned long int op, mpfr_rnd_t rnd)` | [`from_unsigned_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.from_unsigned_prec_round) |
| ✓ | `int mpfr_init_set_si (mpfr_t rop, long int op, mpfr_rnd_t rnd)` | [`from_signed_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.from_signed_prec_round) |
| ✓ | `int mpfr_init_set_d (mpfr_t rop, double op, mpfr_rnd_t rnd)` | [`from_primitive_float_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.from_primitive_float_prec_round) |
| — | `int mpfr_init_set_ld (mpfr_t rop, long double op, mpfr_rnd_t rnd)` | |
| ✓ | `int mpfr_init_set_z (mpfr_t rop, mpz_t op, mpfr_rnd_t rnd)` | [`from_integer_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.from_integer_prec_round) |
| ✓ | `int mpfr_init_set_q (mpfr_t rop, mpq_t op, mpfr_rnd_t rnd)` | [`from_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.from_rational_prec_round) |
| — | `int mpfr_init_set_f (mpfr_t rop, mpf_t op, mpfr_rnd_t rnd)` | |
| ✓ | `int mpfr_init_set_str (mpfr_t x, const char *s, int base, mpfr_rnd_t rnd)` | [`set_str`](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/string/strtofr/fn.set_str.html) |

Each row behaves as its `mpfr_set_*` counterpart above does, including the two — rows, absent
for the same reasons as there.

## [Conversion Functions](https://www.mpfr.org/mpfr-current/mpfr.html#Conversion-Functions) {#conversion-functions}

The outward direction. One theme runs through this section: where a conversion loses
information or has no reasonable answer, MPFR reports through the sticky *inexact* and *erange*
flags, and Malachite reports in the return value, with an
[`Ordering`](https://doc.rust-lang.org/nightly/std/cmp/enum.Ordering.html) for rounding, an
[`Option`](https://doc.rust-lang.org/nightly/std/option/enum.Option.html) or
[`Result`](https://doc.rust-lang.org/nightly/std/result/enum.Result.html) for failure, or a
panic where the caller asked for something the value cannot do. The notes call out each case.

| | MPFR | Malachite |
| :---: | --- | --- |
| ✓ | `float mpfr_get_flt (mpfr_t op, mpfr_rnd_t rnd)` | [`RoundingFrom`](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/primitive_float_from_float/index.html) |
| ✓ | `double mpfr_get_d (mpfr_t op, mpfr_rnd_t rnd)` | [`RoundingFrom`](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/primitive_float_from_float/index.html) |
| — | `long double mpfr_get_ld (mpfr_t op, mpfr_rnd_t rnd)` | |
| — | `mpfr_float128 mpfr_get_float128 (mpfr_t op, mpfr_rnd_t rnd)` | |
| — | `_Decimal64 mpfr_get_decimal64 (mpfr_t op, mpfr_rnd_t rnd)` | |
| — | `_Decimal128 mpfr_get_decimal128 (mpfr_t op, mpfr_rnd_t rnd)` | |
| ≈ | `long int mpfr_get_si (mpfr_t op, mpfr_rnd_t rnd)` | [`RoundingFrom`](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/primitive_int_from_float/index.html), [`TryFrom`](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/primitive_int_from_float/index.html) |
| ≈ | `unsigned long int mpfr_get_ui (mpfr_t op, mpfr_rnd_t rnd)` | [`RoundingFrom`](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/primitive_int_from_float/index.html), [`TryFrom`](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/primitive_int_from_float/index.html) |
| ≈ | `intmax_t mpfr_get_sj (mpfr_t op, mpfr_rnd_t rnd)` | [`RoundingFrom`](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/primitive_int_from_float/index.html) |
| ≈ | `uintmax_t mpfr_get_uj (mpfr_t op, mpfr_rnd_t rnd)` | [`RoundingFrom`](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/primitive_int_from_float/index.html) |
| ≈ | `double mpfr_get_d_2exp (long *exp, mpfr_t op, mpfr_rnd_t rnd)` | [`sci_mantissa_and_exponent_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sci_mantissa_and_exponent_round) |
| — | `long double mpfr_get_ld_2exp (long *exp, mpfr_t op, mpfr_rnd_t rnd)` | |
| ≈ | `int mpfr_frexp (mpfr_exp_t *exp, mpfr_t y, mpfr_t x, mpfr_rnd_t rnd)` | [`SciMantissaAndExponent`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.SciMantissaAndExponent.html) |
| ≈ | `mpfr_exp_t mpfr_get_z_2exp (mpz_t rop, mpfr_t op)` | [`IntegerMantissaAndExponent`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.IntegerMantissaAndExponent.html) |
| ≈ | `int mpfr_get_z (mpz_t rop, mpfr_t op, mpfr_rnd_t rnd)` | [`RoundingFrom`](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/integer_from_float/index.html), [`TryFrom`](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/integer_from_float/index.html) |
| ✓ | `void mpfr_get_q (mpq_t rop, mpfr_t op)` | [`TryFrom`](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/rational_from_float/index.html) |
| — | `int mpfr_get_f (mpf_t rop, mpfr_t op, mpfr_rnd_t rnd)` | |
| ✓ | `size_t mpfr_get_str_ndigits (int b, mpfr_prec_t p)` | [`get_str_digit_count`](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/string/get_str/fn.get_str_digit_count.html) |
| ✓ | `char * mpfr_get_str (char *str, mpfr_exp_t *expptr, int base, size_t n, mpfr_t op, mpfr_rnd_t rnd)` | [`get_str`](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/string/get_str/fn.get_str.html) |
| — | `void mpfr_free_str (char *str)` | |
| ≈ | `int mpfr_fits_ulong_p (mpfr_t op, mpfr_rnd_t rnd)` | [`ConvertibleFrom`](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/primitive_int_from_float/index.html) |
| ≈ | `int mpfr_fits_slong_p (mpfr_t op, mpfr_rnd_t rnd)` | [`ConvertibleFrom`](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/primitive_int_from_float/index.html) |
| ≈ | `int mpfr_fits_uint_p (mpfr_t op, mpfr_rnd_t rnd)` | [`ConvertibleFrom`](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/primitive_int_from_float/index.html) |
| ≈ | `int mpfr_fits_sint_p (mpfr_t op, mpfr_rnd_t rnd)` | [`ConvertibleFrom`](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/primitive_int_from_float/index.html) |
| ≈ | `int mpfr_fits_ushort_p (mpfr_t op, mpfr_rnd_t rnd)` | [`ConvertibleFrom`](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/primitive_int_from_float/index.html) |
| ≈ | `int mpfr_fits_sshort_p (mpfr_t op, mpfr_rnd_t rnd)` | [`ConvertibleFrom`](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/primitive_int_from_float/index.html) |
| ≈ | `int mpfr_fits_uintmax_p (mpfr_t op, mpfr_rnd_t rnd)` | [`ConvertibleFrom`](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/primitive_int_from_float/index.html) |
| ≈ | `int mpfr_fits_intmax_p (mpfr_t op, mpfr_rnd_t rnd)` | [`ConvertibleFrom`](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/primitive_int_from_float/index.html) |

**`mpfr_get_flt`, `mpfr_get_d`, and the types Rust does not have.**
`f32::rounding_from(&op, rnd)` and `f64::rounding_from(&op, rnd)` return the rounded value
together with the ternary value, which `mpfr_get_d`'s bare `double` return cannot carry. The
special values travel intact, `f64` having all of them: NaN to NaN, infinities with their
signs, and zero signs preserved, which MPFR promises only "if possible". `long double`,
`mpfr_float128`, and the decimal types are absent from Rust, as under
[Assignment](#assignment-functions).

**The integer conversions.** `u64::rounding_from(&op, rnd)` and its signed and shorter kin
round to an integer and return the ternary value, with `TryFrom` for the exact-only case; every
width is covered, so the `_ui`/`_uj` split collapses again. The ≈ is what happens at the edges.
MPFR never fails: a NaN becomes 0, an out-of-range value "returns the maximum or the minimum of
the corresponding C type, depending on the direction of the overflow", and the *erange* flag
records the event. Malachite saturates only when the rounding mode points toward the violated
bound, and panics otherwise, NaN included: the flag becomes either a defined answer or a loud
failure, never a quiet wrong-looking one.

**`mpfr_get_d_2exp`, `mpfr_frexp`.** Both split a value into mantissa and exponent with the
mantissa in $$[0.5, 1)$$, and here the exponent-convention offset from
[Conventions](#conventions) reappears in mantissa form: Malachite's
[`sci_mantissa_and_exponent_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sci_mantissa_and_exponent_round)
produces a mantissa in $$[1, 2)$$, IEEE 754 style, so MPFR's $$(d, e)$$ is Malachite's
$$(m/2, e+1)$$. The `f64`-mantissa form with a rounding mode answers `mpfr_get_d_2exp`; for
`mpfr_frexp`, whose mantissa is a rounded `Float`, the
[`SciMantissaAndExponent`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.SciMantissaAndExponent.html)
implementation with `Float` mantissas provides the split, and
[`from_float_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.from_float_prec_round)
the rounding to `y`'s precision.

**`mpfr_get_z_2exp`.** The exact decomposition $$op = m \cdot 2^e$$, with a difference in
which $$m$$ is chosen: MPFR returns the scaled significand "regarded as an integer, with the
precision of `op`", trailing zeros included, while
[`integer_mantissa_and_exponent`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.IntegerMantissaAndExponent.html)
returns the minimal representative, an odd $$m$$. The equation is the same and the pairs
convert by shifting; code that relies on `rop` having exactly `prec` bits notices the
difference.

**`mpfr_get_z`, `mpfr_get_q`.** `Integer::rounding_from(&op, rnd)` returns precisely the
ternary value that `mpfr_get_z` returns, with
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html)
versions alongside; for both functions a NaN or infinity is a panic or an `Err` where MPFR sets
*erange* and stores 0. `mpfr_get_q` "is always exact", and so is `Rational::try_from(&op)`,
[the same pairing seen from the other side](/mapping/gmp-rationals/#conversion-functions) on
the GMP rationals page.

**`mpfr_get_str_ndigits`, `mpfr_get_str`, `mpfr_free_str`.**
[`get_str`](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/string/get_str/fn.get_str.html)
is a port of `mpfr_get_str` and keeps its name and its whole specification: bases 2 to 62 and
the negative range $$-2$$ to $$-36$$ for upper-case digits, kept by MPFR "for compatibility
with GMP's `mpf_get_str` function"; the digit string as a fraction with an implicit radix point
and a separate exponent; `@NaN@` and `@Inf@`; the ties-to-even rule applied to significands
even in odd bases; and `n = 0` requesting the round-trip digit count. It returns
`Option<(Vec<u8>, i64, Ordering)>`, `None` standing in for the null-pointer return on an
invalid base, and the ternary value coming along where MPFR only sets the inexact flag.
`mpfr_get_str_ndigits`, the round-trip digit count
$$m = 1 + \lceil p \log 2 / \log b \rceil$$ itself, is
[`get_str_digit_count`](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/string/get_str/fn.get_str_digit_count.html),
with the abbreviation spelled out. `mpfr_free_str` is the usual absence of
manual deallocation.

**The `fits` family.** The eight C types collapse into the generic
[`ConvertibleFrom`](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/primitive_int_from_float/index.html)
implementations, with one specification difference: MPFR asks whether `op` fits "when rounded
to an integer in the direction `rnd`", so that, as the manual notes, $$-0.5$$ fits an
`unsigned long` under `MPFR_RNDU`, while `u64::convertible_from(&op)` asks whether the value is
exactly representable, with no rounding step. To ask MPFR's question, round first:
`Integer::rounding_from(&op, rnd)` never fails on finite input, and the integer's fit is then
checked exactly. Check the exponent against the type's width before rounding, though:
`mpfr_fits_ulong_p` answers from the exponent in constant time, while the rounding step
materializes the integer part, which is large when the exponent is; only a value within one
step of a boundary of the type's range needs it.

## [Arithmetic Functions](https://www.mpfr.org/mpfr-current/mpfr.html#Arithmetic-Functions) {#arithmetic-functions}

The operators carry this section: `x + y` through `x / y` with the `*Assign` and borrowing
forms, and behind each operator the full family, `add_prec_round`, `sub_prec_round`,
`mul_prec_round`, and `div_prec_round`, whose members return the ternary value. The IEEE 754
special-value rules, signed zeros included, hold on both sides down to the last case, since the
arithmetic is ported. For the mixed-operand variants one recipe covers everything:
`Float::from(c)` is exact for every scalar these functions take, an integer of any width, a
`double`, or an
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html), so
composing the conversion with the base operation reproduces the mixed function exactly,
reversed forms like `mpfr_ui_sub` included.
[`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html)
operands do not even need the recipe: the operators take them directly, through families like
`add_rational_prec_round` and, for the reversed division, `rational_div_float_prec_round`.

| | MPFR | Malachite |
| :---: | --- | --- |
| ✓ | `int mpfr_add (mpfr_t rop, mpfr_t op1, mpfr_t op2, mpfr_rnd_t rnd)` | [`Add`](https://doc.rust-lang.org/nightly/std/ops/trait.Add.html), [`AddAssign`](https://doc.rust-lang.org/nightly/std/ops/trait.AddAssign.html) |
| ✓ | `int mpfr_add_ui (mpfr_t rop, mpfr_t op1, unsigned long int op2, mpfr_rnd_t rnd)` | [`Add`](https://doc.rust-lang.org/nightly/std/ops/trait.Add.html) |
| ✓ | `int mpfr_add_si (mpfr_t rop, mpfr_t op1, long int op2, mpfr_rnd_t rnd)` | [`Add`](https://doc.rust-lang.org/nightly/std/ops/trait.Add.html) |
| ✓ | `int mpfr_add_d (mpfr_t rop, mpfr_t op1, double op2, mpfr_rnd_t rnd)` | [`Add`](https://doc.rust-lang.org/nightly/std/ops/trait.Add.html) |
| ✓ | `int mpfr_add_z (mpfr_t rop, mpfr_t op1, mpz_t op2, mpfr_rnd_t rnd)` | [`Add`](https://doc.rust-lang.org/nightly/std/ops/trait.Add.html) |
| ✓ | `int mpfr_add_q (mpfr_t rop, mpfr_t op1, mpq_t op2, mpfr_rnd_t rnd)` | [`Add`](https://doc.rust-lang.org/nightly/std/ops/trait.Add.html) |
| ✓ | `int mpfr_sub (mpfr_t rop, mpfr_t op1, mpfr_t op2, mpfr_rnd_t rnd)` | [`Sub`](https://doc.rust-lang.org/nightly/std/ops/trait.Sub.html), [`SubAssign`](https://doc.rust-lang.org/nightly/std/ops/trait.SubAssign.html) |
| ✓ | `int mpfr_ui_sub (mpfr_t rop, unsigned long int op1, mpfr_t op2, mpfr_rnd_t rnd)` | [`Sub`](https://doc.rust-lang.org/nightly/std/ops/trait.Sub.html) |
| ✓ | `int mpfr_sub_ui (mpfr_t rop, mpfr_t op1, unsigned long int op2, mpfr_rnd_t rnd)` | [`Sub`](https://doc.rust-lang.org/nightly/std/ops/trait.Sub.html) |
| ✓ | `int mpfr_si_sub (mpfr_t rop, long int op1, mpfr_t op2, mpfr_rnd_t rnd)` | [`Sub`](https://doc.rust-lang.org/nightly/std/ops/trait.Sub.html) |
| ✓ | `int mpfr_sub_si (mpfr_t rop, mpfr_t op1, long int op2, mpfr_rnd_t rnd)` | [`Sub`](https://doc.rust-lang.org/nightly/std/ops/trait.Sub.html) |
| ✓ | `int mpfr_d_sub (mpfr_t rop, double op1, mpfr_t op2, mpfr_rnd_t rnd)` | [`Sub`](https://doc.rust-lang.org/nightly/std/ops/trait.Sub.html) |
| ✓ | `int mpfr_sub_d (mpfr_t rop, mpfr_t op1, double op2, mpfr_rnd_t rnd)` | [`Sub`](https://doc.rust-lang.org/nightly/std/ops/trait.Sub.html) |
| ✓ | `int mpfr_z_sub (mpfr_t rop, mpz_t op1, mpfr_t op2, mpfr_rnd_t rnd)` | [`Sub`](https://doc.rust-lang.org/nightly/std/ops/trait.Sub.html) |
| ✓ | `int mpfr_sub_z (mpfr_t rop, mpfr_t op1, mpz_t op2, mpfr_rnd_t rnd)` | [`Sub`](https://doc.rust-lang.org/nightly/std/ops/trait.Sub.html) |
| ✓ | `int mpfr_sub_q (mpfr_t rop, mpfr_t op1, mpq_t op2, mpfr_rnd_t rnd)` | [`Sub`](https://doc.rust-lang.org/nightly/std/ops/trait.Sub.html) |
| ✓ | `int mpfr_mul (mpfr_t rop, mpfr_t op1, mpfr_t op2, mpfr_rnd_t rnd)` | [`Mul`](https://doc.rust-lang.org/nightly/std/ops/trait.Mul.html), [`MulAssign`](https://doc.rust-lang.org/nightly/std/ops/trait.MulAssign.html) |
| ✓ | `int mpfr_mul_ui (mpfr_t rop, mpfr_t op1, unsigned long int op2, mpfr_rnd_t rnd)` | [`Mul`](https://doc.rust-lang.org/nightly/std/ops/trait.Mul.html) |
| ✓ | `int mpfr_mul_si (mpfr_t rop, mpfr_t op1, long int op2, mpfr_rnd_t rnd)` | [`Mul`](https://doc.rust-lang.org/nightly/std/ops/trait.Mul.html) |
| ✓ | `int mpfr_mul_d (mpfr_t rop, mpfr_t op1, double op2, mpfr_rnd_t rnd)` | [`Mul`](https://doc.rust-lang.org/nightly/std/ops/trait.Mul.html) |
| ✓ | `int mpfr_mul_z (mpfr_t rop, mpfr_t op1, mpz_t op2, mpfr_rnd_t rnd)` | [`Mul`](https://doc.rust-lang.org/nightly/std/ops/trait.Mul.html) |
| ✓ | `int mpfr_mul_q (mpfr_t rop, mpfr_t op1, mpq_t op2, mpfr_rnd_t rnd)` | [`Mul`](https://doc.rust-lang.org/nightly/std/ops/trait.Mul.html) |
| ✓ | `int mpfr_sqr (mpfr_t rop, mpfr_t op, mpfr_rnd_t rnd)` | [`Square`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Square.html), [`SquareAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.SquareAssign.html) |
| ✓ | `int mpfr_div (mpfr_t rop, mpfr_t op1, mpfr_t op2, mpfr_rnd_t rnd)` | [`Div`](https://doc.rust-lang.org/nightly/std/ops/trait.Div.html), [`DivAssign`](https://doc.rust-lang.org/nightly/std/ops/trait.DivAssign.html) |
| ✓ | `int mpfr_ui_div (mpfr_t rop, unsigned long int op1, mpfr_t op2, mpfr_rnd_t rnd)` | [`Div`](https://doc.rust-lang.org/nightly/std/ops/trait.Div.html), [`Reciprocal`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Reciprocal.html) |
| ✓ | `int mpfr_div_ui (mpfr_t rop, mpfr_t op1, unsigned long int op2, mpfr_rnd_t rnd)` | [`Div`](https://doc.rust-lang.org/nightly/std/ops/trait.Div.html) |
| ✓ | `int mpfr_si_div (mpfr_t rop, long int op1, mpfr_t op2, mpfr_rnd_t rnd)` | [`Div`](https://doc.rust-lang.org/nightly/std/ops/trait.Div.html) |
| ✓ | `int mpfr_div_si (mpfr_t rop, mpfr_t op1, long int op2, mpfr_rnd_t rnd)` | [`Div`](https://doc.rust-lang.org/nightly/std/ops/trait.Div.html) |
| ✓ | `int mpfr_d_div (mpfr_t rop, double op1, mpfr_t op2, mpfr_rnd_t rnd)` | [`Div`](https://doc.rust-lang.org/nightly/std/ops/trait.Div.html) |
| ✓ | `int mpfr_div_d (mpfr_t rop, mpfr_t op1, double op2, mpfr_rnd_t rnd)` | [`Div`](https://doc.rust-lang.org/nightly/std/ops/trait.Div.html) |
| ✓ | `int mpfr_div_z (mpfr_t rop, mpfr_t op1, mpz_t op2, mpfr_rnd_t rnd)` | [`Div`](https://doc.rust-lang.org/nightly/std/ops/trait.Div.html) |
| ✓ | `int mpfr_div_q (mpfr_t rop, mpfr_t op1, mpq_t op2, mpfr_rnd_t rnd)` | [`Div`](https://doc.rust-lang.org/nightly/std/ops/trait.Div.html) |
| ✓ | `int mpfr_sqrt (mpfr_t rop, mpfr_t op, mpfr_rnd_t rnd)` | [`Sqrt`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Sqrt.html), [`SqrtAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.SqrtAssign.html) |
| ✓ | `int mpfr_sqrt_ui (mpfr_t rop, unsigned long int op, mpfr_rnd_t rnd)` | [`sqrt_unsigned_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.sqrt_unsigned_prec_round) |
| ✓ | `int mpfr_rec_sqrt (mpfr_t rop, mpfr_t op, mpfr_rnd_t rnd)` | [`ReciprocalSqrt`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ReciprocalSqrt.html) |
| ✓ | `int mpfr_cbrt (mpfr_t rop, mpfr_t op, mpfr_rnd_t rnd)` | [`Cbrt`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Cbrt.html), [`CbrtAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CbrtAssign.html) |
| ✓ | `int mpfr_rootn_ui (mpfr_t rop, mpfr_t op, unsigned long int n, mpfr_rnd_t rnd)` | [`root_u_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.root_u_prec_round), [`Root`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Root.html) |
| ✓ | `int mpfr_rootn_si (mpfr_t rop, mpfr_t op, long int n, mpfr_rnd_t rnd)` | [`root_s_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.root_s_prec_round), [`Root`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Root.html) |
| — | `int mpfr_root (mpfr_t rop, mpfr_t op, unsigned long int n, mpfr_rnd_t rnd)` | |
| ✓ | `int mpfr_neg (mpfr_t rop, mpfr_t op, mpfr_rnd_t rnd)` | [`Neg`](https://doc.rust-lang.org/nightly/std/ops/trait.Neg.html), [`NegAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.NegAssign.html) |
| ✓ | `int mpfr_abs (mpfr_t rop, mpfr_t op, mpfr_rnd_t rnd)` | [`Abs`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Abs.html), [`AbsAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.AbsAssign.html) |
| ✓ | `int mpfr_dim (mpfr_t rop, mpfr_t op1, mpfr_t op2, mpfr_rnd_t rnd)` | [`positive_difference_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/struct.Float.html#method.positive_difference_prec_round) |
| ✓ | `int mpfr_mul_2ui (mpfr_t rop, mpfr_t op1, unsigned long int op2, mpfr_rnd_t rnd)` | [`Shl`](https://doc.rust-lang.org/nightly/std/ops/trait.Shl.html), [`shl_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.shl_prec_round) |
| ✓ | `int mpfr_mul_2si (mpfr_t rop, mpfr_t op1, long int op2, mpfr_rnd_t rnd)` | [`Shl`](https://doc.rust-lang.org/nightly/std/ops/trait.Shl.html), [`shl_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.shl_prec_round) |
| ✓ | `int mpfr_div_2ui (mpfr_t rop, mpfr_t op1, unsigned long int op2, mpfr_rnd_t rnd)` | [`Shr`](https://doc.rust-lang.org/nightly/std/ops/trait.Shr.html), [`shr_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.shr_prec_round) |
| ✓ | `int mpfr_div_2si (mpfr_t rop, mpfr_t op1, long int op2, mpfr_rnd_t rnd)` | [`Shr`](https://doc.rust-lang.org/nightly/std/ops/trait.Shr.html), [`shr_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.shr_prec_round) |
| ✓ | `int mpfr_fac_ui (mpfr_t rop, unsigned long int op, mpfr_rnd_t rnd)` | [`factorial_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/struct.Float.html#method.factorial_prec_round) |
| ✓ | `int mpfr_fma (mpfr_t rop, mpfr_t op1, mpfr_t op2, mpfr_t op3, mpfr_rnd_t rnd)` | [`add_mul_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/struct.Float.html#method.add_mul_prec_round) |
| ✓ | `int mpfr_fms (mpfr_t rop, mpfr_t op1, mpfr_t op2, mpfr_t op3, mpfr_rnd_t rnd)` | [`sub_mul_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/struct.Float.html#method.sub_mul_prec_round) |
| ✓ | `int mpfr_fmma (mpfr_t rop, mpfr_t op1, mpfr_t op2, mpfr_t op3, mpfr_t op4, mpfr_rnd_t rnd)` | [`mul_add_mul_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/struct.Float.html#method.mul_add_mul_prec_round) |
| ✓ | `int mpfr_fmms (mpfr_t rop, mpfr_t op1, mpfr_t op2, mpfr_t op3, mpfr_t op4, mpfr_rnd_t rnd)` | [`mul_sub_mul_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/struct.Float.html#method.mul_sub_mul_prec_round) |
| ✓ | `int mpfr_hypot (mpfr_t rop, mpfr_t x, mpfr_t y, mpfr_rnd_t rnd)` | [`hypot_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/struct.Float.html#method.hypot_prec_round) |
| ✓ | `int mpfr_sum (mpfr_t rop, const mpfr_ptr tab[], unsigned long int n, mpfr_rnd_t rnd)` | [`sum_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/struct.Float.html#method.sum_prec_round) |
| ✓ | `int mpfr_dot (mpfr_t rop, const mpfr_ptr a[], const mpfr_ptr b[], unsigned long int n, mpfr_rnd_t rnd)` | [`dot_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/struct.Float.html#method.dot_prec_round) |

**The operators and mixed forms.** Thirty-two of these rows are the four operations across
their operand types, and the section intro's recipe disposes of them; `mpfr_add_q` and its kin
are direct `Add<Rational>`-style implementations rather than conversions. Two of MPFR's own
remarks transfer directly: `mpfr_sqr` exists because squaring is faster than `mpfr_mul` on
equal operands, and `x.square()` earns its keep the same way; and `mpfr_ui_div` with `op1 = 1`
is the reciprocal, which Malachite spells
[`reciprocal_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.reciprocal_prec_round).
Division by zero gives a signed infinity on both sides, with no exception beyond MPFR's
divide-by-zero flag.

**The roots.** `sqrt` follows IEEE 754, taking $$\sqrt{-0}$$ to $$-0$$ and negative values to
NaN, in both libraries; `sqrt_unsigned_prec_round` is the ported `mpfr_sqrt_ui`, the square
root of a `u64`. `mpfr_rec_sqrt` is
[`ReciprocalSqrt`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ReciprocalSqrt.html),
and MPFR's warning carries over exactly: both libraries take $$\pm 0$$ to $$+\infty$$, which
"is different from the one of the rSqrt function recommended by the IEEE 754 standard".
`mpfr_rootn_ui` and `mpfr_rootn_si` are `root_u_prec_round` and `root_s_prec_round`, with the
plain trait spelling `x.root(n)`, and the IEEE rootn sign rules, odd roots of negatives
negative, even roots NaN, zeros by the limit rules, agree throughout. The deprecated
`mpfr_root`, which differs from `mpfr_rootn_ui` only on $$-0$$ with even `n` and "will be
removed in a future release", is left to its replacements.

**`mpfr_neg`, `mpfr_abs`.** Sign operations that, in MPFR, can also round, when `rop` has a
smaller precision than `op`. Malachite's `-x` and `x.abs()` keep the precision exactly; when
the precision must change too, compose with
[`from_float_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.from_float_prec_round).
The remark that the sign rule "also applies to NaN in order to mimic the IEEE 754" behavior is
vacuous here, the single NaN being signless.

**`mpfr_dim`.** The positive difference, $$op1 - op2$$ if $$op1 > op2$$, $$+0$$ if
$$op1 \le op2$$, and NaN if either operand is NaN, is the
[`positive_difference_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/struct.Float.html#method.positive_difference_prec_round)
family, which follows the usual Malachite conventions for precision and rounding-mode
shorthands. The zero for $$op1 \le op2$$ is a definition choice rather than saturation —
negative differences are representable, and the function returns $$+0$$ anyway — so the family
is named for the operation rather than for a saturating subtraction. Beyond MPFR, the family
also has mixed `Float`-`Rational` forms in both argument orders
([`positive_difference_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/struct.Float.html#method.positive_difference_rational_prec_round)
and
[`rational_positive_difference_float_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/struct.Float.html#method.rational_positive_difference_float_prec_round)),
in which the `Rational` operand enters the comparison and the subtraction exactly.

**The `2exp` family.** `x << k` and `x >> k` are exact multiplications and divisions by
$$2^k$$, adjusting the exponent, and both directions accept signed and unsigned shift amounts
[as elsewhere in Malachite](/mapping/gmp-integers/#arithmetic-functions). The rounding mode in
`mpfr_mul_2ui`'s signature matters only when the shifted value leaves the exponent range, and
for that case the full forms
[`shl_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.shl_prec_round)
and
[`shr_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.shr_prec_round)
carry the mode and return the ternary value; the plain operators round to nearest at the
boundary.

**`mpfr_fac_ui`.** The rounded factorial is
[`factorial_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/struct.Float.html#method.factorial_prec_round)
(with a `factorial_prec` shorthand for rounding to nearest), producing exactly the value of
`Float::from_natural_prec_round(Natural::factorial(op), prec, rnd)` — the factorial is exact
in [`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html) —
but working at a precision near the target throughout, as `mpfr_fac_ui` does, which is far
cheaper than computing every bit of the exact factorial when `prec` is small and `op` is
large. A factorial too large for the exponent range overflows to infinity or to the largest
representable value, depending on the rounding mode.

**The fused operations.** `mpfr_fma` and `mpfr_fms`, the singly rounded
$$op1 \cdot op2 \pm op3$$, are the
[`add_mul_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/struct.Float.html#method.add_mul_prec_round)
and
[`sub_mul_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/struct.Float.html#method.sub_mul_prec_round)
families, with the usual precision and rounding-mode shorthands and with the
[`AddMul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.AddMul.html)
and
[`SubMul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.SubMul.html)
traits at the plain level. The argument order follows those traits rather than MPFR: the addend
comes first, so `mpfr_fma(rop, x, y, z, rnd)` is `z.add_mul_prec_round(x, y, prec, rm)`.
Malachite's `sub_mul` subtracts the product — $$op3 - op1 \cdot op2$$ — while `mpfr_fms`
subtracts the addend; the two are exact negations of each other, so
`mpfr_fms(rop, x, y, z, rnd)` is `-z.sub_mul_prec_round(x, y, prec, -rm)` with the ternary
reversed. The separately rounded spelling `&a * &b + &c` computes a different, twice-rounded
value, which is exactly what fusing exists to avoid.

Beyond MPFR, the families also come in mixed `Float`-`Rational` forms
([`add_mul_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/struct.Float.html#method.add_mul_rational_prec_round)
and
[`sub_mul_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/struct.Float.html#method.sub_mul_rational_prec_round)),
in which a `Rational` multiplicand enters the product exactly; rounding it to a `Float` first
would perturb the result by the other multiplicand times the conversion error. A `Rational`
*addend* is not offered, and neither is a product of two `Rational`s — the latter because
`Rational` multiplication is exact, so composing it with the mixed addition forms already
yields a single rounding.

`mpfr_fmma` and `mpfr_fmms`, the singly rounded $$op1 \cdot op2 \pm op3 \cdot op4$$, are the
[`mul_add_mul_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/struct.Float.html#method.mul_add_mul_prec_round)
and
[`mul_sub_mul_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/struct.Float.html#method.mul_sub_mul_prec_round)
families, with the
[`MulAddMul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.MulAddMul.html)
and
[`MulSubMul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.MulSubMul.html)
traits at the plain level; here the argument orders agree exactly, with no reordering or sign
adjustment. The exact counterparts of the double-product pair on
[the FLINT integers page](/mapping/flint-integers/#basic-arithmetic) fuse for the temporary,
while these fuse for the rounding. Beyond MPFR, these families also come in mixed
`Float`-`Rational` forms
([`mul_add_mul_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/struct.Float.html#method.mul_add_mul_rational_prec_round)
and
[`mul_sub_mul_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/struct.Float.html#method.mul_sub_mul_rational_prec_round)),
in which a `Rational` in the second product enters exactly; since each product's factors
commute and the products of the additive form commute with each other, this one placement
covers every one-`Rational` variant of the sum, and the subtractive variants follow by exact
negation.

**`mpfr_hypot`.** The Euclidean norm $$\sqrt{x^2 + y^2}$$ is ported: `hypot_prec_round`, with
the C99 special-value rule that an infinite operand gives $$+\infty$$ "even if the other number
is NaN", along with the usual `prec`, `round`, and plain levels, in-place `*_assign` forms, the
new `Hypot` and `HypotAssign` traits, and `primitive_float_hypot`, which computes a correctly
rounded `f32` or `f64` hypotenuse with a single rounding, unlike the standard library's `hypot`.
The MPFR source notes (in a FIXME) that the scaled second operand can underflow when the target
precision is very large relative to the exponent range; in that regime Malachite computes the
sum of squares exactly at the integer level, where no exponent range applies, so every input has
a correctly rounded result. The same integer-level path decides the `Exact` rounding mode, which
succeeds precisely on Pythagorean inputs whose hypotenuse fits in the target precision.

**`mpfr_sum`, `mpfr_dot`.** Correctly rounded sums and dot products of arrays, with a single
rounding regardless of length.
[`sum_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/struct.Float.html#method.sum_prec_round)
and its variants
([`sum_prec`](https://docs.rs/malachite-float/latest/malachite_float/struct.Float.html#method.sum_prec),
[`sum_round`](https://docs.rs/malachite-float/latest/malachite_float/struct.Float.html#method.sum_round),
and the [`Sum`](https://doc.rust-lang.org/nightly/core/iter/trait.Sum.html) implementations,
which use the maximum input precision and round to nearest) cover `mpfr_sum`, including its
zero-sign conventions.
[`dot_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/struct.Float.html#method.dot_prec_round)
and its variants
([`dot_prec`](https://docs.rs/malachite-float/latest/malachite_float/struct.Float.html#method.dot_prec),
[`dot_round`](https://docs.rs/malachite-float/latest/malachite_float/struct.Float.html#method.dot_round),
and [`dot`](https://docs.rs/malachite-float/latest/malachite_float/struct.Float.html#method.dot))
cover `mpfr_dot`, and go further on one axis: MPFR notes that `mpfr_dot` is experimental and
"does not yet handle intermediate overflows and underflows" — it computes each product at full
precision and requires those multiplications to be exact, so inputs whose products leave the
exponent range are not handled. Malachite computes the products exactly at the significand
level, tracking their exponents over a range twice as wide as a `Float`'s, so intermediate
overflow and underflow cannot occur; only the final sum is subject to the exponent range
check.

Malachite also provides correctly-rounded *products* of arrays, again with a single rounding
regardless of length —
[`product_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/struct.Float.html#method.product_prec_round),
[`product_prec`](https://docs.rs/malachite-float/latest/malachite_float/struct.Float.html#method.product_prec),
[`product_round`](https://docs.rs/malachite-float/latest/malachite_float/struct.Float.html#method.product_round),
and the [`Product`](https://doc.rust-lang.org/nightly/core/iter/trait.Product.html)
implementations (maximum input precision, rounding to nearest). MPFR has no corresponding
function. Since a product cannot cancel, intermediate exactness is unnecessary: the
implementation multiplies truncated significands at a small working precision, tracks the
exponent exactly on the side, and rounds once, so intermediate overflow and underflow cannot
occur either.

## [Comparison Functions](https://www.mpfr.org/mpfr-current/mpfr.html#Comparison-Functions) {#comparison-functions}

As on every page of this family, comparison is where Malachite is at its most complete: the
three-way comparisons and the IEEE predicates both exist for every operand type, so MPFR's
closing remark for this section, that the predicates compare only floats and "you may need to
do a conversion first", has nothing to attach to.

| | MPFR | Malachite |
| :---: | --- | --- |
| ✓ | `int mpfr_cmp (mpfr_t op1, mpfr_t op2)` | [`PartialOrd`](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialOrd.html) |
| ✓ | `int mpfr_cmp_ui (mpfr_t op1, unsigned long int op2)` | [`PartialOrd`](https://docs.rs/malachite-float/latest/malachite_float/float/comparison/partial_cmp_primitive_int/index.html) |
| ✓ | `int mpfr_cmp_si (mpfr_t op1, long int op2)` | [`PartialOrd`](https://docs.rs/malachite-float/latest/malachite_float/float/comparison/partial_cmp_primitive_int/index.html) |
| ✓ | `int mpfr_cmp_d (mpfr_t op1, double op2)` | [`PartialOrd`](https://docs.rs/malachite-float/latest/malachite_float/float/comparison/partial_cmp_primitive_float/index.html) |
| — | `int mpfr_cmp_ld (mpfr_t op1, long double op2)` | |
| ✓ | `int mpfr_cmp_z (mpfr_t op1, mpz_t op2)` | [`PartialOrd`](https://docs.rs/malachite-float/latest/malachite_float/float/comparison/partial_cmp_integer/index.html) |
| ✓ | `int mpfr_cmp_q (mpfr_t op1, mpq_t op2)` | [`PartialOrd`](https://docs.rs/malachite-float/latest/malachite_float/float/comparison/partial_cmp_rational/index.html) |
| — | `int mpfr_cmp_f (mpfr_t op1, mpf_t op2)` | |
| ✓ | `int mpfr_cmp_ui_2exp (mpfr_t op1, unsigned long int op2, mpfr_exp_t e)` | [`PartialOrd`](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialOrd.html) |
| ✓ | `int mpfr_cmp_si_2exp (mpfr_t op1, long int op2, mpfr_exp_t e)` | [`PartialOrd`](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialOrd.html) |
| ✓ | `int mpfr_cmpabs (mpfr_t op1, mpfr_t op2)` | [`PartialOrdAbs`](https://docs.rs/malachite-base/latest/malachite_base/num/comparison/traits/trait.PartialOrdAbs.html) |
| ✓ | `int mpfr_cmpabs_ui (mpfr_t op1, unsigned long int op2)` | [`PartialOrdAbs`](https://docs.rs/malachite-float/latest/malachite_float/float/comparison/partial_cmp_abs_primitive_int/index.html) |
| ✓ | `int mpfr_nan_p (mpfr_t op)` | [`is_nan`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.is_nan) |
| ✓ | `int mpfr_inf_p (mpfr_t op)` | [`is_infinite`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.is_infinite) |
| ✓ | `int mpfr_number_p (mpfr_t op)` | [`is_finite`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.is_finite) |
| ✓ | `int mpfr_zero_p (mpfr_t op)` | [`is_zero`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.is_zero) |
| ✓ | `int mpfr_regular_p (mpfr_t op)` | [`is_normal`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.is_normal) |
| ✓ | `int mpfr_sgn (mpfr_t op)` | [`PartialOrd`](https://docs.rs/malachite-float/latest/malachite_float/float/comparison/partial_cmp_primitive_int/index.html) |
| ✓ | `int mpfr_greater_p (mpfr_t op1, mpfr_t op2)` | [`PartialOrd`](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialOrd.html) |
| ✓ | `int mpfr_greaterequal_p (mpfr_t op1, mpfr_t op2)` | [`PartialOrd`](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialOrd.html) |
| ✓ | `int mpfr_less_p (mpfr_t op1, mpfr_t op2)` | [`PartialOrd`](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialOrd.html) |
| ✓ | `int mpfr_lessequal_p (mpfr_t op1, mpfr_t op2)` | [`PartialOrd`](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialOrd.html) |
| ✓ | `int mpfr_equal_p (mpfr_t op1, mpfr_t op2)` | [`PartialEq`](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialEq.html) |
| ✓ | `int mpfr_lessgreater_p (mpfr_t op1, mpfr_t op2)` | [`PartialOrd`](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialOrd.html) |
| ✓ | `int mpfr_unordered_p (mpfr_t op1, mpfr_t op2)` | [`PartialOrd`](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialOrd.html) |
| ≈ | `int mpfr_total_order_p (mpfr_t x, mpfr_t y)` | [`ComparableFloat`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.ComparableFloat.html) |

**The `mpfr_cmp` family.** `x.partial_cmp(&y)` compares the values exactly, each "considered
to their full own precision", and where `mpfr_cmp` handles NaN by "set the *erange* flag and
return zero", `partial_cmp` returns `None`, which has the small advantage of being
distinguishable from `Some(Equal)` without consulting anything. The mixed comparisons are all
direct implementations, against integers of every width, `f64`, an
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html), a
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html), or a
[`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html),
always exact. For the `_2exp` pair, shift the operand into place first: `Float::from(op2) << e`
is exact and costs only exponent arithmetic while `e` stays inside the exponent range, and
outside it the comparison is settled by sign and magnitude before any shift is needed.
`mpfr_cmpabs` and `mpfr_cmpabs_ui` are
[`PartialOrdAbs`](https://docs.rs/malachite-base/latest/malachite_base/num/comparison/traits/trait.PartialOrdAbs.html),
`x.partial_cmp_abs(&y)`, comparing magnitudes without materializing either absolute value and
answering NaN with `None` in the same way.

**The classification predicates.** Five names, one vocabulary shift: `is_nan`, `is_infinite`,
`is_finite` for `mpfr_number_p`, `is_zero`, and `is_normal` for `mpfr_regular_p`. The last pair
agree exactly because neither library has subnormals: normal means finite and nonzero on both
sides.

**`mpfr_sgn`.** MPFR defines it as "equivalent to `mpfr_cmp_ui (op, 0)`, but more efficient",
and the mapping takes the definition at its word: `x.partial_cmp(&0u32)`, with either zero
giving `Some(Equal)` and NaN giving `None` in place of the *erange* flag. A warning about a
similar name: `Float`'s
[`Sign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Sign.html)
implementation, `x.sign()`, is not `mpfr_sgn` but the sign bit: it never returns `Equal`,
sends $$-0$$ to `Less` and $$+0$$ to `Greater`, and panics on NaN, which makes it the
counterpart of `mpfr_signbit`, under
[Miscellaneous Functions](#miscellaneous-functions).

**The IEEE predicates.** The five predicate functions are Rust's comparison operators, exactly:
`>`, `>=`, `<`, `<=`, and `==` on a partially ordered type "return zero whenever `op1` and/or
`op2` is NaN", which is precisely how Rust defines the operators through
[`PartialOrd`](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialOrd.html). The two
combined predicates are one-liners over `partial_cmp`: `mpfr_lessgreater_p` is
`x < y || x > y`, and `mpfr_unordered_p` is `x.partial_cmp(&y).is_none()`.

**`mpfr_total_order_p`.** Both libraries offer a total order over all floats, and the two
orders differ in instructive ways.
[`ComparableFloat`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.ComparableFloat.html)
implements [`Ord`](https://doc.rust-lang.org/nightly/std/cmp/trait.Ord.html), so
`ComparableFloat(x) <= ComparableFloat(y)` plays the predicate's role, with the full
[`Ordering`](https://doc.rust-lang.org/nightly/std/cmp/enum.Ordering.html) available where
`mpfr_total_order_p` "returns a binary value". The differences: IEEE 754's totalOrder places
the two signed NaNs at the ends, $$-\mathrm{NaN}$$ below $$-\infty$$ and $$+\mathrm{NaN}$$
above $$+\infty$$, and MPFR notes that "the sign bit of NaN also matters", while Malachite's
single NaN sits between $$-0$$ and $$+0$$; and IEEE's order ignores precision, while
`ComparableFloat` refines equal values by it, ordering the greater precision further from
zero, the same refinement its `Eq` and `Hash` use. On NaN-free data of uniform precision the
two orders agree exactly.

## [Transcendental Functions](https://www.mpfr.org/mpfr-current/mpfr.html#Transcendental-Functions) {#transcendental-functions}

The largest section of the manual, and the one where the state of Malachite's port is most
visible: the logarithm, exponential, and power families are complete, while the trigonometric,
hyperbolic, and special functions are the frontier, mapped below as gaps. Everything here
returns the ternary value, and MPFR's warning about cost applies to both libraries: "in some
domains, computing transcendental functions (even more with correct rounding) is expensive,
even in small precision".

One note on names before the tables, since this section is where the two naming traditions
diverge most. MPFR keeps C's mathematical abbreviations, `log`, `exp2`, `expm1`, `log1p`;
Malachite spells the operations out: the natural logarithm is `ln`, $$2^x$$ is `power_of_2`,
and the shifted variants append `_x_minus_1` or insert `_1_plus_x` where C appends `m1` or
`p1`. The tables give the dictionary row by row.

### Logarithms and exponentials

| | MPFR | Malachite |
| :---: | --- | --- |
| ✓ | `int mpfr_log (mpfr_t rop, mpfr_t op, mpfr_rnd_t rnd)` | [`ln_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.ln_prec_round), [`Ln`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Ln.html) |
| ✓ | `int mpfr_log_ui (mpfr_t rop, unsigned long int op, mpfr_rnd_t rnd)` | [`ln_unsigned_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.ln_unsigned_prec_round) |
| ✓ | `int mpfr_log2 (mpfr_t rop, mpfr_t op, mpfr_rnd_t rnd)` | [`log_base_2_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.log_base_2_prec_round), [`LogBase2`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.LogBase2.html) |
| ✓ | `int mpfr_log10 (mpfr_t rop, mpfr_t op, mpfr_rnd_t rnd)` | [`log_base_10_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.log_base_10_prec_round), [`LogBase10`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.LogBase10.html) |
| ✓ | `int mpfr_log1p (mpfr_t rop, mpfr_t op, mpfr_rnd_t rnd)` | [`ln_1_plus_x_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.ln_1_plus_x_prec_round), [`Ln1PlusX`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Ln1PlusX.html) |
| ✓ | `int mpfr_log2p1 (mpfr_t rop, mpfr_t op, mpfr_rnd_t rnd)` | [`log_base_2_1_plus_x_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.log_base_2_1_plus_x_prec_round), [`LogBase2Of1PlusX`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.LogBase2Of1PlusX.html) |
| ✓ | `int mpfr_log10p1 (mpfr_t rop, mpfr_t op, mpfr_rnd_t rnd)` | [`log_base_10_1_plus_x_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.log_base_10_1_plus_x_prec_round), [`LogBase10Of1PlusX`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.LogBase10Of1PlusX.html) |
| ✓ | `int mpfr_exp (mpfr_t rop, mpfr_t op, mpfr_rnd_t rnd)` | [`exp_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.exp_prec_round), [`Exp`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Exp.html) |
| ✓ | `int mpfr_exp2 (mpfr_t rop, mpfr_t op, mpfr_rnd_t rnd)` | [`power_of_2_of_float_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.power_of_2_of_float_prec_round), [`PowerOf2`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.PowerOf2.html) |
| ✓ | `int mpfr_exp10 (mpfr_t rop, mpfr_t op, mpfr_rnd_t rnd)` | [`power_of_10_of_float_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.power_of_10_of_float_prec_round), [`PowerOf10`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.PowerOf10.html) |
| ✓ | `int mpfr_expm1 (mpfr_t rop, mpfr_t op, mpfr_rnd_t rnd)` | [`exp_x_minus_1_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.exp_x_minus_1_prec_round), [`ExpXMinus1`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ExpXMinus1.html) |
| ✓ | `int mpfr_exp2m1 (mpfr_t rop, mpfr_t op, mpfr_rnd_t rnd)` | [`power_of_2_x_minus_1_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.power_of_2_x_minus_1_prec_round), [`PowerOf2XMinus1`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.PowerOf2XMinus1.html) |
| ✓ | `int mpfr_exp10m1 (mpfr_t rop, mpfr_t op, mpfr_rnd_t rnd)` | [`power_of_10_x_minus_1_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.power_of_10_x_minus_1_prec_round), [`PowerOf10XMinus1`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.PowerOf10XMinus1.html) |

**The logarithms.** All four are ports, `ln_unsigned_prec_round` being the binary-splitting
`mpfr_log_ui`, the logarithm of a `u64`. The special-value rules carry over exactly: the
logarithm of 1 is $$+0$$ "in all rounding modes", the logarithm of $$\pm 0$$ is $$-\infty$$
where "the sign of the zero has no influence on the result", and negative arguments give NaN.

**The shifted logarithms.** `ln_1_plus_x`, `log_base_2_1_plus_x`, and `log_base_10_1_plus_x`
compute $$\log_b(1+x)$$ without forming $$1+x$$, the standard device for keeping accuracy when
$$x$$ is near zero, and take $$-1$$ to $$-\infty$$ as their MPFR counterparts do.

**The exponentials.** `exp` is `Exp`; $$2^x$$ and $$10^x$$ are `PowerOf2` and `PowerOf10`
applied to a `Float` exponent, the `_of_float` in the full method names distinguishing them
from the exact `power_of_2(u64)` constructor. The `m1` trio, $$b^x - 1$$ accurately near zero,
follows the same dictionary with `_x_minus_1`.

**Exact rational arguments.** Every function in this table also has a `_rational` companion,
`ln_rational_prec_round`, `power_of_2_rational_prec_round`, and so on, which evaluates the
function of an exact
[`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html)
argument with a single rounding. MPFR has no counterpart; the nearest spelling there converts
the rational to a float first and accepts the intermediate rounding.

### Powers

| | MPFR | Malachite |
| :---: | --- | --- |
| ✓ | `int mpfr_pow (mpfr_t rop, mpfr_t op1, mpfr_t op2, mpfr_rnd_t rnd)` | [`pow_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.pow_prec_round), [`Pow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Pow.html) |
| ✓ | `int mpfr_powr (mpfr_t rop, mpfr_t op1, mpfr_t op2, mpfr_rnd_t rnd)` | [`powr_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.powr_prec_round) |
| ✓ | `int mpfr_pow_ui (mpfr_t rop, mpfr_t op1, unsigned long int op2, mpfr_rnd_t rnd)` | [`pow_u_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.pow_u_prec_round), [`Pow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Pow.html) |
| ✓ | `int mpfr_pow_si (mpfr_t rop, mpfr_t op1, long int op2, mpfr_rnd_t rnd)` | [`pow_s_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.pow_s_prec_round), [`Pow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Pow.html) |
| ✓ | `int mpfr_pow_uj (mpfr_t rop, mpfr_t op1, uintmax_t op2, mpfr_rnd_t rnd)` | [`pow_u_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.pow_u_prec_round) |
| ✓ | `int mpfr_pow_sj (mpfr_t rop, mpfr_t op1, intmax_t op2, mpfr_rnd_t rnd)` | [`pow_s_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.pow_s_prec_round) |
| ✓ | `int mpfr_pown (mpfr_t rop, mpfr_t op1, intmax_t op2, mpfr_rnd_t rnd)` | [`pow_s_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.pow_s_prec_round) |
| ✓ | `int mpfr_pow_z (mpfr_t rop, mpfr_t op1, mpz_t op2, mpfr_rnd_t rnd)` | [`pow_integer_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.pow_integer_prec_round), [`Pow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Pow.html) |
| ✓ | `int mpfr_ui_pow_ui (mpfr_t rop, unsigned long int op1, unsigned long int op2, mpfr_rnd_t rnd)` | [`unsigned_pow_unsigned_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.unsigned_pow_unsigned_prec_round) |
| ✓ | `int mpfr_ui_pow (mpfr_t rop, unsigned long int op1, mpfr_t op2, mpfr_rnd_t rnd)` | [`unsigned_pow_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.unsigned_pow_prec_round) |
| ✗ | `int mpfr_compound_si (mpfr_t rop, mpfr_t op, long int n, mpfr_rnd_t rnd)` | |

**`mpfr_pow`.** `pow_prec_round` and the
[`Pow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Pow.html)
trait, a port. The fifteen ISO C99 special-value rules the manual enumerates hold verbatim,
including the two that surprise, `pow(+1, y)` returning 1 "for any `y`, even a NaN", and
`pow(x, ±0)` likewise; and both libraries detect exactly representable powers up front, so
exact cases return `Equal` rather than looping toward an unreachable rounding decision.

**`mpfr_powr`.** IEEE 754's other power, "the exponential of `op2` multiplied by the logarithm
of `op1`": restricted to nonnegative bases, with NaN where `pow`'s rules would conjure a value
from a negative base, and agreeing with `pow` wherever both are defined. `powr_prec_round`
heads a family of inherent methods; there is no trait spelling, the `Pow` name being taken by
`pow`'s semantics.

**The integer exponents.** `pow_u_prec_round` and `pow_s_prec_round` take any unsigned or
signed width, collapsing `pow_ui`/`pow_uj` and `pow_si`/`pow_sj`; `mpfr_pown` collapses with
them, being MPFR's own alias for `mpfr_pow_sj` "to follow the C2x function `pown`". A full
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html)
exponent goes to `pow_integer_prec_round`. All are reachable through `Pow` as well, with
`u64`, `i64`, and `Integer` exponents.

**The `u64` bases.** `mpfr_ui_pow_ui` and `mpfr_ui_pow` map to
`unsigned_pow_unsigned_prec_round` and `unsigned_pow_prec_round` directly, integer-based powers
computed at the target precision rather than exactly.

**`mpfr_compound_si`.** $$(1+op)^n$$, IEEE 754's compound-interest function, accurate without
forming $$1+op$$, is a gap. The exact interim leaves the field as usual, and pays for it:
`Rational::from(op) + 1` is exact, its `n`th power is exact, and
[`from_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.from_rational_prec_round)
rounds once, but the exact power grows with `n` where a dedicated implementation works at the
target precision throughout.

**Beyond MPFR.** The power lattice fills in combinations MPFR does not have: a
[`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html)
exponent on a `Float` base (`pow_rational_prec_round`, with exact algebraic cases like
rational powers of perfect powers detected and returned exactly), a `Rational` base under a
`Float` exponent (`rational_pow_prec_round`), both at once (`rational_pow_rational_prec_round`),
and a `u64` base under a `Rational` exponent (`unsigned_pow_rational_prec_round`).

### Trigonometric functions

| | MPFR | Malachite |
| :---: | --- | --- |
| ✗ | `int mpfr_cos (mpfr_t rop, mpfr_t op, mpfr_rnd_t rnd)` | |
| ✗ | `int mpfr_sin (mpfr_t rop, mpfr_t op, mpfr_rnd_t rnd)` | |
| ✗ | `int mpfr_tan (mpfr_t rop, mpfr_t op, mpfr_rnd_t rnd)` | |
| ✗ | `int mpfr_cosu (mpfr_t rop, mpfr_t op, unsigned long int u, mpfr_rnd_t rnd)` | |
| ✗ | `int mpfr_sinu (mpfr_t rop, mpfr_t op, unsigned long int u, mpfr_rnd_t rnd)` | |
| ✗ | `int mpfr_tanu (mpfr_t rop, mpfr_t op, unsigned long int u, mpfr_rnd_t rnd)` | |
| ✗ | `int mpfr_cospi (mpfr_t rop, mpfr_t op, mpfr_rnd_t rnd)` | |
| ✗ | `int mpfr_sinpi (mpfr_t rop, mpfr_t op, mpfr_rnd_t rnd)` | |
| ✗ | `int mpfr_tanpi (mpfr_t rop, mpfr_t op, mpfr_rnd_t rnd)` | |
| ✗ | `int mpfr_sin_cos (mpfr_t sop, mpfr_t cop, mpfr_t op, mpfr_rnd_t rnd)` | |
| ✗ | `int mpfr_sec (mpfr_t rop, mpfr_t op, mpfr_rnd_t rnd)` | |
| ✗ | `int mpfr_csc (mpfr_t rop, mpfr_t op, mpfr_rnd_t rnd)` | |
| ✗ | `int mpfr_cot (mpfr_t rop, mpfr_t op, mpfr_rnd_t rnd)` | |
| ✗ | `int mpfr_acos (mpfr_t rop, mpfr_t op, mpfr_rnd_t rnd)` | |
| ✗ | `int mpfr_asin (mpfr_t rop, mpfr_t op, mpfr_rnd_t rnd)` | |
| ✗ | `int mpfr_atan (mpfr_t rop, mpfr_t op, mpfr_rnd_t rnd)` | |
| ✗ | `int mpfr_acosu (mpfr_t rop, mpfr_t op, unsigned long int u, mpfr_rnd_t rnd)` | |
| ✗ | `int mpfr_asinu (mpfr_t rop, mpfr_t op, unsigned long int u, mpfr_rnd_t rnd)` | |
| ✗ | `int mpfr_atanu (mpfr_t rop, mpfr_t op, unsigned long int u, mpfr_rnd_t rnd)` | |
| ✗ | `int mpfr_acospi (mpfr_t rop, mpfr_t op, mpfr_rnd_t rnd)` | |
| ✗ | `int mpfr_asinpi (mpfr_t rop, mpfr_t op, mpfr_rnd_t rnd)` | |
| ✗ | `int mpfr_atanpi (mpfr_t rop, mpfr_t op, mpfr_rnd_t rnd)` | |
| ✗ | `int mpfr_atan2 (mpfr_t rop, mpfr_t y, mpfr_t x, mpfr_rnd_t rnd)` | |
| ✗ | `int mpfr_atan2u (mpfr_t rop, mpfr_t y, mpfr_t x, unsigned long int u, mpfr_rnd_t rnd)` | |
| ✗ | `int mpfr_atan2pi (mpfr_t rop, mpfr_t y, mpfr_t x, mpfr_rnd_t rnd)` | |

**The family.** Twenty-five rows, one gap: trigonometry is the next port after the exponential
and power families, and none of it exists in Malachite yet, with no interim spelling to offer,
correct rounding not being something a detour through `f64` can provide. This is also the
family MPFR's cost warning names first: for large arguments the work is dominated by argument
reduction, dividing out a $$2\pi$$ known to enough precision, and that cost is intrinsic to
the specification.

**The `u` and `pi` variants.** MPFR measures angles three ways: in radians, in turns scaled by
`u` (`mpfr_sinu` computes the sine of $$op \times 2\pi/u$$, so `u = 360` "gets the... sine...
for `op` in degrees"), and in half-turns (`mpfr_sinpi` and kin; `mpfr_atan2pi` "is the same as
`mpfr_atan2u` with `u = 2`"). The scaled variants carry IEEE 754 exactness contracts that any
future implementation inherits: `mpfr_cosu` returns $$+0$$ at half-integers of
$$op \times 2/u$$ "so that the function is even", and `mpfr_sinu` returns a zero with the sign
of `op` at integers "so that the function is odd", which means the rational points with exact
answers must be detected rather than approached through ever-higher precision.

**`mpfr_sin_cos`.** Both values in one argument reduction. The C signature forces two rounded
results into one `int`, "s + 4c", zero "iff both results are exact", and forbids aliasing
between the outputs; a Rust return of two values with two ternary
[`Ordering`](https://doc.rust-lang.org/nightly/std/cmp/enum.Ordering.html)s makes both
constraints structural when this is built. `mpfr_sec`, `mpfr_csc`, and `mpfr_cot` are the
reciprocal trio, gaps with their primaries.

**The inverse functions.** `acos`, `asin`, `atan`, their `u` and `pi` scalings, and the
two-argument `atan2` family with its twenty ISO C99 special cases, signed zeros along the axes
and quarter-$$\pi$$ values at the infinite corners. One remark from the manual is worth keeping
for its subtlety, and will hold in Malachite too: `acos(-1)` rounds $$\pi$$, and the rounded
number "might not be in the output range" of the mathematical function; "still, the result
lies in the image of the output range by the rounding function".

### Hyperbolic functions

| | MPFR | Malachite |
| :---: | --- | --- |
| ✗ | `int mpfr_cosh (mpfr_t rop, mpfr_t op, mpfr_rnd_t rnd)` | |
| ✗ | `int mpfr_sinh (mpfr_t rop, mpfr_t op, mpfr_rnd_t rnd)` | |
| ✗ | `int mpfr_tanh (mpfr_t rop, mpfr_t op, mpfr_rnd_t rnd)` | |
| ✗ | `int mpfr_sinh_cosh (mpfr_t sop, mpfr_t cop, mpfr_t op, mpfr_rnd_t rnd)` | |
| ✗ | `int mpfr_sech (mpfr_t rop, mpfr_t op, mpfr_rnd_t rnd)` | |
| ✗ | `int mpfr_csch (mpfr_t rop, mpfr_t op, mpfr_rnd_t rnd)` | |
| ✗ | `int mpfr_coth (mpfr_t rop, mpfr_t op, mpfr_rnd_t rnd)` | |
| ✗ | `int mpfr_acosh (mpfr_t rop, mpfr_t op, mpfr_rnd_t rnd)` | |
| ✗ | `int mpfr_asinh (mpfr_t rop, mpfr_t op, mpfr_rnd_t rnd)` | |
| ✗ | `int mpfr_atanh (mpfr_t rop, mpfr_t op, mpfr_rnd_t rnd)` | |

**The family.** Ten rows, one gap, but a different kind of gap from the trigonometric block
above. The hyperbolic functions are algebraic combinations of the exponential machinery this
page has already marked ✓: $$\sinh x = (e^x - e^{-x})/2$$ with `exp_x_minus_1` carrying the
accuracy near zero, and $$\operatorname{atanh} x$$ living the same way on `ln_1_plus_x`. There
is no argument-reduction wall here; when these are built they will be layers over the ported
exponential and logarithm cores. What keeps the rows at ✗ in the meantime is the usual seam:
composing two correctly rounded steps rounds twice, and twice rounded is not correctly
rounded. `mpfr_sinh_cosh` shares `mpfr_sin_cos`'s packed return value, zero "iff both results
are exact", and the same observation about a two-`Ordering` Rust signature applies; `sech`,
`csch`, and `coth` are the reciprocal trio, and `acosh`, `asinh`, and `atanh` the inverses.

### Special functions

| | MPFR | Malachite |
| :---: | --- | --- |
| ✗ | `int mpfr_eint (mpfr_t rop, mpfr_t op, mpfr_rnd_t rnd)` | |
| ✗ | `int mpfr_li2 (mpfr_t rop, mpfr_t op, mpfr_rnd_t rnd)` | |
| ✗ | `int mpfr_gamma (mpfr_t rop, mpfr_t op, mpfr_rnd_t rnd)` | |
| ✗ | `int mpfr_gamma_inc (mpfr_t rop, mpfr_t op, mpfr_t op2, mpfr_rnd_t rnd)` | |
| ✗ | `int mpfr_lngamma (mpfr_t rop, mpfr_t op, mpfr_rnd_t rnd)` | |
| ✗ | `int mpfr_lgamma (mpfr_t rop, int *signp, mpfr_t op, mpfr_rnd_t rnd)` | |
| ✗ | `int mpfr_digamma (mpfr_t rop, mpfr_t op, mpfr_rnd_t rnd)` | |
| ✗ | `int mpfr_beta (mpfr_t rop, mpfr_t op1, mpfr_t op2, mpfr_rnd_t rnd)` | |
| ✗ | `int mpfr_zeta (mpfr_t rop, mpfr_t op, mpfr_rnd_t rnd)` | |
| ✗ | `int mpfr_zeta_ui (mpfr_t rop, unsigned long int op, mpfr_rnd_t rnd)` | |
| ✗ | `int mpfr_erf (mpfr_t rop, mpfr_t op, mpfr_rnd_t rnd)` | |
| ✗ | `int mpfr_erfc (mpfr_t rop, mpfr_t op, mpfr_rnd_t rnd)` | |
| ✗ | `int mpfr_j0 (mpfr_t rop, mpfr_t op, mpfr_rnd_t rnd)` | |
| ✗ | `int mpfr_j1 (mpfr_t rop, mpfr_t op, mpfr_rnd_t rnd)` | |
| ✗ | `int mpfr_jn (mpfr_t rop, long int n, mpfr_t op, mpfr_rnd_t rnd)` | |
| ✗ | `int mpfr_y0 (mpfr_t rop, mpfr_t op, mpfr_rnd_t rnd)` | |
| ✗ | `int mpfr_y1 (mpfr_t rop, mpfr_t op, mpfr_rnd_t rnd)` | |
| ✗ | `int mpfr_yn (mpfr_t rop, long int n, mpfr_t op, mpfr_rnd_t rnd)` | |
| ✓ | `int mpfr_agm (mpfr_t rop, mpfr_t op1, mpfr_t op2, mpfr_rnd_t rnd)` | [`agm_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.agm_prec_round) |
| ✗ | `int mpfr_ai (mpfr_t rop, mpfr_t x, mpfr_rnd_t rnd)` | |

**The special-function block.** Eighteen gaps in one sweep: the exponential integral and the
dilogarithm; the Gamma family, `gamma`, the upper incomplete `gamma_inc`, `lngamma`,
`lgamma`, and `digamma`, with `beta` alongside; the Riemann zeta function, on a `Float` and on
a `u64`; the error-function pair; and the Bessel functions of the first and second kinds at
orders 0, 1, and `n`. One signature deserves a forward-looking remark: `mpfr_lgamma` returns
the sign of $$\Gamma(op)$$ through an out-parameter `signp`, which a Rust signature would fold
into the return value alongside the ternary.

**`mpfr_agm`.** The arithmetic-geometric mean is ported: `agm_prec_round`, agreeing with MPFR
down to the special-value fine print, NaN when "any operand is negative and the other one is
not zero", $$+0$$ for a zero paired with a finite value, and NaN for a zero paired with an
infinity. It is the engine under the logarithm family above, and the one inhabitant of this
subsection that is already built.

**`mpfr_ai`.** The Airy function comes with a documented restriction in MPFR: "The current
implementation is not intended to be used with large arguments. It works with $$|x|$$
typically smaller than 500. For larger arguments, other methods should be used and will be
implemented in a future version." The gap recorded here is the whole function, all argument
ranges: the large-argument regimes are reachable with asymptotic expansions carrying rigorous
tail bounds, the far positive range collapses to an early underflow test, and the oscillatory
negative range shares its argument-reduction machinery with the trigonometric family above,
which it will naturally follow.

### Constants

| | MPFR | Malachite |
| :---: | --- | --- |
| ✓ | `int mpfr_const_log2 (mpfr_t rop, mpfr_rnd_t rnd)` | [`ln_2_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.ln_2_prec_round) |
| ✓ | `int mpfr_const_pi (mpfr_t rop, mpfr_rnd_t rnd)` | [`pi_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.pi_prec_round) |
| ✗ | `int mpfr_const_euler (mpfr_t rop, mpfr_rnd_t rnd)` | |
| ✗ | `int mpfr_const_catalan (mpfr_t rop, mpfr_rnd_t rnd)` | |

**The constants.** `Float::ln_2_prec_round(prec, rnd)` and `Float::pi_prec_round(prec, rnd)`
compute $$\log 2$$ and $$\pi$$ to any requested precision, statically, where MPFR fills a
prepared variable and caches the computation internally, the cache being the
[state behind `mpfr_free_cache`](#memory-handling-functions). Euler's constant $$\gamma$$ and Catalan's constant are gaps, a slightly
surprising pair of ✗ rows given the company they would keep: Malachite currently computes
around three dozen constants, from $$e$$, $$\tau$$, $$\varphi$$, and the square-root and
logarithm families to the lemniscate, Gauss, Gelfond, and Ramanujan constants and
digit-defined numbers like the Prouhet-Thue-Morse and prime constants, and MPFR's remaining
two will join them.

## [Input and Output Functions](https://www.mpfr.org/mpfr-current/mpfr.html#Input-and-Output-Functions) {#input-and-output-functions}

As on [every I/O section in this family](/mapping/flint-integers/#input-and-output), the
`FILE *` half of these functions has no Rust counterpart to need: strings are the interface,
and `write!` reaches any stream.

| | MPFR | Malachite |
| :---: | --- | --- |
| ≈ | `size_t mpfr_out_str (FILE *stream, int base, size_t n, mpfr_t op, mpfr_rnd_t rnd)` | [`Display`](https://doc.rust-lang.org/nightly/std/fmt/trait.Display.html), [`get_str`](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/string/get_str/fn.get_str.html) |
| ✓ | `size_t mpfr_inp_str (mpfr_t rop, FILE *stream, int base, mpfr_rnd_t rnd)` | [`set_str`](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/string/strtofr/fn.set_str.html) |
| ≈ | `int mpfr_fpif_export (FILE *stream, mpfr_t op)` | [`Serialize`](https://docs.rs/serde/latest/serde/trait.Serialize.html) |
| ≈ | `int mpfr_fpif_import (mpfr_t op, FILE *stream)` | [`Deserialize`](https://docs.rs/serde/latest/serde/trait.Deserialize.html) |
| ✓ | `void mpfr_dump (mpfr_t op)` | [`Debug`](https://doc.rust-lang.org/nightly/std/fmt/trait.Debug.html) |

**`mpfr_out_str`, `mpfr_inp_str`.** The output function is `mpfr_get_str` with presentation:
the same bases, 2 to 62 and $$-2$$ to $$-36$$, the same `n = 0` digit-count default, and a
fixed layout, first digit, locale-defined decimal point, remaining digits, then an exponent
whose prefix is `e` in small bases and `@` in large ones. In base 10, `Display` prints the
value in Malachite's own layout, which does not always show an exponent and always uses a
period; for other bases and for MPFR's exact layout,
[`get_str`](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/string/get_str/fn.get_str.html)
supplies the digits and exponent and the caller places the point, which is the ≈. The input
function reads a whitespace-delimited word and "parses it using `mpfr_set_str`"; with the word
in hand, Malachite's `set_str` is the same parse, so only the tokenizing is left to the
caller.

**`mpfr_fpif_export`, `mpfr_fpif_import`.** MPFR's interchange format and Malachite's `serde`
support answer the same need, moving floats between machines with the precision preserved:
"one can export on a 32-bit computer and import on a 64-bit computer, or export on a
little-endian computer and import on a big-endian computer", and a `Float` serialized on any
platform deserializes on any other, `Limb` width included. The encodings differ, a custom
binary format, which MPFR marks experimental, against a textual form through `serde`, which
composes with any `serde` format and carries the value, the precision, and the specials; the
sign of NaN, which `mpfr_fpif_export` stores, does not exist here. Both sides validate on the
way in, `mpfr_fpif_import` rejecting invalid precisions as Malachite's deserialization
[rejects invalid values](/mapping/gmp-rationals/#input-and-output-functions).

**`mpfr_dump`.** A debugging print in "some unspecified format" that "may change without
breaking the ABI"; Malachite's `Debug` is the same idea with the same non-promise, currently
printing what `Display` prints. When the debugging question is about the representation
rather than the value, the precision-revealing form is
[`ComparableFloat`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.ComparableFloat.html)'s
`Display`, whose hexadecimal output makes precision visible.

## [Formatted Output Functions](https://www.mpfr.org/mpfr-current/mpfr.html#Formatted-Output-Functions) {#formatted-output-functions}

MPFR extends `printf` with the `R` type: a conversion specification
`% [flags] [width] [.precision] R [rounding] conv` formats an `mpfr_t` argument, with `conv`
drawn from C99's float conversions plus `b` for binary. Malachite splits the job along a
natural seam. The `R`-conversion engine is ported:
[`format_float_str`](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/string/format_float/fn.format_float_str.html)`(&x, "%.10RYf")`
takes one `Float` and one conversion specification and returns the formatted piece. Everything
around it is either Rust's own
[`format!`](https://doc.rust-lang.org/nightly/std/macro.format.html) and `write!`, or the
[`gmp_format!`](https://docs.rs/malachite-base/latest/malachite_base/macro.gmp_format.html)
macro, which interprets a whole template as `mpfr_printf` would, mixing `R` conversions with the
`Z` and `Q` types and the plain C integer, character, and string conversions. Either way the
choice of sink is Rust's, which also retires the buffer-management distinctions that give this
section most of its rows.

| | MPFR | Malachite |
| :---: | --- | --- |
| ≈ | `int mpfr_fprintf (FILE *stream, const char *template, ...)` | [`format_float_str`](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/string/format_float/fn.format_float_str.html), [`write!`](https://doc.rust-lang.org/nightly/std/macro.write.html) |
| — | `int mpfr_vfprintf (FILE *stream, const char *template, va_list ap)` | |
| ≈ | `int mpfr_printf (const char *template, ...)` | [`format_float_str`](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/string/format_float/fn.format_float_str.html), [`print!`](https://doc.rust-lang.org/nightly/std/macro.print.html) |
| — | `int mpfr_vprintf (const char *template, va_list ap)` | |
| ≈ | `int mpfr_sprintf (char *buf, const char *template, ...)` | [`format_float_str`](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/string/format_float/fn.format_float_str.html), [`format!`](https://doc.rust-lang.org/nightly/std/macro.format.html) |
| — | `int mpfr_vsprintf (char *buf, const char *template, va_list ap)` | |
| ≈ | `int mpfr_snprintf (char *buf, size_t n, const char *template, ...)` | [`format_float_str`](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/string/format_float/fn.format_float_str.html), [`format!`](https://doc.rust-lang.org/nightly/std/macro.format.html) |
| — | `int mpfr_vsnprintf (char *buf, size_t n, const char *template, va_list ap)` | |
| ≈ | `int mpfr_asprintf (char **str, const char *template, ...)` | [`format_float_str`](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/string/format_float/fn.format_float_str.html), [`format!`](https://doc.rust-lang.org/nightly/std/macro.format.html) |
| — | `int mpfr_vasprintf (char **str, const char *template, va_list ap)` | |

**The family.** Ten functions, one engine. The five direct functions differ only in where the
characters go, a stream, `stdout`, a caller's buffer with or without a size limit, or an
allocated string; in Rust the destination is `write!`'s first argument or `format!`'s
returned [`String`](https://doc.rust-lang.org/nightly/std/string/struct.String.html), and the
truncating and allocating variants collapse. The `va_list` five are C's variadic plumbing, and
the Requirements subsection's concerns, variadic support and header order, come with them. A
row is ≈ rather than ✓ because
[`gmp_format!`](https://docs.rs/malachite-base/latest/malachite_base/macro.gmp_format.html)
stops short of the full C library underneath `mpfr_printf`: the C float conversions on primitive
floats, `%n`, `%p`, `%m`, and the `mpf_t`, limb, and `mpfr_prec_t` types are not interpreted.

**The conversion specification.** `format_float_str` accepts MPFR's grammar for a single
conversion: the `printf` flags, width, and precision; the rounding characters, `U` for
[`Ceiling`](https://docs.rs/malachite-base/latest/malachite_base/rounding_modes/enum.RoundingMode.html),
`D` for `Floor`, `Y` for `Up` (away from zero, "because ISO C reserves the `A` specifier for
hexadecimal output"), `Z` for `Down`, and `N` for the default `Nearest`, with `Exact`
additionally supported by the engine and panicking unless the digits represent the value
exactly; and the conversions `a`/`A`, `b`, `e`/`E`, `f`/`F`, and `g`/`G`. The `*` rounding
character, which in C fetches the mode from the argument list, has no argument list to fetch
from in a single-value call. The missing-precision defaults carry over exactly: `a`, `A`, and
`b` print "an exact representation with no trailing zeros", `e` and `E` print the
$$\lceil p \log 2/\log 10\rceil$$ round-trip digit count "matching the choice done for
`mpfr_get_str`", and `f`, `F`, `g`, and `G` default to 6. A failed conversion returns
[`None`](https://doc.rust-lang.org/nightly/std/option/enum.Option.html) where MPFR returns
$$-1$$ with an `errno`.

**Documented divergences.** Two design-level differences, both downstream of decisions already
on this page. A zero formatted with an empty precision under `e`-style conversions uses
precision 1, since Malachite zeros [carry no precision](#conventions) for the default to read.
And the `'` grouping flag always groups with a comma, where MPFR takes the separator from the
locale, which in the default C locale is empty, so that MPFR prints no separators there at
all.

## [Integer and Remainder Related Functions](https://www.mpfr.org/mpfr-current/mpfr.html#Integer-and-Remainder-Related-Functions) {#integer-and-remainder-related-functions}

Rounding a `Float` to an integral `Float`, and taking its fractional part, are covered by the
`round_to_integer` and `fractional_part` families, and the floating-point remainders by `%`
and the `rem` and `ieee_remainder` families; every row in the section is filled.

| | MPFR | Malachite |
| :---: | --- | --- |
| ✓ | `int mpfr_rint (mpfr_t rop, mpfr_t op, mpfr_rnd_t rnd)` | [`round_to_integer_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/struct.Float.html#method.round_to_integer_prec_round) |
| ✓ | `int mpfr_ceil (mpfr_t rop, mpfr_t op)` | [`round_to_integer_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/struct.Float.html#method.round_to_integer_prec_round) |
| ✓ | `int mpfr_floor (mpfr_t rop, mpfr_t op)` | [`round_to_integer_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/struct.Float.html#method.round_to_integer_prec_round) |
| ✓ | `int mpfr_round (mpfr_t rop, mpfr_t op)` | [`round_to_integer_ties_away_prec`](https://docs.rs/malachite-float/latest/malachite_float/struct.Float.html#method.round_to_integer_ties_away_prec) |
| ✓ | `int mpfr_roundeven (mpfr_t rop, mpfr_t op)` | [`round_to_integer_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/struct.Float.html#method.round_to_integer_prec_round) |
| ✓ | `int mpfr_trunc (mpfr_t rop, mpfr_t op)` | [`round_to_integer_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/struct.Float.html#method.round_to_integer_prec_round) |
| ✓ | `int mpfr_rint_ceil (mpfr_t rop, mpfr_t op, mpfr_rnd_t rnd)` | [`round_to_integer_then_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/struct.Float.html#method.round_to_integer_then_prec_round) |
| ✓ | `int mpfr_rint_floor (mpfr_t rop, mpfr_t op, mpfr_rnd_t rnd)` | [`round_to_integer_then_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/struct.Float.html#method.round_to_integer_then_prec_round) |
| ✓ | `int mpfr_rint_round (mpfr_t rop, mpfr_t op, mpfr_rnd_t rnd)` | [`round_to_integer_ties_away_then_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/struct.Float.html#method.round_to_integer_ties_away_then_prec_round) |
| ✓ | `int mpfr_rint_roundeven (mpfr_t rop, mpfr_t op, mpfr_rnd_t rnd)` | [`round_to_integer_then_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/struct.Float.html#method.round_to_integer_then_prec_round) |
| ✓ | `int mpfr_rint_trunc (mpfr_t rop, mpfr_t op, mpfr_rnd_t rnd)` | [`round_to_integer_then_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/struct.Float.html#method.round_to_integer_then_prec_round) |
| ✓ | `int mpfr_frac (mpfr_t rop, mpfr_t op, mpfr_rnd_t rnd)` | [`fractional_part_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/struct.Float.html#method.fractional_part_prec_round) |
| ✓ | `int mpfr_modf (mpfr_t iop, mpfr_t fop, mpfr_t op, mpfr_rnd_t rnd)` | [`integer_and_fractional_parts_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/struct.Float.html#method.integer_and_fractional_parts_prec_round) |
| ✓ | `int mpfr_fmod (mpfr_t r, mpfr_t x, mpfr_t y, mpfr_rnd_t rnd)` | [`rem_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/struct.Float.html#method.rem_prec_round) |
| ✓ | `int mpfr_fmod_ui (mpfr_t r, mpfr_t x, unsigned long int y, mpfr_rnd_t rnd)` | [`rem_unsigned_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/struct.Float.html#method.rem_unsigned_prec_round) |
| ✓ | `int mpfr_fmodquo (mpfr_t r, long int* q, mpfr_t x, mpfr_t y, mpfr_rnd_t rnd)` | [`rem_and_quotient_bits_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/struct.Float.html#method.rem_and_quotient_bits_prec_round) |
| ✓ | `int mpfr_remainder (mpfr_t r, mpfr_t x, mpfr_t y, mpfr_rnd_t rnd)` | [`ieee_remainder_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/struct.Float.html#method.ieee_remainder_prec_round) |
| ✓ | `int mpfr_remquo (mpfr_t r, long int* q, mpfr_t x, mpfr_t y, mpfr_rnd_t rnd)` | [`ieee_remainder_and_quotient_bits_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/struct.Float.html#method.ieee_remainder_and_quotient_bits_prec_round) |
| ✓ | `int mpfr_integer_p (mpfr_t op)` | [`is_integer`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.is_integer) |

**Rounding to an integer.** Eleven rows, two carefully distinguished semantics, both
honored. `mpfr_rint` and the five fixed-mode functions round `op` to a *representable* integer
in a single rounding: "no double rounding is performed", so 10.5 at 2-bit precision goes
directly to 12. [`round_to_integer_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/struct.Float.html#method.round_to_integer_prec_round) is that
operation, with the fixed modes folded into its
[`RoundingMode`](https://docs.rs/malachite-base/latest/malachite_base/rounding_modes/enum.RoundingMode.html)
argument, exactly as MPFR implements `mpfr_ceil`, `mpfr_floor`, `mpfr_trunc`, and
`mpfr_roundeven` as entries into `mpfr_rint`. MPFR's refined ternary — 0 for an integer
representable as-is, $$\pm 1$$ for an integer that had to be re-rounded, $$\pm 2$$ for a
non-integer — is returned losslessly as an
[`Ordering`](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html) paired with a
`bool` recording whether the input was an integer. `mpfr_round`'s ties-away rule
(roundTiesToAway) has no `RoundingMode` counterpart, so it gets its own functions,
[`round_to_integer_ties_away_prec`](https://docs.rs/malachite-float/latest/malachite_float/struct.Float.html#method.round_to_integer_ties_away_prec) and friends. The
`mpfr_rint_*` five, the same operations "regarded in the same way as any other mathematical
function" — round to the integer first, then correctly round that exact integer to the target
precision — are
[`round_to_integer_then_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/struct.Float.html#method.round_to_integer_then_prec_round) and its ties-away
sibling; under it, 10.5 with both modes nearest becomes 10 and then 8, where the
single-rounding form gives 12.

**`mpfr_frac`, `mpfr_modf`.** The fractional part, "having the same sign as `op`", with `rnd`
rounding the exact fraction rather than shaping it, and zero with `op`'s sign at integers and
infinities: [`fractional_part_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/struct.Float.html#method.fractional_part_prec_round) and its family.
`mpfr_modf` packages the truncation and the fraction in one call, with the two ternary values
packed into one return;
[`integer_and_fractional_parts_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/struct.Float.html#method.integer_and_fractional_parts_prec_round)
returns the two pairs directly, takes an independent precision for each part as the two `rop`
arguments do, and rounds the integral part exactly as `mpfr_rint_trunc` does.

**The remainders.** $$r = x - ny$$ with the quotient `n` "rounded toward zero" for the `fmod`
three and "to the nearest integer (ties rounded to even)" for `remainder` and `remquo`,
following ISO C99 F.9.7.1 for specials: NaN for infinite `x` or zero `y`, `x` rounded to `r`'s
precision for infinite `y`, and a zero `r` taking `x`'s sign. The truncated-quotient remainder
is the `%` operator (at the maximum of the input precisions, rounding to nearest, like the
other `Float` operators) and the
[`rem_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/struct.Float.html#method.rem_prec_round)
family; the nearest-quotient remainder — the IEEE 754 `remainder` operation — is the
[`ieee_remainder_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/struct.Float.html#method.ieee_remainder_prec_round)
family. Both use the modular reduction that the manual hints at ("`x` may be so large in
magnitude relative to `y` that an exact representation of the quotient is not practical"): the
exponent gap enters through a modular exponentiation rather than being materialized, so the
cost scales with the operands' precisions. The `quo` variants are the `*_and_quotient_bits`
functions, whose extra `i64` equals the quotient modulo $$2^{63}$$ with the quotient's sign,
matching the documented `mpfr_fmodquo`/`mpfr_remquo` contract; in the corner where the low 63
bits are all ones and the nearest quotient rounds away, incrementing them overflows a C
`long`, and Malachite defines the result by the modular contract instead. `mpfr_fmod_ui` is
the
[`rem_unsigned_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/struct.Float.html#method.rem_unsigned_prec_round)
family, with a zero modulus yielding NaN.

**Beyond MPFR.** Both remainder families also come in mixed `Float`-`Rational` forms
(`rem_rational_prec_round`, `rational_rem_float_prec_round`, and the `ieee_remainder`
counterparts, along with their `*_and_quotient_bits` variants and the `%` operators between
the two types). A remainder is unusually sensitive to its operands — perturbing the modulus by
$$\varepsilon$$ moves the result by up to $$n\varepsilon$$ — so a
[`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html)
operand, which is used exactly, preserves correct rounding where a lossy conversion to `Float`
would not. The quotient-bits forms with a `Rational` modulus serve additive argument
reduction against non-dyadic constants.

**`mpfr_integer_p`.** The one row already filled:
[`is_integer`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.is_integer),
via the
[`IsInteger`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.IsInteger.html)
trait, true exactly when the value is an integer, NaN and infinities included on the false
side.

## [Rounding-Related Functions](https://www.mpfr.org/mpfr-current/mpfr.html#Rounding_002dRelated-Functions) {#rounding-related-functions}

| | MPFR | Malachite |
| :---: | --- | --- |
| — | `void mpfr_set_default_rounding_mode (mpfr_rnd_t rnd)` | |
| — | `mpfr_rnd_t mpfr_get_default_rounding_mode (void)` | |
| ✓ | `int mpfr_prec_round (mpfr_t x, mpfr_prec_t prec, mpfr_rnd_t rnd)` | [`set_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.set_prec_round), [`set_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.set_prec) |
| ✓ | `int mpfr_can_round (mpfr_t b, mpfr_exp_t err, mpfr_rnd_t rnd1, mpfr_rnd_t rnd2, mpfr_prec_t prec)` | [`can_round`](https://docs.rs/malachite-float/latest/malachite_float/struct.Float.html#method.can_round) |
| ✓ | `mpfr_prec_t mpfr_min_prec (mpfr_t x)` | [`get_min_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.get_min_prec) |
| ✓ | `const char * mpfr_print_rnd_mode (mpfr_rnd_t rnd)` | [`Display`](https://docs.rs/malachite-base/latest/malachite_base/rounding_modes/enum.RoundingMode.html) |
| — | `mpfr_round_nearest_away (foo, rop, op, ...)` | |

**The default rounding mode.** "To nearest initially", and a global, so the pair of rows is —
for the reason set out under [Conventions](#conventions): the rounding mode is an argument
everywhere, or `Nearest` by the `_prec` families' definition, and there is nothing to set or
get.

**`mpfr_prec_round`.** The function that Malachite's
[`set_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.set_prec_round)
and `set_prec` actually correspond to, resolving the naming warning under
[Initialization Functions](#initialization-functions): round the value in place to a new
precision, returning the ternary value. The manual demonstrates its role in a Newton
iteration, sharpening and widening precision between steps, and the idiom transfers verbatim.
Its remarks about significand storage, fresh limbs zero-filled on widening and no reallocation
on narrowing, are C-side bookkeeping with no counterpart to need.

**`mpfr_can_round`.** The Ziv gate, exported: given an approximation `b` "of an unknown number
`x` in the direction `rnd1` with error at most two to the power `EXP(b) − err`", decide
whether `x` can be correctly rounded to `prec` in `rnd2`, treating the error as two-sided when
`rnd1` is nearest. [`can_round`](https://docs.rs/malachite-float/latest/malachite_float/struct.Float.html#method.can_round) is the exported version of the same test,
with the full error-direction generality of `rnd1`; Malachite's own Ziv loops continue to use
the cheaper internal form, which assumes a two-sided error. MPFR's faithful rounding mode
(`MPFR_RNDF`) has dedicated cases in this function; as everywhere on this page, that mode has
no Malachite counterpart, and that does not count against the row.

**`mpfr_min_prec`.** [`get_min_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.get_min_prec)
returns "the minimal number of bits required to store the significand" as an
[`Option`](https://doc.rust-lang.org/nightly/std/option/enum.Option.html), `None` standing in
for MPFR's 0 "for special values, including 0", the same shape as the `mpfr_get_prec` row.

**`mpfr_print_rnd_mode`.** [`RoundingMode`](https://docs.rs/malachite-base/latest/malachite_base/rounding_modes/enum.RoundingMode.html)
implements `Display` and prints its own vocabulary, `Nearest` where MPFR prints
`"MPFR_RNDN"`; the null pointer for an invalid mode has no counterpart, an enum having no
invalid values.

**`mpfr_round_nearest_away`.** A macro that manufactures a rounding mode neither library
offers: ties-away is not a supported member of `mpfr_rnd_t` any more than of
[`RoundingMode`](https://docs.rs/malachite-base/latest/malachite_base/rounding_modes/enum.RoundingMode.html),
and MPFR expresses it only through dedicated devices, the `mpfr_round` function
[noted above](#integer-and-remainder-related-functions) and this macro, which computes `foo`
twice to synthesize the rule for any function. The two libraries are at parity here, and the
macro's double-evaluation mechanics are C plumbing with nothing to abbreviate.

## [Miscellaneous Functions](https://www.mpfr.org/mpfr-current/mpfr.html#Miscellaneous-Functions) {#miscellaneous-functions}

| | MPFR | Malachite |
| :---: | --- | --- |
| ≈ | `void mpfr_nexttoward (mpfr_t x, mpfr_t y)` | [`increment`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.increment), [`decrement`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.decrement) |
| ≈ | `void mpfr_nextabove (mpfr_t x)` | [`increment`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.increment) |
| ≈ | `void mpfr_nextbelow (mpfr_t x)` | [`decrement`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.decrement) |
| ✓ | `int mpfr_min (mpfr_t rop, mpfr_t op1, mpfr_t op2, mpfr_rnd_t rnd)` | [`min_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.min_prec_round) |
| ✓ | `int mpfr_max (mpfr_t rop, mpfr_t op1, mpfr_t op2, mpfr_rnd_t rnd)` | [`max_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.max_prec_round) |
| ≈ | `int mpfr_urandomb (mpfr_t rop, gmp_randstate_t state)` | [`uniform_random_non_negative_floats_less_than_one`](https://docs.rs/malachite-float/latest/malachite_float/float/random/fn.uniform_random_non_negative_floats_less_than_one.html) |
| ≈ | `int mpfr_urandom (mpfr_t rop, gmp_randstate_t state, mpfr_rnd_t rnd)` | [`uniform_random_non_negative_floats_at_most_one`](https://docs.rs/malachite-float/latest/malachite_float/float/random/fn.uniform_random_non_negative_floats_at_most_one.html) |
| ≈ | `int mpfr_nrandom (mpfr_t rop1, gmp_randstate_t state, mpfr_rnd_t rnd)` | [`normal_random_floats`](https://docs.rs/malachite-float/latest/malachite_float/float/random/fn.normal_random_floats.html) |
| — | `int mpfr_grandom (mpfr_t rop1, mpfr_t rop2, gmp_randstate_t state, mpfr_rnd_t rnd)` | |
| ≈ | `int mpfr_erandom (mpfr_t rop1, gmp_randstate_t state, mpfr_rnd_t rnd)` | [`exponential_random_floats`](https://docs.rs/malachite-float/latest/malachite_float/float/random/fn.exponential_random_floats.html) |
| ✓ | `mpfr_exp_t mpfr_get_exp (mpfr_t x)` | [`get_exponent`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.get_exponent) |
| ≈ | `int mpfr_set_exp (mpfr_t x, mpfr_exp_t e)` | [`Shl`](https://doc.rust-lang.org/nightly/std/ops/trait.Shl.html) |
| ✓ | `int mpfr_signbit (mpfr_t op)` | [`is_sign_negative`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.is_sign_negative) |
| ✓ | `int mpfr_setsign (mpfr_t rop, mpfr_t op, int s, mpfr_rnd_t rnd)` | [`Abs`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Abs.html), [`Neg`](https://doc.rust-lang.org/nightly/std/ops/trait.Neg.html) |
| ✓ | `int mpfr_copysign (mpfr_t rop, mpfr_t op1, mpfr_t op2, mpfr_rnd_t rnd)` | [`Abs`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Abs.html), [`Neg`](https://doc.rust-lang.org/nightly/std/ops/trait.Neg.html) |
| — | `const char * mpfr_get_version (void)` | |
| — | `MPFR_VERSION`, `MPFR_VERSION_NUM (major, minor, patchlevel)` | |
| — | `const char * mpfr_get_patches (void)` | |
| — | `int mpfr_buildopt_tls_p (void)` | |
| — | `int mpfr_buildopt_float128_p (void)` | |
| — | `int mpfr_buildopt_decimal_p (void)` | |
| — | `int mpfr_buildopt_gmpinternals_p (void)` | |
| — | `int mpfr_buildopt_sharedcache_p (void)` | |
| — | `const char * mpfr_buildopt_tune_case (void)` | |

**The neighbor functions.** MPFR's stepping functions are "similar to the nextUp and nextDown
operations from IEEE 754", and Malachite's
[`increment`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.increment)
and `decrement` do the same walk along the representable numbers at the value's own precision,
with `mpfr_nexttoward` recovered by comparing first and stepping second. Two differences make
the rows ≈. MPFR's functions are total: NaN passes through, zero steps to the smallest float
of the appropriate sign, and the infinities act as endpoints, while `increment` and
`decrement` panic on NaN, infinities, and zeros. And at a binade boundary the conventions
part: stepping down from a power of two, `decrement` subtracts the ulp of the binade being
left, taking a larger step than `mpfr_nextbelow`'s move to the nearest representable below.

**`mpfr_min`, `mpfr_max`.** These follow IEEE 754-2019's minimumNumber and maximumNumber: a
single NaN operand is ignored, only two NaNs produce NaN, and zeros of different signs order
as $$-0 < +0$$. Malachite's [`min_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.min_prec_round) and
[`max_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.max_prec_round) implement the same selection rules; since a `Float`
result does not carry a preexisting precision the way `rop` does, the target precision and
rounding mode are explicit arguments, and the MPFR ternary value is returned as an
[`Ordering`]. The rest of the family follows the usual Malachite conventions:
[`min_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.min_prec) and [`max_prec`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.max_prec) round to nearest,
[`min_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.min_round) and [`max_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.max_round) target the maximum of the
operands' precisions, and [`min`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.min) and [`max`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.max) do both, in which case
the result is always exact; each method also has `_val_ref`, `_ref_val`, and `_ref_ref`
variants that take their arguments by reference. Beyond MPFR, the family also has mixed
`Float`-`Rational` forms (`min_rational_prec_round`, `max_rational_prec_round`, and their
shorthands): the comparison against the
[`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html)
is exact and only the winning operand is rounded, so the right operand is selected even when
a lossy conversion to `Float` would land on the other side of the comparison.

**The random generators.** MPFR's four are distribution samplers: `mpfr_urandomb` draws
uniformly from $$[0, 1)$$ with exactly `rop`'s precision, `mpfr_urandom` behaves "as if a
random real number is generated according to the continuous uniform distribution on the
interval [0, 1] and then rounded", and `mpfr_nrandom` and `mpfr_erandom` sample the standard
normal and mean-one exponential distributions, correctly rounded; these are available as
[`normal_random_floats`](https://docs.rs/malachite-float/latest/malachite_float/float/random/fn.normal_random_floats.html) and
[`exponential_random_floats`](https://docs.rs/malachite-float/latest/malachite_float/float/random/fn.exponential_random_floats.html), which use the same exact
rejection algorithms (Karney's algorithm N and von Neumann's method, respectively). Malachite's
[`random`](https://docs.rs/malachite-float/latest/malachite_float/float/random/index.html)
module generates values for testing, streams with mean-parameterized precisions and
exponents and striped bit patterns, the same philosophy noted
[on the FLINT rationals page](/mapping/flint-rationals/#random-number-generation); the
distribution samplers are all available. `mpfr_urandomb`'s
distribution — each value $k/2^p$ for uniform $k$, at a fixed precision $p$ — is sampled by
[`uniform_random_non_negative_floats_less_than_one`](https://docs.rs/malachite-float/latest/malachite_float/float/random/fn.uniform_random_non_negative_floats_less_than_one.html); the latter — a continuous uniform variable on the unit
interval, correctly rounded to a fixed precision with any rounding mode — by
[`uniform_random_non_negative_floats_at_most_one`](https://docs.rs/malachite-float/latest/malachite_float/float/random/fn.uniform_random_non_negative_floats_at_most_one.html).
The ≈ reflects that Malachite draws from its own seeded streams rather than a GMP randstate, so
the two libraries produce different sequences. `mpfr_grandom` is deprecated in favor of
`mpfr_nrandom`, which MPFR notes is "much more efficient", and rests with its replacement.

**`mpfr_get_exp`, `mpfr_set_exp`.**
[`get_exponent`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.get_exponent)
returns the exponent under the same $$[1/2, 1)$$ significand convention
[set out under Conventions](#conventions), answering
[`None`](https://doc.rust-lang.org/nightly/std/option/enum.Option.html) where MPFR declares
the behavior "undefined" for NaN, infinities, and zero. Its inverse is a shift:
`x << (e - x.get_exponent().unwrap())` moves the exponent exactly, the signed shift amount
[reversing direction as needed](/mapping/gmp-integers/#arithmetic-functions). The ≈ is the
failure mode at the edge of the exponent range, where `mpfr_set_exp` refuses and leaves `x`
unchanged while a shift saturates by the rounding rules.

**The sign functions.** `mpfr_signbit` is
[`is_sign_negative`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.is_sign_negative),
with one NaN-shaped difference that costs nothing: MPFR reports the NaN's stored sign bit,
while the single signless NaN answers `false`. This is also where the
[`Sign` trait warning](#comparison-functions) from Comparison Functions lands: `x.sign()` is
the `Ordering`-valued reader of the same bit for non-NaN values. `mpfr_setsign` and
`mpfr_copysign` are the absolute value with an optional negation, `-op.abs()` or `op.abs()`
as the requested sign directs, with
[`from_float_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.from_float_prec_round)
supplying the rounding when the precision changes too; their careful clauses about NaN signs
have nothing to govern here.

**Versions and build options.** Nine rows of build metadata with no counterpart to need:
Cargo pins and reports crate versions, so the runtime-versus-header comparison that
`mpfr_get_version` exists to support cannot arise; patch tracking belongs to the package
manager; and the build options either do not exist in Malachite, like the `float128` and
decimal types, or are visible in the types themselves, like the `32_bit_limbs` limb choice.
`mpfr_buildopt_tune_case`'s thresholds file has its analogue in Malachite's tuning
machinery, but as a development concern rather than a runtime query.

## [Exception Related Functions](https://www.mpfr.org/mpfr-current/mpfr.html#Exception-Related-Functions) {#exception-related-functions}

The section where the [global-state design difference](#conventions) is settled row by row.
MPFR keeps a movable exponent range and six sticky exception flags; Malachite fixes the range
and returns each operation's story with the operation. The result is a table of mostly —
rows, each backed by a specific answer to "where did that information go".

| | MPFR | Malachite |
| :---: | --- | --- |
| ✓ | `mpfr_exp_t mpfr_get_emin (void)` | [`Float::MIN_EXPONENT`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#associatedconstant.MIN_EXPONENT) |
| ✓ | `mpfr_exp_t mpfr_get_emax (void)` | [`Float::MAX_EXPONENT`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#associatedconstant.MAX_EXPONENT) |
| — | `int mpfr_set_emin (mpfr_exp_t exp)` | |
| — | `int mpfr_set_emax (mpfr_exp_t exp)` | |
| — | `mpfr_exp_t mpfr_get_emin_min (void)` | |
| — | `mpfr_exp_t mpfr_get_emin_max (void)` | |
| — | `mpfr_exp_t mpfr_get_emax_min (void)` | |
| — | `mpfr_exp_t mpfr_get_emax_max (void)` | |
| — | `int mpfr_check_range (mpfr_t x, int t, mpfr_rnd_t rnd)` | |
| ≈ | `int mpfr_subnormalize (mpfr_t x, int t, mpfr_rnd_t rnd)` | [`subnormalize`](https://docs.rs/malachite-float/latest/malachite_float/struct.Float.html#method.subnormalize) |
| — | `void mpfr_clear_underflow (void)` | |
| — | `void mpfr_clear_overflow (void)` | |
| — | `void mpfr_clear_divby0 (void)` | |
| — | `void mpfr_clear_nanflag (void)` | |
| — | `void mpfr_clear_inexflag (void)` | |
| — | `void mpfr_clear_erangeflag (void)` | |
| — | `void mpfr_clear_flags (void)` | |
| — | `void mpfr_set_underflow (void)` | |
| — | `void mpfr_set_overflow (void)` | |
| — | `void mpfr_set_divby0 (void)` | |
| — | `void mpfr_set_nanflag (void)` | |
| — | `void mpfr_set_inexflag (void)` | |
| — | `void mpfr_set_erangeflag (void)` | |
| ✓ | `int mpfr_underflow_p (void)` | [`test_underflow`](https://docs.rs/malachite-float/latest/malachite_float/fn.test_underflow.html) |
| ✓ | `int mpfr_overflow_p (void)` | [`test_overflow`](https://docs.rs/malachite-float/latest/malachite_float/fn.test_overflow.html) |
| — | `int mpfr_divby0_p (void)` | |
| — | `int mpfr_nanflag_p (void)` | |
| — | `int mpfr_inexflag_p (void)` | |
| — | `int mpfr_erangeflag_p (void)` | |
| — | `void mpfr_flags_clear (mpfr_flags_t mask)` | |
| — | `void mpfr_flags_set (mpfr_flags_t mask)` | |
| — | `mpfr_flags_t mpfr_flags_test (mpfr_flags_t mask)` | |
| — | `mpfr_flags_t mpfr_flags_save (void)` | |
| — | `void mpfr_flags_restore (mpfr_flags_t flags, mpfr_flags_t mask)` | |

**The exponent range.** `mpfr_get_emin` and `mpfr_get_emax` answer with
[`Float::MIN_EXPONENT`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#associatedconstant.MIN_EXPONENT)
and `Float::MAX_EXPONENT`, the very values a default-configured MPFR reports, with the
smallest positive value $$\tfrac{1}{2} \times 2^{emin}$$ and the largest
$$(1 - \varepsilon) \times 2^{emax}$$ in both libraries. The setters and the four
`mpfr_get_e*_min`/`_max` bounds are — because the range does not move; and with them goes a
class of hazard the manual spends paragraphs on, that after a range change "it is the user's
responsibility to check that any floating-point value used as an input is in the new exponent
range", on pain of undefined behavior. `mpfr_check_range`, which re-rounds a result computed
under one range into another while propagating the ternary value, manages a seam that does
not exist here.

**`mpfr_subnormalize`.** The tool for emulating gradual underflow, and through it whole IEEE
formats; the manual's example reproduces binary64 arithmetic by setting the range and
subnormalizing every result. Malachite's [`subnormalize`](https://docs.rs/malachite-float/latest/malachite_float/struct.Float.html#method.subnormalize) (with
[`subnormalize_ref`](https://docs.rs/malachite-float/latest/malachite_float/struct.Float.html#method.subnormalize_ref) and
[`subnormalize_assign`](https://docs.rs/malachite-float/latest/malachite_float/struct.Float.html#method.subnormalize_assign) variants) plays the same role, with two
adaptations that follow from the fixed exponent range: the emulated format's minimum normal
exponent is an explicit argument rather than a global setting, and values below the smallest
subnormal — which an MPFR computation would already have flushed at its exponent floor — are
handled by the function itself. Computing at the format's precision and then subnormalizing
each result reproduces the format's arithmetic exactly, and Malachite's own emulation
machinery is built this way.

**The flags, translated.** MPFR's six sticky flags accumulate across a computation, thread
locally, and are queried at the end; Malachite hands each operation's report back with the
operation, so the query functions map to information already in the caller's hands, and the
thirteen clear/set rows and five `mpfr_flags_*` group rows, machinery for managing and
restoring the accumulator, have nothing to manage. The dictionary, flag by flag:

- *inexact* is the ternary
  [`Ordering`](https://doc.rust-lang.org/nightly/std/cmp/enum.Ordering.html) that every
  operation returns.
- *erange* is a defined value at each site this page has noted: `None` from `partial_cmp`, an
  `Err` from `TryFrom`, a panic where the caller asked for the impossible.
- *invalid* (the NaN flag) is `result.is_nan()`.
- *divide-by-zero* is an infinite result from finite operands, visible at the call site that
  divided.
- *overflow* and *underflow* are the two with real subtlety, and they map to dedicated
  functions: [`test_overflow`](https://docs.rs/malachite-float/latest/malachite_float/fn.test_overflow.html)`(&result, o)`
  and `test_underflow(&result, o)` decide, from the result and its ternary value alone,
  whether the true value left the representable range. The subtlety is the directed rounding
  modes, where an overflow saturates to the largest finite value rather than infinity; the
  pair is still unambiguous, because saturation yields exactly the extreme value with an
  inexact ternary, a combination no in-range rounding can produce. An exact infinity, as from
  an infinite input, is not an overflow on either side, matching the manual's rule that an
  infinite result "is considered as inexact when it was obtained by overflow, and exact
  otherwise".

What stickiness itself buys, one query after a long computation, is folded by hand here: a
sequence of operations tests each step, or carries the disjunction along. That is the honest
cost of statelessness, paid for the usual gains, no thread-local state to save and restore,
which is what `mpfr_flags_save` and `mpfr_flags_restore` exist to do.

## [Memory Handling Functions](https://www.mpfr.org/mpfr-current/mpfr.html#Memory-Handling-Functions) {#memory-handling-functions}

| | MPFR | Malachite |
| :---: | --- | --- |
| — | `void mpfr_free_cache (void)` | |
| — | `void mpfr_free_cache2 (mpfr_free_cache_t way)` | |
| — | `void mpfr_free_pool (void)` | |
| — | `int mpfr_mp_memory_cleanup (void)` | |

**The caches and pools.** Four rows of maintenance for state Malachite does not keep. MPFR
caches computed constants and pools working memory, thread-locally and globally, and asks that
`mpfr_free_cache` be called "before terminating a thread, even if you did not call
`mpfr_const_*` functions directly (they could have been called internally)". Malachite
[computes constants on demand](#transcendental-functions) at the requested precision and
frees every allocation when its owner drops, so there is nothing to remember to free;
`mpfr_mp_memory_cleanup`'s handshake with GMP's `mp_set_memory_functions` belongs to the C
allocator world.

## [Compatibility With MPF](https://www.mpfr.org/mpfr-current/mpfr.html#Compatibility-with-MPF) {#compatibility-with-mpf}

This section is where [the mapping index](/mapping/)'s promise about GMP's float type is kept:
GMP's manual steers new projects toward MPFR, and the road from `mpf_t` runs through this
page. MPFR ships a header, `mpf2mpfr.h`, that recompiles many MPF programs unchanged, and its
list of caveats is a fair summary of what an `mpf_t` user gains by moving to either library:
exact-bit precision where MPF's is a lower bound, a defined exponent range, and different
output formatting. The section's five functions exist for that compatibility layer.

| | MPFR | Malachite |
| :---: | --- | --- |
| — | `void mpfr_set_prec_raw (mpfr_t x, mpfr_prec_t prec)` | |
| — | `int mpfr_eq (mpfr_t op1, mpfr_t op2, unsigned long int op3)` | |
| — | `void mpfr_reldiff (mpfr_t rop, mpfr_t op1, mpfr_t op2, mpfr_rnd_t rnd)` | |
| ✓ | `int mpfr_mul_2exp (mpfr_t rop, mpfr_t op1, unsigned long int op2, mpfr_rnd_t rnd)` | [`Shl`](https://doc.rust-lang.org/nightly/std/ops/trait.Shl.html) |
| ✓ | `int mpfr_div_2exp (mpfr_t rop, mpfr_t op1, unsigned long int op2, mpfr_rnd_t rnd)` | [`Shr`](https://doc.rust-lang.org/nightly/std/ops/trait.Shr.html) |

**The five functions.** `mpfr_set_prec_raw` is `mpfr_set_prec`'s undefined-behavior sibling,
managing storage that manages itself here. `mpfr_eq`, an MPF-style first-`n`-bits comparison,
comes with MPFR's own advice not to use it, and its warning that "1.011111 and 1.100000 are
regarded as different for any value of `op3` larger than 1" explains why no counterpart is
wanted; numeric closeness is a subtraction and a comparison. `mpfr_reldiff` computes
$$|op1 - op2|/op1$$ with no correct-rounding guarantee, which is exactly what writing the
formula out achieves. The `2exp` pair are, in the manual's words, "identical to
`mpfr_mul_2ui` and `mpfr_div_2ui`", and map as
[those rows do](#arithmetic-functions).

## [Custom Interface](https://www.mpfr.org/mpfr-current/mpfr.html#Custom-Interface) {#custom-interface}

| | MPFR | Malachite |
| :---: | --- | --- |
| — | `size_t mpfr_custom_get_size (mpfr_prec_t prec)` | |
| — | `void mpfr_custom_init (void *significand, mpfr_prec_t prec)` | |
| — | `void mpfr_custom_init_set (mpfr_t x, int kind, mpfr_exp_t exp, mpfr_prec_t prec, void *significand)` | |
| — | `int mpfr_custom_get_kind (mpfr_t x)` | |
| — | `void * mpfr_custom_get_significand (mpfr_t x)` | |
| — | `mpfr_exp_t mpfr_custom_get_exp (mpfr_t x)` | |
| — | `void mpfr_custom_move (mpfr_t x, void *new_position)` | |

**The interface.** An escape hatch for applications that "use a stack to handle the memory
and their objects", building `mpfr_t` values over caller-owned storage with the caller
responsible for every allocation and its lifetime. Rust draws the ownership lines differently:
a [`Float`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html)
owns its significand, values below 65 bits of precision
[live inline without allocating](#conventions), and reclamation is `Drop` rather than
"garbaging the used memory". The interface's components, sizing, placement, kind and
significand access, and relocation, are C memory management with no seam to attach to; the
readable parts of a `Float`, the significand, exponent, and classification, are reached
through the ordinary accessors this page has already mapped.
