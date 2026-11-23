---
layout: default
title: "Malachite for FLINT Users: Rational Matrices"
permalink: /mapping/flint-rational-matrices/
theme: jekyll-theme-slate
---

# Malachite for FLINT Users: Rational Matrices

This page maps the functions of FLINT's rational matrix type, `fmpq_mat_t`, onto their Malachite
counterpart:
[`RationalMatrix`](https://docs.rs/malachite-q/latest/malachite_q/rational_matrix/struct.RationalMatrix.html),
from the `malachite-q` crate. It follows the organization of the [fmpq_mat.h
chapter](https://flintlib.org/doc/fmpq_mat.html) of the FLINT manual, as of FLINT 3.6.0. It is a
companion to [Malachite for FLINT Users: Integer Matrices](/mapping/flint-integer-matrices/),
whose [conventions](/mapping/flint-integer-matrices/#conventions) apply here as well, and to
[Malachite for FLINT Users: Rationals](/mapping/flint-rationals/); the [mapping index](/mapping/)
lists the whole family.

Every section of the chapter is mapped below: 93 rows across 25 sections, of which 41 are ✓, 17 ⚙,
24 ≈, 4 —, and 7 ✗. Functions whose names begin with an underscore are omitted, as on [the fmpq
page](/mapping/flint-rationals/), and functions that `fmpq_mat.h` declares but the chapter does not
document are mentioned in the notes of the sections they belong to.

## Conventions {#conventions}

### The `fmpq_mat` representation {#representation}

An `fmpq_mat_struct` has the same layout as an `fmpz_mat_struct` (see [the integer matrix
page](/mapping/flint-integer-matrices/#representation)): `entries` in row-major order, the
dimensions `r` and `c`, and a `stride` for windows. Each entry is an `fmpq` of its own, in lowest
terms, rather than a numerator over a denominator shared by the whole matrix, as in `fmpq_poly`.
[`RationalMatrix`](https://docs.rs/malachite-q/latest/malachite_q/rational_matrix/struct.RationalMatrix.html)
stores its elements in the same way, in a private
[`Vec<Rational>`](https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html), with a private
height and width and no stride; a matrix always owns its elements.

FLINT expects its inputs to be in canonical form and leaves that to the caller, since an entry's
numerator and denominator can be written separately through `fmpq_mat_entry_num` and
`fmpq_mat_entry_den`. A [`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html)
is always in lowest terms, so every element of a `RationalMatrix` is too. As the chapter's
introduction says, FLINT does most of the heavy work, such as multiplication and solving, by
clearing denominators and working over the integers; Malachite's rational matrix functions do the
same internally.

### Categories

The categories are those of [the integer matrix
page](/mapping/flint-integer-matrices/#categories): ✓, ≈, ⚙, —, and ✗.

## [Types, macros and constants](https://flintlib.org/doc/fmpq_mat.html#types-macros-and-constants) {#types-macros-and-constants}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `fmpq_mat_struct` | [`RationalMatrix`](https://docs.rs/malachite-q/latest/malachite_q/rational_matrix/struct.RationalMatrix.html) |
| — | `fmpq_mat_t` | |

`fmpq_mat_t` is the pass-by-reference device that `&` and `&mut` already are.

## [Memory management](https://flintlib.org/doc/fmpq_mat.html#memory-management) {#memory-management}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpq_mat_init(fmpq_mat_t mat, slong rows, slong cols)` | [`zero`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.zero) |
| ✓ | `void fmpq_mat_init_set(fmpq_mat_t mat1, const fmpq_mat_t mat2)` | [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html) |
| — | `void fmpq_mat_clear(fmpq_mat_t mat)` | |
| ✓ | `void fmpq_mat_swap(fmpq_mat_t mat1, fmpq_mat_t mat2)` | [`mem::swap`](https://doc.rust-lang.org/nightly/core/mem/fn.swap.html) |
| — | `void fmpq_mat_swap_entrywise(fmpq_mat_t mat1, fmpq_mat_t mat2)` | |

`fmpq_mat_init` sets every entry to $$0/1$$; unlike `fmpz_mat_init`, it cannot use `calloc`,
since a zeroed `fmpq` would have denominator 0. Its counterpart is
[`zero`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.zero),
the [`Matrix`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html)
method: `RationalMatrix::zero(rows, cols)`. As for integer matrices (see [the integer matrix
page](/mapping/flint-integer-matrices/#memory-management)), either dimension may be zero, and a
shape whose number of elements does not fit in a word makes FLINT throw and
[`zero`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.zero)
panic. `fmpq_mat_init_set` is
[`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html), and `fmpq_mat_clear` is
[`Drop`](https://doc.rust-lang.org/nightly/core/ops/trait.Drop.html), which runs when the matrix
goes out of scope.

`fmpq_mat_swap` exchanges the two structs and `fmpq_mat_swap_entrywise` the entries one by one;
they differ only for a window, whose entries belong to its parent. Malachite has no windows, so
[`mem::swap`](https://doc.rust-lang.org/nightly/core/mem/fn.swap.html) covers both, as on [the
integer matrix page](/mapping/flint-integer-matrices/#basic-assignment-and-manipulation).

## [Entry access](https://flintlib.org/doc/fmpq_mat.html#entry-access) {#entry-access}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `fmpq * fmpq_mat_entry(const fmpq_mat_t mat, slong i, slong j)` | [`Index`](https://doc.rust-lang.org/nightly/core/ops/trait.Index.html), [`IndexMut`](https://doc.rust-lang.org/nightly/core/ops/trait.IndexMut.html), [`get`](https://docs.rs/malachite-q/latest/malachite_q/rational_matrix/struct.RationalMatrix.html#method.get), [`get_mut`](https://docs.rs/malachite-q/latest/malachite_q/rational_matrix/struct.RationalMatrix.html#method.get_mut) |
| ≈ | `fmpz * fmpq_mat_entry_num(const fmpq_mat_t mat, slong i, slong j)` | [`numerator_ref`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.numerator_ref), [`mutate_numerator`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.mutate_numerator) |
| ≈ | `fmpz * fmpq_mat_entry_den(const fmpq_mat_t mat, slong i, slong j)` | [`denominator_ref`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.denominator_ref), [`mutate_denominator`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.mutate_denominator) |
| ✓ | `slong fmpq_mat_nrows(const fmpq_mat_t mat)` | [`height`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.height) |
| ✓ | `slong fmpq_mat_ncols(const fmpq_mat_t mat)` | [`width`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.width) |

As for integer matrices (see [the integer matrix
page](/mapping/flint-integer-matrices/#basic-assignment-and-manipulation)), `fmpq_mat_entry` is
indexing, `a[(i, j)]`, with [`usize`](https://doc.rust-lang.org/nightly/core/primitive.usize.html)
indices that panic when out of range, or
[`get`](https://docs.rs/malachite-q/latest/malachite_q/rational_matrix/struct.RationalMatrix.html#method.get)
and
[`get_mut`](https://docs.rs/malachite-q/latest/malachite_q/rational_matrix/struct.RationalMatrix.html#method.get_mut),
with [`u64`](https://doc.rust-lang.org/nightly/core/primitive.u64.html) indices that return
[`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None); FLINT
checks nothing.
[`height`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.height)
and [`width`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.width)
return [`u64`](https://doc.rust-lang.org/nightly/core/primitive.u64.html)s.

`fmpq_mat_entry_num` and `fmpq_mat_entry_den` return pointers into an entry, so the numerator
and denominator can be read or written separately, leaving the entry possibly not in lowest
terms. In Malachite they are the corresponding methods of the element, as for
[`fmpq_numref` and `fmpq_denref`](/mapping/flint-rationals/#types-macros-and-constants): read with
`a[(i, j)].numerator_ref()` and `denominator_ref()`, with the sign asked for separately, since the
numerator is stored as a magnitude, and write with
[`mutate_numerator`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.mutate_numerator)
or
[`mutate_denominator`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.mutate_denominator)
on `a[(i, j)]`, which reduce the element when the closure returns, hence ≈.

## [Basic assignment](https://flintlib.org/doc/fmpq_mat.html#basic-assignment) {#basic-assignment}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpq_mat_set(fmpq_mat_t dest, const fmpq_mat_t src)` | [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html) |
| ✓ | `void fmpq_mat_zero(fmpq_mat_t mat)` | [`set_zero`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.set_zero) |
| ✓ | `void fmpq_mat_one(fmpq_mat_t mat)` | [`rectangular_identity`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.rectangular_identity), [`identity`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.identity) |
| ✓ | `void fmpq_mat_transpose(fmpq_mat_t rop, const fmpq_mat_t op)` | [`transpose`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.transpose), [`transpose_assign`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.transpose_assign) |
| ≈ | `void fmpq_mat_swap_rows(fmpq_mat_t mat, slong * perm, slong r, slong s)` | [`swap_rows`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.swap_rows) |
| ≈ | `void fmpq_mat_swap_cols(fmpq_mat_t mat, slong * perm, slong r, slong s)` | [`swap_columns`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.swap_columns) |
| ≈ | `void fmpq_mat_invert_rows(fmpq_mat_t mat, slong * perm)` | [`reverse_rows`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.reverse_rows) |
| ≈ | `void fmpq_mat_invert_cols(fmpq_mat_t mat, slong * perm)` | [`reverse_columns`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.reverse_columns) |

These match their `fmpz_mat` counterparts, described on the integer matrix page under [Basic
assignment and manipulation](/mapping/flint-integer-matrices/#basic-assignment-and-manipulation)
and [Transpose](/mapping/flint-integer-matrices/#transpose). `fmpq_mat_set` assumes, without
checking, that the shapes match;
[`clone_from`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)
does not need them to. `fmpq_mat_one` accepts any shape and produces the identity truncated to it,
which is
[`rectangular_identity`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.rectangular_identity),
`RationalMatrix::rectangular_identity(m, n)`;
[`identity`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.identity)
is the square case.
`fmpq_mat_transpose` goes through the generic `gr_mat_transpose`, which throws unless the output
has the transposed shape, so it can transpose in place only a square matrix;
[`transpose_assign`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.transpose_assign)
handles any shape.

`fmpq_mat_swap_rows` and `fmpq_mat_swap_cols` are
[`swap_rows`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.swap_rows)
and
[`swap_columns`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.swap_columns),
≈ because they take no permutation array, as on [the integer matrix
page](/mapping/flint-integer-matrices/#basic-assignment-and-manipulation), and so are
`fmpq_mat_invert_rows` and `fmpq_mat_invert_cols`, which are
[`reverse_rows`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.reverse_rows)
and
[`reverse_columns`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.reverse_columns).
As there,
the manual says that `invert_rows` swaps rows $$i$$ and $$r - i$$, and `invert_cols` columns
$$i$$ and $$c - i$$, but the code (`fmpq_mat/invert.c`) swaps $$i$$ with $$r - 1 - i$$ and
$$c - 1 - i$$, reversing the order.

## [Addition, scalar multiplication](https://flintlib.org/doc/fmpq_mat.html#addition-scalar-multiplication) {#addition-scalar-multiplication}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpq_mat_add(fmpq_mat_t mat, const fmpq_mat_t mat1, const fmpq_mat_t mat2)` | [`Add`](https://doc.rust-lang.org/nightly/core/ops/trait.Add.html), [`AddAssign`](https://doc.rust-lang.org/nightly/core/ops/trait.AddAssign.html) |
| ✓ | `void fmpq_mat_sub(fmpq_mat_t mat, const fmpq_mat_t mat1, const fmpq_mat_t mat2)` | [`Sub`](https://doc.rust-lang.org/nightly/core/ops/trait.Sub.html), [`SubAssign`](https://doc.rust-lang.org/nightly/core/ops/trait.SubAssign.html) |
| ✓ | `void fmpq_mat_neg(fmpq_mat_t rop, const fmpq_mat_t op)` | [`Neg`](https://doc.rust-lang.org/nightly/core/ops/trait.Neg.html), [`NegAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.NegAssign.html) |
| ✓ | `void fmpq_mat_scalar_mul_fmpq(fmpq_mat_t rop, const fmpq_mat_t op, const fmpq_t x)` | [`Mul`](https://doc.rust-lang.org/nightly/core/ops/trait.Mul.html), [`MulAssign`](https://doc.rust-lang.org/nightly/core/ops/trait.MulAssign.html) |
| ✓ | `void fmpq_mat_scalar_mul_fmpz(fmpq_mat_t rop, const fmpq_mat_t op, const fmpz_t x)` | [`Mul`](https://doc.rust-lang.org/nightly/core/ops/trait.Mul.html), [`MulAssign`](https://doc.rust-lang.org/nightly/core/ops/trait.MulAssign.html) |
| ✓ | `void fmpq_mat_scalar_mul_si(fmpq_mat_t rop, const fmpq_mat_t op, slong x)` | [`Mul`](https://doc.rust-lang.org/nightly/core/ops/trait.Mul.html), [`MulAssign`](https://doc.rust-lang.org/nightly/core/ops/trait.MulAssign.html) |
| ✓ | `void fmpq_mat_scalar_mul_ui(fmpq_mat_t rop, const fmpq_mat_t op, ulong x)` | [`Mul`](https://doc.rust-lang.org/nightly/core/ops/trait.Mul.html), [`MulAssign`](https://doc.rust-lang.org/nightly/core/ops/trait.MulAssign.html) |
| ✗ | `void fmpq_mat_scalar_div_fmpz(fmpq_mat_t rop, const fmpq_mat_t op, const fmpz_t x)` |  |
| ✗ | `void fmpq_mat_scalar_div_si(fmpq_mat_t rop, const fmpq_mat_t op, slong x)` |  |
| ✗ | `void fmpq_mat_scalar_div_ui(fmpq_mat_t rop, const fmpq_mat_t op, ulong x)` |  |

As for integer matrices (see [the integer matrix
page](/mapping/flint-integer-matrices/#addition-and-subtraction)), the operators take their
operands by value or by reference, the `Assign` forms are the aliased calls, and
[`Add`](https://doc.rust-lang.org/nightly/core/ops/trait.Add.html) and
[`Sub`](https://doc.rust-lang.org/nightly/core/ops/trait.Sub.html) panic if the shapes differ,
which FLINT assumes but does not check. `scalar_mul` is
[`Mul`](https://doc.rust-lang.org/nightly/core/ops/trait.Mul.html) of a `RationalMatrix` and a
[`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html), with
the scalar on either side; the `_fmpz`, `_si`, and `_ui` forms take a `Rational` made from the
integer.

Malachite does not yet divide a matrix by a scalar. Meanwhile, multiply by the reciprocal,
`a * Rational::from(x).reciprocal()`, with
[`Reciprocal`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Reciprocal.html),
which panics when $$x = 0$$; FLINT's division by zero throws. FLINT has no `scalar_div_fmpq`. The
manual's prototypes give the scalar of `scalar_div_si` and `scalar_div_ui` the type `const fmpz_t`;
`fmpq_mat.h` declares `slong` and `ulong`.

## [Input and output](https://flintlib.org/doc/fmpq_mat.html#input-and-output) {#input-and-output}

| | FLINT | Malachite |
| :---: | --- | --- |
| ≈ | `void fmpq_mat_print(const fmpq_mat_t mat)` | [`Display`](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html), [`Serialize`](https://docs.rs/serde/latest/serde/trait.Serialize.html) |

`fmpq_mat_print` writes a header with the shape, then each row in brackets on its own line, with
the elements separated by `, ` and written as `fmpq_print` writes them, and then a blank line;
the matrix with rows $$(1/2, -3)$$ and $$(2/3, 0)$$ is printed as

```
<2 x 2 matrix over Q>
[1/2, -3]
[2/3, 0]

```

Malachite's [`Display`](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html) writes the
same matrix on one line as `[1/2, -3; 2/3, 0]`, with the shape shown only for a matrix with no
elements, as in `[]_{4,0}`, hence ≈; the element format is the same as FLINT's.
[`FromStr`](https://doc.rust-lang.org/nightly/core/str/trait.FromStr.html) reads that form back,
though FLINT has no function to read a rational matrix. With the `serde` feature, a
`RationalMatrix` is serialized as its height, its width, and its elements in row-major order. As
on [the integer matrix page](/mapping/flint-integer-matrices/#input-and-output),
[`Display`](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html) writes to any
[`Formatter`](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html), so there is no
separate printing function for each stream.

## [Random matrix generation](https://flintlib.org/doc/fmpq_mat.html#random-matrix-generation) {#random-matrix-generation}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpq_mat_randbits(fmpq_mat_t mat, flint_rand_t state, flint_bitcnt_t bits)` | [`random_rational_matrices_with_dimensions_from_iterator`](https://docs.rs/malachite-q/latest/malachite_q/rational_matrix/random/fn.random_rational_matrices_with_dimensions_from_iterator.html) |
| ≈ | `void fmpq_mat_randtest(fmpq_mat_t mat, flint_rand_t state, flint_bitcnt_t bits)` | [`random_rational_matrices_with_dimensions`](https://docs.rs/malachite-q/latest/malachite_q/rational_matrix/random/fn.random_rational_matrices_with_dimensions.html), [`striped_random_rational_matrices_with_dimensions`](https://docs.rs/malachite-q/latest/malachite_q/rational_matrix/random/fn.striped_random_rational_matrices_with_dimensions.html) |

Both functions fill every entry with the corresponding single-value function, which [the
rationals page](/mapping/flint-rationals/#random-number-generation) maps:
`fmpq_randbits`, whose numerator and denominator have `bits` bits before reduction, and
`fmpq_randtest`, which caps their size at `bits` and mixes in special values. Unlike
`fmpz_mat_randtest` (see [the integer matrix
page](/mapping/flint-integer-matrices/#random-matrix-generation)), `fmpq_mat_randtest` has no
sparse mode. Malachite's generators are infinite iterators of matrices of a given shape, seeded
once. For `randbits`, pass a stream of such rationals, built with
[`get_random_natural_with_bits`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/random/fn.get_random_natural_with_bits.html)
and
[`from_sign_and_naturals`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.from_sign_and_naturals),
to
[`random_rational_matrices_with_dimensions_from_iterator`](https://docs.rs/malachite-q/latest/malachite_q/rational_matrix/random/fn.random_rational_matrices_with_dimensions_from_iterator.html).
The counterparts of `randtest` take a *mean* bit length with no cap, so the distributions differ,
hence ≈. Forms without `_with_dimensions` draw the height and width as well.

## [Window](https://flintlib.org/doc/fmpq_mat.html#window) {#window}

| | FLINT | Malachite |
| :---: | --- | --- |
| ≈ | `void fmpq_mat_window_init(fmpq_mat_t window, const fmpq_mat_t mat, slong r1, slong c1, slong r2, slong c2)` | [`window`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.window), [`into_window`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.into_window), [`set_window`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.set_window), [`set_window_owned`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.set_window_owned) |
| — | `void fmpq_mat_window_clear(fmpq_mat_t window)` | |

These are the rational versions of the `fmpz_mat` window functions, and the [integer matrix
page's notes](/mapping/flint-integer-matrices/#window) apply unchanged. A window shares part of
its parent's entries, while a Malachite matrix always owns its elements, so the counterparts copy
the block, hence ≈:
[`window`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.window)
and
[`into_window`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.into_window)
copy a block out, and
[`set_window`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.set_window)
and
[`set_window_owned`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.set_window_owned)
write one back. A block can also be read or written row by row with
[`row_iter`](https://docs.rs/malachite-q/latest/malachite_q/rational_matrix/struct.RationalMatrix.html#method.row_iter)
and
[`row_iter_mut`](https://docs.rs/malachite-q/latest/malachite_q/rational_matrix/struct.RationalMatrix.html#method.row_iter_mut).
The inline implementation in `fmpq_mat.h` has the same two details: the window can write to a
parent declared `const`, and the bounds are checked only by `FLINT_ASSERT`, whose column check
tests `c2 >= 0` rather than `c1 >= 0`.

## [Concatenate](https://flintlib.org/doc/fmpq_mat.html#concatenate) {#concatenate}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpq_mat_concat_vertical(fmpq_mat_t res, const fmpq_mat_t mat1, const fmpq_mat_t mat2)` | [`concat_vertical`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.concat_vertical), [`into_concat_vertical`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.into_concat_vertical) |
| ✓ | `void fmpq_mat_concat_horizontal(fmpq_mat_t res, const fmpq_mat_t mat1, const fmpq_mat_t mat2)` | [`concat_horizontal`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.concat_horizontal), [`into_concat_horizontal`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.into_concat_horizontal) |

These match their `fmpz_mat` counterparts (see [the integer matrix
page](/mapping/flint-integer-matrices/#concatenate)): the by-reference methods clone the elements
and the `into_` methods move them, and they panic if the shapes do not fit, which FLINT does not
check.

## [Special matrices](https://flintlib.org/doc/fmpq_mat.html#special-matrices) {#special-matrices}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void fmpq_mat_hilbert_matrix(fmpq_mat_t mat)` | |

`fmpq_mat_hilbert_matrix` sets the entry in row $$i$$ and column $$j$$, counting from 0, to
$$1/(i + j + 1)$$; it fills a matrix of any shape, not only a square one. Malachite does not have
it yet. Meanwhile, collect
`(0..h).flat_map(|i| (0..w).map(move |j| Rational::from_unsigneds(1u64, i + j + 1)))` and pass it
to
[`from_owned_elements`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.from_owned_elements)
with height $$h$$ and width $$w$$, using
[`from_unsigneds`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.from_unsigneds).

## [Basic comparison and properties](https://flintlib.org/doc/fmpq_mat.html#basic-comparison-and-properties) {#basic-comparison-and-properties}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `int fmpq_mat_equal(const fmpq_mat_t mat1, const fmpq_mat_t mat2)` | [`PartialEq`](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html) |
| ✓ | `int fmpq_mat_is_integral(const fmpq_mat_t mat)` | [`ConvertibleFrom`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.ConvertibleFrom.html) |
| ✓ | `int fmpq_mat_is_zero(const fmpq_mat_t mat)` | [`is_zero`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.is_zero) |
| ✓ | `int fmpq_mat_is_one(const fmpq_mat_t mat)` | [`is_rectangular_identity`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.is_rectangular_identity) |
| ✓ | `int fmpq_mat_is_empty(const fmpq_mat_t mat)` | [`is_empty`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.is_empty) |
| ✓ | `int fmpq_mat_is_square(const fmpq_mat_t mat)` | [`is_square`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.is_square) |

These match their `fmpz_mat` counterparts (see [the integer matrix
page](/mapping/flint-integer-matrices/#comparison)).
[`PartialEq`](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html) compares shapes as
well as elements, and also compares a `RationalMatrix` with an `IntegerMatrix`, a `NaturalMatrix`,
or an `UnsignedMatrix`. FLINT's `equal` and `is_integral` assume that the entries are in lowest
terms, which a `Rational` always is. `is_integral` is
[`ConvertibleFrom`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.ConvertibleFrom.html):
`IntegerMatrix::convertible_from(&a)`; `NaturalMatrix::convertible_from(&a)` also requires the
elements to be nonnegative, and
[`TryFrom`](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html) performs the
conversion. `fmpq_mat_is_one`, like `fmpz_mat_is_one`, accepts the truncated identity of any
shape, including every matrix with no elements, which is
[`is_rectangular_identity`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.is_rectangular_identity);
[`is_identity`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.is_identity)
also requires a square matrix.

## [Integer matrix conversion](https://flintlib.org/doc/fmpq_mat.html#integer-matrix-conversion) {#integer-matrix-conversion}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `int fmpq_mat_get_fmpz_mat(fmpz_mat_t dest, const fmpq_mat_t mat)` | [`TryFrom`](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html) |
| ✗ | `void fmpq_mat_get_fmpz_mat_entrywise(fmpz_mat_t num, fmpz_mat_t den, const fmpq_mat_t mat)` |  |
| ✓ | `void fmpq_mat_get_fmpz_mat_matwise(fmpz_mat_t num, fmpz_t den, const fmpq_mat_t mat)` | [`to_numerators_and_denominator`](https://docs.rs/malachite-q/latest/malachite_q/rational_matrix/struct.RationalMatrix.html#method.to_numerators_and_denominator), [`into_numerators_and_denominator`](https://docs.rs/malachite-q/latest/malachite_q/rational_matrix/struct.RationalMatrix.html#method.into_numerators_and_denominator) |
| ✓ | `void fmpq_mat_get_fmpz_mat_rowwise(fmpz_mat_t num, fmpz * den, const fmpq_mat_t mat)` | [`to_numerators_and_row_denominators`](https://docs.rs/malachite-q/latest/malachite_q/rational_matrix/struct.RationalMatrix.html#method.to_numerators_and_row_denominators), [`into_numerators_and_row_denominators`](https://docs.rs/malachite-q/latest/malachite_q/rational_matrix/struct.RationalMatrix.html#method.into_numerators_and_row_denominators) |
| ⚙ | `void fmpq_mat_get_fmpz_mat_rowwise_2(fmpz_mat_t num, fmpz_mat_t num2, fmpz * den, const fmpq_mat_t mat, const fmpq_mat_t mat2)` | internal |
| ✓ | `void fmpq_mat_get_fmpz_mat_colwise(fmpz_mat_t num, fmpz * den, const fmpq_mat_t mat)` | [`to_numerators_and_column_denominators`](https://docs.rs/malachite-q/latest/malachite_q/rational_matrix/struct.RationalMatrix.html#method.to_numerators_and_column_denominators), [`into_numerators_and_column_denominators`](https://docs.rs/malachite-q/latest/malachite_q/rational_matrix/struct.RationalMatrix.html#method.into_numerators_and_column_denominators) |
| ✓ | `void fmpq_mat_set_fmpz_mat(fmpq_mat_t dest, const fmpz_mat_t src)` | [`From`](https://doc.rust-lang.org/nightly/core/convert/trait.From.html) |
| ≈ | `void fmpq_mat_set_fmpz_mat_div_fmpz(fmpq_mat_t mat, const fmpz_mat_t num, const fmpz_t den)` | [`from_numerators_and_denominator`](https://docs.rs/malachite-q/latest/malachite_q/rational_matrix/struct.RationalMatrix.html#method.from_numerators_and_denominator) |

`get_fmpz_mat` is
[`TryFrom`](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html)`<&RationalMatrix>`
(or `<RationalMatrix>`) for
[`IntegerMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_matrix/struct.IntegerMatrix.html),
which fails unless every element is an integer, where FLINT leaves `dest` undefined and returns
0; [`ConvertibleFrom`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.ConvertibleFrom.html)
asks the question without converting (see [Basic comparison and
properties](#basic-comparison-and-properties)). `set_fmpz_mat` is
[`From`](https://doc.rust-lang.org/nightly/core/convert/trait.From.html)`<IntegerMatrix>`, which
takes the `IntegerMatrix` by value.

`get_fmpz_mat_matwise` writes the matrix over a common denominator: the least common multiple of
the elements' denominators, with each numerator scaled to it, and 1 for a matrix with no
elements. That is
[`to_numerators_and_denominator`](https://docs.rs/malachite-q/latest/malachite_q/rational_matrix/struct.RationalMatrix.html#method.to_numerators_and_denominator),
which returns an `IntegerMatrix` and a
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html), as
[`RationalVector::to_numerators_and_denominator`](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/struct.RationalVector.html#method.to_numerators_and_denominator)
does for vectors;
[`into_numerators_and_denominator`](https://docs.rs/malachite-q/latest/malachite_q/rational_matrix/struct.RationalMatrix.html#method.into_numerators_and_denominator)
consumes the matrix instead of cloning its elements.
`get_fmpz_mat_rowwise` and `get_fmpz_mat_colwise` give each row, or each column, a common
denominator of its own. They are
[`to_numerators_and_row_denominators`](https://docs.rs/malachite-q/latest/malachite_q/rational_matrix/struct.RationalMatrix.html#method.to_numerators_and_row_denominators)
and
[`to_numerators_and_column_denominators`](https://docs.rs/malachite-q/latest/malachite_q/rational_matrix/struct.RationalMatrix.html#method.to_numerators_and_column_denominators),
which return an `IntegerMatrix` and a
[`NaturalVector`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_vector/struct.NaturalVector.html)
of denominators, with `into_` versions that consume the matrix. FLINT leaves the denominators unset
when the matrix has no elements; Malachite gives each row (or column) of an empty matrix the
denominator 1. `rowwise_2` does the rows of two matrices together, as if they were placed side by
side: join each row of one matrix to the same row of the other, build a matrix from the joined rows
with
[`from_rows`](https://docs.rs/malachite-q/latest/malachite_q/rational_matrix/struct.RationalMatrix.html#method.from_rows),
and clear its rows. Malachite has it as an internal function for its future solvers; unlike
FLINT's, it also works when the first matrix has no elements, where FLINT writes nothing.
`get_fmpz_mat_entrywise` writes the numerators and denominators into two separate integer
matrices; collect `x.numerator_ref()` with the sign of `x`, and `x.denominator_ref()`, for each
element `x`.

`set_fmpz_mat_div_fmpz` is the inverse of the common-denominator form: it divides an integer
matrix by an integer. Its counterpart is
[`from_numerators_and_denominator`](https://docs.rs/malachite-q/latest/malachite_q/rational_matrix/struct.RationalMatrix.html#method.from_numerators_and_denominator),
whose denominator is a
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html), so a
negative denominator, which FLINT accepts, must first be moved into the numerators by negating
them, hence ≈. Both reduce every element to lowest terms.

## [Modular reduction and rational reconstruction](https://flintlib.org/doc/fmpq_mat.html#modular-reduction-and-rational-reconstruction) {#modular-reduction-and-rational-reconstruction}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void fmpq_mat_get_fmpz_mat_mod_fmpz(fmpz_mat_t dest, const fmpq_mat_t mat, const fmpz_t mod)` | |
| ≈ | `int fmpq_mat_set_fmpz_mat_mod_fmpz(fmpq_mat_t X, const fmpz_mat_t Xmod, const fmpz_t mod)` | [`reconstruct`](https://docs.rs/malachite-q/latest/malachite_q/rational_matrix/struct.RationalMatrix.html#method.reconstruct), [`reconstruct_ref`](https://docs.rs/malachite-q/latest/malachite_q/rational_matrix/struct.RationalMatrix.html#method.reconstruct_ref) |

`get_fmpz_mat_mod_fmpz` is not a matrix operation in Malachite yet; it applies a single-value
function, mapped on [the rationals
page](/mapping/flint-rationals/#modular-reduction-and-rational-reconstruction), to each entry. It reduces each entry $$n/d$$ to $$n d^{-1} \bmod m$$, as
`fmpq_mod_fmpz` does; meanwhile, use
[`ModInverse`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModInverse.html)
and
[`ModMul`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModMul.html)
on each element, which give
[`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None) when a
denominator has no inverse. The FLINT function returns nothing, so it ignores that failure:
`fmpq_mod_fmpz` reports it, but the matrix function discards the report, and the entry it writes
is then undefined.

`set_fmpz_mat_mod_fmpz` recovers a rational matrix from its residues by rational
reconstruction, returning 0 if some entry cannot be reconstructed. It does not reconstruct each
entry independently: it first multiplies each residue by the product of the denominators found
so far, so that entries sharing a large denominator need only small reconstructions after the
first, and it can therefore succeed where reconstructing every entry separately with
[`Rational::reconstruct`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.reconstruct)
would fail or give a different fraction: modulo 73, the residues $$(61, 71)$$ give
$$(1/6, 1/36)$$, since the second is reconstructed as $$6 \cdot 1/36 = 1/6$$, although $$71$$ on its
own reconstructs to $$-2$$. In Malachite it is
[`RationalMatrix::reconstruct`](https://docs.rs/malachite-q/latest/malachite_q/rational_matrix/struct.RationalMatrix.html#method.reconstruct)
(or [`reconstruct_ref`](https://docs.rs/malachite-q/latest/malachite_q/rational_matrix/struct.RationalMatrix.html#method.reconstruct_ref), by reference), which takes a
[`NaturalMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_matrix/struct.NaturalMatrix.html)
of residues and a
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html) modulus
and returns an
[`Option`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html), and makes the same
choices, hence the same results, with two differences, hence ≈: like
[`Rational::reconstruct`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.reconstruct),
it panics unless the residues are reduced and the modulus is at least 3. FLINT reduces the residues
itself, and with a modulus of 1 or 2, where the balanced bound is 0, reconstructs only zeros.

## [Matrix multiplication](https://flintlib.org/doc/fmpq_mat.html#matrix-multiplication) {#matrix-multiplication}

| | FLINT | Malachite |
| :---: | --- | --- |
| ⚙ | `void fmpq_mat_mul_direct(fmpq_mat_t C, const fmpq_mat_t A, const fmpq_mat_t B)` | internal |
| ⚙ | `void fmpq_mat_mul_cleared(fmpq_mat_t C, const fmpq_mat_t A, const fmpq_mat_t B)` | internal |
| ✓ | `void fmpq_mat_mul(fmpq_mat_t C, const fmpq_mat_t A, const fmpq_mat_t B)` | [`Mul`](https://doc.rust-lang.org/nightly/core/ops/trait.Mul.html), [`MulAssign`](https://doc.rust-lang.org/nightly/core/ops/trait.MulAssign.html) |
| ✓ | `void fmpq_mat_mul_fmpz_mat(fmpq_mat_t C, const fmpq_mat_t A, const fmpz_mat_t B)` | [`Mul`](https://doc.rust-lang.org/nightly/core/ops/trait.Mul.html), [`MulAssign`](https://doc.rust-lang.org/nightly/core/ops/trait.MulAssign.html) |
| ✓ | `void fmpq_mat_mul_r_fmpz_mat(fmpq_mat_t C, const fmpz_mat_t A, const fmpq_mat_t B)` | [`Mul`](https://doc.rust-lang.org/nightly/core/ops/trait.Mul.html) |
| ≈ | `void fmpq_mat_mul_fmpq_vec(fmpq * c, const fmpq_mat_t A, const fmpq * b, slong blen)` | [`Mul`](https://doc.rust-lang.org/nightly/core/ops/trait.Mul.html) |
| ≈ | `void fmpq_mat_mul_fmpz_vec(fmpq * c, const fmpq_mat_t A, const fmpz * b, slong blen)` | [`Mul`](https://doc.rust-lang.org/nightly/core/ops/trait.Mul.html) |
| ≈ | `void fmpq_mat_mul_fmpq_vec_ptr(fmpq * const * c, const fmpq_mat_t A, const fmpq * const * b, slong blen)` | [`Mul`](https://doc.rust-lang.org/nightly/core/ops/trait.Mul.html) |
| ≈ | `void fmpq_mat_mul_fmpz_vec_ptr(fmpq * const * c, const fmpq_mat_t A, const fmpz * const * b, slong blen)` | [`Mul`](https://doc.rust-lang.org/nightly/core/ops/trait.Mul.html) |
| ≈ | `void fmpq_mat_fmpq_vec_mul(fmpq * c, const fmpq * a, slong alen, const fmpq_mat_t B)` | [`Mul`](https://doc.rust-lang.org/nightly/core/ops/trait.Mul.html) |
| ≈ | `void fmpq_mat_fmpz_vec_mul(fmpq * c, const fmpz * a, slong alen, const fmpq_mat_t B)` | [`Mul`](https://doc.rust-lang.org/nightly/core/ops/trait.Mul.html) |
| ≈ | `void fmpq_mat_fmpq_vec_mul_ptr(fmpq * const * c, const fmpq * const * a, slong alen, const fmpq_mat_t B)` | [`Mul`](https://doc.rust-lang.org/nightly/core/ops/trait.Mul.html) |
| ≈ | `void fmpq_mat_fmpz_vec_mul_ptr(fmpq * const * c, const fmpz * const * a, slong alen, const fmpq_mat_t B)` | [`Mul`](https://doc.rust-lang.org/nightly/core/ops/trait.Mul.html) |

`fmpq_mat_mul` is
[`Mul`](https://doc.rust-lang.org/nightly/core/ops/trait.Mul.html)
of two `RationalMatrix` values, with each operand by value or by reference, and
[`MulAssign`](https://doc.rust-lang.org/nightly/core/ops/trait.MulAssign.html)
for the product in place, which FLINT allows by aliasing `C` with `A`. They panic unless the width
of the first matrix is the height of the second, which FLINT does not check. In FLINT,
`fmpq_mat_mul` simply calls `mul_cleared`, with a comment that `mul_direct` may be faster for
$$1 \times 1$$ or $$2 \times 2$$ matrices. Malachite uses `mul_cleared` too: each row of `A` and
each column of `B` is written as integers over a common denominator, as by
`to_numerators_and_row_denominators` and `to_numerators_and_column_denominators`, the two integer
matrices are multiplied as `IntegerMatrix` values (see [the integer matrix
page](/mapping/flint-integer-matrices/#matrix-multiplication)), and each element of the product is
divided by its row's and column's denominators and reduced once. A matrix taken by value is
cleared in place, without cloning its numerators. When the inner dimension is 1, though, each
element of the product is a single product of elements, and Malachite multiplies in rational
arithmetic, as `mul_direct` does: measured, that was up to three times faster than clearing
denominators, while for an inner dimension of 2 or more clearing was faster, by up to 13 times, except for
some products of matrices no larger than $$4 \times 4$$ whose elements had at most 16 bits and
different denominators, where it was at most a third slower.

FLINT has no `fmpq_mat_sqr`. Malachite's
[`Square`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Square.html)
and
[`SquareAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.SquareAssign.html)
for a square `RationalMatrix` multiply it by itself as above, except that when every row and every
column has the same denominator $$d$$, as for an integer matrix divided by $$d$$, the matrix is
$$N/d$$ and its square is $$N^2/d^2$$, with $$N^2$$ computed as `IntegerMatrix`'s square, which
uses Bodrato's algorithm for small matrices (see [the integer matrix
page](/mapping/flint-integer-matrices/#matrix-multiplication)); measured, that was up to 1.7 times
faster for small elements.

FLINT has no `fmpq_mat_pow` either. Malachite's
[`Pow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Pow.html)`<u64>`
and
[`PowAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.PowAssign.html)`<u64>`
for a square `RationalMatrix` usually write it as $$N/d$$, with $$d$$ the common denominator of all
its elements, compute $$N^k$$ as `IntegerMatrix`'s power, and reduce each element of $$N^k/d^k$$
once; measured, that was up to three times faster than binary powering in rational arithmetic,
which reduces every element at every step. When $$d$$ is much larger than the rows' own
denominators, putting the whole matrix over it inflates the numerators enough that binary powering
in rational arithmetic, with the products and squares above, is used instead.

`mul_fmpz_mat` and `mul_r_fmpz_mat` are
[`Mul`](https://doc.rust-lang.org/nightly/core/ops/trait.Mul.html)
of a `RationalMatrix` and an
[`IntegerMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_matrix/struct.IntegerMatrix.html),
in either order and with each operand by value or by reference, and `MulAssign` multiplies a
`RationalMatrix` by an `IntegerMatrix` in place. As in FLINT, only the rational factor's
denominators are cleared, row by row for `mul_fmpz_mat` and column by column for
`mul_r_fmpz_mat`.

The matrix-vector products are
[`Mul`](https://doc.rust-lang.org/nightly/core/ops/trait.Mul.html) of a `RationalMatrix` and a
[`RationalVector`](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/struct.RationalVector.html),
in that order for $$Ab$$ and in the other for $$aB$$, with each operand by value or by reference;
for the `fmpz_vec` forms, convert an
[`IntegerVector`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/struct.IntegerVector.html)
with [`From`](https://doc.rust-lang.org/nightly/core/convert/trait.From.html) first, and for the
`_ptr` forms, collect the elements into a vector. As for integer matrices, FLINT truncates or
zero-extends the vector to the length the product needs, while Malachite panics unless the
dimensions match, hence ≈. The methods differ too: FLINT clears the vector's denominators and then
accumulates each element of the result in rational arithmetic, while Malachite clears the
denominators of both the matrix and the vector, does one integer product, and divides each
element of the result by the product of the two denominators once.

## [Kronecker product](https://flintlib.org/doc/fmpq_mat.html#kronecker-product) {#kronecker-product}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpq_mat_kronecker_product(fmpq_mat_t C, const fmpq_mat_t A, const fmpq_mat_t B)` | [`KroneckerProduct`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.KroneckerProduct.html), [`KroneckerProductAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.KroneckerProductAssign.html) |

This is the rational version of `fmpz_mat_kronecker_product`, and the [integer matrix
page's note](/mapping/flint-integer-matrices/#kronecker-product) applies unchanged:
[`KroneckerProduct`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.KroneckerProduct.html)
and
[`KroneckerProductAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.KroneckerProductAssign.html)
are implemented for `RationalMatrix` too, with each element of the product computed as a product of
two rationals. FLINT does not check the shape of `C`.

## [Trace](https://flintlib.org/doc/fmpq_mat.html#trace) {#trace}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpq_mat_trace(fmpq_t trace, const fmpq_mat_t mat)` | [`Trace`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.Trace.html) |

As for [integer matrices](/mapping/flint-integer-matrices/#trace), the trace is the sum of the
diagonal elements, and 0 for the $$0 \times 0$$ matrix: [`Trace`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.Trace.html), as `a.to_trace()` or
`a.into_trace()`, returning a `Rational`. The manual says that squareness is required and not
checked, and `fmpq_mat/trace.c`, like the integer version, sums positions $$(i, i)$$ for every row
$$i$$, so for a matrix with more rows than columns it reads past the last element. `Trace` panics
if the matrix is not square.

## [Determinant](https://flintlib.org/doc/fmpq_mat.html#determinant) {#determinant}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpq_mat_det(fmpq_t det, const fmpq_mat_t mat)` | [`Determinant`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.Determinant.html) |

In Malachite, this is [`Determinant`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.Determinant.html) on a [`RationalMatrix`](https://docs.rs/malachite-q/latest/malachite_q/rational_matrix/struct.RationalMatrix.html), by value or by reference, which returns a
[`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html) with FLINT's
result (see also [the integer matrix page](/mapping/flint-integer-matrices/#determinant)). It
panics unless the matrix is square. `fmpq_mat_det` computes dimensions 0 to 2
directly, with the $$0 \times 0$$ determinant 1, and otherwise clears the denominators row by row,
takes the integer determinant of the numerators, and divides it by the product of the row
denominators, since scaling a row scales the determinant by the same factor. The direct cases do
not check that the matrix is square (a $$1 \times n$$ matrix gives its first element), while the
integer determinant throws on a non-square matrix.

## [Permanent](https://flintlib.org/doc/fmpq_mat.html#permanent) {#permanent}

| | FLINT | Malachite |
| :---: | --- | --- |
| ≈ | `int fmpq_mat_permanent(fmpq_t res, const fmpq_mat_t A)` | [`Permanent`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.Permanent.html) |

`fmpq_mat_permanent` computes dimensions 0 to 2 directly and otherwise clears the denominators
row by row, takes the [integer permanent](/mapping/flint-integer-matrices/#permanent) of the
numerators, and divides it by the product of the row denominators, as for the
[determinant](#determinant). It inherits the integer version's limit: above dimension 62 it
returns 0, whatever the matrix, so it refuses even the identity matrix, whose permanent is 1.
Malachite's
[`Permanent`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.Permanent.html)
for a
[`RationalMatrix`](https://docs.rs/malachite-q/latest/malachite_q/rational_matrix/struct.RationalMatrix.html),
`a.permanent()`, clears the denominators in the same way, but it always returns the permanent,
hence ≈: large dense matrices take exponential time, as they must, while sparse and structured ones
(permutation, triangular, block-diagonal) remain cheap. See the [integer
page](/mapping/flint-integer-matrices/#permanent) for how.

## [Nonsingular solving](https://flintlib.org/doc/fmpq_mat.html#nonsingular-solving) {#nonsingular-solving}

| | FLINT | Malachite |
| :---: | --- | --- |
| ⚙ | `int fmpq_mat_solve_fraction_free(fmpq_mat_t X, const fmpq_mat_t A, const fmpq_mat_t B)` | internal |
| ⚙ | `int fmpq_mat_solve_dixon(fmpq_mat_t X, const fmpq_mat_t A, const fmpq_mat_t B)` | internal |
| ⚙ | `int fmpq_mat_solve_multi_mod(fmpq_mat_t X, const fmpq_mat_t A, const fmpq_mat_t B)` | internal |
| ≈ | `int fmpq_mat_solve(fmpq_mat_t X, const fmpq_mat_t A, const fmpq_mat_t B)` | [`Solve`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.Solve.html) |
| ⚙ | `int fmpq_mat_solve_fmpz_mat_fraction_free(fmpq_mat_t X, const fmpz_mat_t A, const fmpz_mat_t B)` | internal |
| ⚙ | `int fmpq_mat_solve_fmpz_mat_dixon(fmpq_mat_t X, const fmpz_mat_t A, const fmpz_mat_t B)` | internal |
| ⚙ | `int fmpq_mat_solve_fmpz_mat_multi_mod(fmpq_mat_t X, const fmpz_mat_t A, const fmpz_mat_t B)` | internal |
| ⚙ | `int fmpq_mat_solve_fmpz_mat(fmpq_mat_t X, const fmpz_mat_t A, const fmpz_mat_t B)` | internal |
| ⚙ | `int fmpq_mat_can_solve_multi_mod(fmpq_mat_t X, const fmpq_mat_t A, const fmpq_mat_t B)` | internal |
| ⚙ | `int fmpq_mat_can_solve_fraction_free(fmpq_mat_t X, const fmpq_mat_t A, const fmpq_mat_t B)` | internal |
| ⚙ | `int fmpq_mat_can_solve_fmpz_mat_dixon(fmpq_mat_t X, const fmpz_mat_t A, const fmpz_mat_t B)` | internal |
| ⚙ | `int fmpq_mat_can_solve_dixon(fmpq_mat_t X, const fmpq_mat_t A, const fmpq_mat_t B)` | internal |
| ≈ | `int fmpq_mat_can_solve(fmpq_mat_t X, const fmpq_mat_t A, const fmpq_mat_t B)` | [`Solve`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.Solve.html) |

These solve $$AX = B$$ over the rationals, writing $$X$$ as a rational matrix. The `solve`
functions require a square nonsingular $$A$$ (they return 0 otherwise, and nonzero when $$B$$ has no
columns); the `can_solve` functions accept any $$A$$ with as many rows as $$B$$ and report whether a
solution exists. The `fmpz_mat` forms take integer $$A$$ and $$B$$. All of them clear denominators
and solve over the integers (see [the integer matrix
page](/mapping/flint-integer-matrices/#nonsingular-solving)).

In Malachite, both `fmpq_mat_solve` and `fmpq_mat_can_solve` are
[`Solve`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.Solve.html)
on a [`RationalMatrix`](https://docs.rs/malachite-q/latest/malachite_q/rational_matrix/struct.RationalMatrix.html), by value or by reference, with a [`RationalMatrix`](https://docs.rs/malachite-q/latest/malachite_q/rational_matrix/struct.RationalMatrix.html) or a
[`RationalVector`](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/struct.RationalVector.html)
right-hand side, also by value or by reference. It returns `Some(X)` if the system has a solution
and `None` otherwise, for any shape: when $$A$$ is square and nonsingular, the result is the unique
solution, which `fmpq_mat_solve` also returns, and otherwise it is `fmpq_mat_can_solve`'s. It is ≈
to both because it panics if $$A$$ and $$B$$ have different heights, and because a square singular
system that has solutions gets one, where `fmpq_mat_solve` returns 0. The algorithm-specific forms
are internal choices behind `Solve` and give FLINT's results. There is no public form for integer
inputs; convert them with
[`From`](https://doc.rust-lang.org/nightly/core/convert/trait.From.html). `solve_fmpz_mat_dixon`,
`solve_fmpz_mat_multi_mod`, `can_solve_fmpz_mat_dixon`, and the undocumented
`can_solve_fmpz_mat_multi_mod` are internal functions in `malachite-nz`, the cores of the integer
solver, [`Solve`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.Solve.html)
on an
[`IntegerMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_matrix/struct.IntegerMatrix.html);
`solve_fmpz_mat` and `solve_fmpz_mat_fraction_free` are internal in `malachite-q`.

`fmpq_mat_solve` and `solve_fmpz_mat` use the fraction-free method up to dimension 15, Dixon's
$$p$$-adic lifting when $$B$$ has at most 32 columns, and the multimodular method otherwise;
`can_solve` uses the fraction-free method up to 15 rows and the multimodular one above that. The
fraction-free forms are fraction-free only up to dimension 15: they call the dispatching
`fmpz_mat_solve`, which itself uses Dixon or the multimodular method for larger systems. In the
manual, `can_solve_fmpz_mat_dixon` and `can_solve_dixon` lack the directive that marks them as
functions, so they appear as plain text, and `fmpq_mat.h` also declares an undocumented
`fmpq_mat_can_solve_fmpz_mat_multi_mod`.

## [Inverse](https://flintlib.org/doc/fmpq_mat.html#inverse) {#inverse}

| | FLINT | Malachite |
| :---: | --- | --- |
| ≈ | `int fmpq_mat_inv(fmpq_mat_t B, const fmpq_mat_t A)` | [`Inverse`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Inverse.html) |

`fmpq_mat_inv` sets `B` to $$A^{-1}$$ and returns 0 if $$A$$ is singular; nothing checks that $$A$$
is square. In Malachite, this is
[`Inverse`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Inverse.html)
on a [`RationalMatrix`](https://docs.rs/malachite-q/latest/malachite_q/rational_matrix/struct.RationalMatrix.html), by value or by reference, which returns `Some(B)` with FLINT's result, or `None` if
$$A$$ is singular; it is ≈ because it panics if $$A$$ is not square. FLINT computes dimensions 0 to 2 directly, and
otherwise clears the denominators row by row, writing $$A = D^{-1}\bar{A}$$ with $$\bar{A}$$ an
integer matrix and $$D$$ the diagonal matrix of row denominators; since $$A^{-1} = \bar{A}^{-1}D$$,
it solves $$\bar{A}X = D$$ with [`fmpq_mat_solve_fmpz_mat`](#nonsingular-solving).

## [Echelon form](https://flintlib.org/doc/fmpq_mat.html#echelon-form) {#echelon-form}

| | FLINT | Malachite |
| :---: | --- | --- |
| ⚙ | `int fmpq_mat_pivot(slong * perm, fmpq_mat_t mat, slong r, slong c)` | internal |
| ⚙ | `slong fmpq_mat_rref_classical(fmpq_mat_t B, const fmpq_mat_t A)` | internal |
| ⚙ | `slong fmpq_mat_rref_fraction_free(fmpq_mat_t B, const fmpq_mat_t A)` | internal |
| ≈ | `slong fmpq_mat_rref(fmpq_mat_t B, const fmpq_mat_t A)` | [`ReducedRowEchelonForm`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ReducedRowEchelonForm.html), [`ReducedRowEchelonFormAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ReducedRowEchelonFormAssign.html) |

The `rref` functions set `B` to the reduced row echelon form of $$A$$, whose pivots are 1, and
return the rank; unlike the [integer matrix
functions](/mapping/flint-integer-matrices/#row-reduction), they need no separate denominator.
In Malachite, `fmpq_mat_rref` is [`ReducedRowEchelonForm`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ReducedRowEchelonForm.html) on a [`RationalMatrix`](https://docs.rs/malachite-q/latest/malachite_q/rational_matrix/struct.RationalMatrix.html), by value or by
reference, and [`ReducedRowEchelonFormAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ReducedRowEchelonFormAssign.html); they are ≈ because the rank is not returned,
but it is the number of nonzero rows. The algorithm-specific forms and `fmpq_mat_pivot` are the
internal choices behind them. FLINT has no rank or nullspace for `fmpq_mat`; Malachite also has
[`Rank`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.Rank.html), the rank of the matrix cleared of row denominators, and [`Nullspace`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.Nullspace.html), the
basis of the right nullspace with a 1 in each free variable's position, read off the reduced row
echelon form. `fmpq_mat_rref` uses Gauss–Jordan elimination over the rationals
(`rref_classical`) when the matrix has at most 2 rows or at most 2 columns, and `rref_fraction_free`
otherwise. That clears the denominators row by row, which does not change the reduced row echelon
form, and calls `fmpz_mat_rref`, which chooses between fraction-free and multimodular elimination by
shape, so the fraction-free form is not always fraction-free. `fmpq_mat_pivot` is the classical
algorithm's pivot search: it finds a nonzero entry at or below row $$r$$ in column $$c$$, swaps it
into row $$r$$, applying the swap to `perm` too, and returns 1 if no swap was needed, $$-1$$ after a
swap, and 0 if the column has no such entry.

## [Gram-Schmidt Orthogonalisation](https://flintlib.org/doc/fmpq_mat.html#gram-schmidt-orthogonalisation) {#gram-schmidt-orthogonalisation}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpq_mat_gso(fmpq_mat_t B, const fmpq_mat_t A)` | [`GramSchmidt`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.GramSchmidt.html), [`GramSchmidtAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.GramSchmidtAssign.html) |

`fmpq_mat_gso` applies the Gram–Schmidt process, exactly, to the *columns* of $$A$$ (the LLL
functions on [the integer matrix page](/mapping/flint-integer-matrices/#lll) work on rows): column
$$i$$ of `B` is column $$i$$ of $$A$$ minus its projections onto the earlier columns of `B`. The
columns of `B` are orthogonal but not normalized, since normalizing would need square roots. A
column that depends on the earlier ones becomes zero, and the later columns skip it, so `B` keeps
all $$n$$ columns and spans the same space. FLINT throws unless `B` has the shape of `A`. In
Malachite it is [`GramSchmidt`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.GramSchmidt.html) on a
[`RationalMatrix`](https://docs.rs/malachite-q/latest/malachite_q/rational_matrix/struct.RationalMatrix.html),
which returns the result, so there is no shape to mismatch, or
[`GramSchmidtAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.GramSchmidtAssign.html) in place. The result is the same, columns and all,
but it is computed differently: each column is scaled to integers by the LCM of its denominators,
the integral columns are orthogonalized without fractions, dividing exactly by Gram determinants,
and each column of the result is formed as a single fraction at the end, where FLINT works with
fractions throughout.

FLINT has no Gram–Schmidt process modulo a number. Malachite also has
[`ModGramSchmidt`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModGramSchmidt.html) and
[`ModPowerOf2GramSchmidt`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModPowerOf2GramSchmidt.html), for an
[`UnsignedMatrix`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_matrix/struct.UnsignedMatrix.html)
or a [`NaturalMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_matrix/struct.NaturalMatrix.html)
modulo $$m$$ or $$2^k$$. They return `None` when a squared length they need to divide by is not
invertible, which includes a nonzero column orthogonal to itself, such as $$(1, 1)$$ modulo 2.

## [Transforms](https://flintlib.org/doc/fmpq_mat.html#transforms) {#transforms}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpq_mat_similarity(fmpq_mat_t A, slong r, fmpq_t d)` | [`SimilarityTransformAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.SimilarityTransformAssign.html), [`SimilarityTransform`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.SimilarityTransform.html) |

This is the rational version of `fmpz_mat_similarity`, and the [integer matrix
page's note](/mapping/flint-integer-matrices/#transforms) applies unchanged: it replaces $$M$$
with $$P^{-1}MP$$, preserving the determinant and the characteristic and minimal polynomials. In
Malachite it is [`SimilarityTransformAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.SimilarityTransformAssign.html)
on a
[`RationalMatrix`](https://docs.rs/malachite-q/latest/malachite_q/rational_matrix/struct.RationalMatrix.html),
with `d` a [`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html)
by value or by reference, or [`SimilarityTransform`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.SimilarityTransform.html),
which returns the result; it computes exactly what the code does. `fmpq_mat/similarity.c` has the same loops as the integer
version, so for $$r \ge 1$$ the $$P$$ it applies leaves the entry in row $$r$$ and column $$r - 1$$
at $$0$$, rather than the one the manual describes, and nothing checks that the matrix is square or
that $$r$$ is in range; Malachite panics in either case.

## [Characteristic polynomial](https://flintlib.org/doc/fmpq_mat.html#characteristic-polynomial) {#characteristic-polynomial}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpq_mat_charpoly(fmpq_poly_t pol, const fmpq_mat_t mat)` | [`CharacteristicPolynomial`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.CharacteristicPolynomial.html) |

`fmpq_mat_charpoly` sets `pol` to $$\det(xI - A)$$, which is monic, and 1 for the $$0 \times 0$$
matrix; it throws on a non-square matrix. In Malachite it is
[`CharacteristicPolynomial`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.CharacteristicPolynomial.html)
on a `RationalMatrix`, by value or by reference, returning a
[`RationalPolynomial`](https://docs.rs/malachite-q/latest/malachite_q/rational_polynomial/struct.RationalPolynomial.html);
it panics on a non-square matrix. Both libraries write $$A = Z/p$$ over the common denominator
$$p$$ of its elements (see [Integer matrix conversion](#integer-matrix-conversion)), compute the
[integer characteristic polynomial](/mapping/flint-integer-matrices/#characteristic-polynomial)
$$\chi_Z$$, and use $$\det(xI - A) = p^{-n}\chi_Z(px)$$, scaling the coefficients by powers of
$$p$$.
(`_fmpq_mat_charpoly` is omitted.)

## [Minimal polynomial](https://flintlib.org/doc/fmpq_mat.html#minimal-polynomial) {#minimal-polynomial}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void fmpq_mat_minpoly(fmpq_poly_t pol, const fmpq_mat_t mat)` | |

`fmpq_mat_minpoly` sets `pol` to the minimal polynomial of $$A$$, the monic polynomial $$f$$ of
least degree with $$f(A) = 0$$, and 1 for the $$0 \times 0$$ matrix; it throws on a non-square
matrix. Malachite does not compute it yet; when it does, the result will be a
[`RationalPolynomial`](https://docs.rs/malachite-q/latest/malachite_q/rational_polynomial/struct.RationalPolynomial.html).
As for the [characteristic polynomial](#characteristic-polynomial), FLINT writes $$A = Z/p$$ over a
common denominator, computes the [integer minimal
polynomial](/mapping/flint-integer-matrices/#minimal-polynomial) $$m_Z$$, of some degree $$d$$, and
uses $$m_A(x) = p^{-d} m_Z(px)$$. (`_fmpq_mat_minpoly` is omitted.)
