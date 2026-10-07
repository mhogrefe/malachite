---
layout: default
title: "Malachite for FLINT Users: Integers mod n"
permalink: /mapping/flint-integers-mod-n/
theme: jekyll-theme-slate
---

# Malachite for FLINT Users: Integers mod n

This page maps the functions of FLINT's `fmpz_mod.h`, arithmetic modulo integers, onto their
Malachite counterparts: the `Mod*` traits, applied to
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html)
residues. It follows the organization of the
[fmpz_mod.h chapter](https://flintlib.org/doc/fmpz_mod.html) of the FLINT manual, as of FLINT
3.6.0, and is a companion to
[Malachite for FLINT Users: Integers](/mapping/flint-integers/); the
[mapping index](/mapping/) lists the whole family. The conventions of
[the GMP page](/mapping/gmp-integers/#conventions) and
[the fmpz page](/mapping/flint-integers/#conventions) apply here as well.

## Conventions {#conventions}

### Canonical residues are the shared ground

FLINT's functions expect their arguments in the canonical range `[0, n)`, and so do Malachite's
`Mod*` traits,
[`ModAdd`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModAdd.html),
[`ModMul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModMul.html),
[`ModPow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPow.html),
and the rest; reduction into that range is a separate, explicit step. Residues are
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html)s, so a
positive modulus is a fact of the types. Malachite enforces what FLINT trusts: passing an
unreduced operand to a `Mod*` function panics, where FLINT's behavior is undefined. The
`Mod*`-prefixed traits (reduced operands) differ from the `*Mod`-suffixed ones (remainders of
division, any operands); see [the fmpz page](/mapping/flint-integers/#modular-arithmetic) under
`fmpz_negmod`.

### What becomes of the context

Every function in this chapter takes an `fmpz_mod_ctx_t`, holding the modulus and precomputed
reduction data. A `Mod*` function takes the modulus as an ordinary argument instead: where FLINT
writes `fmpz_mod_mul(a, b, c, ctx)`, Malachite writes `b.mod_mul(&c, &m)`, and there is no object
to initialize, reconfigure, or free.

For multiplication,
[`ModMulPrecomputed`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModMulPrecomputed.html)
computes a reusable `Data` value once, with `precompute_mod_mul_data(&m)`, which
`mod_mul_precomputed` then uses; it is implemented for `Natural` and the primitive types.
[`ModPowPrecomputed`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPowPrecomputed.html)
does the same for exponentiation, for the primitive types only. No single object serves every
operation; addition, subtraction, and negation need no precomputed data.

When the modulus is $$2^k$$,
[`ModPowerOf2Add`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPowerOf2Add.html)
and its relatives take `k` as the argument, with no precomputation.

### Categories

Each function falls into one of four categories:

| | meaning |
| :---: | --- |
| ✓ | A Malachite function does the same thing. |
| ≈ | A Malachite function serves the same purpose, but its specification differs. The notes say how. |
| — | No counterpart is needed, either because Rust handles it for you or because it is outside Malachite's scope. The notes say which. |
| ✗ | Malachite does not fully support this yet, but will in a future version. |

## [Context object](https://flintlib.org/doc/fmpz_mod.html#context-object) {#context-object}

| | FLINT | Malachite |
| :---: | --- | --- |
| ≈ | `void fmpz_mod_ctx_init (fmpz_mod_ctx_t ctx, const fmpz_t n)` | [`ModMulPrecomputed`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModMulPrecomputed.html) |
| — | `void fmpz_mod_ctx_clear (fmpz_mod_ctx_t ctx)` | |
| ≈ | `void fmpz_mod_ctx_set_modulus (fmpz_mod_ctx_t ctx, const fmpz_t n)` | [`ModMulPrecomputed`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModMulPrecomputed.html) |

**`fmpz_mod_ctx_init`, `fmpz_mod_ctx_set_modulus`.** Keep the modulus as a
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html) and,
for repeated multiplication, pass the `Data` from `precompute_mod_mul_data(&m)` to each
`mod_mul_precomputed`. The rows are ≈ because the precomputation covers only multiplication. To
change the modulus, compute new `Data`.

**`fmpz_mod_ctx_clear`.** The `Data` value and the modulus are dropped like any other value.

## [Conversions](https://flintlib.org/doc/fmpz_mod.html#conversions) {#conversions}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpz_mod_set_fmpz (fmpz_t a, const fmpz_t b, const fmpz_mod_ctx_t ctx)` | [`Mod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Mod.html), [`ModEuclidean`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModEuclidean.html) |

**`fmpz_mod_set_fmpz`.** A
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html)
reduces with `b.mod_op(&m)`; an
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html)
reduces with
[`ModEuclidean`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModEuclidean.html),
whose remainder lies in `[0, n)`, as FLINT's does.

## [Arithmetic](https://flintlib.org/doc/fmpz_mod.html#arithmetic) {#arithmetic}

The context argument becomes `&m`, and each function maps to the `Mod*` trait of the same name.

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `int fmpz_mod_is_canonical (const fmpz_t a, const fmpz_mod_ctx_t ctx)` | [`ModIsReduced`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModIsReduced.html) |
| ✓ | `int fmpz_mod_is_one (const fmpz_t a, const fmpz_mod_ctx_t ctx)` | [`PartialEq`](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialEq.html) |
| ✓ | `void fmpz_mod_add (fmpz_t a, const fmpz_t b, const fmpz_t c, const fmpz_mod_ctx_t ctx)` | [`ModAdd`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModAdd.html), [`ModAddAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModAddAssign.html) |
| ✓ | `void fmpz_mod_add_fmpz (fmpz_t a, const fmpz_t b, const fmpz_t c, const fmpz_mod_ctx_t ctx)` | [`ModAdd`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModAdd.html), [`Mod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Mod.html) |
| ✓ | `void fmpz_mod_add_ui (fmpz_t a, const fmpz_t b, ulong c, const fmpz_mod_ctx_t ctx)` | [`ModAdd`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModAdd.html), [`Mod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Mod.html) |
| ✓ | `void fmpz_mod_add_si (fmpz_t a, const fmpz_t b, slong c, const fmpz_mod_ctx_t ctx)` | [`ModAdd`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModAdd.html), [`ModEuclidean`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModEuclidean.html) |
| ✓ | `void fmpz_mod_sub (fmpz_t a, const fmpz_t b, const fmpz_t c, const fmpz_mod_ctx_t ctx)` | [`ModSub`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModSub.html), [`ModSubAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModSubAssign.html) |
| ✓ | `void fmpz_mod_sub_fmpz (fmpz_t a, const fmpz_t b, const fmpz_t c, const fmpz_mod_ctx_t ctx)` | [`ModSub`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModSub.html), [`Mod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Mod.html) |
| ✓ | `void fmpz_mod_sub_ui (fmpz_t a, const fmpz_t b, ulong c, const fmpz_mod_ctx_t ctx)` | [`ModSub`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModSub.html), [`Mod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Mod.html) |
| ✓ | `void fmpz_mod_sub_si (fmpz_t a, const fmpz_t b, slong c, const fmpz_mod_ctx_t ctx)` | [`ModSub`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModSub.html), [`ModEuclidean`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModEuclidean.html) |
| ✓ | `void fmpz_mod_fmpz_sub (fmpz_t a, const fmpz_t b, const fmpz_t c, const fmpz_mod_ctx_t ctx)` | [`ModSub`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModSub.html), [`Mod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Mod.html) |
| ✓ | `void fmpz_mod_ui_sub (fmpz_t a, ulong b, const fmpz_t c, const fmpz_mod_ctx_t ctx)` | [`ModSub`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModSub.html), [`Mod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Mod.html) |
| ✓ | `void fmpz_mod_si_sub (fmpz_t a, slong b, const fmpz_t c, const fmpz_mod_ctx_t ctx)` | [`ModSub`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModSub.html), [`ModEuclidean`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModEuclidean.html) |
| ✓ | `void fmpz_mod_neg (fmpz_t a, const fmpz_t b, const fmpz_mod_ctx_t ctx)` | [`ModNeg`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModNeg.html), [`ModNegAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModNegAssign.html) |
| ✓ | `void fmpz_mod_mul (fmpz_t a, const fmpz_t b, const fmpz_t c, const fmpz_mod_ctx_t ctx)` | [`ModMul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModMul.html), [`ModMulPrecomputed`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModMulPrecomputed.html) |
| ✓ | `void fmpz_mod_inv (fmpz_t a, const fmpz_t b, const fmpz_mod_ctx_t ctx)` | [`ModInverse`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModInverse.html) |
| ✓ | `int fmpz_mod_divides (fmpz_t a, const fmpz_t b, const fmpz_t c, const fmpz_mod_ctx_t ctx)` | [`ModDiv`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModDiv.html) |
| ✓ | `void fmpz_mod_pow_ui (fmpz_t a, const fmpz_t b, ulong e, const fmpz_mod_ctx_t ctx)` | [`ModPow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPow.html), [`ModPowAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPowAssign.html) |
| ✓ | `int fmpz_mod_pow_fmpz (fmpz_t a, const fmpz_t b, const fmpz_t e, const fmpz_mod_ctx_t ctx)` | [`ModPow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPow.html), [`ModInverse`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModInverse.html) |

**`fmpz_mod_is_canonical`, `fmpz_mod_is_one`.** `a.mod_is_reduced(&m)` tests canonicality.
`fmpz_mod_is_one` is `a == 1`, except when `n` is 1: FLINT then answers true, while the
canonical residue is 0 and `a == 1` is false.

**Addition, subtraction, negation.** `b.mod_add(&c, &m)`, `b.mod_sub(&c, &m)`, and
`b.mod_neg(&m)`, each with an `Assign` form. In the nine mixed variants only one operand is
assumed canonical (`b`, or `c` for the reversed subtractions); reduce the other first, with
`Mod` for a `ulong`,
[`ModEuclidean`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModEuclidean.html)
for an `slong`, and, for an `fmpz`, `Mod` against the modulus as an `Integer` followed by
`Natural::exact_from` (the remainder is nonnegative but is returned as an `Integer`), then apply
the canonical operation. For the reversed subtractions,
`fmpz_mod_fmpz_sub` and its `ui`/`si` forms, reduce `b` and write `b_reduced.mod_sub(&c, &m)`.

**`fmpz_mod_mul`.** `b.mod_mul(&c, &m)`; in a loop over one modulus, use
`mod_mul_precomputed`, as under [Conventions](#conventions).

**`fmpz_mod_inv`.** `b.mod_inverse(&m)`. Where FLINT throws when `b` is not invertible,
`mod_inverse` returns [`Option`](https://doc.rust-lang.org/nightly/std/option/enum.Option.html).

**`fmpz_mod_divides`.** `b.mod_div(&c, &m)` solves $$a c \equiv b \pmod n$$, succeeding whenever
`gcd(c, n)` divides `b`, even when `c` is not invertible; FLINT's flag-plus-output protocol
becomes an [`Option`](https://doc.rust-lang.org/nightly/std/option/enum.Option.html). When the
solution is not unique, both functions return the same particular solution. For the full
solution set, see `fmpz_divides_mod_list`
[on the fmpz page](/mapping/flint-integers/#modular-arithmetic).

**`fmpz_mod_pow_ui`, `fmpz_mod_pow_fmpz`.** `b.mod_pow(&e, &m)`, with a
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html)
exponent of any size, covers both rows. For a negative exponent, which `fmpz_mod_pow_fmpz`
accepts when `b` is invertible, call `b.mod_inverse(&m)`, whose
[`None`](https://doc.rust-lang.org/nightly/std/option/enum.Option.html) corresponds to FLINT's
`0` return, then `mod_pow` with the exponent's absolute value.

## [Discrete Logarithms via Pohlig-Hellman](https://flintlib.org/doc/fmpz_mod.html#discrete-logarithms-via-pohlig-hellman) {#discrete-logarithms-via-pohlig-hellman}

Malachite has no counterpart to the five `fmpz_mod_discrete_log_pohlig_hellman` functions or to
`fmpz_next_smooth_prime`.