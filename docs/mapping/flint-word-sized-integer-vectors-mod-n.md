---
layout: default
title: "Malachite for FLINT Users: Word-Sized Integer Vectors mod n"
permalink: /mapping/flint-word-sized-integer-vectors-mod-n/
theme: jekyll-theme-slate
---

# Malachite for FLINT Users: Word-Sized Integer Vectors mod n

This page maps the functions of FLINT's `nmod_vec.h`, vectors over the integers mod $$n$$ for a
modulus $$n$$ that fits in a machine word, onto their Malachite counterpart:
[`UnsignedVector`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/struct.UnsignedVector.html),
from the `malachite-base` crate, with the `Mod*` traits. It follows the organization of the
[nmod_vec.h chapter](https://flintlib.org/doc/nmod_vec.html) of the FLINT manual, as of FLINT
3.6.0. It is a companion to [Malachite for FLINT Users: Integer Vectors mod
n](/mapping/flint-integer-vectors-mod-n/), the multi-word version, and to [Malachite for FLINT
Users: Word-Sized Modular Polynomials](/mapping/flint-word-sized-modular-polynomials/), whose
[conventions](/mapping/flint-word-sized-modular-polynomials/#conventions) on word sizes and
precomputed data apply here as well; the [mapping index](/mapping/) lists the whole family.

Every section of the chapter is mapped below: 35 rows across 7 sections, of which 14 are ✓, 13 ⚙,
5 ≈, 1 —, and 2 ✗. All of the chapter's functions have names beginning with an underscore, and all
are included (see [Representation](#representation)). Many rows are ⚙: much of the chapter exposes
FLINT's choice of algorithm (Shoup or Montgomery multiplication, unreduced accumulation, dot
product methods), which Malachite keeps internal. The two ✗ rows, `_nmod_vec_invert` and
`_nmod_vec_dot_rev`, are operations that
[`UnsignedVector`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/struct.UnsignedVector.html)
does not yet provide as methods; each note gives the interim way to do the same thing one element
at a time. Functions that `nmod_vec.h` declares but the chapter does not document have no rows;
the sections that mention them name them.

## Conventions {#conventions}

### Representation {#representation}

`nmod_vec.h` has no vector object. Every function works on a bare array of `ulong`s with a
separate `slong` length, and every function name begins with an underscore; as on [the integer
vector page](/mapping/flint-integer-vectors/#underscore-functions), those functions are mapped
here. Most functions also take an `nmod_t`, which holds the modulus $$n$$ and precomputed data for
reducing modulo it, and expect the entries to be residues in $$[0, n)$$.

The Malachite counterpart is an
[`UnsignedVector<T>`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/struct.UnsignedVector.html)
whose elements are all less than $$n$$, which
[`ModIsReduced`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModIsReduced.html)
checks; the modulus is a `T` passed to each `Mod*` operation. `T` may be any primitive unsigned
type, and this chapter is the `T = u64` case (see [word
sizes](/mapping/flint-word-sized-modular-polynomials/#word-sized)). FLINT trusts that its inputs
are reduced; Malachite's modular vector functions check, and panic if any element of an operand is
not less than $$n$$. Multi-word moduli are the subject of [the `fmpz_mod_vec`
page](/mapping/flint-integer-vectors-mod-n/), where the counterpart is
[`NaturalVector`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_vector/struct.NaturalVector.html).

### Categories

Each function falls into one of five categories:

| | meaning |
| :---: | --- |
| ✓ | A Malachite function does the same thing. |
| ≈ | A Malachite function serves the same purpose, but its specification differs. The notes say how. |
| ⚙ | Malachite does not expose this algorithm or helper; the Malachite column or the notes say what to call instead. |
| — | No counterpart is needed, either because Rust handles it for you or because it is outside Malachite's scope. The notes say which. |
| ✗ | Malachite does not fully support this yet, but will in a future version. |

## [Memory management](https://flintlib.org/doc/nmod_vec.html#memory-management) {#memory-management}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `nn_ptr _nmod_vec_init(slong len)` | [`zero`](https://docs.rs/malachite-base/latest/malachite_base/vector/trait.Vector.html#tymethod.zero) |
| — | `void _nmod_vec_clear(nn_ptr vec)` | |

`_nmod_vec_init` allocates room for `len` words without initializing them, so its entries are not
necessarily zero. Safe Rust has no uninitialized values, so the counterpart is
[`zero`](https://docs.rs/malachite-base/latest/malachite_base/vector/trait.Vector.html#tymethod.zero), the
[`Vector`](https://docs.rs/malachite-base/latest/malachite_base/vector/trait.Vector.html) method
that returns the zero vector of a given dimension: `UnsignedVector::<u64>::zero(len)`. Its
elements are reduced modulo every $$n$$.
[`Vec::with_capacity`](https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html#method.with_capacity)
reserves room without creating any elements. `_nmod_vec_clear` frees the array, and takes no
length because its entries own no memory; it is
[`Drop`](https://doc.rust-lang.org/nightly/core/ops/trait.Drop.html), which runs when the vector goes out of scope.

## [Random functions](https://flintlib.org/doc/nmod_vec.html#random-functions) {#random-functions}

| | FLINT | Malachite |
| :---: | --- | --- |
| ≈ | `void _nmod_vec_randtest(nn_ptr vec, flint_rand_t state, slong len, nmod_t mod)` | [`random_unsigned_vectors_with_dimension_from_iterator`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/random/fn.random_unsigned_vectors_with_dimension_from_iterator.html), [`striped_random_unsigned_range`](https://docs.rs/malachite-base/latest/malachite_base/num/random/striped/fn.striped_random_unsigned_range.html) |
| ✓ | `void _nmod_vec_rand(nn_ptr vec, flint_rand_t state, slong len, nmod_t mod)` | [`random_unsigned_vectors_with_dimension_from_iterator`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/random/fn.random_unsigned_vectors_with_dimension_from_iterator.html), [`random_unsigneds_less_than`](https://docs.rs/malachite-base/latest/malachite_base/num/random/fn.random_unsigneds_less_than.html) |

Malachite's generators are infinite iterators of vectors.
[`random_unsigned_vectors_with_dimension_from_iterator`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/random/fn.random_unsigned_vectors_with_dimension_from_iterator.html)
gives vectors of a fixed dimension whose elements come from any iterator, so a generator of
reduced vectors is that function with a generator of residues.

`_nmod_vec_rand` draws each element uniformly from $$[0, n)$$; with
[`random_unsigneds_less_than`](https://docs.rs/malachite-base/latest/malachite_base/num/random/fn.random_unsigneds_less_than.html)`(seed, n)` as
the element iterator, Malachite's distribution is the same.

`_nmod_vec_randtest` draws each element as a word from `n_randtest`, reduced modulo $$n$$.
`n_randtest` picks a random bit length and favours the special values 0, 1, `COEFF_MAX`, and
`WORD_MAX` and values with sparse binary representations. Half the time `_nmod_vec_randtest` also
makes the vector sparse, as [`_fmpz_vec_randtest`](/mapping/flint-integer-vectors/#randomisation)
does. The closest Malachite element iterator is
[`striped_random_unsigned_range`](https://docs.rs/malachite-base/latest/malachite_base/num/random/striped/fn.striped_random_unsigned_range.html)
with the range $$[0, n)$$, which favours values with long runs of 0 or 1 bits among the residues
directly. The distributions differ, and there is no sparse mode, hence ≈.

## [Basic manipulation and comparison](https://flintlib.org/doc/nmod_vec.html#basic-manipulation-and-comparison) {#basic-manipulation-and-comparison}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void _nmod_vec_set(nn_ptr res, nn_srcptr vec, slong len)` | [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html) |
| ✓ | `void _nmod_vec_zero(nn_ptr vec, slong len)` | [`set_zero`](https://docs.rs/malachite-base/latest/malachite_base/vector/trait.Vector.html#tymethod.set_zero) |
| ✓ | `void _nmod_vec_swap(nn_ptr a, nn_ptr b, slong length)` | [`mem::swap`](https://doc.rust-lang.org/nightly/core/mem/fn.swap.html) |
| ✓ | `void _nmod_vec_reduce(nn_ptr res, nn_srcptr vec, slong len, nmod_t mod)` | [`Mod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Mod.html), [`ModAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModAssign.html) |
| ✓ | `flint_bitcnt_t _nmod_vec_max_bits(nn_srcptr vec, slong len)` | [`height_significant_bits`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Height.html#tymethod.height_significant_bits), [`max_bits`](https://docs.rs/malachite-base/latest/malachite_base/vector/trait.Vector.html#tymethod.max_bits) |
| ✓ | `int _nmod_vec_equal(nn_srcptr vec, nn_srcptr vec2, slong len)` | [`PartialEq`](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html) |

`_nmod_vec_set` copies the elements, which is
[`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html) (or
[`clone_from`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from),
to reuse an allocation). `_nmod_vec_swap` exchanges two vectors of the same length one element at
a time; [`mem::swap`](https://doc.rust-lang.org/nightly/core/mem/fn.swap.html) exchanges two
vectors of any dimensions in constant time. `_nmod_vec_equal` compares over a length the caller
supplies; [`PartialEq`](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html) also
requires the dimensions to match.

`_nmod_vec_reduce` reduces arbitrary words into $$[0, n)$$, which is
[`Mod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Mod.html) on
[`UnsignedVector`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/struct.UnsignedVector.html)
with a `T` modulus, or [`ModAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModAssign.html) in place;
[`Rem`](https://doc.rust-lang.org/nightly/core/ops/trait.Rem.html) is the same for unsigned
elements. Like FLINT's, it accepts unreduced input, since producing reduced elements is its
purpose.

`_nmod_vec_max_bits` returns the largest bit length of any element, which is
[`height_significant_bits`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Height.html#tymethod.height_significant_bits);
FLINT computes it as the bit length of the bitwise OR of all the elements, stopping early once the
top bit is set. `_nmod_vec_zero` sets every element to zero, which is
[`set_zero`](https://docs.rs/malachite-base/latest/malachite_base/vector/trait.Vector.html#tymethod.set_zero),
a [`Vector`](https://docs.rs/malachite-base/latest/malachite_base/vector/trait.Vector.html)
method.

`nmod_vec.h` also defines `_nmod_vec_is_zero`, which the chapter does not document; it has no row
here, and its counterpart is
[`is_zero`](https://docs.rs/malachite-base/latest/malachite_base/vector/trait.Vector.html#tymethod.is_zero),
as for `_fmpz_vec_is_zero` on [the integer vector page](/mapping/flint-integer-vectors/#comparison).

## [Printing](https://flintlib.org/doc/nmod_vec.html#printing) {#printing}

| | FLINT | Malachite |
| :---: | --- | --- |
| ≈ | `void _nmod_vec_print_pretty(nn_srcptr vec, slong len, nmod_t mod)` | [`Display`](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html) |
| ≈ | `int _nmod_vec_fprint_pretty(FILE * file, nn_srcptr vec, slong len, nmod_t mod)` | [`Display`](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html) |
| ≈ | `int _nmod_vec_print(nn_srcptr vec, slong len, nmod_t mod)` | [`Display`](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html), [`Serialize`](https://docs.rs/serde/latest/serde/trait.Serialize.html) |
| ≈ | `int _nmod_vec_fprint(FILE * f, nn_srcptr vec, slong len, nmod_t mod)` | [`Display`](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html), [`Serialize`](https://docs.rs/serde/latest/serde/trait.Serialize.html) |

All four functions write the same format: a header naming the length and the modulus, then the
elements in square brackets, separated by spaces and each right-aligned to the number of decimal
digits of the modulus, as in

```text
<length-3 integer vector mod 197>
[ 33 181   7]
```

For the empty vector, only the header is written. Malachite's
[`Display`](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html) writes `(33, 181, 7)`,
with `()` for the 0-dimensional vector; a vector does not know its modulus, so there is no header,
and the elements are not padded. The formats differ, hence ≈. The manual says `print` and `fprint`
are currently the same as the pretty versions; with the `serde` feature,
[`Serialize`](https://docs.rs/serde/latest/serde/trait.Serialize.html) gives a plain list of the
elements for machine-readable output. `_nmod_vec_print_pretty` returns nothing, while the other
three return a value that is positive on success. As on [the fmpz
page](/mapping/flint-integers/#input-and-output), Malachite has no `FILE *` functions: formatting
works with any Rust writer.

## [Arithmetic operations](https://flintlib.org/doc/nmod_vec.html#arithmetic-operations) {#arithmetic-operations}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void _nmod_vec_add(nn_ptr res, nn_srcptr vec1, nn_srcptr vec2, slong len, nmod_t mod)` | [`ModAdd`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModAdd.html), [`ModAddAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModAddAssign.html) |
| ✓ | `void _nmod_vec_sub(nn_ptr res, nn_srcptr vec1, nn_srcptr vec2, slong len, nmod_t mod)` | [`ModSub`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModSub.html), [`ModSubAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModSubAssign.html) |
| ✓ | `void _nmod_vec_neg(nn_ptr res, nn_srcptr vec, slong len, nmod_t mod)` | [`ModNeg`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModNeg.html), [`ModNegAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModNegAssign.html) |
| ✗ | `void _nmod_vec_invert(nn_ptr res, nn_srcptr vec, slong len, nmod_t mod)` | |
| ✓ | `void _nmod_vec_scalar_mul_nmod(nn_ptr res, nn_srcptr vec, slong len, ulong c, nmod_t mod)` | [`ModMul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModMul.html), [`ModMulAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModMulAssign.html) |
| ⚙ | `void _nmod_vec_scalar_mul_nmod_shoup(nn_ptr res, nn_srcptr vec, slong len, ulong c, nmod_t mod)` | |
| ⚙ | `void _nmod_vec_scalar_mul_nmod_redc(nn_ptr res, nn_srcptr vec, slong len, ulong c, nmod_t mod)` | |
| ✓ | `void _nmod_vec_scalar_addmul_nmod(nn_ptr res, nn_srcptr vec, slong len, ulong c, nmod_t mod)` | [`ModAddMul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModAddMul.html), [`ModAddMulAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModAddMulAssign.html) |

Addition, subtraction, and negation are the
[`UnsignedVector`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/struct.UnsignedVector.html)
implementations of the `Mod*` traits, with the modulus as a `T` argument, as on [the
`fmpz_mod_vec` page](/mapping/flint-integer-vectors-mod-n/#arithmetic). Writing the result over an
operand is the `*_assign` form. Malachite panics if the dimensions differ or if any element is not
less than $$n$$; FLINT checks neither.

`_nmod_vec_invert` replaces each element by its inverse modulo $$n$$, assuming, without
checking, that every element is invertible. It does not invert each element separately: it
multiplies all the elements together, inverts the product once, and recovers each inverse from
partial products, so it computes one inverse and about three multiplications per element.
The vector types do not yet have this; until they do, apply
[`ModInverse`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModInverse.html) to each element, which returns
[`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None) for an
element that is not invertible, at the cost of one inverse per element.

`_nmod_vec_scalar_mul_nmod` multiplies each element by $$c$$, which is
[`ModMul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModMul.html)
of an
[`UnsignedVector`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/struct.UnsignedVector.html)
by a `T` modulo a `T`, or
[`ModMulAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModMulAssign.html)
when `res` is `vec`. It computes the data for multiplying modulo $$n$$ once and reuses it for every
element, and panics if $$c$$ or any element is not less than $$n$$. `_nmod_vec_scalar_addmul_nmod`
adds $$c$$ times each element of `vec` to the corresponding element of `res`, which is
[`ModAddMulAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModAddMulAssign.html) of an `UnsignedVector`, another `UnsignedVector`, a
`T` scalar, and a `T` modulus; [`ModAddMul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModAddMul.html) returns the result instead.
Like FLINT, it handles $$c$$ equal to 0, 1, or $$n - 1$$ directly and otherwise computes the data
for multiplying modulo $$n$$ once. It panics if the dimensions differ or if $$c$$ or any element is
not less than $$n$$. [`ModSubMul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModSubMul.html) and
[`ModSubMulAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModSubMulAssign.html) subtract the multiple instead, which FLINT has no
`nmod_vec` function for.

`_nmod_vec_scalar_mul_nmod` chooses among several algorithms: it handles $$c$$ equal to 0, 1,
or $$n - 1$$ directly; uses Montgomery multiplication (`_redc`) for an odd modulus of a full
64 bits and at least 8 elements; uses Shoup's method (`_shoup`), which precomputes a quotient for
$$c$$, for at least 10 elements and a modulus below $$2^{63}$$; and otherwise multiplies
directly. The `_shoup` and `_redc` functions expose two of those choices, hence ⚙: Malachite does
not expose its choice of multiplication algorithm, and `ModMul` above is the operation to call. Their preconditions differ from the general function's: `_shoup` requires
$$n < 2^{63}$$ but accepts unreduced elements, and `_redc` requires an odd $$n$$.

`nmod_vec.h` also declares several variants that the chapter does not document:
`_nmod_vec_scalar_mul_nmod_generic`, `_nmod_vec_scalar_addmul_nmod_generic`,
`_nmod_vec_scalar_addmul_nmod_shoup`, and `_nmod_vec_invert_naive`, `_nmod_vec_invert_generic`, and
`_nmod_vec_invert_shoup`. They have no rows here.

## [Arithmetic operations without reduction](https://flintlib.org/doc/nmod_vec.html#arithmetic-operations-without-reduction) {#arithmetic-operations-without-reduction}

| | FLINT | Malachite |
| :---: | --- | --- |
| ⚙ | `void _nmod_vec_nored_scalar_addmul_halflimb(nn_ptr res, nn_srcptr vec, slong len, ulong c)` | |
| ⚙ | `void _nmod_vec_nored_ll_scalar_addmul_halflimb(nn_ptr res, nn_srcptr vec, slong len, ulong c)` | |
| ⚙ | `void _nmod_vec_nored_ll_scalar_addmul(nn_ptr res, nn_srcptr vec, slong len, ulong c)` | |
| ⚙ | `void _nmod_vec_nored_lll_scalar_addmul(nn_ptr res, nn_srcptr vec, slong len, ulong c)` | |

These functions set $$r_i \gets r_i + v_i c$$ without reducing modulo anything; none of them takes
a modulus. The accumulators $$r_i$$ are one, two, or three words wide (`ll` and `lll` store each as
2 or 3 consecutive words), and the `halflimb` forms also require $$v_i$$ and $$c$$ to fit in half
a word, so each product fits in one word. The caller must guarantee that no accumulator overflows.

They are the building blocks of *delayed reduction*: an algorithm adds up many products in a wide
accumulator and reduces once at the end, instead of after every product. Within FLINT they are
used by classical polynomial division and by delayed-reduction LU decomposition of matrices, and
`_nmod_vec_nored_scalar_addmul_halflimb` uses AVX2 instructions when they are available. Malachite
does not expose its arithmetic at this level, hence ⚙; the user-level operation is the reduced
`_nmod_vec_scalar_addmul_nmod` (see [Arithmetic operations](#arithmetic-operations)), or the
higher-level operation that the delayed reduction serves.

## [Dot products](https://flintlib.org/doc/nmod_vec.html#dot-products) {#dot-products}

| | FLINT | Malachite |
| :---: | --- | --- |
| ⚙ | `dot_params_t` | |
| ⚙ | `dot_params_t _nmod_vec_dot_params(slong len, nmod_t mod)` | |
| ✓ | `ulong _nmod_vec_dot(nn_srcptr vec1, nn_srcptr vec2, slong len, nmod_t mod, dot_params_t params)` | [`ModDot`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModDot.html) |
| ✗ | `ulong _nmod_vec_dot_rev(nn_srcptr vec1, nn_srcptr vec2, slong len, nmod_t mod, dot_params_t params)` | |
| ⚙ | `ulong _nmod_vec_dot_strided(nn_srcptr vec1, slong stride1, nn_srcptr vec2, slong stride2, slong len, nmod_t mod, dot_params_t params)` | |
| ⚙ | `ulong _nmod_vec_dot_ptr(nn_srcptr vec1, const nn_ptr * vec2, slong offset, slong len, nmod_t mod, dot_params_t params)` | |
| ⚙ | `NMOD_VEC_DOT(res, i, len, expr1, expr2, mod, params)` | |
| ⚙ | `int _nmod_vec_dot_bound_limbs(slong len, nmod_t mod)` | |
| ⚙ | `int _nmod_vec_dot_bound_limbs_from_params(slong len, nmod_t mod, dot_params_t params)` | |

FLINT computes a modular dot product without reducing until the end, accumulating the unreduced
sum in one, two, or three words. Which method it uses depends on the modulus and on how large the
unreduced sum can get, $$\mathit{len} \cdot (n - 1)^2$$; for moduli below about $$2^{30.5}$$, or
powers of 2 up to $$2^{32}$$, it can use single-word arithmetic and AVX2 instructions.
`_nmod_vec_dot_params` makes that choice once for a given modulus and maximum length and stores
it in a `dot_params_t`, which every dot product function then takes, and
`_nmod_vec_dot_bound_limbs` and `_nmod_vec_dot_bound_limbs_from_params` return the number of
words, 0 to 3, that the unreduced sum needs. All of this is the machinery of choosing an
algorithm, which Malachite does not expose, hence ⚙.

`_nmod_vec_dot` is $$\sum_i a_i b_i \bmod n$$, and `_nmod_vec_dot_rev` pairs $$a_i$$ with
$$b_{\mathit{len}-1-i}$$, reversing the second vector. `_nmod_vec_dot` is
[`ModDot`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModDot.html)'s `v.mod_dot(w, n)` for `UnsignedVector`, which accumulates the unreduced sum in
one, two, or three words, as FLINT does, and reduces once; it chooses the number of words for each
call, so there is nothing like `dot_params_t` to pass. For a power-of-2 modulus,
[`ModPowerOf2Dot`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPowerOf2Dot.html)'s `v.mod_power_of_2_dot(w, k)` sums the products in wrapping arithmetic
and keeps the low $$k$$ bits. The reversed form has no counterpart yet; until it does, start from
0 and, for each pair of elements, multiply with
[`ModMulPrecomputed`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModMulPrecomputed.html)
and add with
[`ModAddAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModAddAssign.html),
iterating over the second vector in reverse.

The remaining three read their elements in other patterns: `_nmod_vec_dot_strided` takes every
`stride1`-th and `stride2`-th element (the strides may be negative), `_nmod_vec_dot_ptr` takes
`vec2[i][offset]` from an array of rows, and the macro `NMOD_VEC_DOT` evaluates two arbitrary
expressions for each index. They serve FLINT's matrix and polynomial code, where the elements to
be multiplied are not contiguous. In Rust such patterns are iterators, such as
[`step_by`](https://doc.rust-lang.org/nightly/core/iter/trait.Iterator.html#method.step_by) or a
[`map`](https://doc.rust-lang.org/nightly/core/iter/trait.Iterator.html#method.map) over the
rows, so a dot product over iterators would cover all three; they are ⚙ rather than separate
operations.
