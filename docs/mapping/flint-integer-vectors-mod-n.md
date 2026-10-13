---
layout: default
title: "Malachite for FLINT Users: Integer Vectors mod n"
permalink: /mapping/flint-integer-vectors-mod-n/
theme: jekyll-theme-slate
---

# Malachite for FLINT Users: Integer Vectors mod n

This page maps the functions of FLINT's `fmpz_mod_vec.h`, vectors over the integers mod $$n$$,
onto their Malachite counterpart:
[`NaturalVector`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_vector/struct.NaturalVector.html),
from the `malachite-nz` crate, with the `Mod*` traits. It follows the organization of the
[fmpz_mod_vec.h chapter](https://flintlib.org/doc/fmpz_mod_vec.html) of the FLINT manual, as of
FLINT 3.6.0. It is a companion to [Malachite for FLINT Users: Integers mod
n](/mapping/flint-integers-mod-n/), whose [conventions](/mapping/flint-integers-mod-n/#conventions)
apply here as well, and to [Malachite for FLINT Users: Integer
Vectors](/mapping/flint-integer-vectors/); the [mapping index](/mapping/) lists the whole family.

Every section of the chapter is mapped below: 11 rows across 5 sections, of which 8 are ✓ and 3
✗. All of the chapter's functions have names beginning with an underscore, and all are included
(see [Representation](#representation)). The ✗ rows are division by a scalar, the reversed dot
product, and the pointwise product, which
[`NaturalVector`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_vector/struct.NaturalVector.html)
does not yet provide as methods; each note gives the interim way to do the same thing one element
at a time.

## Conventions {#conventions}

### Representation {#representation}

`fmpz_mod_vec.h` has no vector object. Every function works on a bare `fmpz *` array with a
separate `slong` length, whose entries are residues in $$[0, n)$$, and every function name begins
with an underscore; as on [the integer vector
page](/mapping/flint-integer-vectors/#underscore-functions), those functions are mapped here.

The Malachite counterpart is a
[`NaturalVector`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_vector/struct.NaturalVector.html)
whose elements are all less than $$n$$, which
[`ModIsReduced`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModIsReduced.html)
checks. Every function in the chapter takes an `fmpz_mod_ctx_t` holding the modulus; the `Mod*`
traits take the modulus as an ordinary
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html)
argument instead, as for single residues (see [the integers mod n
page](/mapping/flint-integers-mod-n/#conventions)). FLINT trusts that its inputs are reduced;
Malachite's modular vector functions check, and panic if any element of an operand is not less
than $$n$$.

For a modulus that fits in a machine word, FLINT has a separate module, `nmod_vec`, whose
counterpart is
[`UnsignedVector`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/struct.UnsignedVector.html),
with the same `Mod*` traits and a primitive modulus.

### Categories

Each function falls into one of five categories:

| | meaning |
| :---: | --- |
| ✓ | A Malachite function does the same thing. |
| ≈ | A Malachite function serves the same purpose, but its specification differs. The notes say how. |
| ⚙ | Malachite does not expose this algorithm or helper; the Malachite column or the notes say what to call instead. |
| — | No counterpart is needed, either because Rust handles it for you or because it is outside Malachite's scope. The notes say which. |
| ✗ | Malachite does not fully support this yet, but will in a future version. |

## [Conversions](https://flintlib.org/doc/fmpz_mod_vec.html#conversions) {#conversions}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void _fmpz_mod_vec_set_fmpz_vec(fmpz * A, const fmpz * B, slong len, const fmpz_mod_ctx_t ctx)` | [`Mod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Mod.html) |
| ✓ | `void _fmpz_mod_vec_get_fmpz_vec_smod(fmpz * res, const fmpz * vec, slong len, const fmpz_mod_ctx_t ctx)` | [`BalancedMod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.BalancedMod.html) |

`_fmpz_mod_vec_set_fmpz_vec` reduces every element of an integer vector into $$[0, n)$$, using
several threads for long vectors or large elements. It is
[`Mod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Mod.html)
on an
[`IntegerVector`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/struct.IntegerVector.html)
with a [`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html)
modulus, which returns a
[`NaturalVector`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_vector/struct.NaturalVector.html)
(see `_fmpz_vec_scalar_mod_fmpz` on [the integer vector
page](/mapping/flint-integer-vectors/#reduction-mod-p)); Malachite does not split the work across
threads.
[`NaturalVector`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_vector/struct.NaturalVector.html)
also implements
[`Mod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Mod.html),
for reducing a vector of nonnegative integers that are not yet less than $$n$$.

`_fmpz_mod_vec_get_fmpz_vec_smod` converts residues in $$[0, n)$$ to their balanced
representatives in $$(-n/2, n/2]$$. It is
[`BalancedMod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.BalancedMod.html)
on a
[`NaturalVector`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_vector/struct.NaturalVector.html)
with a [`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html)
modulus, which returns an
[`IntegerVector`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/struct.IntegerVector.html),
as `BalancedMod` does for a single residue. The FLINT function is `_fmpz_vec_scalar_smod_fmpz`
specialized to inputs already known to be reduced, which it does not check; `BalancedMod` accepts
elements that are not yet reduced, since producing reduced output is its purpose.

## [Arithmetic](https://flintlib.org/doc/fmpz_mod_vec.html#arithmetic) {#arithmetic}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void _fmpz_mod_vec_neg(fmpz * A, const fmpz * B, slong len, const fmpz_mod_ctx_t ctx)` | [`ModNeg`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModNeg.html), [`ModNegAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModNegAssign.html) |
| ✓ | `void _fmpz_mod_vec_add(fmpz * a, const fmpz * b, const fmpz * c, slong n, const fmpz_mod_ctx_t ctx)` | [`ModAdd`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModAdd.html), [`ModAddAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModAddAssign.html) |
| ✓ | `void _fmpz_mod_vec_sub(fmpz * a, const fmpz * b, const fmpz * c, slong n, const fmpz_mod_ctx_t ctx)` | [`ModSub`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModSub.html), [`ModSubAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModSubAssign.html) |

Each function applies the corresponding single-residue operation (`fmpz_mod_neg`, `fmpz_mod_add`,
or `fmpz_mod_sub`; see [the integers mod n page](/mapping/flint-integers-mod-n/)) to every
element. On
[`NaturalVector`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_vector/struct.NaturalVector.html),
`v.mod_add(w, m)` takes the modulus as an argument, and each operation takes the vectors and
the modulus by value or by reference. Writing the result over the first operand, as FLINT allows
by passing the same array twice, is the `*_assign` form: `v.mod_add_assign(&w, &m)`. Malachite
panics if the two vectors have different dimensions or if any element of either is not less than
$$m$$; FLINT checks neither. The power-of-2 forms,
[`ModPowerOf2Neg`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPowerOf2Neg.html),
[`ModPowerOf2Add`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPowerOf2Add.html),
and
[`ModPowerOf2Sub`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPowerOf2Sub.html),
take the exponent $$k$$ of a modulus $$2^k$$ instead.

In the manual, the descriptions of `_fmpz_mod_vec_add` and `_fmpz_mod_vec_sub` are missing an
opening backtick and name the arguments `A`, `B`, and `C`, while the prototypes call them `a`,
`b`, and `c`.

## [Scalar Multiplication](https://flintlib.org/doc/fmpz_mod_vec.html#scalar-multiplication) {#scalar-multiplication}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void _fmpz_mod_vec_scalar_mul_fmpz_mod(fmpz * A, const fmpz * B, slong len, const fmpz_t c, const fmpz_mod_ctx_t ctx)` | [`ModMul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModMul.html), [`ModMulAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModMulAssign.html) |
| ✓ | `void _fmpz_mod_vec_scalar_addmul_fmpz_mod(fmpz * A, const fmpz * B, slong len, const fmpz_t c, const fmpz_mod_ctx_t ctx)` | [`ModAddMul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModAddMul.html), [`ModAddMulAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModAddMulAssign.html) |
| ✗ | `void _fmpz_mod_vec_scalar_div_fmpz_mod(fmpz * A, const fmpz * B, slong len, const fmpz_t c, const fmpz_mod_ctx_t ctx)` | |

`scalar_mul_fmpz_mod` multiplies each element by $$c$$ modulo $$n$$, which is
[`ModMul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModMul.html)
of a [`NaturalVector`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_vector/struct.NaturalVector.html)
by a [`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html)
modulo another, or
[`ModMulAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModMulAssign.html)
when `A` is `B`. Like FLINT's context, it computes the data for multiplying modulo $$n$$ once and
reuses it for every element. Malachite panics if $$c$$ or any element is not less than $$n$$;
FLINT does not check.

`scalar_addmul_fmpz_mod` adds $$c$$ times each element of `B` to the corresponding element of
`A`, which is [`ModAddMulAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModAddMulAssign.html) of a
[`NaturalVector`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_vector/struct.NaturalVector.html),
another `NaturalVector`, a [`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html)
scalar, and a `Natural` modulus; [`ModAddMul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModAddMul.html) returns the result instead.
Like FLINT, it handles $$c$$ equal to 0 and 1 directly (and also $$n - 1$$, which FLINT does not)
and otherwise computes the data for multiplying modulo $$n$$ once. Malachite panics if the
dimensions differ or if $$c$$ or any element is not less than $$n$$; FLINT checks neither.
[`ModSubMul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModSubMul.html) and [`ModSubMulAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModSubMulAssign.html) subtract the
multiple instead, which this chapter has no function for.

The vector types do not yet have `scalar_div_fmpz_mod`, which divides each element by $$c$$.
FLINT computes the inverse of $$c$$ once and multiplies by it; until the vector types have it, do
the same with
[`ModInverse`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModInverse.html) followed by [`ModMulAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModMulAssign.html) on the
vector.

The manual says only that `scalar_div_fmpz_mod` assumes $$c$$ is nonzero, but what it needs
is that $$c$$ be invertible modulo $$n$$: FLINT throws an exception when $$\gcd(c, n) \ne 1$$,
which for a composite modulus can happen even when $$c$$ is nonzero.
[`mod_inverse`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModInverse.html#tymethod.mod_inverse) returns
[`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None) in that
case.

## [Dot Product](https://flintlib.org/doc/fmpz_mod_vec.html#dot-product) {#dot-product}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void _fmpz_mod_vec_dot(fmpz_t d, const fmpz * A, const fmpz * B, slong len, const fmpz_mod_ctx_t ctx)` | [`ModDot`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModDot.html) |
| ✗ | `void _fmpz_mod_vec_dot_rev(fmpz_t d, const fmpz * A, const fmpz * B, slong len, const fmpz_mod_ctx_t ctx)` | |

`_fmpz_mod_vec_dot` is $$\sum_i a_i b_i \bmod n$$, and `_fmpz_mod_vec_dot_rev` pairs
$$a_i$$ with $$b_{\mathit{len}-1-i}$$ instead, reversing the second vector. Neither reduces as
it goes: each computes the whole sum of products as an integer, with the [integer vector dot
product](/mapping/flint-integer-vectors/#dot-product), and reduces it modulo $$n$$ once at the
end.

`_fmpz_mod_vec_dot` is [`ModDot`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModDot.html)'s `v.mod_dot(w, n)` for `NaturalVector`, with the
vectors and the modulus each taken by value or by reference. It uses the same strategy: the whole
dot product, computed as an integer, reduced once. When $$n$$ fits in a limb, every element does
too, and the products are summed in one to three limbs instead, as by the word-sized
`_nmod_vec_dot`. For a power-of-2 modulus,
[`ModPowerOf2Dot`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPowerOf2Dot.html) does the same with the low $$k$$ bits. The reversed form has no counterpart
yet; until it does, apply
[`AddMulAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.AddMulAssign.html)
to each pair of elements, iterating over the second vector in reverse, and reduce the result once
with [`Mod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Mod.html).

## [Multiplication](https://flintlib.org/doc/fmpz_mod_vec.html#multiplication) {#multiplication}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void _fmpz_mod_vec_mul(fmpz * A, const fmpz * B, const fmpz * C, slong len, const fmpz_mod_ctx_t ctx)` | |

`_fmpz_mod_vec_mul` multiplies two vectors element by element, setting $$a_i$$ to
$$b_i c_i \bmod n$$; it is the pointwise product, not a convolution. The vector types do not yet
have it. Until they do, zip the two
[`elements`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_vector/struct.NaturalVector.html#structfield.elements)
fields and apply [`ModMul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModMul.html) or [`ModMulAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModMulAssign.html) to each
pair.
