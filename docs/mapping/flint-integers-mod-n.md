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
[the fmpz page](/mapping/flint-integers/#conventions) apply here as well. The chapter has no
underscore-prefixed internal functions to omit; every section appears below, the discrete
logarithms in prose.

## Conventions {#conventions}

### Canonical residues are the shared ground

FLINT opens the chapter's arithmetic section with its ground rule: "all functions here expect
their relevant arguments to be in the canonical range `[0, n)`". That is exactly the contract
of Malachite's `Mod*` traits,
[`ModAdd`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModAdd.html),
[`ModMul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModMul.html),
[`ModPow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPow.html),
and the rest, whose operands must be already reduced. The two libraries reached the same
position: modular arithmetic proper operates on residues, and reduction into the residue range
is a separate, explicit step. So where the GMP page spends its modular notes on a mismatch,
`mpz_powm` reducing for you while `mod_pow` does not, this chapter and Malachite agree from the
start, and most rows below are plain ✓.

Two consequences follow. First, a canonical residue is nonnegative, so the residues here are
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html)s, and
a positive modulus, which FLINT documents as expected and checks when a context is created, is
here a fact of the types.
Second, Malachite enforces what FLINT trusts: passing an unreduced operand to a `Mod*` function
panics, where FLINT's behavior is undefined. The distinction between the `Mod*`-prefixed traits
(reduced operands, enforced) and the `*Mod`-suffixed ones (remainders of division, any operands)
is drawn [on the fmpz page](/mapping/flint-integers/#modular-arithmetic) under `fmpz_negmod`;
this chapter lives entirely on the `Mod*` side.

### What becomes of the context

Every function in this chapter takes an `fmpz_mod_ctx_t`, which holds two different things: the
modulus, and precomputed data that speeds up reduction, with FLINT choosing a strategy by the
size of the modulus. The two halves map differently.

The modulus half dissolves. A `Mod*` function takes the modulus as an ordinary argument, so
where FLINT writes `fmpz_mod_mul(a, b, c, ctx)`, Malachite writes `b.mod_mul(&c, &m)`, and
there is no object to initialize, reconfigure, or free.

The precomputation half partially exists. For multiplication, Malachite has the same idea in
trait form:
[`ModMulPrecomputed`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModMulPrecomputed.html)
computes a reusable `Data` value once, with `precompute_mod_mul_data(&m)`, and
`mod_mul_precomputed` then multiplies without repeating the setup; it is implemented for
`Natural` and for the primitive types, whose version is FLINT's own preinverted-word design.
Exponentiation has
[`ModPowPrecomputed`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPowPrecomputed.html)
for the primitive types only, and there is no single object that serves every operation the way
`fmpz_mod_ctx_t` does; whether Malachite should grow one is the design question this chapter
poses, and the `nmod_t` and `fmpz_preinvn_t` notes on the fmpz page point at the same place. For
addition, subtraction, and negation there is no data to precompute in either library. FLINT's
context does pre-select a size-specialized code path for them, one of three regimes chosen by
the width of the modulus; Malachite's inline-or-vector representation makes the analogous choice
at each call.

There is also a family where the context is structural:
[`ModPowerOf2Add`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPowerOf2Add.html)
and its relatives do arithmetic modulo $$2^k$$ with `k` as the argument, no precomputation
needed, since reduction is a mask. When the modulus has that shape, that family is the fast
path in place of everything below.

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

**`fmpz_mod_ctx_init`, `fmpz_mod_ctx_set_modulus`.** As described under
[Conventions](#conventions), initializing a context splits into two steps: keep the
modulus, an ordinary
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html), and,
when a multiplication loop justifies it, call `precompute_mod_mul_data(&m)` and pass the
resulting `Data` to each `mod_mul_precomputed`. The rows are ≈ because the precomputation
covers multiplication rather than every operation. `fmpz_mod_ctx_set_modulus` reconfigures a
context in place; the `Data` value is not reconfigured but replaced, by calling the precompute
function again for the new modulus. That is no loss of fidelity: FLINT's
`fmpz_mod_ctx_set_modulus` is implemented as clearing the context and initializing a fresh one.

**`fmpz_mod_ctx_clear`.** The `Data` value and the modulus are ordinary values; dropping them
releases them.

## [Conversions](https://flintlib.org/doc/fmpz_mod.html#conversions) {#conversions}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpz_mod_set_fmpz (fmpz_t a, const fmpz_t b, const fmpz_mod_ctx_t ctx)` | [`Mod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Mod.html), [`ModEuclidean`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModEuclidean.html) |

**`fmpz_mod_set_fmpz`.** The doorway from arbitrary integers into the canonical range, and the
one place in the chapter where the `*Mod`-suffixed world does the work for the `Mod*`-prefixed
one. A [`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html)
reduces with `b.mod_op(&m)`; an
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html)
reduces with
[`ModEuclidean`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModEuclidean.html),
whose remainder is nonnegative by definition, landing in `[0, n)` exactly as FLINT's reduction
does.

## [Arithmetic](https://flintlib.org/doc/fmpz_mod.html#arithmetic) {#arithmetic}

The heart of the chapter, and thanks to the shared canonical-range contract described under
[Conventions](#conventions), the cleanest arithmetic section on any of these pages: the context
argument becomes `&m`, and each function lands on the `Mod*` trait of the same name. Malachite
also has a few members of the family with no `fmpz_mod` counterpart:
[`ModSquare`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModSquare.html)
where FLINT would multiply a residue by itself, and
[`ModShl`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModShl.html)
and
[`ModShr`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModShr.html)
for modular shifts.

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

**`fmpz_mod_is_canonical`, `fmpz_mod_is_one`.** `a.mod_is_reduced(&m)` is the canonical-range
test. `fmpz_mod_is_one` is `a == 1`, with one corner from FLINT's source to know about: FLINT
also answers true for any `a` when `n` is 1, since modulo 1 everything is one,
while the canonical residue there is 0 and `a == 1` says no. FLINT's own note that equality and
zero tests need no context is the same observation that `==` needs no modulus.

**Addition, subtraction, negation.** `b.mod_add(&c, &m)`, `b.mod_sub(&c, &m)`, and
`b.mod_neg(&m)`, each with an `Assign` form that reuses `b`'s storage. The nine mixed variants
relax canonicality for one operand, which FLINT documents as "only `b` is assumed to be
canonical" (or only `c`, for the reversed subtractions); the unreduced side reduces first,
with `Mod` for a `Natural` or `ulong` and
[`ModEuclidean`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModEuclidean.html)
for an `slong`, whose value may be negative, and then the canonical operation applies. The
reversed subtractions, `fmpz_mod_fmpz_sub` and its `ui`/`si` forms, need no separate machinery:
reduce `b` and write `b_reduced.mod_sub(&c, &m)`.

**`fmpz_mod_mul`.** `b.mod_mul(&c, &m)`. This is the row the context exists for, and the
performance half of [Conventions](#conventions) lands here: in a loop over one modulus,
precompute once with `precompute_mod_mul_data(&m)` and multiply with `mod_mul_precomputed`, the
same amortization `fmpz_mod_ctx_t` performs.

**`fmpz_mod_inv`.** `b.mod_inverse(&m)`, with the reduced-operand requirement already satisfied
by this chapter's ground rule, so the difference that made `fmpz_invmod` a ≈
[on the fmpz page](/mapping/flint-integers/#modular-arithmetic) does not arise here. FLINT
throws when `b` is not invertible and suggests testing invertibility separately; `mod_inverse`
returns [`Option`](https://doc.rust-lang.org/nightly/std/option/enum.Option.html), so the test
and the computation are one call.

**`fmpz_mod_divides`.** General modular division: solve $$a c \equiv b \pmod n$$ for `a`,
succeeding whenever `gcd(c, n)` divides `b`, even when `c` is not invertible.
`b.mod_div(&c, &m)` is the same computation, with FLINT's flag-plus-output protocol becoming an
[`Option`](https://doc.rust-lang.org/nightly/std/option/enum.Option.html), as with
`fmpz_mod_inv` above. When `c` is not invertible the solution is not unique; both functions
then return the same particular solution, a bit-for-bit agreement that Malachite's
differential tests pin down, so ported code sees identical values, not merely valid ones. The
full solution set is the business of `fmpz_divides_mod_list`
[on the fmpz page](/mapping/flint-integers/#modular-arithmetic), whose `mod_div_list` names
the whole arithmetic progression at once. The unsigned primitive types implement `ModDiv` as
well.

**`fmpz_mod_pow_ui`, `fmpz_mod_pow_fmpz`.** `b.mod_pow(&e, &m)`, where the exponent is a
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html) of any
size, covering both rows at once. A negative exponent, which `fmpz_mod_pow_fmpz` accepts when
`b` is invertible, splits into the two steps the operation is made of: `b.mod_inverse(&m)`,
whose [`None`](https://doc.rust-lang.org/nightly/std/option/enum.Option.html) is FLINT's `0`
return, followed by `mod_pow` with the exponent's absolute value.

## [Discrete Logarithms via Pohlig-Hellman](https://flintlib.org/doc/fmpz_mod.html#discrete-logarithms-via-pohlig-hellman) {#discrete-logarithms-via-pohlig-hellman}

Not mapped yet. Malachite has no discrete-logarithm facility, and this one sits downstream of
work already on the queue: Pohlig-Hellman earns its speed by factoring $$p - 1$$, so the
facility belongs after bignum factorization and primality, in the same dependency class as the
multiplicative functions
[on the fmpz page](/mapping/flint-integers/#special-functions). The five
`fmpz_mod_discrete_log_pohlig_hellman` functions form one precompute-then-run context, the
largest context object in the chapter; the section will be mapped when the facility exists.

The sixth function, `fmpz_next_smooth_prime`, is a different case, and not part of that plan.
Its specification is deliberately loose, promising smooth primes whose properties "should not
be relied upon", and the implementation explains the looseness: the function binary-searches a
fixed table of 334 precomputed primes of the form
$$2^a 5^b 7^c 11^d 13^e 17^f 19^g 23^h + 1$$, and reports failure for any bound at or above the
table's largest entry, near $$5.8 \cdot 10^{57}$$. FLINT uses it to produce
discrete-log-friendly moduli for this section's tests and for sparse polynomial GCD
interpolation. A helper with that contract is one Malachite would keep internal to whatever
needs it, so it is recorded here as outside the public mapping rather than as a gap to fill.