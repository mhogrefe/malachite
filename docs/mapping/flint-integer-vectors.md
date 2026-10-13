---
layout: default
title: "Malachite for FLINT Users: Integer Vectors"
permalink: /mapping/flint-integer-vectors/
theme: jekyll-theme-slate
---

# Malachite for FLINT Users: Integer Vectors

This page maps the functions of FLINT's integer vector module, `fmpz_vec.h`, onto their Malachite
counterpart:
[`IntegerVector`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/struct.IntegerVector.html),
from the `malachite-nz` crate. It follows the organization of the [fmpz_vec.h
chapter](https://flintlib.org/doc/fmpz_vec.html) of the FLINT manual, as of FLINT 3.6.0, and is a
companion to [Malachite for FLINT Users: Integers](/mapping/flint-integers/), whose
[conventions](/mapping/flint-integers/#conventions), including word types and aliasing, apply here
as well; the [mapping index](/mapping/) lists the whole family.

Every section of the chapter is mapped below: 78 rows across 15 sections, of which 60 are ✓, 2 ⚙,
7 ≈, 8 —, and 1 ✗. Functions whose names begin with an underscore are included, since they make
up most of the chapter (see [Underscore functions](#underscore-functions)). Two functions that
`fmpz_vec.h` declares but the chapter does not document, `_fmpz_vec_min` and
`_fmpz_vec_min_inplace`, have no rows (see [Comparison](#comparison)).

The one ✗ row is `_fmpz_vec_dot_general`, the dot product with an initial term, a subtraction
flag, and a reversal flag; its note gives the interim way to compute it.

## Conventions {#conventions}

### Two kinds of vector {#representation}

Most of `fmpz_vec.h` works on a bare array: an `fmpz *` together with a separate `slong` length,
which the caller allocates, tracks, and frees. Only the [Resizable vectors](#resizable-vectors)
section has a vector object, `fmpz_vec_t`, which holds `entries`, `alloc`, and `length` in one
struct.

Malachite has one type for both.
[`IntegerVector`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/struct.IntegerVector.html)
is a single public field,
[`elements`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/struct.IntegerVector.html#structfield.elements),
of type
[`Vec<Integer>`](https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html), whose pointer,
capacity, and length are the counterparts of `entries`, `alloc`, and `length`. Every
[`Vec`](https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html) method is therefore available
through the field, and a function that takes an `(fmpz *, slong)` pair becomes a method on
[`IntegerVector`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/struct.IntegerVector.html)
or a function on a slice
[`&[Integer]`](https://doc.rust-lang.org/nightly/core/primitive.slice.html). The construction and
access methods common to all of Malachite's vector types are in the
[`Vector`](https://docs.rs/malachite-base/latest/malachite_base/vector/trait.Vector.html) trait.

### Underscore functions {#underscore-functions}

The other FLINT pages omit functions whose names begin with an underscore. This page does not:
the bare-array functions are most of the chapter, and nearly all of them have underscore names.

### Elements that are not `Integer`s

Malachite has four vector types differing only in their elements:
[`IntegerVector`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/struct.IntegerVector.html)
over $$\mathbb{Z}$$,
[`NaturalVector`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_vector/struct.NaturalVector.html)
over $$\mathbb{N}$$,
[`UnsignedVector`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/struct.UnsignedVector.html)
with primitive unsigned elements (in `malachite-base`), and
[`RationalVector`](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/struct.RationalVector.html)
over $$\mathbb{Q}$$. They are the counterparts of FLINT's `fmpz_vec`, `fmpz_mod_vec`, `nmod_vec`,
and `fmpq_vec` modules respectively:
[`NaturalVector`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_vector/struct.NaturalVector.html)
and
[`UnsignedVector`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/struct.UnsignedVector.html)
also hold residues for the `Mod*` and `ModPowerOf2*` traits, which check that their arguments are
reduced. Every row below names the
[`IntegerVector`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/struct.IntegerVector.html)
version; the memory-management and resizing rows apply unchanged to the other three.

### Categories

Each function falls into one of five categories:

| | meaning |
| :---: | --- |
| ✓ | A Malachite function does the same thing. |
| ≈ | A Malachite function serves the same purpose, but its specification differs. The notes say how. |
| ⚙ | Malachite does not expose this algorithm or helper; the Malachite column or the notes say what to call instead. |
| — | No counterpart is needed, either because Rust handles it for you or because it is outside Malachite's scope. The notes say which. |
| ✗ | Malachite does not fully support this yet, but will in a future version. |

## [Memory management](https://flintlib.org/doc/fmpz_vec.html#memory-management) {#memory-management}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `fmpz * _fmpz_vec_init(slong len)` | [`zero`](https://docs.rs/malachite-base/latest/malachite_base/vector/trait.Vector.html#tymethod.zero) |
| — | `void _fmpz_vec_clear(fmpz * vec, slong len)` | |

`_fmpz_vec_init` returns `len` zeros (it is a `calloc`, and a zeroed `fmpz` is $$0$$), and
`fmpz_vec_init` does the same for the resizable vector. Both are
[`zero`](https://docs.rs/malachite-base/latest/malachite_base/vector/trait.Vector.html#tymethod.zero), the
[`Vector`](https://docs.rs/malachite-base/latest/malachite_base/vector/trait.Vector.html) method
that returns the zero vector of a given dimension: `IntegerVector::zero(len)`. The length travels
with the result, so there is no separate `len` to keep. `_fmpz_vec_clear` is
[`Drop`](https://doc.rust-lang.org/nightly/core/ops/trait.Drop.html), which runs when the vector goes out of scope.

## [Resizable vectors](https://flintlib.org/doc/fmpz_vec.html#resizable-vectors) {#resizable-vectors}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `fmpz_vec_struct` | [`IntegerVector`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/struct.IntegerVector.html) |
| — | `fmpz_vec_t` | |
| ✓ | `void fmpz_vec_init(fmpz_vec_t vec, slong len)` | [`zero`](https://docs.rs/malachite-base/latest/malachite_base/vector/trait.Vector.html#tymethod.zero) |
| — | `void fmpz_vec_clear(fmpz_vec_t vec)` | |
| ✓ | `fmpz * fmpz_vec_entry_ptr(fmpz_vec_t vec, slong i)` | [`IndexMut`](https://doc.rust-lang.org/nightly/core/ops/trait.IndexMut.html) |
| ✓ | `const fmpz * fmpz_vec_entry_srcptr(const fmpz_vec_t vec, slong i)` | [`Index`](https://doc.rust-lang.org/nightly/core/ops/trait.Index.html) |
| ✓ | `slong fmpz_vec_length(const fmpz_vec_t vec)` | [`dimension`](https://docs.rs/malachite-base/latest/malachite_base/vector/trait.Vector.html#tymethod.dimension) |
| — | `void fmpz_vec_fit_length(fmpz_vec_t vec, slong len)` | |
| ✓ | `void fmpz_vec_set_length(fmpz_vec_t vec, slong len)` | [`set_dimension`](https://docs.rs/malachite-base/latest/malachite_base/vector/trait.Vector.html#tymethod.set_dimension) |
| ✓ | `void fmpz_vec_set(fmpz_vec_t res, const fmpz_vec_t src)` | [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html) |
| ✓ | `void fmpz_vec_append(fmpz_vec_t vec, const fmpz_t f)` | [`push`](https://docs.rs/malachite-base/latest/malachite_base/vector/trait.Vector.html#tymethod.push) |
| ✓ | `void fmpz_vec_append_ui(fmpz_vec_t vec, ulong f)` | [`push`](https://docs.rs/malachite-base/latest/malachite_base/vector/trait.Vector.html#tymethod.push) |

`fmpz_vec_t` is the pass-by-reference device that `&` and `&mut` already are, and
`fmpz_vec_clear` is [`Drop`](https://doc.rust-lang.org/nightly/core/ops/trait.Drop.html). FLINT also
documents that an `fmpz_vec_t` may be cast to a `gr_vec_t`, its generic-ring vector; Malachite
has no generic-ring layer, so that has no counterpart.

The two entry functions are `v[i]`, through
[`IndexMut`](https://doc.rust-lang.org/nightly/core/ops/trait.IndexMut.html) and
[`Index`](https://doc.rust-lang.org/nightly/core/ops/trait.Index.html) with a
[`usize`](https://doc.rust-lang.org/nightly/core/primitive.usize.html) index. FLINT requires the
index to be in bounds and does not check it; Malachite panics if it is out of bounds.
[`dimension`](https://docs.rs/malachite-base/latest/malachite_base/vector/trait.Vector.html#tymethod.dimension)
returns a [`u64`](https://doc.rust-lang.org/nightly/core/primitive.u64.html).

`fmpz_vec_set` is [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html);
[`clone_from`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)
reuses the destination's allocation, as `fmpz_vec_set` does.

`fmpz_vec_append` and `fmpz_vec_append_ui` are
[`push`](https://docs.rs/malachite-base/latest/malachite_base/vector/trait.Vector.html#tymethod.push), the
[`Vector`](https://docs.rs/malachite-base/latest/malachite_base/vector/trait.Vector.html) method that
appends one element, with `Integer::from(f)` for the
[`ulong`](https://doc.rust-lang.org/nightly/core/primitive.u64.html) version. To append many
elements, [`Extend`](https://doc.rust-lang.org/nightly/core/iter/trait.Extend.html) takes them
from any iterator.

`fmpz_vec_set_length` is
[`set_dimension`](https://docs.rs/malachite-base/latest/malachite_base/vector/trait.Vector.html#tymethod.set_dimension),
which pads with zeros when growing and discards the trailing elements when shrinking, as
`fmpz_vec_set_length` does. `fmpz_vec_fit_length`, which only reserves room without changing the
length, is a memory-management detail; if it matters,
[`reserve`](https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html#method.reserve) on the
[`elements`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/struct.IntegerVector.html#structfield.elements)
field does it. `fmpz_vec_fit_length(vec, len)` takes the total number of elements to make room
for, and `reserve` takes the number beyond the current length, so the equivalent call is
`v.elements.reserve(len.saturating_sub(v.elements.len()))`.

## [Randomisation](https://flintlib.org/doc/fmpz_vec.html#randomisation) {#randomisation}

| | FLINT | Malachite |
| :---: | --- | --- |
| ≈ | `void _fmpz_vec_randtest(fmpz * f, flint_rand_t state, slong len, flint_bitcnt_t bits)` | [`random_integer_vectors_with_dimension`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/random/fn.random_integer_vectors_with_dimension.html), [`random_integer_vectors_with_dimension_from_iterator`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/random/fn.random_integer_vectors_with_dimension_from_iterator.html), [`striped_random_integer_vectors_with_dimension`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/random/fn.striped_random_integer_vectors_with_dimension.html) |
| ≈ | `void _fmpz_vec_randtest_unsigned(fmpz * f, flint_rand_t state, slong len, flint_bitcnt_t bits)` | [`random_natural_vectors_with_dimension`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_vector/random/fn.random_natural_vectors_with_dimension.html), [`random_natural_vectors_with_dimension_from_iterator`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_vector/random/fn.random_natural_vectors_with_dimension_from_iterator.html), [`striped_random_natural_vectors_with_dimension`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_vector/random/fn.striped_random_natural_vectors_with_dimension.html) |

Both FLINT functions fill a vector of length `len` and draw each element with
[`fmpz_randtest`](/mapping/flint-integers/#random-generation) or `fmpz_randtest_unsigned`, which cap
its bit length at `bits`. Malachite's counterparts are infinite iterators of vectors of a given
dimension, whose elements come from
[`random_integers`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/random/fn.random_integers.html)
or
[`random_naturals`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/random/fn.random_naturals.html)
(or their striped forms). Those take a *mean* bit length, with no cap, so the distributions
differ, as they do for [single integers](/mapping/flint-integers/#random-generation).

The distributions also differ in a second way. Half the time, FLINT makes the vector sparse: it
draws $$s$$ uniformly from $$1$$ to $$\max(2, \mathit{len})$$, and then each element is zero with
probability $$1 - 1/s$$. Malachite's generators have no sparse mode; every element is drawn the
same way. `randtest_unsigned` is a change of type, to
[`NaturalVector`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_vector/struct.NaturalVector.html).

For a hard bound on the elements, pass a bounded element stream to
[`random_integer_vectors_with_dimension_from_iterator`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/random/fn.random_integer_vectors_with_dimension_from_iterator.html),
which takes the elements of each vector from any iterator. For example,
[`uniform_random_integer_inclusive_range`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/random/fn.uniform_random_integer_inclusive_range.html)
with the range $$[-(2^{\mathit{bits}} - 1), 2^{\mathit{bits}} - 1]$$ caps the bit length at `bits`,
as FLINT does, though it is uniform over values rather than over bit lengths, so most elements
have close to `bits` bits. To let the dimension vary as well, use
[`random_integer_vectors_from_iterator`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/random/fn.random_integer_vectors_from_iterator.html)
or
[`random_integer_vectors`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/random/fn.random_integer_vectors.html)
and its siblings, which draw the dimension from a geometric distribution with a given mean.

## [Bit sizes and norms](https://flintlib.org/doc/fmpz_vec.html#bit-sizes-and-norms) {#bit-sizes-and-norms}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `slong _fmpz_vec_max_bits(const fmpz * vec, slong len)` | [`max_bits`](https://docs.rs/malachite-base/latest/malachite_base/vector/trait.Vector.html#tymethod.max_bits) |
| — | `slong _fmpz_vec_max_bits_ref(const fmpz * vec, slong len)` | |
| ✓ | `void _fmpz_vec_sum_max_bits(slong * sumabs, slong * maxabs, const fmpz * vec, slong len)` | [`sum_max_bits`](https://docs.rs/malachite-base/latest/malachite_base/vector/trait.Vector.html#tymethod.sum_max_bits) |
| — | `slong _fmpz_vec_max_limbs(const fmpz * vec, slong len)` | |
| ✓ | `void _fmpz_vec_height(fmpz_t height, const fmpz * vec, slong len)` | [`Height`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Height.html), [`HeightRef`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.HeightRef.html) |
| ✓ | `slong _fmpz_vec_height_index(const fmpz * vec, slong len)` | [`height_index`](https://docs.rs/malachite-base/latest/malachite_base/vector/trait.Vector.html#tymethod.height_index) |

The height of a vector is the largest absolute value of its elements, its $$\ell^\infty$$ norm, and
$$0$$ for the 0-dimensional vector. `_fmpz_vec_height` is
[`Height`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Height.html)'s
[`to_height`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Height.html#tymethod.to_height),
or
[`into_height`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Height.html#tymethod.into_height)
to move the element out instead of cloning it; [`HeightRef`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.HeightRef.html) lends it instead, since the height is the
absolute value of one of the elements. All four vector types implement
[`Height`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Height.html), as the polynomial types do (see [the polynomial
page](/mapping/flint-integer-polynomials/#bit-sizes-and-norms)).

`max_bits` returns the bit length $$b$$ of the height, *negated* when any element is negative.
[`max_bits`](https://docs.rs/malachite-base/latest/malachite_base/vector/trait.Vector.html#tymethod.max_bits),
a [`Vector`](https://docs.rs/malachite-base/latest/malachite_base/vector/trait.Vector.html)
method, returns the same two facts as a pair, `(b, negative)`; the bit count alone is
[`height_significant_bits`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Height.html#tymethod.height_significant_bits).
Both use FLINT's method, or-ing together the top limbs of the elements with the most limbs, so no
element's bit length is computed separately.

`sum_max_bits` returns two bit lengths: that of the sum of the absolute values (the $$\ell^1$$
norm), and that of the height. It is
[`sum_max_bits`](https://docs.rs/malachite-base/latest/malachite_base/vector/trait.Vector.html#tymethod.sum_max_bits),
which returns them as a pair in the same order. Like FLINT's, it adds the elements in two words
while each fits in one, and computes the sum exactly only when an element does not. The norm
itself is
[`to_l1_norm`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.L1Norm.html#tymethod.to_l1_norm),
and its bit length alone is
[`l1_norm_significant_bits`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.L1Norm.html#tymethod.l1_norm_significant_bits).

`height_index` is [`height_index`](https://docs.rs/malachite-base/latest/malachite_base/vector/trait.Vector.html#tymethod.height_index), a
[`Vector`](https://docs.rs/malachite-base/latest/malachite_base/vector/trait.Vector.html)
method. Both return the index of the first element of largest absolute value when several tie
(the manual does not say which; this is what FLINT's implementation does). FLINT requires a
nonempty vector, while `height_index` returns
[`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None) for the
0-dimensional one.

`max_bits_ref` is a slower reference implementation of `max_bits`, kept for testing. Malachite
keeps reference implementations in its test code rather than its public API. `max_limbs` is a
memory-estimation helper; limbs are an implementation detail in Malachite.

## [Input and output](https://flintlib.org/doc/fmpz_vec.html#input-and-output) {#input-and-output}

As on [the fmpz page](/mapping/flint-integers/#input-and-output), Malachite has no `FILE *`
functions: formatting and parsing work with any Rust reader or writer.

| | FLINT | Malachite |
| :---: | --- | --- |
| ≈ | `int _fmpz_vec_fread(FILE * file, fmpz ** vec, slong * len)` | [`FromStr`](https://doc.rust-lang.org/nightly/core/str/trait.FromStr.html), [`Deserialize`](https://docs.rs/serde/latest/serde/trait.Deserialize.html) |
| ≈ | `int _fmpz_vec_read(fmpz ** vec, slong * len)` | [`FromStr`](https://doc.rust-lang.org/nightly/core/str/trait.FromStr.html), [`Deserialize`](https://docs.rs/serde/latest/serde/trait.Deserialize.html) |
| ≈ | `int _fmpz_vec_fprint(FILE * file, const fmpz * vec, slong len)` | [`Display`](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html), [`Serialize`](https://docs.rs/serde/latest/serde/trait.Serialize.html) |
| ≈ | `int _fmpz_vec_print(const fmpz * vec, slong len)` | [`Display`](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html), [`Serialize`](https://docs.rs/serde/latest/serde/trait.Serialize.html) |

The formats differ, hence ≈. FLINT writes the length, two spaces, and then the elements separated
by single spaces, so $$(1, -2, 3)$$ is `3  1 -2 3` and the empty vector is `0`; this is the same
as `fmpz_poly`'s plain format. Malachite's
[`Display`](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html) writes `(1, -2, 3)`,
with `()` for the 0-dimensional vector, and
[`FromStr`](https://doc.rust-lang.org/nightly/core/str/trait.FromStr.html) reads exactly that
form back, returning an `Err` on failure where FLINT returns a non-positive value. Neither
writes the length. With the `serde` feature,
[`IntegerVector`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/struct.IntegerVector.html)
is serialized as a plain list of its elements.

`_fmpz_vec_fread` either allocates a vector of the length it reads or, given an existing vector,
fails unless the length read matches. Parsing in Malachite always produces a new vector; to
require a particular dimension, compare
[`dimension`](https://docs.rs/malachite-base/latest/malachite_base/vector/trait.Vector.html#tymethod.dimension)
afterwards. The `print`/`fprint` and `read`/`fread` splits disappear because
[`Display`](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html) writes to any
[`Formatter`](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html), and parsing
works on any string, however it was read.

## [Conversions](https://flintlib.org/doc/fmpz_vec.html#conversions) {#conversions}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void _fmpz_vec_get_nmod_vec(nn_ptr res, const fmpz * poly, slong len, nmod_t mod)` | [`Mod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Mod.html) |
| ≈ | `void _fmpz_vec_set_nmod_vec(fmpz * res, nn_srcptr poly, slong len, nmod_t mod)` | [`BalancedMod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.BalancedMod.html) |
| ⚙ | `void _fmpz_vec_get_fft(ulong ** coeffs_f, const fmpz * coeffs_m, slong l, slong length)` | [`Mul`](https://doc.rust-lang.org/nightly/core/ops/trait.Mul.html) |
| ⚙ | `void _fmpz_vec_set_fft(fmpz * coeffs_m, slong length, const nn_ptr * coeffs_f, slong limbs, slong sign)` | [`Mul`](https://doc.rust-lang.org/nightly/core/ops/trait.Mul.html) |
| — | `slong _fmpz_vec_get_d_vec_2exp(double * appv, const fmpz * vec, slong len)` | |

Malachite has no `nmod_vec` type: a vector of residues modulo a word-sized $$n$$ is an
[`UnsignedVector<u64>`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/struct.UnsignedVector.html)
whose elements are less than $$n$$. `get_nmod_vec` reduces each element modulo $$n$$ into
$$[0, n)$$; it is
[`Mod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Mod.html)
with a primitive unsigned modulus, `v.mod_op(n)`, which returns an `UnsignedVector` of the
modulus's type. The same reduction modulo a
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html) returns
a
[`NaturalVector`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_vector/struct.NaturalVector.html),
and a `NaturalVector` can be reduced modulo a primitive unsigned modulus in the same way.

`set_nmod_vec` converts each residue to its balanced representative, as
[`BalancedMod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.BalancedMod.html)
does for a single value (see [`fmpz_set_ui_smod`](/mapping/flint-integers/#conversion)). For
vectors this is `BalancedMod` on a `NaturalVector`, which returns an `IntegerVector`; convert the
`UnsignedVector` of residues to a `NaturalVector` with
[`From`](https://doc.rust-lang.org/nightly/core/convert/trait.From.html) first, hence ≈. Unlike
`set_nmod_vec`, it accepts elements that are not yet reduced. The manual gives the balanced range
as $$[-n/2, n/2)$$, but the implementation (`fmpz_set_ui_smod`) keeps $$x$$ when $$x \le n/2$$, so
the range is $$(-n/2, n/2]$$, the same as
[`BalancedMod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.BalancedMod.html)'s.
The two differ only for even $$n$$, where FLINT returns $$n/2$$, not $$-n/2$$.

`get_fft` and `set_fft` convert between integers and the residues modulo a Fermat number
$$2^{wl} + 1$$ that Schönhage–Strassen polynomial multiplication works with. They are steps
inside multiplication, and Malachite keeps them internal; multiply the polynomials instead, with
[`Mul`](https://doc.rust-lang.org/nightly/core/ops/trait.Mul.html) (see [the polynomial
page](/mapping/flint-integer-polynomials/#multiplication)). In FLINT 3.6.0, `set_fft` gives the
wrong sign to a negative coefficient whose top limb is exactly half the limb range; this was fixed
in FLINT's development branch after the 3.6.0 release.

`get_d_vec_2exp` approximates every element by a
[`f64`](https://doc.rust-lang.org/nightly/core/primitive.f64.html) scaled by one shared power of
2: it returns the largest element exponent $$e$$, and element $$i$$ is approximately
$$\mathit{appv}_i \cdot 2^e$$ with $$|\mathit{appv}_i| \le 1$$. For a single value this is
[`SciMantissaAndExponent`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.SciMantissaAndExponent.html)
(see [`fmpz_get_d_2exp`](/mapping/flint-integers/#conversion)), whose mantissas lie in
$$[1, 2)$$ rather than FLINT's $$[1/2, 1)$$. The shared-exponent vector form is outside
Malachite's scope, hence —; apply
[`sci_mantissa_and_exponent`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.SciMantissaAndExponent.html#tymethod.sci_mantissa_and_exponent)
to the absolute value of each element and rescale by the largest exponent.

## [Assignment and basic manipulation](https://flintlib.org/doc/fmpz_vec.html#assignment-and-basic-manipulation) {#assignment-and-basic-manipulation}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void _fmpz_vec_set(fmpz * vec1, const fmpz * vec2, slong len2)` | [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html) |
| ✓ | `void _fmpz_vec_swap(fmpz * vec1, fmpz * vec2, slong len2)` | [`mem::swap`](https://doc.rust-lang.org/nightly/core/mem/fn.swap.html) |
| ✓ | `void _fmpz_vec_zero(fmpz * vec, slong len)` | [`set_zero`](https://docs.rs/malachite-base/latest/malachite_base/vector/trait.Vector.html#tymethod.set_zero) |
| ✓ | `void _fmpz_vec_neg(fmpz * vec1, const fmpz * vec2, slong len2)` | [`Neg`](https://doc.rust-lang.org/nightly/core/ops/trait.Neg.html), [`NegAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.NegAssign.html) |
| ✓ | `void _fmpz_vec_scalar_abs(fmpz * vec1, const fmpz * vec2, slong len2)` | [`EntrywiseAbs`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.EntrywiseAbs.html), [`EntrywiseAbsAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.EntrywiseAbsAssign.html), [`EntrywiseUnsignedAbs`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.EntrywiseUnsignedAbs.html) |

`_fmpz_vec_set` is [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html), as
`fmpz_vec_set` is in [Resizable vectors](#resizable-vectors).
`_fmpz_vec_swap` exchanges the contents of two vectors of the same length one element at a time;
[`mem::swap`](https://doc.rust-lang.org/nightly/core/mem/fn.swap.html) exchanges two
[`IntegerVector`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/struct.IntegerVector.html)s
of any dimensions in constant time. `_fmpz_vec_neg` with `vec1` equal to `vec2` negates in place,
which is
[`NegAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.NegAssign.html);
otherwise it is `-&v`, or `-v` when `v` is no longer needed.

`_fmpz_vec_zero` sets every element to zero while keeping the dimension, which is
[`set_zero`](https://docs.rs/malachite-base/latest/malachite_base/vector/trait.Vector.html#tymethod.set_zero),
a [`Vector`](https://docs.rs/malachite-base/latest/malachite_base/vector/trait.Vector.html) method.
`_fmpz_vec_scalar_abs` replaces every element by its absolute value, which is
[`EntrywiseAbs`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.EntrywiseAbs.html),
or [`EntrywiseAbsAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.EntrywiseAbsAssign.html)
when `vec1` equals `vec2`. The name is not
[`Abs`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Abs.html), which for a vector would be its
Euclidean length. Since every absolute value is nonnegative, the result can also be a
[`NaturalVector`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_vector/struct.NaturalVector.html),
which is [`EntrywiseUnsignedAbs`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.EntrywiseUnsignedAbs.html),
as [`UnsignedAbs`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.UnsignedAbs.html) is for a single
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html).

## [Comparison](https://flintlib.org/doc/fmpz_vec.html#comparison) {#comparison}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `int _fmpz_vec_equal(const fmpz * vec1, const fmpz * vec2, slong len)` | [`PartialEq`](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html) |
| ✓ | `int _fmpz_vec_is_zero(const fmpz * vec, slong len)` | [`is_zero`](https://docs.rs/malachite-base/latest/malachite_base/vector/trait.Vector.html#tymethod.is_zero) |
| ✓ | `void _fmpz_vec_max(fmpz * vec1, const fmpz * vec2, const fmpz * vec3, slong len)` | [`EntrywiseMax`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.EntrywiseMax.html) |
| ✓ | `void _fmpz_vec_max_inplace(fmpz * vec1, const fmpz * vec2, slong len)` | [`EntrywiseMaxAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.EntrywiseMaxAssign.html) |

`_fmpz_vec_equal` compares two arrays over a length the caller supplies;
[`PartialEq`](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html) on
[`IntegerVector`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/struct.IntegerVector.html)
also requires the dimensions to match, so vectors of different dimensions are unequal. The vector
types can also be compared with one another, so an
[`IntegerVector`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/struct.IntegerVector.html)
equals a
[`NaturalVector`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_vector/struct.NaturalVector.html)
or
[`UnsignedVector`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/struct.UnsignedVector.html)
with the same elements.

`_fmpz_vec_is_zero` is true when every element is zero, including for the empty vector. It is
[`is_zero`](https://docs.rs/malachite-base/latest/malachite_base/vector/trait.Vector.html#tymethod.is_zero),
a [`Vector`](https://docs.rs/malachite-base/latest/malachite_base/vector/trait.Vector.html)
method, which agrees on the empty vector. Malachite also has
[`standard_basis_index`](https://docs.rs/malachite-base/latest/malachite_base/vector/trait.Vector.html#tymethod.standard_basis_index),
which FLINT lacks: it returns the index of the 1 in a standard basis vector, and `None` for any
other vector.

`_fmpz_vec_max` sets each element of the result to the larger of the corresponding elements of
two vectors, and `_fmpz_vec_max_inplace` does the same with the result in the first vector. These
are [`EntrywiseMax`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.EntrywiseMax.html) and
[`EntrywiseMaxAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.EntrywiseMaxAssign.html),
which every vector type implements, in every combination of value and reference. The vectors
must have the same dimension.

`fmpz_vec.h` also declares the pointwise minimum functions `_fmpz_vec_min` and
`_fmpz_vec_min_inplace`, which the chapter does not document; they have no rows here. Their
counterparts are [`EntrywiseMin`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.EntrywiseMin.html) and
[`EntrywiseMinAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.EntrywiseMinAssign.html).

## [Sorting](https://flintlib.org/doc/fmpz_vec.html#sorting) {#sorting}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void _fmpz_vec_sort(fmpz * vec, slong len)` | [`sort`](https://docs.rs/malachite-base/latest/malachite_base/vector/trait.Vector.html#tymethod.sort) |

`_fmpz_vec_sort` sorts the elements in ascending order, using the C library's `qsort` with
`fmpz_cmp`. It is
[`sort`](https://docs.rs/malachite-base/latest/malachite_base/vector/trait.Vector.html#tymethod.sort),
a [`Vector`](https://docs.rs/malachite-base/latest/malachite_base/vector/trait.Vector.html)
method, which sorts in place with
[`sort_unstable`](https://doc.rust-lang.org/nightly/core/primitive.slice.html#method.sort_unstable).
Stability does not matter here: equal integers are indistinguishable.

## [Addition and subtraction](https://flintlib.org/doc/fmpz_vec.html#addition-and-subtraction) {#addition-and-subtraction}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void _fmpz_vec_add(fmpz * res, const fmpz * vec1, const fmpz * vec2, slong len2)` | [`Add`](https://doc.rust-lang.org/nightly/core/ops/trait.Add.html), [`AddAssign`](https://doc.rust-lang.org/nightly/core/ops/trait.AddAssign.html) |
| ✓ | `void _fmpz_vec_sub(fmpz * res, const fmpz * vec1, const fmpz * vec2, slong len2)` | [`Sub`](https://doc.rust-lang.org/nightly/core/ops/trait.Sub.html), [`SubAssign`](https://doc.rust-lang.org/nightly/core/ops/trait.SubAssign.html) |

Both operations work element by element, and Malachite panics if the two vectors have different
dimensions; FLINT assumes, without checking, that both arrays have `len2` elements. Calling
`_fmpz_vec_add` or `_fmpz_vec_sub` with `res` equal to `vec1` is `v += w` or `v -= w`. Each
operator also takes either operand by value or by reference, and taking an operand by value lets
Malachite reuse its storage for the result, as `&v - w` does.

The modular counterparts, which belong to FLINT's `fmpz_mod_vec` and `nmod_vec` modules, are
[`ModAdd`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModAdd.html)
and
[`ModSub`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModSub.html)
(and their power-of-2 forms) on
[`NaturalVector`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_vector/struct.NaturalVector.html)
and
[`UnsignedVector`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/struct.UnsignedVector.html).

## [Scalar multiplication and division](https://flintlib.org/doc/fmpz_vec.html#scalar-multiplication-and-division) {#scalar-multiplication-and-division}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void _fmpz_vec_scalar_mul_fmpz(fmpz * vec1, const fmpz * vec2, slong len2, const fmpz_t x)` | [`Mul`](https://doc.rust-lang.org/nightly/core/ops/trait.Mul.html), [`MulAssign`](https://doc.rust-lang.org/nightly/core/ops/trait.MulAssign.html) |
| ✓ | `void _fmpz_vec_scalar_mul_si(fmpz * vec1, const fmpz * vec2, slong len2, slong c)` | [`Mul`](https://doc.rust-lang.org/nightly/core/ops/trait.Mul.html), [`MulAssign`](https://doc.rust-lang.org/nightly/core/ops/trait.MulAssign.html) |
| ✓ | `void _fmpz_vec_scalar_mul_ui(fmpz * vec1, const fmpz * vec2, slong len2, ulong c)` | [`Mul`](https://doc.rust-lang.org/nightly/core/ops/trait.Mul.html), [`MulAssign`](https://doc.rust-lang.org/nightly/core/ops/trait.MulAssign.html) |
| ✓ | `void _fmpz_vec_scalar_mul_2exp(fmpz * vec1, const fmpz * vec2, slong len2, ulong exp)` | [`Shl`](https://doc.rust-lang.org/nightly/core/ops/trait.Shl.html), [`ShlAssign`](https://doc.rust-lang.org/nightly/core/ops/trait.ShlAssign.html) |
| ✓ | `void _fmpz_vec_scalar_divexact_fmpz(fmpz * vec1, const fmpz * vec2, slong len2, const fmpz_t x)` | [`DivExact`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivExact.html), [`DivExactAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivExactAssign.html) |
| ✓ | `void _fmpz_vec_scalar_divexact_si(fmpz * vec1, const fmpz * vec2, slong len2, slong c)` | [`DivExact`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivExact.html), [`DivExactAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivExactAssign.html) |
| ✓ | `void _fmpz_vec_scalar_divexact_ui(fmpz * vec1, const fmpz * vec2, slong len2, ulong c)` | [`DivExact`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivExact.html), [`DivExactAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivExactAssign.html) |
| ✓ | `void _fmpz_vec_scalar_fdiv_q_fmpz(fmpz * vec1, const fmpz * vec2, slong len2, const fmpz_t c)` | [`EntrywiseDivRound`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.EntrywiseDivRound.html), [`EntrywiseDivRoundAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.EntrywiseDivRoundAssign.html) |
| ✓ | `void _fmpz_vec_scalar_fdiv_q_si(fmpz * vec1, const fmpz * vec2, slong len2, slong c)` | [`EntrywiseDivRound`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.EntrywiseDivRound.html), [`EntrywiseDivRoundAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.EntrywiseDivRoundAssign.html) |
| ✓ | `void _fmpz_vec_scalar_fdiv_q_ui(fmpz * vec1, const fmpz * vec2, slong len2, ulong c)` | [`EntrywiseDivRound`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.EntrywiseDivRound.html), [`EntrywiseDivRoundAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.EntrywiseDivRoundAssign.html) |
| ✓ | `void _fmpz_vec_scalar_fdiv_q_2exp(fmpz * vec1, const fmpz * vec2, slong len2, ulong exp)` | [`Shr`](https://doc.rust-lang.org/nightly/core/ops/trait.Shr.html), [`ShrAssign`](https://doc.rust-lang.org/nightly/core/ops/trait.ShrAssign.html) |
| ✓ | `void _fmpz_vec_scalar_fdiv_r_2exp(fmpz * vec1, const fmpz * vec2, slong len2, ulong exp)` | [`ModPowerOf2`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPowerOf2.html) |
| ✓ | `void _fmpz_vec_scalar_tdiv_q_fmpz(fmpz * vec1, const fmpz * vec2, slong len2, const fmpz_t c)` | [`Div`](https://doc.rust-lang.org/nightly/core/ops/trait.Div.html), [`DivAssign`](https://doc.rust-lang.org/nightly/core/ops/trait.DivAssign.html) |
| ✓ | `void _fmpz_vec_scalar_tdiv_q_si(fmpz * vec1, const fmpz * vec2, slong len2, slong c)` | [`Div`](https://doc.rust-lang.org/nightly/core/ops/trait.Div.html), [`DivAssign`](https://doc.rust-lang.org/nightly/core/ops/trait.DivAssign.html) |
| ✓ | `void _fmpz_vec_scalar_tdiv_q_ui(fmpz * vec1, const fmpz * vec2, slong len2, ulong c)` | [`Div`](https://doc.rust-lang.org/nightly/core/ops/trait.Div.html), [`DivAssign`](https://doc.rust-lang.org/nightly/core/ops/trait.DivAssign.html) |
| ✓ | `void _fmpz_vec_scalar_tdiv_q_2exp(fmpz * vec1, const fmpz * vec2, slong len2, ulong exp)` | [`EntrywiseShrRound`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.EntrywiseShrRound.html), [`EntrywiseShrRoundAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.EntrywiseShrRoundAssign.html) |
| ✓ | `void _fmpz_vec_scalar_addmul_si(fmpz * vec1, const fmpz * vec2, slong len2, slong c)` | [`AddMul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.AddMul.html), [`AddMulAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.AddMulAssign.html) |
| ✓ | `void _fmpz_vec_scalar_addmul_ui(fmpz * vec1, const fmpz * vec2, slong len2, ulong c)` | [`AddMul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.AddMul.html), [`AddMulAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.AddMulAssign.html) |
| ✓ | `void _fmpz_vec_scalar_addmul_fmpz(fmpz * vec1, const fmpz * vec2, slong len2, const fmpz_t c)` | [`AddMul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.AddMul.html), [`AddMulAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.AddMulAssign.html) |
| ✓ | `void _fmpz_vec_scalar_addmul_si_2exp(fmpz * vec1, const fmpz * vec2, slong len2, slong c, ulong exp)` | [`AddMulShl`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.AddMulShl.html), [`AddMulShlAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.AddMulShlAssign.html) |
| ✓ | `void _fmpz_vec_scalar_submul_fmpz(fmpz * vec1, const fmpz * vec2, slong len2, const fmpz_t x)` | [`SubMul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.SubMul.html), [`SubMulAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.SubMulAssign.html) |
| ✓ | `void _fmpz_vec_scalar_submul_si(fmpz * vec1, const fmpz * vec2, slong len2, slong c)` | [`SubMul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.SubMul.html), [`SubMulAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.SubMulAssign.html) |
| ✓ | `void _fmpz_vec_scalar_submul_si_2exp(fmpz * vec1, const fmpz * vec2, slong len2, slong c, ulong e)` | [`SubMulShl`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.SubMulShl.html), [`SubMulShlAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.SubMulShlAssign.html) |

`scalar_mul` multiplies every element by one integer. It is
[`Mul`](https://doc.rust-lang.org/nightly/core/ops/trait.Mul.html) of an
[`IntegerVector`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/struct.IntegerVector.html)
and an
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html), with the
scalar on either side, or
[`MulAssign`](https://doc.rust-lang.org/nightly/core/ops/trait.MulAssign.html) when `vec1` is
`vec2`;
[`NaturalVector`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_vector/struct.NaturalVector.html)
and [`RationalVector`](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/struct.RationalVector.html)
multiply by a [`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html)
and a [`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html) in the
same way. Like FLINT's, the integer version special-cases multiplication by 0, 1, and $$-1$$;
the natural and rational versions special-case 0 and 1. `scalar_mul_2exp` is
[`Shl`](https://doc.rust-lang.org/nightly/core/ops/trait.Shl.html) (`<<`) by any primitive unsigned
type, or [`ShlAssign`](https://doc.rust-lang.org/nightly/core/ops/trait.ShlAssign.html) (`<<=`) when
`vec1` is `vec2`, implemented by
[`IntegerVector`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/struct.IntegerVector.html),
[`NaturalVector`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_vector/struct.NaturalVector.html),
and
[`RationalVector`](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/struct.RationalVector.html).

`scalar_addmul` adds $$c$$ times `vec2` to `vec1`, which is
[`AddMulAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.AddMulAssign.html) of an
[`IntegerVector`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/struct.IntegerVector.html),
another `IntegerVector`, and an
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html);
[`AddMul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.AddMul.html) returns the result instead of updating `vec1`. `scalar_submul`
subtracts in the same way, with [`SubMul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.SubMul.html) and
[`SubMulAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.SubMulAssign.html). Every combination of value and reference is
implemented, and like FLINT's, both special-case multiplication by 0, 1, and $$-1$$. For
[`NaturalVector`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_vector/struct.NaturalVector.html),
`AddMul` and `AddMulAssign` take a
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html) scalar.
The `_si` and `_ui` forms take an `Integer` made from the primitive scalar.

`scalar_fdiv_q_2exp` is
[`Shr`](https://doc.rust-lang.org/nightly/core/ops/trait.Shr.html) (`>>`), which takes the floor
of every element, or [`ShrAssign`](https://doc.rust-lang.org/nightly/core/ops/trait.ShrAssign.html)
(`>>=`) when `vec1` is `vec2`. `scalar_tdiv_q_2exp`, which rounds toward zero, is
[`EntrywiseShrRound`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.EntrywiseShrRound.html)
with `Down`, or
[`EntrywiseShrRoundAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.EntrywiseShrRoundAssign.html)
when `vec1` is `vec2`. Its other rounding modes give the remaining quotients; unlike
[`ShrRound`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ShrRound.html),
it returns no [`Ordering`](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html), since
the elements may round in different directions. All of these take any primitive unsigned or
signed shift amount, a negative one shifting the other way.

`scalar_divexact` is
[`DivExact`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivExact.html),
or [`DivExactAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivExactAssign.html)
when `vec1` is `vec2`; the `_si` and `_ui` forms divide by an
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html) made from
the word. Like FLINT's, the integer version special-cases division by 1 and $$-1$$. Neither
library checks that each division is exact. To check, use
[`DivisibleBy`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivisibleBy.html)
on each element first.

`scalar_tdiv_q`, which rounds toward zero, is
[`Div`](https://doc.rust-lang.org/nightly/core/ops/trait.Div.html) (`/`), or
[`DivAssign`](https://doc.rust-lang.org/nightly/core/ops/trait.DivAssign.html) (`/=`) when `vec1`
is `vec2`, by an
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html); the `_si`
and `_ui` forms divide by an `Integer` made from the word. `scalar_fdiv_q`, which rounds toward
$$-\infty$$, is
[`EntrywiseDivRound`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.EntrywiseDivRound.html)
with `Floor`, or
[`EntrywiseDivRoundAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.EntrywiseDivRoundAssign.html)
when `vec1` is `vec2`; its other rounding modes give the remaining quotients. Like
`EntrywiseShrRound`, it returns no
[`Ordering`](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html), unlike the scalar
[`DivRound`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivRound.html).

The `_si_2exp` forms of `scalar_addmul` and `scalar_submul` multiply by $$c \cdot 2^e$$. They
are [`AddMulShlAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.AddMulShlAssign.html) and
[`SubMulShlAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.SubMulShlAssign.html) of an `IntegerVector`, another `IntegerVector`, an
`Integer` scalar, and a `u64` shift; [`AddMulShl`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.AddMulShl.html) and
[`SubMulShl`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.SubMulShl.html) return the result instead. Like FLINT's, they handle a zero scalar,
a zero shift, and a scalar of $$\pm 1$$ directly. `NaturalVector` implements `AddMulShl` and
`AddMulShlAssign` with a `Natural` scalar, and the scalar traits are implemented for `Integer`,
`Natural` (`AddMulShl` only), and `Rational`.

`scalar_fdiv_r_2exp`, the remainder in $$[0, 2^e)$$, is
[`ModPowerOf2`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPowerOf2.html)
on an
[`IntegerVector`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/struct.IntegerVector.html).
Every remainder is nonnegative, so it returns a
[`NaturalVector`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_vector/struct.NaturalVector.html),
as `Mod` does for `scalar_mod_fmpz` (see [Reduction mod `p`](#reduction-mod-p)).

The `si` and `ui` forms disappear, since Malachite's
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html)
operations take
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html)
operands; convert the scalar once with `Integer::from(c)`.

## [Sums and products](https://flintlib.org/doc/fmpz_vec.html#sums-and-products) {#sums-and-products}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void _fmpz_vec_sum(fmpz_t res, const fmpz * vec, slong len)` | [`ElementSum`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ElementSum.html) |
| ✓ | `void _fmpz_ui_vec_prod(fmpz_t res, nn_srcptr vec, slong len)` | [`WideningElementProduct`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.WideningElementProduct.html) |
| ✓ | `void _fmpz_vec_prod(fmpz_t res, const fmpz * vec, slong len)` | [`ElementProduct`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ElementProduct.html) |

The sum of an empty vector is $$0$$ and the product is $$1$$. `_fmpz_vec_sum` is
[`ElementSum`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ElementSum.html)'s
[`to_element_sum`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ElementSum.html#tymethod.to_element_sum), and `_fmpz_vec_prod` is
[`ElementProduct`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ElementProduct.html)'s
[`to_element_product`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ElementProduct.html#tymethod.to_element_product); the `into_` forms consume the vector.
They are distinct from [`Sum`](https://doc.rust-lang.org/nightly/core/iter/trait.Sum.html) for the
vector types, which adds vectors to each other.
`_fmpz_ui_vec_prod` takes an array of words, an
[`UnsignedVector<u64>`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/struct.UnsignedVector.html)
in Malachite, and its product is a
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html):
[`WideningElementProduct`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.WideningElementProduct.html)'s
[`to_widening_element_product`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.WideningElementProduct.html#tymethod.to_widening_element_product), whose output type is a
parameter so that the product cannot overflow. `ElementProduct` would return a `u64`. The same
computation is available for any iterator of unsigned primitive integers through
[`Product`](https://doc.rust-lang.org/nightly/core/iter/trait.Product.html) for `Natural`, as in
`Natural::product(xs.iter())`.

FLINT computes the products by binary splitting, multiplying the products of the two halves so
that the operands of each multiplication have similar sizes; below 16 words, `_fmpz_ui_vec_prod`
multiplies one word at a time. `ElementProduct` multiplies through
[`Product`](https://doc.rust-lang.org/nightly/core/iter/trait.Product.html) for
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html) and
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html), which
does the same, combining the elements in a balanced tree, and stops as soon as it meets a zero.
`WideningElementProduct` first multiplies adjacent words together while their product fits in a
word, then combines the resulting words in a balanced tree; it also stops at a zero.

## [Reduction mod `p`](https://flintlib.org/doc/fmpz_vec.html#reduction-mod-p) {#reduction-mod-p}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void _fmpz_vec_scalar_mod_fmpz(fmpz * res, const fmpz * vec, slong len, const fmpz_t p)` | [`Mod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Mod.html) |
| ✓ | `void _fmpz_vec_scalar_smod_fmpz(fmpz * res, const fmpz * vec, slong len, const fmpz_t p)` | [`BalancedMod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.BalancedMod.html), [`BalancedModAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.BalancedModAssign.html) |
| ✓ | `void _fmpz_vec_multi_CRT_ui(fmpz * res, nn_srcptr * residues, slong len, nn_srcptr primes, slong num_primes, int sign)` | [`NaturalVector::multi_crt`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_vector/struct.NaturalVector.html#method.multi_crt), [`IntegerVector::multi_balanced_crt`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/struct.IntegerVector.html#method.multi_balanced_crt) |

`scalar_mod_fmpz` reduces each element into $$[0, p)$$, and `scalar_smod_fmpz` into
$$(-p/2, p/2]$$. For a single value these are
[`Mod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Mod.html)
and
[`BalancedMod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.BalancedMod.html).
`scalar_mod_fmpz` is `Mod` on an
[`IntegerVector`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/struct.IntegerVector.html)
with a [`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html)
modulus. Since every residue in $$[0, p)$$ is nonnegative, the result is a
[`NaturalVector`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_vector/struct.NaturalVector.html),
as `Mod` on an `IntegerPolynomial` returns a `NaturalPolynomial` (see [the polynomial
page](/mapping/flint-integer-polynomials/#scalar)). `scalar_smod_fmpz` is
`BalancedMod` on an `IntegerVector` with an
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html)
modulus, which keeps the result an `IntegerVector`; only the modulus's magnitude matters, so a
negative modulus, which FLINT does not accept, gives the same result as its absolute value.

`multi_CRT_ui` is given the residues of a vector modulo several word-sized primes, one residue
vector per prime, and reconstructs each element by Chinese remaindering; `sign` chooses the
representative in $$[0, M)$$ or in $$(-M/2, M/2]$$, where $$M$$ is the product of the primes.
These are
[`NaturalVector::multi_crt`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_vector/struct.NaturalVector.html#method.multi_crt),
with `sign = 0`, and
[`IntegerVector::multi_balanced_crt`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/struct.IntegerVector.html#method.multi_balanced_crt),
with `sign = 1`. Each takes the moduli as a slice of
[`Limb`](https://docs.rs/malachite-nz/latest/malachite_nz/platform/type.Limb.html)s and the
residues as one
[`UnsignedVector<Limb>`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/struct.UnsignedVector.html)
per modulus, in FLINT's order. Like FLINT, they precompute once for the moduli and reuse it for
every element; FLINT also splits long vectors across threads, which Malachite does not. They
follow
[`Natural::multi_crt`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html#method.multi_crt)
and
[`Integer::multi_balanced_crt`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html#method.multi_balanced_crt)
(see [Chinese remaindering](/mapping/flint-integers/#chinese-remaindering)) in their conventions:
unusable moduli give
[`None`](https://doc.rust-lang.org/nightly/std/option/enum.Option.html) (with one modulus, a
modulus of 0; with more, any 0 or 1, or any pair that is not coprime), the residues must be
reduced, and an empty list of moduli panics, where FLINT returns a vector of zeros of the given
length.

## [Gaussian content](https://flintlib.org/doc/fmpz_vec.html#gaussian-content) {#gaussian-content}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void _fmpz_vec_content(fmpz_t res, const fmpz * vec, slong len)` | [`Content`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Content.html) |
| ✓ | `void _fmpz_vec_content_chained(fmpz_t res, const fmpz * vec, slong len, const fmpz_t input)` | [`ContentChained`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ContentChained.html) |
| ✓ | `void _fmpz_vec_lcm(fmpz_t res, const fmpz * vec, slong len)` | [`ElementLcm`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ElementLcm.html) |

The content is the GCD of the elements, as a nonnegative
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html); both
libraries define the content of the empty vector and of a vector of zeros to be $$0$$, and both
stop as soon as the GCD reaches $$1$$. The same family of traits also gives the vector divided by
its content:
[`PrimitivePart`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.PrimitivePart.html)
and
[`ContentAndPrimitivePart`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ContentAndPrimitivePart.html),
along with canonical forms whose first nonzero element is positive.

`content_chained` computes the GCD of `input` and the elements of the vector, which makes it
possible to accumulate the common content of several vectors one at a time, starting from $$0$$.
It is [`ContentChained`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ContentChained.html)'s `v.content_chained(input)`, with `input` a
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html) taken by
value or by reference. Like FLINT's, it skips zeros at both ends, returns $$1$$ at once if an end
element is $$\pm 1$$, and otherwise takes the elements in pairs from both ends, stopping as soon as
the GCD reaches $$1$$. `ContentChained` is also implemented for
[`NaturalVector`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_vector/struct.NaturalVector.html),
[`UnsignedVector`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/struct.UnsignedVector.html),
and
[`RationalVector`](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/struct.RationalVector.html),
where `input` is a `Rational` and the result is the rational GCD.

`_fmpz_vec_lcm` is [`ElementLcm`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ElementLcm.html)'s
[`to_element_lcm`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ElementLcm.html#tymethod.to_element_lcm), or
[`into_element_lcm`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ElementLcm.html#tymethod.into_element_lcm) when the vector is no longer needed. Both
libraries define the LCM of the empty vector to be $$1$$ and the LCM of a vector with a zero entry to
be $$0$$, stopping at the first zero, and both return the LCM of the absolute values as a nonnegative
number. `ElementLcm` is implemented for `IntegerVector` and `NaturalVector`.

## [Dot product](https://flintlib.org/doc/fmpz_vec.html#dot-product) {#dot-product}

| | FLINT | Malachite |
| :---: | --- | --- |
| — | `void _fmpz_vec_dot_general_naive(fmpz_t res, const fmpz_t initial, int subtract, const fmpz * a, const fmpz * b, int reverse, slong len)` | |
| ✗ | `void _fmpz_vec_dot_general(fmpz_t res, const fmpz_t initial, int subtract, const fmpz * a, const fmpz * b, int reverse, slong len)` | |
| ✓ | `void _fmpz_vec_dot(fmpz_t res, const fmpz * vec1, const fmpz * vec2, slong len2)` | [`Dot`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Dot.html) |

`_fmpz_vec_dot` is $$\sum_i a_i b_i$$. `_fmpz_vec_dot_general` generalizes it to
$$s \pm \sum_i a_i b_i$$, with an optional initial term $$s$$, a flag that subtracts the sum instead
of adding it, and a flag that pairs $$a_i$$ with $$b_{n-1-i}$$, reversing the second vector, as in
the coefficient of a polynomial product. `_fmpz_vec_dot_general_naive` is a slower reference
implementation of the same function, kept for testing; Malachite keeps reference implementations
in its test code rather than its public API.

`_fmpz_vec_dot` is [`Dot`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Dot.html)'s `v.dot(w)`, for `IntegerVector` and `NaturalVector`, with
each vector taken by value or by reference; the vectors must have the same dimension, and the dot
product of two 0-dimensional vectors is $$0$$. It uses the same accumulation as FLINT, described
below. The general form has no counterpart yet; until it does, start from the initial term (or
zero) and apply
[`AddMulAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.AddMulAssign.html)
or
[`SubMulAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.SubMulAssign.html)
for each pair of elements, iterating over the second vector in reverse for the `reverse` form.
FLINT's `_fmpz_vec_dot_general` accumulates the small products in three words and the large ones
in two separate limb buffers, one for positive terms and one for negative, so it creates no
intermediate
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html)s, and
`Dot` does the same; the element-by-element fold does not avoid them. Malachite does have a dot product for floating-point
numbers,
[`Float::dot`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.dot)
and its rounding variants, the counterpart of MPFR's `mpfr_dot`.
