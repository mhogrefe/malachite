---
layout: default
title: "Malachite for FLINT Users: Integer Matrices"
permalink: /mapping/flint-integer-matrices/
theme: jekyll-theme-slate
---

# Malachite for FLINT Users: Integer Matrices

This page maps the functions of FLINT's integer matrix type, `fmpz_mat_t`, onto their Malachite
counterpart:
[`IntegerMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_matrix/struct.IntegerMatrix.html),
from the `malachite-nz` crate. It follows the organization of the
[fmpz_mat.h chapter](https://flintlib.org/doc/fmpz_mat.html) of the FLINT manual, as of FLINT
3.6.0, and is a companion to [Malachite for FLINT Users: Integers](/mapping/flint-integers/),
whose [conventions](/mapping/flint-integers/#conventions), including word types and aliasing,
apply here as well; the [mapping index](/mapping/) lists the whole family.

Every section of the chapter is mapped below: 169 rows across 36 sections, of which 70 are ✓,
39 ⚙, 30 ≈, 10 —, and 20 ✗. Functions whose names begin with an underscore are omitted, as on
[the fmpq page](/mapping/flint-rationals/), and functions that `fmpz_mat.h` declares but the
chapter does not document are mentioned in the notes of the sections they belong to.

## Conventions {#conventions}

### The `fmpz_mat` representation {#representation}

An `fmpz_mat_struct` holds `entries`, a pointer to the elements in row-major order, the
dimensions `r` and `c`, and a `stride`, the distance between the starts of consecutive rows. An
[`IntegerMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_matrix/struct.IntegerMatrix.html)
stores its elements in the same row-major order, in a private
[`Vec<Integer>`](https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html): the element in row
$$i$$ and column $$j$$ is at index `i * width + j`. The elements, height, and width are all private,
so that the number of elements always equals the product of the height and width. Elements are read
and written by indexing, `m[(i, j)]`, or with `get` and `get_mut`, and
[`elements_ref`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.elements_ref)
and
[`into_elements`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.into_elements)
give all of them in row-major order.

There is no stride. FLINT's stride exists so that a window can share its parent's entries, and an
[`IntegerMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_matrix/struct.IntegerMatrix.html)
always owns its elements.

Heights, widths, and indices are
[`u64`](https://doc.rust-lang.org/nightly/core/primitive.u64.html)s, as they are for Malachite's
vectors, rather than `slong`s, so they cannot be negative. The construction and access methods
common to all of Malachite's matrix types are in the
[`Matrix`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html) trait.

### Elements that are not `Integer`s

Malachite has four matrix types differing only in their elements:
[`IntegerMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_matrix/struct.IntegerMatrix.html)
over $$\mathbb{Z}$$,
[`NaturalMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_matrix/struct.NaturalMatrix.html)
over $$\mathbb{N}$$,
[`UnsignedMatrix`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_matrix/struct.UnsignedMatrix.html)
with primitive unsigned elements (in `malachite-base`), and
[`RationalMatrix`](https://docs.rs/malachite-q/latest/malachite_q/rational_matrix/struct.RationalMatrix.html)
over $$\mathbb{Q}$$. They are the counterparts of FLINT's `fmpz_mat`, `fmpz_mod_mat`, `nmod_mat`,
and `fmpq_mat` modules respectively:
[`NaturalMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_matrix/struct.NaturalMatrix.html)
and
[`UnsignedMatrix`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_matrix/struct.UnsignedMatrix.html)
also hold residues for the `Mod*` and `ModPowerOf2*` traits, which check that their arguments are
reduced. Every row below names the
[`IntegerMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_matrix/struct.IntegerMatrix.html)
version; the memory-management rows apply unchanged to the other three.

### Categories

Each function falls into one of five categories:

| | meaning |
| :---: | --- |
| ✓ | A Malachite function does the same thing. |
| ≈ | A Malachite function serves the same purpose, but its specification differs. The notes say how. |
| ⚙ | Malachite does not expose this algorithm or helper; the Malachite column or the notes say what to call instead. |
| — | No counterpart is needed, either because Rust handles it for you or because it is outside Malachite's scope. The notes say which. |
| ✗ | Malachite does not fully support this yet, but will in a future version. |

## [Types, macros and constants](https://flintlib.org/doc/fmpz_mat.html#types-macros-and-constants) {#types-macros-and-constants}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `fmpz_mat_struct` | [`IntegerMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_matrix/struct.IntegerMatrix.html) |
| — | `fmpz_mat_t` | |

`fmpz_mat_t` is the pass-by-reference device that `&` and `&mut` already are.

## [Memory management](https://flintlib.org/doc/fmpz_mat.html#memory-management) {#memory-management}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpz_mat_init(fmpz_mat_t mat, slong rows, slong cols)` | [`zero`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.zero) |
| — | `void fmpz_mat_clear(fmpz_mat_t mat)` | |

`fmpz_mat_init` returns the zero matrix of the given shape (its entries come from `calloc`, and a
zeroed `fmpz` is $$0$$). That is
[`zero`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.zero),
the [`Matrix`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html)
method: `IntegerMatrix::zero(rows, cols)`. Either dimension may be zero in both libraries, and
both reject a shape whose number of elements does not fit in a word: FLINT throws, and
[`zero`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.zero)
panics. `fmpz_mat_clear` is [`Drop`](https://doc.rust-lang.org/nightly/core/ops/trait.Drop.html),
which runs when the matrix goes out of scope.

## [Basic assignment and manipulation](https://flintlib.org/doc/fmpz_mat.html#basic-assignment-and-manipulation) {#basic-assignment-and-manipulation}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpz_mat_set(fmpz_mat_t mat1, const fmpz_mat_t mat2)` | [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html) |
| ✓ | `void fmpz_mat_init_set(fmpz_mat_t mat, const fmpz_mat_t src)` | [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html) |
| ✓ | `slong fmpz_mat_nrows(const fmpz_mat_t mat)` | [`height`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.height) |
| ✓ | `slong fmpz_mat_ncols(const fmpz_mat_t mat)` | [`width`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.width) |
| ✓ | `void fmpz_mat_swap(fmpz_mat_t mat1, fmpz_mat_t mat2)` | [`mem::swap`](https://doc.rust-lang.org/nightly/core/mem/fn.swap.html) |
| — | `void fmpz_mat_swap_entrywise(fmpz_mat_t mat1, fmpz_mat_t mat2)` | |
| ✓ | `fmpz * fmpz_mat_entry(const fmpz_mat_t mat, slong i, slong j)` | [`Index`](https://doc.rust-lang.org/nightly/core/ops/trait.Index.html), [`IndexMut`](https://doc.rust-lang.org/nightly/core/ops/trait.IndexMut.html), [`get`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_matrix/struct.IntegerMatrix.html#method.get), [`get_mut`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_matrix/struct.IntegerMatrix.html#method.get_mut) |
| ✓ | `void fmpz_mat_zero(fmpz_mat_t mat)` | [`set_zero`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.set_zero) |
| ✓ | `void fmpz_mat_one(fmpz_mat_t mat)` | [`rectangular_identity`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.rectangular_identity), [`identity`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.identity) |
| ≈ | `void fmpz_mat_swap_rows(fmpz_mat_t mat, slong * perm, slong r, slong s)` | [`swap_rows`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.swap_rows) |
| ≈ | `void fmpz_mat_swap_cols(fmpz_mat_t mat, slong * perm, slong r, slong s)` | [`swap_columns`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.swap_columns) |
| ≈ | `void fmpz_mat_invert_rows(fmpz_mat_t mat, slong * perm)` | [`reverse_rows`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.reverse_rows) |
| ≈ | `void fmpz_mat_invert_cols(fmpz_mat_t mat, slong * perm)` | [`reverse_columns`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.reverse_columns) |

`fmpz_mat_set` and `fmpz_mat_init_set` are both
[`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html):
[`clone_from`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)
reuses the destination's allocation as `fmpz_mat_set` does, and does not need the two shapes to
match. [`height`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.height)
and [`width`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.width)
return [`u64`](https://doc.rust-lang.org/nightly/core/primitive.u64.html)s.

`fmpz_mat_swap` exchanges the two structs, and `fmpz_mat_swap_entrywise` exchanges the entries
one by one. They differ only for a window, whose entries belong to its parent: swapping the
structs leaves the parent unchanged. Malachite has no windows (see
[Conventions](#representation)), so
[`mem::swap`](https://doc.rust-lang.org/nightly/core/mem/fn.swap.html) covers both.

`fmpz_mat_entry` returns a pointer that can be read or written. The counterparts are indexing,
`a[(i, j)]`, which takes [`usize`](https://doc.rust-lang.org/nightly/core/primitive.usize.html)
indices and panics when one is out of range, and
[`get`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_matrix/struct.IntegerMatrix.html#method.get)
and
[`get_mut`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_matrix/struct.IntegerMatrix.html#method.get_mut),
which take [`u64`](https://doc.rust-lang.org/nightly/core/primitive.u64.html) indices and return
[`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None) when one is
out of range. FLINT does not check the indices.

`fmpz_mat_one` sets a matrix of any shape to have ones in positions $$(i, i)$$ and zeros elsewhere;
for an $$m \times n$$ matrix with $$m \neq n$$ this is the "truncation of a unit matrix", the
identity with zero rows or columns appended. That is
[`rectangular_identity`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.rectangular_identity),
`IntegerMatrix::rectangular_identity(m, n)`, which returns a new matrix of that shape;
[`identity`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.identity)
takes one size and returns the square case.

`fmpz_mat_swap_rows` and `fmpz_mat_swap_cols` are
[`swap_rows`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.swap_rows)
and
[`swap_columns`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.swap_columns),
`a.swap_rows(r, s)`. They are ≈ only because they take no permutation: FLINT's optional array
`perm` records the moves, while a Malachite caller that tracks the permutation keeps its own `Vec`
and applies the same move with
[`swap`](https://doc.rust-lang.org/nightly/core/primitive.slice.html#method.swap),
`perm.swap(r, s)`. Malachite panics on an index out of range, which FLINT does not check.
`fmpz_mat_invert_rows` and `fmpz_mat_invert_cols` reverse the order of the rows or columns; they
are
[`reverse_rows`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.reverse_rows)
and
[`reverse_columns`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.reverse_columns),
named for what they do rather than after the inverse of a matrix, and ≈ for the same reason, with
[`reverse`](https://doc.rust-lang.org/nightly/core/primitive.slice.html#method.reverse)
reversing a separate permutation. The manual says that `fmpz_mat_invert_rows` swaps rows $$i$$
and $$r - i$$, and `fmpz_mat_invert_cols` columns $$i$$ and $$c - i$$; the code
(`fmpz_mat/invert.c`) swaps rows $$i$$ and $$r - 1 - i$$, and columns $$i$$ and $$c - 1 - i$$, so
each function reverses the order.

## [Window](https://flintlib.org/doc/fmpz_mat.html#window) {#window}

| | FLINT | Malachite |
| :---: | --- | --- |
| ≈ | `void fmpz_mat_window_init(fmpz_mat_t window, const fmpz_mat_t mat, slong r1, slong c1, slong r2, slong c2)` | [`window`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.window), [`into_window`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.into_window), [`set_window`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.set_window), [`set_window_owned`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.set_window_owned) |
| — | `void fmpz_mat_window_clear(fmpz_mat_t window)` | |

A window is an `fmpz_mat_t` whose `entries` point into another matrix: rows $$r_1$$ to $$r_2 - 1$$
and columns $$c_1$$ to $$c_2 - 1$$ of `mat`, read and written in place through the parent's stride
(see [Conventions](#representation)). FLINT uses windows mainly to recurse on blocks, as in
Strassen multiplication, without copying. `fmpz_mat_window_clear` does nothing; it exists so that
every initialised matrix has a matching clear.

Malachite has no windows that share their parent's elements: a matrix always owns its elements,
so there is no type that borrows part of another matrix. Its counterparts copy the block instead,
hence ≈. `a.window(r1..r2, c1..c2)`, with
[`window`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.window),
returns rows $$r_1$$ to $$r_2 - 1$$ and columns $$c_1$$ to $$c_2 - 1$$ as a new matrix.
[`into_window`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.into_window)
does the same while taking ownership of `a` and reusing its storage; for a prefix of the rows it
only truncates the elements. `a.set_window(r1, c1, &w)`, with
[`set_window`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.set_window),
or
[`set_window_owned`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.set_window_owned),
writes a block back. Code that recursed on windows therefore copies each block out, works on the
copy, and copies the result back. Each copy costs time and memory proportional to the size of the
block, which in a recursive algorithm such as Strassen multiplication or block LU decomposition is
small next to the arithmetic done on it. A block can also be read or written in place, row by row,
with
[`row_iter`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_matrix/struct.IntegerMatrix.html#method.row_iter)
or
[`row_iter_mut`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_matrix/struct.IntegerMatrix.html#method.row_iter_mut),
skipping $$c_1$$ elements of each row in $$r_1..r_2$$ and taking $$c_2 - c_1$$.
`fmpz_mat_window_clear` has no counterpart, since a copy is dropped like any other matrix.

Two details of the FLINT implementation (an inline function in `fmpz_mat.h`) matter when porting
code that uses windows; Malachite's functions panic when a range runs backwards or past the
matrix. Although `mat` is declared `const`, the window can be written through, and
the writes change `mat`. The bounds are checked only by `FLINT_ASSERT`, so not at all in a release
build, and the column check tests `c2 >= 0` where the row check tests the lower bound, so a
negative `c1` is not caught even in a debug build.

## [Random matrix generation](https://flintlib.org/doc/fmpz_mat.html#random-matrix-generation) {#random-matrix-generation}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpz_mat_randbits(fmpz_mat_t mat, flint_rand_t state, flint_bitcnt_t bits)` | [`random_integer_matrices_with_dimensions_from_iterator`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_matrix/random/fn.random_integer_matrices_with_dimensions_from_iterator.html) |
| ≈ | `void fmpz_mat_randtest(fmpz_mat_t mat, flint_rand_t state, flint_bitcnt_t bits)` | [`random_integer_matrices_with_dimensions`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_matrix/random/fn.random_integer_matrices_with_dimensions.html), [`striped_random_integer_matrices_with_dimensions`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_matrix/random/fn.striped_random_integer_matrices_with_dimensions.html) |
| ✗ | `void fmpz_mat_randintrel(fmpz_mat_t mat, flint_rand_t state, flint_bitcnt_t bits)` | |
| ✗ | `void fmpz_mat_randsimdioph(fmpz_mat_t mat, flint_rand_t state, flint_bitcnt_t bits, flint_bitcnt_t bits2)` | |
| ✗ | `void fmpz_mat_randntrulike(fmpz_mat_t mat, flint_rand_t state, flint_bitcnt_t bits, ulong q)` | |
| ✗ | `void fmpz_mat_randntrulike2(fmpz_mat_t mat, flint_rand_t state, flint_bitcnt_t bits, ulong q)` | |
| ✗ | `void fmpz_mat_randajtai(fmpz_mat_t mat, flint_rand_t state, double alpha)` | |
| ✓ | `int fmpz_mat_randpermdiag(fmpz_mat_t mat, flint_rand_t state, const fmpz * diag, slong n)` | [`random_integer_matrices_with_permuted_diagonal`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_matrix/random/fn.random_integer_matrices_with_permuted_diagonal.html) |
| ≈ | `void fmpz_mat_randrank(fmpz_mat_t mat, flint_rand_t state, slong rank, flint_bitcnt_t bits)` | [`random_integer_matrices_with_rank`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_matrix/random/fn.random_integer_matrices_with_rank.html) |
| ≈ | `void fmpz_mat_randdet(fmpz_mat_t mat, flint_rand_t state, const fmpz_t det)` | [`random_integer_matrices_with_determinant`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_matrix/random/fn.random_integer_matrices_with_determinant.html) |
| ✓ | `void fmpz_mat_randops(fmpz_mat_t mat, flint_rand_t state, slong count)` | [`random_integer_matrices_from_elementary_operations`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_matrix/random/fn.random_integer_matrices_from_elementary_operations.html) |

As for [single integers](/mapping/flint-integers/#random-generation), Malachite's random
generators are infinite iterators seeded once, rather than functions that fill one value from a
`flint_rand_t`. The matrix generators come in a fixed-shape form, `_with_dimensions`, and a form
that draws the height and width from a geometric distribution with a given mean.

`fmpz_mat_randbits` fills every element with
[`fmpz_randbits`](/mapping/flint-integers/#random-generation): exactly `bits` bits and a random
sign. Pass a stream of such integers, built with
[`get_random_natural_with_bits`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/random/fn.get_random_natural_with_bits.html)
and
[`from_sign_and_abs`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html#method.from_sign_and_abs),
to
[`random_integer_matrices_with_dimensions_from_iterator`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_matrix/random/fn.random_integer_matrices_with_dimensions_from_iterator.html),
which takes the elements of each matrix from any iterator.

`fmpz_mat_randtest` differs from its counterparts as
[`_fmpz_vec_randtest`](/mapping/flint-integer-vectors/#randomisation) does: its elements come from
`fmpz_randtest`, which caps their bit length at `bits`, where Malachite's take a mean bit
length with no cap, and half the time FLINT makes the matrix sparse, which Malachite's generators
never do. (`fmpz_mat/randtest.c` follows the vector function: it draws $$s$$ uniformly from
$$1$$ to $$\max(2, rc)$$, and each element is then zero with probability $$1 - 1/s$$.)

The next five functions build the structured inputs that FLINT uses to test lattice reduction
(LLL): `randintrel` a column of random integers beside an identity, `randsimdioph` a power of 2
followed by random integers in the first row above a diagonal of `2^bits`, `randntrulike` and
`randntrulike2` the block matrices $$\begin{pmatrix} I & H \\ 0 & qI \end{pmatrix}$$ and
$$\begin{pmatrix} qI & 0 \\ H & I \end{pmatrix}$$ with $$H$$ circulant, and `randajtai` a lower
triangular matrix whose bit sizes shrink down the diagonal. Each throws unless the matrix has the
shape it needs. Malachite has none of them.

The last four make matrices with a prescribed rank or determinant. `randpermdiag` places the given
diagonal entries at the positions of a random row permutation and a random column permutation, and
returns the parity of the combined permutation; it assumes, without checking, that `n` is at most
both dimensions. `randrank` calls it with `rank` random nonzero entries, drawn with
`fmpz_randtest_not_zero`, so they have *up to* `bits` bits rather than the "given bit size" the
manual says. `randdet` factors `det` and spreads its prime factors over the diagonal before
calling `randpermdiag`, then negates one entry if the permutation was odd. `randops` makes such a
sparse matrix dense, by `count` random additions or subtractions of one row to another, or one
column to another; a step that picks the same row or column twice is skipped, which is why the
manual says "at most".

Malachite's counterparts are iterators with the shape and parameters as arguments.
[`random_integer_matrices_with_permuted_diagonal`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_matrix/random/fn.random_integer_matrices_with_permuted_diagonal.html)
yields each matrix with the parity, as `randpermdiag` returns it, and panics if the diagonal does
not fit.
[`random_integer_matrices_with_rank`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_matrix/random/fn.random_integer_matrices_with_rank.html)
gives its nonzero entries *exactly* `bits` bits, hence ≈.
[`random_integer_matrices_with_determinant`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_matrix/random/fn.random_integer_matrices_with_determinant.html)
follows `randdet`, but factors the determinant with the word-sized
[`Factor`](https://docs.rs/malachite-base/latest/malachite_base/num/factorization/traits/trait.Factor.html),
so for now its absolute value must fit in a `u64`, hence ≈.
[`random_integer_matrices_from_elementary_operations`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_matrix/random/fn.random_integer_matrices_from_elementary_operations.html)
applies `randops` to every matrix of another iterator, so `randrank` or `randdet` followed by
`randops` is one composed iterator. All of them panic on an impossible shape, rank, or
determinant, which FLINT does not check.

## [Input and output](https://flintlib.org/doc/fmpz_mat.html#input-and-output) {#input-and-output}

As on [the fmpz page](/mapping/flint-integers/#input-and-output), Malachite has no `FILE *`
functions: formatting and parsing work with any Rust reader or writer.

| | FLINT | Malachite |
| :---: | --- | --- |
| ≈ | `int fmpz_mat_fprint(FILE * file, const fmpz_mat_t mat)` | [`Display`](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html), [`Serialize`](https://docs.rs/serde/latest/serde/trait.Serialize.html) |
| ≈ | `int fmpz_mat_fprint_pretty(FILE * file, const fmpz_mat_t mat)` | [`Display`](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html) |
| ≈ | `int fmpz_mat_print(const fmpz_mat_t mat)` | [`Display`](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html), [`Serialize`](https://docs.rs/serde/latest/serde/trait.Serialize.html) |
| ≈ | `int fmpz_mat_print_pretty(const fmpz_mat_t mat)` | [`Display`](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html) |
| ≈ | `int fmpz_mat_fread(FILE * file, fmpz_mat_t mat)` | [`FromStr`](https://doc.rust-lang.org/nightly/core/str/trait.FromStr.html), [`Deserialize`](https://docs.rs/serde/latest/serde/trait.Deserialize.html) |
| ≈ | `int fmpz_mat_read(fmpz_mat_t mat)` | [`FromStr`](https://doc.rust-lang.org/nightly/core/str/trait.FromStr.html), [`Deserialize`](https://docs.rs/serde/latest/serde/trait.Deserialize.html) |

The formats differ, hence ≈. FLINT's plain format is the number of rows, a space, the number of
columns, two spaces, and then the elements in row-major order separated by single spaces, so the
matrix with rows $$(1, 2)$$ and $$(3, 4)$$ is `2 2  1 2 3 4`. The pretty format puts each row in
brackets on its own line, inside one more pair of brackets:

```
[[1 2]
[3 4]
]
```

The closing bracket is on a line of its own: `fmpz_mat/io.c` writes a newline after every row,
although the example at the top of the chapter shows `[3 4]]`.

Malachite's [`Display`](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html) writes the
same matrix as `[1, 2; 3, 4]`, on one line. A matrix with no elements is written with its shape,
as in `[]_{4,0}`, since there is no row to show it, and
[`FromStr`](https://doc.rust-lang.org/nightly/core/str/trait.FromStr.html) reads every form back,
returning an `Err` on failure where FLINT returns a non-positive value. With the `serde` feature,
an [`IntegerMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_matrix/struct.IntegerMatrix.html)
is serialized as its height, its width, and its elements in row-major order: the same three
parts as FLINT's plain format.

`fmpz_mat_fread` resizes a $$0 \times 0$$ matrix to the shape it reads; given any other matrix, it
throws unless the shapes match. If an element fails to parse, it returns 0 and leaves the elements
read so far in place. Parsing in Malachite always produces a new matrix, or nothing; to require a
particular shape, compare
[`height`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.height)
and [`width`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.width)
afterwards. The `print`/`fprint` and `read`/`fread` splits disappear because
[`Display`](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html) writes to any
[`Formatter`](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html), and parsing works
on any string, however it was read.

## [Comparison](https://flintlib.org/doc/fmpz_mat.html#comparison) {#comparison}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `int fmpz_mat_equal(const fmpz_mat_t mat1, const fmpz_mat_t mat2)` | [`PartialEq`](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html) |
| ✓ | `int fmpz_mat_is_zero(const fmpz_mat_t mat)` | [`is_zero`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.is_zero) |
| ✓ | `int fmpz_mat_is_one(const fmpz_mat_t mat)` | [`is_rectangular_identity`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.is_rectangular_identity) |
| ✓ | `int fmpz_mat_is_empty(const fmpz_mat_t mat)` | [`is_empty`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.is_empty) |
| ✓ | `int fmpz_mat_is_square(const fmpz_mat_t mat)` | [`is_square`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.is_square) |
| ✓ | `int fmpz_mat_is_zero_row(const fmpz_mat_t mat, slong i)` | [`is_zero_row`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.is_zero_row) |
| ✓ | `int fmpz_mat_equal_col(fmpz_mat_t M, slong m, slong n)` | [`equal_columns`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.equal_columns) |
| ✓ | `int fmpz_mat_equal_row(fmpz_mat_t M, slong m, slong n)` | [`equal_rows`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.equal_rows) |

Like `fmpz_mat_equal`,
[`PartialEq`](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html) compares the shapes
as well as the elements, so a $$0 \times 2$$ matrix does not equal a $$2 \times 0$$ one. It also
compares an
[`IntegerMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_matrix/struct.IntegerMatrix.html)
with a
[`NaturalMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_matrix/struct.NaturalMatrix.html)
or an
[`UnsignedMatrix`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_matrix/struct.UnsignedMatrix.html),
in either order. [`is_zero`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.is_zero)
is true for every matrix with no elements, as `fmpz_mat_is_zero` is.

`fmpz_mat_is_one` accepts the same matrices that `fmpz_mat_one` produces (see
[Basic assignment and manipulation](#basic-assignment-and-manipulation)): any shape, with ones in
positions $$(i, i)$$ and zeros elsewhere, and so every matrix with no elements, such as a
$$0 \times 3$$ one. That is
[`is_rectangular_identity`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.is_rectangular_identity).
[`is_identity`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.is_identity)
also requires the matrix to be square, so it is the test for the identity of matrix
multiplication.

[`is_empty`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.is_empty)
is true when the height or the width is zero, as `fmpz_mat_is_empty` is.
[`is_zero_row`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.is_zero_row)
panics if the row index is out of range; `fmpz_mat_is_zero_row` does not check it. Likewise,
[`equal_rows`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.equal_rows)
and
[`equal_columns`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.equal_columns)
panic if an index is out of range, which `fmpz_mat_equal_row` and `fmpz_mat_equal_col` do not
check.

## [Transpose](https://flintlib.org/doc/fmpz_mat.html#transpose) {#transpose}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpz_mat_transpose(fmpz_mat_t B, const fmpz_mat_t A)` | [`transpose`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.transpose), [`transpose_assign`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.transpose_assign) |

[`transpose`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.transpose)
returns a new matrix of the transposed shape, so there is no output whose shape must be right.
`fmpz_mat_transpose` calls the generic `gr_mat_transpose`, which throws (through
`GR_MUST_SUCCEED`) unless `B` is $$c \times r$$ for an $$r \times c$$ matrix `A`. For the same
reason, FLINT can transpose in place, with `B` and `A` the same matrix, only when the matrix is
square.
[`transpose_assign`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.transpose_assign)
transposes a matrix of any shape in place: a square one by swapping elements across the
diagonal, as FLINT does, and a rectangular one by rearranging its elements and exchanging its
height and width.

## [Concatenate](https://flintlib.org/doc/fmpz_mat.html#concatenate) {#concatenate}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpz_mat_concat_vertical(fmpz_mat_t res, const fmpz_mat_t mat1, const fmpz_mat_t mat2)` | [`concat_vertical`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.concat_vertical), [`into_concat_vertical`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.into_concat_vertical) |
| ✓ | `void fmpz_mat_concat_horizontal(fmpz_mat_t res, const fmpz_mat_t mat1, const fmpz_mat_t mat2)` | [`concat_horizontal`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.concat_horizontal), [`into_concat_horizontal`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.into_concat_horizontal) |

Each FLINT function writes into a `res` of the right shape; the Malachite methods return a new
matrix instead. `a.concat_vertical(&b)` and `a.concat_horizontal(&b)` clone the elements, and
`into_concat_vertical` and `into_concat_horizontal` take both matrices by value and move them.
Because the elements are stored in row-major order,
[`into_concat_vertical`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.into_concat_vertical)
only appends the elements of `b` to those of `a`.

`fmpz_mat/concat.c` checks none of the shapes the manual gives: if the shapes do not fit, it
writes outside `res` or leaves part of it unset. The Malachite methods panic unless the widths
(for vertical concatenation) or the heights (for horizontal concatenation) are equal. Matrices
with no elements keep their shapes, so a $$0 \times 2$$ matrix cannot be stacked on a
$$0 \times 3$$ one.

## [Modular reduction and reconstruction](https://flintlib.org/doc/fmpz_mat.html#modular-reduction-and-reconstruction) {#modular-reduction-and-reconstruction}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpz_mat_get_nmod_mat(nmod_mat_t Amod, const fmpz_mat_t A)` | [`Mod`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.Mod.html) |
| ≈ | `void fmpz_mat_set_nmod_mat(fmpz_mat_t A, const nmod_mat_t Amod)` | [`BalancedMod`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.BalancedMod.html) |
| ✓ | `void fmpz_mat_set_nmod_mat_unsigned(fmpz_mat_t A, const nmod_mat_t Amod)` | [`From`](https://doc.rust-lang.org/nightly/core/convert/trait.From.html) |
| ✓ | `void fmpz_mat_CRT_ui(fmpz_mat_t res, const fmpz_mat_t mat1, const fmpz_t m1, const nmod_mat_t mat2, int sign)` | [`Crt`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.Crt.html), [`BalancedCrt`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.BalancedCrt.html) |
| ⚙ | `void fmpz_mat_multi_mod_ui_precomp(nmod_mat_t * residues, slong nres, const fmpz_mat_t mat, const fmpz_comb_t comb)` | internal |
| ✓ | `void fmpz_mat_multi_mod_ui(nmod_mat_t * residues, slong nres, const fmpz_mat_t mat)` | [`IntegerMatrix::multi_mod`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_matrix/struct.IntegerMatrix.html#method.multi_mod), [`NaturalMatrix::multi_mod`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_matrix/struct.NaturalMatrix.html#method.multi_mod) |
| ⚙ | `void fmpz_mat_multi_CRT_ui_precomp(fmpz_mat_t mat, nmod_mat_t * const residues, slong nres, const fmpz_comb_t comb, int sign)` | internal |
| ✓ | `void fmpz_mat_multi_CRT_ui(fmpz_mat_t mat, nmod_mat_t * const residues, slong nres, int sign)` | [`NaturalMatrix::multi_crt`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_matrix/struct.NaturalMatrix.html#method.multi_crt), [`IntegerMatrix::multi_balanced_crt`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_matrix/struct.IntegerMatrix.html#method.multi_balanced_crt) |

Malachite has no `nmod_mat` type: a matrix of residues modulo a word-sized $$n$$ is an
[`UnsignedMatrix<u64>`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_matrix/struct.UnsignedMatrix.html)
whose elements are less than $$n$$, and the modulus is passed to each operation. These rows
follow the vector page's [Conversions](/mapping/flint-integer-vectors/#conversions) and [Reduction
mod p](/mapping/flint-integer-vectors/#reduction-mod-p).

`get_nmod_mat` is
[`Mod`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.Mod.html)
with a primitive unsigned modulus, `a.mod_op(n)`, which returns an `UnsignedMatrix` of the
modulus's type. (For a square matrix, FLINT copies the residue of $$A_{ij}$$ to position
$$(j, i)$$ as long as the matrix is still symmetric, rather than reducing $$A_{ji}$$ again.)
`set_nmod_mat_unsigned` is the conversion
[`From`](https://doc.rust-lang.org/nightly/core/convert/trait.From.html)`<UnsignedMatrix<T>>`.
`set_nmod_mat` converts each residue to its balanced representative: convert the
`UnsignedMatrix` to a
[`NaturalMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_matrix/struct.NaturalMatrix.html)
with [`From`](https://doc.rust-lang.org/nightly/core/convert/trait.From.html) and then apply
[`BalancedMod`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.BalancedMod.html),
which returns an `IntegerMatrix`, hence ≈. The manual gives the balanced range as
$$[-m/2, m/2)$$, but the implementation (`fmpz_set_ui_smod`) keeps $$x$$ when $$x \le m/2$$, so the
range is $$(-m/2, m/2]$$, the same as
[`BalancedMod`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.BalancedMod.html)'s;
this is the same discrepancy as for `_fmpz_vec_set_nmod_vec`. (The manual's descriptions of both
`set` functions also say "Sets the entries of `Amod`" where `A` is meant.)

`CRT_ui` combines a matrix of residues modulo an arbitrary $$m_1$$ with a matrix of residues
modulo a word-sized $$m_2$$, coprime to $$m_1$$, into residues modulo $$m_1 m_2$$. The
counterparts are
[`Crt`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.Crt.html)
on a
[`NaturalMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_matrix/struct.NaturalMatrix.html),
`a.crt(&m1, &b, m2)` (`sign = 0`), and
[`BalancedCrt`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.BalancedCrt.html)
on an [`IntegerMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_matrix/struct.IntegerMatrix.html),
`a.balanced_crt(&m1, &b, m2)` (`sign = 1`), where `b` is an `UnsignedMatrix<Limb>`. Like FLINT,
they invert $$m_1$$ modulo $$m_2$$ once rather than once per element. FLINT throws when the moduli
are not coprime; Malachite returns
[`None`](https://doc.rust-lang.org/nightly/std/option/enum.Option.html), as the scalar `Crt` and
`BalancedCrt` do. As there, the elements of `a` must be reduced modulo $$m_1$$, except that
`balanced_crt` accepts any representative in $$[-m_1, m_1)$$, as FLINT's `sign = 1` does, and the
elements of `b` must be reduced modulo $$m_2$$; FLINT reduces neither. Mismatched shapes or a zero
modulus panic. With `sign = 1`, the manual says the result is in $$[-m_1m_2/2, m_1m_2/2)$$, but on a
tie the code keeps the positive representative, so the range is $$(-m_1m_2/2, m_1m_2/2]$$, which is
`BalancedCrt`'s.

`multi_mod_ui` reduces one matrix modulo several word-sized moduli. Its counterpart is `multi_mod`
on an
[`IntegerMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_matrix/struct.IntegerMatrix.html#method.multi_mod)
or a
[`NaturalMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_matrix/struct.NaturalMatrix.html#method.multi_mod),
which returns one `UnsignedMatrix<Limb>` per modulus; the vector types have it too
([`IntegerVector::multi_mod`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/struct.IntegerVector.html#method.multi_mod),
[`NaturalVector::multi_mod`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_vector/struct.NaturalVector.html#method.multi_mod)).
`multi_CRT_ui` reconstructs a matrix from such residues:
[`NaturalMatrix::multi_crt`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_matrix/struct.NaturalMatrix.html#method.multi_crt)
with `sign = 0`, and
[`IntegerMatrix::multi_balanced_crt`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_matrix/struct.IntegerMatrix.html#method.multi_balanced_crt)
with `sign = 1`. These follow the vector functions (see the vector page's [Reduction
mod p](/mapping/flint-integer-vectors/#reduction-mod-p)) in their conventions: unusable moduli give
`None`, the residues must be reduced, and an empty list of moduli panics. Like FLINT, both
precompute once for the moduli, and with more than 200 moduli `multi_mod` reduces each element
through a comb, as FLINT does; FLINT also splits the work across threads, which Malachite does not.
The manual does not restrict `multi_mod_ui`'s moduli, but with more than 200 of them FLINT builds
an `fmpz_comb_t`, whose moduli the manual assumes to be primes. `multi_mod` accepts any positive
moduli, and uses its comb only when they are pairwise coprime and at least 2. The manual's
description of `multi_mod_ui` points to `fmpz_mat_multi_mod_precomp`, which does not exist; it
means `fmpz_mat_multi_mod_ui_precomp`.

The `_precomp` forms take an `fmpz_comb_t` precomputed for the moduli, so that it can be reused
across calls. Malachite's comb is internal, hence ⚙: call `multi_mod`, `multi_crt`, or
`multi_balanced_crt`, which precompute once per call.

## [Addition and subtraction](https://flintlib.org/doc/fmpz_mat.html#addition-and-subtraction) {#addition-and-subtraction}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpz_mat_add(fmpz_mat_t C, const fmpz_mat_t A, const fmpz_mat_t B)` | [`Add`](https://doc.rust-lang.org/nightly/core/ops/trait.Add.html), [`AddAssign`](https://doc.rust-lang.org/nightly/core/ops/trait.AddAssign.html) |
| ✓ | `void fmpz_mat_sub(fmpz_mat_t C, const fmpz_mat_t A, const fmpz_mat_t B)` | [`Sub`](https://doc.rust-lang.org/nightly/core/ops/trait.Sub.html), [`SubAssign`](https://doc.rust-lang.org/nightly/core/ops/trait.SubAssign.html) |
| ✓ | `void fmpz_mat_neg(fmpz_mat_t B, const fmpz_mat_t A)` | [`Neg`](https://doc.rust-lang.org/nightly/core/ops/trait.Neg.html), [`NegAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.NegAssign.html) |

Each operator takes its operands by value or by reference, and the `Assign` forms are the aliased
calls `fmpz_mat_add(A, A, B)` and `fmpz_mat_neg(A, A)`. The manual requires all inputs to be of
the same size; FLINT does not check this (it takes the shape from the output), while
[`Add`](https://doc.rust-lang.org/nightly/core/ops/trait.Add.html) and
[`Sub`](https://doc.rust-lang.org/nightly/core/ops/trait.Sub.html) panic if the shapes differ.
The modular counterparts, `fmpz_mod_mat` and `nmod_mat` addition, are
[`ModAdd`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModAdd.html),
[`ModSub`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModSub.html),
and [`ModNeg`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModNeg.html)
(and their `ModPowerOf2` forms) on
[`NaturalMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_matrix/struct.NaturalMatrix.html)
and
[`UnsignedMatrix`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_matrix/struct.UnsignedMatrix.html).

## [Matrix-scalar arithmetic](https://flintlib.org/doc/fmpz_mat.html#matrix-scalar-arithmetic) {#matrix-scalar-arithmetic}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpz_mat_scalar_mul_si(fmpz_mat_t B, const fmpz_mat_t A, slong c)` | [`Mul`](https://doc.rust-lang.org/nightly/core/ops/trait.Mul.html), [`MulAssign`](https://doc.rust-lang.org/nightly/core/ops/trait.MulAssign.html) |
| ✓ | `void fmpz_mat_scalar_mul_ui(fmpz_mat_t B, const fmpz_mat_t A, ulong c)` | [`Mul`](https://doc.rust-lang.org/nightly/core/ops/trait.Mul.html), [`MulAssign`](https://doc.rust-lang.org/nightly/core/ops/trait.MulAssign.html) |
| ✓ | `void fmpz_mat_scalar_mul_fmpz(fmpz_mat_t B, const fmpz_mat_t A, const fmpz_t c)` | [`Mul`](https://doc.rust-lang.org/nightly/core/ops/trait.Mul.html), [`MulAssign`](https://doc.rust-lang.org/nightly/core/ops/trait.MulAssign.html) |
| ✓ | `void fmpz_mat_scalar_addmul_si(fmpz_mat_t B, const fmpz_mat_t A, slong c)` | [`AddMul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.AddMul.html), [`AddMulAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.AddMulAssign.html) |
| ✓ | `void fmpz_mat_scalar_addmul_ui(fmpz_mat_t B, const fmpz_mat_t A, ulong c)` | [`AddMul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.AddMul.html), [`AddMulAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.AddMulAssign.html) |
| ✓ | `void fmpz_mat_scalar_addmul_fmpz(fmpz_mat_t B, const fmpz_mat_t A, const fmpz_t c)` | [`AddMul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.AddMul.html), [`AddMulAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.AddMulAssign.html) |
| ✓ | `void fmpz_mat_scalar_submul_si(fmpz_mat_t B, const fmpz_mat_t A, slong c)` | [`SubMul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.SubMul.html), [`SubMulAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.SubMulAssign.html) |
| ✓ | `void fmpz_mat_scalar_submul_ui(fmpz_mat_t B, const fmpz_mat_t A, ulong c)` | [`SubMul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.SubMul.html), [`SubMulAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.SubMulAssign.html) |
| ✓ | `void fmpz_mat_scalar_submul_fmpz(fmpz_mat_t B, const fmpz_mat_t A, const fmpz_t c)` | [`SubMul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.SubMul.html), [`SubMulAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.SubMulAssign.html) |
| ✓ | `void fmpz_mat_scalar_addmul_nmod_mat_ui(fmpz_mat_t B, const nmod_mat_t A, ulong c)` | [`From`](https://doc.rust-lang.org/nightly/core/convert/trait.From.html), [`AddMulAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.AddMulAssign.html) |
| ⚙ | `void fmpz_mat_scalar_addmul_nmod_mat_fmpz(fmpz_mat_t B, const nmod_mat_t A, const fmpz_t c)` | internal |
| ✓ | `void fmpz_mat_scalar_divexact_si(fmpz_mat_t B, const fmpz_mat_t A, slong c)` | [`DivExact`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivExact.html), [`DivExactAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivExactAssign.html) |
| ✓ | `void fmpz_mat_scalar_divexact_ui(fmpz_mat_t B, const fmpz_mat_t A, ulong c)` | [`DivExact`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivExact.html), [`DivExactAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivExactAssign.html) |
| ✓ | `void fmpz_mat_scalar_divexact_fmpz(fmpz_mat_t B, const fmpz_mat_t A, const fmpz_t c)` | [`DivExact`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivExact.html), [`DivExactAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivExactAssign.html) |
| ✓ | `void fmpz_mat_scalar_mul_2exp(fmpz_mat_t B, const fmpz_mat_t A, ulong exp)` | [`Shl`](https://doc.rust-lang.org/nightly/core/ops/trait.Shl.html), [`ShlAssign`](https://doc.rust-lang.org/nightly/core/ops/trait.ShlAssign.html) |
| ✓ | `void fmpz_mat_scalar_tdiv_q_2exp(fmpz_mat_t B, const fmpz_mat_t A, ulong exp)` | [`EntrywiseShrRound`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.EntrywiseShrRound.html), [`EntrywiseShrRoundAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.EntrywiseShrRoundAssign.html) |
| ✓ | `void fmpz_mat_scalar_smod(fmpz_mat_t B, const fmpz_mat_t A, const fmpz_t P)` | [`BalancedMod`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.BalancedMod.html), [`BalancedModAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.BalancedModAssign.html) |

`scalar_mul` is [`Mul`](https://doc.rust-lang.org/nightly/core/ops/trait.Mul.html) of an
[`IntegerMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_matrix/struct.IntegerMatrix.html)
and an [`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html),
with the scalar on either side, or
[`MulAssign`](https://doc.rust-lang.org/nightly/core/ops/trait.MulAssign.html) when `B` is `A`;
the `_si` and `_ui` forms take an `Integer` made from the primitive scalar.
[`NaturalMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_matrix/struct.NaturalMatrix.html)
and [`RationalMatrix`](https://docs.rs/malachite-q/latest/malachite_q/rational_matrix/struct.RationalMatrix.html)
multiply by a [`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html)
and a [`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html) in
the same way. `scalar_mul_2exp` is
[`Shl`](https://doc.rust-lang.org/nightly/core/ops/trait.Shl.html) (`<<`) by any primitive unsigned
type, or [`ShlAssign`](https://doc.rust-lang.org/nightly/core/ops/trait.ShlAssign.html) (`<<=`) when
`B` is `A`.

`scalar_tdiv_q_2exp`, which rounds toward zero, is
[`EntrywiseShrRound`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.EntrywiseShrRound.html)
with `Down`, or
[`EntrywiseShrRoundAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.EntrywiseShrRoundAssign.html)
when `B` is `A`. Its other rounding modes give the remaining quotients; unlike
[`ShrRound`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ShrRound.html),
it returns no [`Ordering`](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html), since
the elements may round in different directions. FLINT has no matrix `fdiv_q_2exp`; in Malachite it
is [`Shr`](https://doc.rust-lang.org/nightly/core/ops/trait.Shr.html) (`>>`), which takes the floor
of every element, or [`ShrAssign`](https://doc.rust-lang.org/nightly/core/ops/trait.ShrAssign.html)
(`>>=`). These shifts take any primitive unsigned or signed shift amount, a negative amount
shifting the other way.
[`NaturalMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_matrix/struct.NaturalMatrix.html)
has the same shifts, and for it `>>` already rounds toward zero;
[`UnsignedMatrix`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_matrix/struct.UnsignedMatrix.html)
has `>>` and `EntrywiseShrRound` by primitive unsigned amounts.

`scalar_divexact` is
[`DivExact`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivExact.html)
of an
[`IntegerMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_matrix/struct.IntegerMatrix.html)
by an [`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html),
or [`DivExactAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivExactAssign.html)
when `B` is `A`, with the matrix and the scalar by value or by reference; the `_si` and `_ui` forms
take an `Integer` made from the primitive scalar. As in FLINT, every element must be divisible by
the scalar, which is not checked.
[`NaturalMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_matrix/struct.NaturalMatrix.html)
and
[`UnsignedMatrix`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_matrix/struct.UnsignedMatrix.html)
divide exactly by a
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html) and by a
primitive unsigned scalar in the same way.

`scalar_smod` reduces every element into $$(-P/2, P/2]$$; it is
[`BalancedMod`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.BalancedMod.html) with an
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html) modulus,
or [`BalancedModAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.BalancedModAssign.html) when `B` is `A`. FLINT requires
$$P > 0$$; Malachite uses only the modulus's magnitude, so a negative modulus gives the same result
as its absolute value. `fmpz_mat.h` also declares an undocumented `fmpz_mat_scalar_mod_fmpz`,
which reduces every element into $$[0, m)$$; that is [`Mod`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.Mod.html) with a
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html) modulus,
which returns a
[`NaturalMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_matrix/struct.NaturalMatrix.html)
(see [Modular reduction and reconstruction](#modular-reduction-and-reconstruction)).

`scalar_addmul` is
[`AddMul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.AddMul.html),
`B.add_mul(A, c)`, which returns $$B + cA$$, or
[`AddMulAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.AddMulAssign.html),
`B.add_mul_assign(A, c)`, which updates `B` in place as FLINT does; `scalar_submul` is
[`SubMul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.SubMul.html)
or [`SubMulAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.SubMulAssign.html)
in the same way. The matrices and the scalar may each be taken by value or by reference, and the
`_si` and `_ui` forms take an `Integer` made from the primitive scalar. Unlike FLINT, Malachite
panics if the matrices have different shapes.
[`NaturalMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_matrix/struct.NaturalMatrix.html)
has `AddMul` with a
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html) scalar,
and [`RationalMatrix`](https://docs.rs/malachite-q/latest/malachite_q/rational_matrix/struct.RationalMatrix.html)
has both with a
[`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html) scalar.

`scalar_addmul_nmod_mat_ui`, which adds $$c$$ times a matrix of residues, converts the
[`UnsignedMatrix`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_matrix/struct.UnsignedMatrix.html)
of residues to an `IntegerMatrix` with
[`From`](https://doc.rust-lang.org/nightly/core/convert/trait.From.html), by value or by reference,
and then adds with `add_mul_assign`, the scalar being an `Integer` made from the primitive, as for
the `_ui` forms above: `b.add_mul_assign(IntegerMatrix::from(&a), Integer::from(c))`. The
`_fmpz` form, which takes the scalar as an `Integer` and skips the conversion's allocation, is an
internal function used by Malachite's Dixon solver.

## [Matrix multiplication](https://flintlib.org/doc/fmpz_mat.html#matrix-multiplication) {#matrix-multiplication}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpz_mat_mul(fmpz_mat_t C, const fmpz_mat_t A, const fmpz_mat_t B)` | [`Mul`](https://doc.rust-lang.org/nightly/core/ops/trait.Mul.html), [`MulAssign`](https://doc.rust-lang.org/nightly/core/ops/trait.MulAssign.html) |
| ⚙ | `void fmpz_mat_mul_classical(fmpz_mat_t C, const fmpz_mat_t A, const fmpz_mat_t B)` | internal |
| ⚙ | `void fmpz_mat_mul_waksman(fmpz_mat_t C, const fmpz_mat_t A, const fmpz_mat_t B)` | internal |
| ⚙ | `void fmpz_mat_mul_strassen(fmpz_mat_t C, const fmpz_mat_t A, const fmpz_mat_t B)` | internal |
| ⚙ | `void fmpz_mat_mul_multi_mod(fmpz_mat_t C, const fmpz_mat_t A, const fmpz_mat_t B)` | internal |
| — | `int fmpz_mat_mul_blas(fmpz_mat_t C, const fmpz_mat_t A, const fmpz_mat_t B)` |  |
| — | `void fmpz_mat_mul_fft(fmpz_mat_t C, const fmpz_mat_t A, const fmpz_mat_t B)` |  |
| ✓ | `void fmpz_mat_sqr(fmpz_mat_t B, const fmpz_mat_t A)` | [`Square`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Square.html), [`SquareAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.SquareAssign.html) |
| ⚙ | `void fmpz_mat_sqr_bodrato(fmpz_mat_t B, const fmpz_mat_t A)` | internal |
| ✓ | `void fmpz_mat_pow(fmpz_mat_t B, const fmpz_mat_t A, ulong e)` | [`Pow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Pow.html), [`PowAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.PowAssign.html) |
| ≈ | `void fmpz_mat_mul_fmpz_vec(fmpz * c, const fmpz_mat_t A, const fmpz * b, slong blen)` | [`Mul`](https://doc.rust-lang.org/nightly/core/ops/trait.Mul.html) |
| ≈ | `void fmpz_mat_mul_fmpz_vec_ptr(fmpz * const * c, const fmpz_mat_t A, const fmpz * const * b, slong blen)` | [`Mul`](https://doc.rust-lang.org/nightly/core/ops/trait.Mul.html) |
| ≈ | `void fmpz_mat_fmpz_vec_mul(fmpz * c, const fmpz * a, slong alen, const fmpz_mat_t B)` | [`Mul`](https://doc.rust-lang.org/nightly/core/ops/trait.Mul.html) |
| ≈ | `void fmpz_mat_fmpz_vec_mul_ptr(fmpz * const * c, const fmpz * const * a, slong alen, const fmpz_mat_t B)` | [`Mul`](https://doc.rust-lang.org/nightly/core/ops/trait.Mul.html) |

`fmpz_mat_mul` is [`Mul`](https://doc.rust-lang.org/nightly/core/ops/trait.Mul.html) of two
[`IntegerMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_matrix/struct.IntegerMatrix.html)es,
with each operand by value or by reference, and [`MulAssign`](https://doc.rust-lang.org/nightly/core/ops/trait.MulAssign.html) for the product in place, which
FLINT allows by aliasing `C` with `A`. They panic unless the width of the first matrix is the
height of the second; FLINT checks this only in debug builds. The same products exist for
[`NaturalMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_matrix/struct.NaturalMatrix.html).
The algorithm-specific variants `_classical`, `_waksman`, `_strassen`, and `_multi_mod` are
internal choices behind `Mul`, as they are for polynomial multiplication. `fmpz_mat_mul_fft`, which
views each element as a polynomial and multiplies matrices of Schönhage–Strassen transforms of
them pointwise, is not one of `fmpz_mat_mul`'s choices, and Malachite has it only as a reference
in its tests, hence —: it was slower than `fmpz_mat_mul` at every size measured, from 2000-bit to
million-bit elements and from dimension 1 to 32, in FLINT and in Malachite alike. As with
`fmpz_poly_mul_SS`, FLINT 3.6.0 misreads some large negative coefficients of the transformed
product as positive, so its `fmpz_mat_mul_fft` returns wrong results for some inputs, such as the
product of $$\begin{pmatrix} -(2^{31488}-1) & -(2^{31488}-1) \end{pmatrix}$$ and its negated
transpose; FLINT has fixed this since. `Mul`
chooses among the algorithms as `fmpz_mat_mul` does with one thread and without
BLAS: FLINT's kernels for elements that fit in one word (summing in one, two, or three words, with
blocking for larger matrices) and in two words (summing in four or five), the multimodular algorithm
for large matrices, Waksman's algorithm for small matrices with large elements of similar sizes,
Strassen's for large matrices with large elements (and with small ones, above a dimension of
1300), and the classical algorithm otherwise. The multimodular algorithm reduces both matrices
modulo a power of 2 and, when that does not suffice to determine the product, word-sized primes,
multiplies the residue matrices with the
[`ModMul`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModMul.html)
of two
[`UnsignedMatrix`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_matrix/struct.UnsignedMatrix.html)es,
and reconstructs the product by the Chinese remainder theorem; when the two matrices are the same,
it reduces them once. For elements that fit in two words, FLINT chooses it when the dimension
exceeds 300 by more than a limit computed as `(bits - 2 * FLINT_BITS) / 8`, squared, in unsigned
arithmetic, where `bits` bounds the size of the product's elements; when `bits` is less than
`2 * FLINT_BITS`, the subtraction wraps around. Malachite takes the limit to be 0 in that case. Waksman's and Strassen's algorithms have negative intermediate
values, so for a `NaturalMatrix` they work with `Integer`s. Strassen's algorithm copies the blocks
that it multiplies directly, where FLINT references them through windows, since Malachite's
matrices own their elements. The classical
algorithm, like FLINT's, handles an inner dimension of 1 or 2 directly and otherwise takes a dot
product of each row with each column, but it reads the columns in place, at a stride, where FLINT
builds a transposed array of pointers to the second matrix's elements. FLINT's unblocked
one-word kernels read the second matrix down its columns; Malachite's read a transposed copy,
which is a few percent faster for all but the smallest products. `MulAssign` writes the product
back into the first matrix's storage, a row at a time for the classical algorithm, so multiplying
by a square matrix needs only one row of additional memory; FLINT computes into a temporary
matrix and swaps it in. `fmpz_mat_mul_blas`
hands the product to an external BLAS library, which Malachite, written in pure Rust, does not
use. (The underscore functions `_fmpz_mat_mul_multi_mod`, `_fmpz_mat_mul_small`, and
`_fmpz_mat_mul_double_word` are omitted.)

`fmpz_mat_sqr` is
[`Square`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Square.html)
for an `IntegerMatrix`, by value or by reference, and
[`SquareAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.SquareAssign.html)
for squaring in place; `fmpz_mat_pow` is
[`Pow<u64>`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Pow.html)
and
[`PowAssign<u64>`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.PowAssign.html).
The same operations exist for `NaturalMatrix`. They panic unless the matrix is square, which FLINT
does not check, and $$A^0$$ is the identity matrix of $$A$$'s dimension. `Square` chooses as
`fmpz_mat_sqr` does between Bodrato's squaring algorithm, `fmpz_mat_sqr_bodrato`, which is
internal, and multiplying the matrix by itself; in the latter case the multimodular algorithm
reduces the matrix only once, as FLINT's does. For dimension 3, Bodrato's algorithm multiplies
elements that fit in two words with the double-word kernel, as FLINT's does. When the dimension is
odd, FLINT computes the last row of the square as the last row of the matrix times a window onto
the matrix's other columns; Malachite, which would have to copy those columns, multiplies by the
whole matrix instead and so computes the last element a second time. `Pow` stops early when a
power is the zero matrix, which happens when the matrix is nilpotent.

Two details of the FLINT implementations differ from the manual. `fmpz_mat_sqr` is documented as
calling `fmpz_mat_mul` for dimensions less than 12, but `fmpz_mat/sqr.c` calls
`fmpz_mat_sqr_bodrato` for dimensions up to 3, and for dimension 4 when the entries have at least
1024 bits, and `fmpz_mat_mul` otherwise. `fmpz_mat_pow` is plain left-to-right binary powering,
and it sets `B` to the identity when the exponent is 0 or the matrix is $$0 \times 0$$; nothing
checks that `A` is square.

The matrix-vector products are
[`Mul`](https://doc.rust-lang.org/nightly/core/ops/trait.Mul.html) of an
[`IntegerMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_matrix/struct.IntegerMatrix.html)
and an
[`IntegerVector`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/struct.IntegerVector.html),
in that order for $$Ab$$ and in the other for $$aB$$, with each operand by value or by
reference; the result is a new `IntegerVector`. They differ from FLINT's in one way, hence ≈:
FLINT truncates or zero-extends the vector to the length the product needs, while Malachite
panics unless the vector's dimension is the matrix's width (for $$Ab$$) or height (for $$aB$$).
The `_ptr` forms read and write through arrays of pointers to elements, which may be scattered;
in Malachite, collect the elements into an `IntegerVector` first. Each element of $$Ab$$ is a dot
product of a row with the vector, as in FLINT; Malachite computes $$aB$$ by adding multiples of
the rows of $$B$$ in storage order, where FLINT goes down each column. The same products exist for
[`NaturalMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_matrix/struct.NaturalMatrix.html)
and
[`RationalMatrix`](https://docs.rs/malachite-q/latest/malachite_q/rational_matrix/struct.RationalMatrix.html),
and modular ones, through
[`ModMul`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModMul.html)
and
[`ModPowerOf2Mul`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModPowerOf2Mul.html),
for
[`NaturalMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_matrix/struct.NaturalMatrix.html)
and
[`UnsignedMatrix`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_matrix/struct.UnsignedMatrix.html).

## [Inverse](https://flintlib.org/doc/fmpz_mat.html#inverse) {#inverse}

| | FLINT | Malachite |
| :---: | --- | --- |
| ≈ | `int fmpz_mat_inv(fmpz_mat_t Ainv, fmpz_t den, const fmpz_mat_t A)` | [`Inverse`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Inverse.html) |

`fmpz_mat_inv` writes the inverse of $$A$$ as an integer matrix `Ainv` and a common denominator
`den`, so that $$A^{-1} = \mathit{Ainv} / \mathit{den}$$, and returns 0 when $$A$$ is singular. The
denominator divides $$\det A$$ but need not be the smallest possible, and its sign is not fixed:
for dimensions 1 and 2 it is $$\det A$$ itself. In Malachite, this is
[`Inverse`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Inverse.html)
on an [`IntegerMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_matrix/struct.IntegerMatrix.html), by value or by reference, which returns `Some((Ainv, den))` with FLINT's results, or
`None` if $$A$$ is singular; it is ≈ because it panics if $$A$$ is not square.

The manual says that above dimension 2 the inverse is found by fraction-free LU decomposition.
In FLINT 3.6.0, `fmpz_mat/inv.c` instead solves $$AX = I$$ with `fmpz_mat_solve`, which uses
Cramer's rule up to dimension 3, fraction-free LU decomposition up to dimension 15, Dixon's
$$p$$-adic lifting up to dimension 32, and a multimodular method above that (see [Nonsingular
solving](#nonsingular-solving)). Nothing checks that $$A$$ is square.

## [Kronecker product](https://flintlib.org/doc/fmpz_mat.html#kronecker-product) {#kronecker-product}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpz_mat_kronecker_product(fmpz_mat_t C, const fmpz_mat_t A, const fmpz_mat_t B)` | [`KroneckerProduct`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.KroneckerProduct.html), [`KroneckerProductAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.KroneckerProductAssign.html) |

For an $$m \times n$$ matrix $$A$$ and a $$p \times q$$ matrix $$B$$, the Kronecker product
$$A \otimes B$$ is the $$mp \times nq$$ block matrix whose block $$(i, j)$$ is $$A_{ij} B$$. It is
[`KroneckerProduct`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.KroneckerProduct.html)
of two `IntegerMatrix` values, with each operand by value or by reference, and
[`KroneckerProductAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.KroneckerProductAssign.html)
replaces the first matrix with the product, which FLINT allows by aliasing `C` with `A` only when
the product has the same shape. The same traits are implemented for
[`NaturalMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_matrix/struct.NaturalMatrix.html).
FLINT writes the product one block at a time; Malachite writes it one row at a time, in the order
in which it is stored. FLINT does not check the shape of `C`. FLINT has no Kronecker product modulo a number;
Malachite's
[`ModKroneckerProduct`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModKroneckerProduct.html)
and
[`ModKroneckerProductAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModKroneckerProductAssign.html)
compute it modulo $$m$$, and
[`ModPowerOf2KroneckerProduct`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModPowerOf2KroneckerProduct.html)
and
[`ModPowerOf2KroneckerProductAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.ModPowerOf2KroneckerProductAssign.html)
modulo $$2^k$$, for
[`UnsignedMatrix`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_matrix/struct.UnsignedMatrix.html)es
and `NaturalMatrix`es whose elements are reduced.

## [Content](https://flintlib.org/doc/fmpz_mat.html#content) {#content}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpz_mat_content(fmpz_t mat_gcd, const fmpz_mat_t A)` | [`Content`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Content.html) |

The content is the GCD of all the elements; FLINT starts from $$0$$ and stops as soon as the GCD
reaches $$1$$, so a matrix with no elements, or with only zeros, has content $$0$$. (The function
returns nothing; the manual's "Returns 0 if the matrix is empty" means that it sets `mat_gcd` to
$$0$$.)
[`Content`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Content.html)
for an
[`IntegerMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_matrix/struct.IntegerMatrix.html),
`a.content()`, does the same, including the early stop, and returns a
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html), as
for vectors (see the vector page's [Gaussian
content](/mapping/flint-integer-vectors/#gaussian-content)).

FLINT has no matrix primitive part, but Malachite has the vector family for matrices too.
[`PrimitivePart`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.PrimitivePart.html)
divides every element by the content, keeping its sign, and
[`CanonicalPrimitivePart`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CanonicalPrimitivePart.html)
also makes the first nonzero element, in row-major order, positive.
[`ContentAndPrimitivePart`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ContentAndPrimitivePart.html)
and
[`ContentAndCanonicalPrimitivePart`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ContentAndCanonicalPrimitivePart.html)
return the content with them, and
[`ContentChained`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ContentChained.html)
gives the GCD of a number and the elements. All of these are also implemented for
[`NaturalMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_matrix/struct.NaturalMatrix.html),
[`UnsignedMatrix`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_matrix/struct.UnsignedMatrix.html),
and
[`RationalMatrix`](https://docs.rs/malachite-q/latest/malachite_q/rational_matrix/struct.RationalMatrix.html),
whose content is a rational and whose primitive part is an `IntegerMatrix`.

## [Trace](https://flintlib.org/doc/fmpz_mat.html#trace) {#trace}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpz_mat_trace(fmpz_t trace, const fmpz_mat_t mat)` | [`Trace`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.Trace.html) |

The trace is the sum of the diagonal elements of a square matrix, and $$0$$ for the $$0 \times 0$$
matrix. It is
[`Trace`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.Trace.html):
`a.to_trace()` takes the matrix by reference and `a.into_trace()` by value. It is implemented for
[`IntegerMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_matrix/struct.IntegerMatrix.html),
returning an `Integer`, and for
[`NaturalMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_matrix/struct.NaturalMatrix.html),
returning a `Natural`. The manual requires the matrix to be square, but nothing checks it:
`fmpz_mat/trace.c` sums positions $$(i, i)$$ for every row $$i$$, so for a matrix with more rows
than columns it reads past the last element. `Trace` panics if the matrix is not square.

## [Determinant](https://flintlib.org/doc/fmpz_mat.html#determinant) {#determinant}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpz_mat_det(fmpz_t det, const fmpz_mat_t A)` | [`Determinant`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.Determinant.html) |
| ⚙ | `void fmpz_mat_det_cofactor(fmpz_t det, const fmpz_mat_t A)` | internal |
| ⚙ | `void fmpz_mat_det_bareiss(fmpz_t det, const fmpz_mat_t A)` | internal |
| ⚙ | `void fmpz_mat_det_modular(fmpz_t det, const fmpz_mat_t A, int proved)` | internal |
| ⚙ | `void fmpz_mat_det_modular_accelerated(fmpz_t det, const fmpz_mat_t A, int proved)` | internal |
| ⚙ | `void fmpz_mat_det_modular_given_divisor(fmpz_t det, const fmpz_mat_t A, const fmpz_t d, int proved)` | internal |
| ✓ | `void fmpz_mat_det_bound(fmpz_t bound, const fmpz_mat_t A)` | [`determinant_bound`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_matrix/struct.IntegerMatrix.html#method.determinant_bound) |
| ✓ | `void fmpz_mat_det_bound_submatrix(fmpz_t bound, const fmpz_mat_t A)` | [`submatrix_determinant_bound`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_matrix/struct.IntegerMatrix.html#method.submatrix_determinant_bound) |
| ⚙ | `void fmpz_mat_det_divisor(fmpz_t d, const fmpz_mat_t A)` | internal |

In Malachite, `fmpz_mat_det` is [`Determinant`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.Determinant.html) on an [`IntegerMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_matrix/struct.IntegerMatrix.html), by value or by reference, which
returns an [`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html)
with FLINT's result and panics unless the matrix is square. `fmpz_mat_det` throws unless the matrix is square,
defines the determinant of the $$0 \times 0$$ matrix to be $$1$$, and chooses its algorithm by
dimension (`fmpz_mat/det.c`): cofactor expansion below 5 (`det_cofactor` throws above 4),
Bareiss's fraction-free elimination below 25, the multimodular algorithm below 60, and above that
the multimodular algorithm if the dimension is less than the entries' bit size, and the
accelerated multimodular algorithm otherwise. Malachite makes the same choices; the variants are
internal functions behind
[`Determinant`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.Determinant.html),
and give FLINT's results, including the probabilistic ones below.

The multimodular functions also take a `proved` flag. With `proved = 0` they return a
*probabilistic* value: they stop once the reconstructed determinant has stayed the same modulo
primes whose product exceeds $$2^{100}$$. The accelerated version first computes a divisor $$d$$ of
the determinant with `fmpz_mat_det_divisor` and then reconstructs $$\det A / d$$, which needs fewer
primes when $$d$$ is large; `det_modular_given_divisor` does the same with a caller-supplied $$d$$,
which is not checked. `det_divisor` takes $$d$$ to be the least common denominator of the solution
of $$Ax = b$$ for $$b = (-1, 1, -1, \ldots)$$, which it finds by lifting the solution modulo a power
of a prime with `fmpz_mat_solve_dixon` and reconstructing it; Malachite's internal version finds
the same denominator with its Dixon solver, which reconstructs the solution as it goes.

`det_bound` returns a $$B$$ with $$|\det A| \le B$$ from Hadamard's inequality, taking the smaller
of the products of the row norms and of the column norms; it computes with upper-bounding
floating-point magnitudes, so $$B$$ is rigorous but may exceed the exact Hadamard bound, and it is
$$1$$ for a matrix with no elements. `det_bound_submatrix` leaves out zero rows and columns, so that
the bound holds for every square submatrix. In Malachite they are the
[`IntegerMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_matrix/struct.IntegerMatrix.html) methods
[`determinant_bound`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_matrix/struct.IntegerMatrix.html#method.determinant_bound)
and
[`submatrix_determinant_bound`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_matrix/struct.IntegerMatrix.html#method.submatrix_determinant_bound),
which return a
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html) and
compute it with the same magnitudes and roundings, so the bounds are identical to FLINT's. The
rounding makes them looser than they look: the bound for the $$1 \times 1$$ matrix $$(1)$$, and for
the $$3 \times 3$$ identity matrix, is $$2$$, and `det_bound_submatrix` of a zero matrix is $$2$$ as well. The manual
says that `det_bound` assumes a square matrix; Malachite's
[`determinant_bound`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_matrix/struct.IntegerMatrix.html#method.determinant_bound)
panics on one that is not, since a zero column the submatrix leaves out would make the bound
$$0$$. `det_divisor` solves $$Ax = b$$ by Dixon's algorithm
for the fixed right-hand side $$b = (-1, 1, -1, 1, \ldots)$$ (the manual says "arbitrarily
chosen") and takes the least common multiple of the denominators of $$x$$, recovered by rational
reconstruction; it returns $$0$$ when $$A$$ is singular.

## [Permanent](https://flintlib.org/doc/fmpz_mat.html#permanent) {#permanent}

| | FLINT | Malachite |
| :---: | --- | --- |
| ≈ | `int fmpz_mat_permanent(fmpz_t res, const fmpz_mat_t A)` | [`Permanent`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.Permanent.html) |

The permanent is the determinant's formula without the signs: the sum over all permutations
$$\sigma$$ of $$\prod_i A_{i,\sigma(i)}$$, and $$1$$ for the $$0 \times 0$$ matrix. FLINT computes it
directly up to dimension 2, by cofactor expansion for dimension 3 (and up to 4 when the entries are
large), and otherwise by Glynn's formula, a sum over $$2^{n-1}$$ sign vectors, which it runs on
single words when the entries are small enough. The "too large" of the manual is a fixed limit:
above dimension $$62$$ (`FLINT_BITS - 2` on a 64-bit machine) the function returns 0, since the
enumeration of sign vectors is indexed by a word. As the manual says, nothing checks that the matrix
is square.

Malachite's
[`Permanent`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.Permanent.html)
for an
[`IntegerMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_matrix/struct.IntegerMatrix.html),
`a.permanent()`, uses the same algorithms, but always returns the permanent, hence ≈. FLINT's limit
rejects easy inputs, such as the identity matrix, while a dense matrix just above it would take
centuries anyway. So before the dense algorithms, Malachite uses the matrix's structure: a zero row
or column gives 0; a row or column with one nonzero element $$x$$ contributes $$x$$ times the
permanent of the minor; a row or column with two nonzero elements $$x$$ and $$y$$ is merged away,
since by linearity the permanent equals that of the matrix without that row and with the two columns
replaced by $$y$$ times the first plus $$x$$ times the second; and independent blocks of nonzero
elements contribute the product of their permanents. What remains above dimension 64 is expanded
along its sparsest row or column. So permutation, triangular, block-diagonal, and banded matrices of
any size are cheap. Malachite panics if the matrix is not square, and has no threaded version.
[`NaturalMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_matrix/struct.NaturalMatrix.html)
has `Permanent` too, returning a
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html).

## [Transforms](https://flintlib.org/doc/fmpz_mat.html#transforms) {#transforms}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpz_mat_similarity(fmpz_mat_t A, slong r, fmpz_t d)` | [`SimilarityTransformAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.SimilarityTransformAssign.html), [`SimilarityTransform`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.SimilarityTransform.html) |

`fmpz_mat_similarity` replaces a square matrix $$M$$ (the argument `A`) with $$P^{-1}MP$$, which
has the same determinant and characteristic and minimal polynomials. In Malachite it is
[`SimilarityTransformAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.SimilarityTransformAssign.html)
on an
[`IntegerMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_matrix/struct.IntegerMatrix.html),
`a.similarity_transform_assign(r, d)`, with `d` an
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html) by value
or by reference;
[`SimilarityTransform`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.SimilarityTransform.html)
returns the transformed matrix instead, taking the matrix by value or by reference.

According to the manual, $$P$$ is the identity matrix with every zero in row $$r$$ replaced by
$$d$$. The code (`fmpz_mat/similarity.c`) uses a slightly different $$P$$: its loops run over
$$j < r - 1$$ and $$j > r$$, so for $$r \ge 1$$ the entry of $$P$$ in row $$r$$ and column $$r - 1$$
stays $$0$$; that is, it adds $$d$$ times column $$r$$ to every column except $$r - 1$$ and $$r$$,
and then subtracts $$d$$ times the same rows from row $$r$$. The result is still a similarity
transform, so the invariants above are preserved, but it is not the one the manual describes.
Malachite computes exactly what the code does, and its documentation spells this out; in
particular, a $$2 \times 2$$ matrix with $$r = 1$$ is left unchanged. The `nmod_mat`,
`fmpz_mod_mat`, `fmpq_mat`, and `fq_mat` versions have the same loops. As the manual says, FLINT
checks neither the squareness of the matrix nor the range of $$r$$; Malachite panics if the
matrix is not square or if $$r$$ is not a row index, so always for the $$0 \times 0$$ matrix,
which FLINT leaves unchanged.

## [Characteristic polynomial](https://flintlib.org/doc/fmpz_mat.html#characteristic-polynomial) {#characteristic-polynomial}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpz_mat_charpoly_bound(fmpz_t bound, const fmpz_mat_t A)` | [`IntegerMatrix::characteristic_polynomial_bound`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_matrix/struct.IntegerMatrix.html#method.characteristic_polynomial_bound) |
| ⚙ | `void fmpz_mat_charpoly_berkowitz(fmpz_poly_t cp, const fmpz_mat_t mat)` | internal |
| ⚙ | `void fmpz_mat_charpoly_modular(fmpz_poly_t cp, const fmpz_mat_t mat)` | internal |
| ✓ | `void fmpz_mat_charpoly(fmpz_poly_t cp, const fmpz_mat_t mat)` | [`CharacteristicPolynomial`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.CharacteristicPolynomial.html) |

The characteristic polynomial of an $$n \times n$$ matrix $$A$$ is $$\det(xI - A)$$, a monic
polynomial of degree $$n$$; that of the $$0 \times 0$$ matrix is $$1$$. In Malachite it is
[`CharacteristicPolynomial`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.CharacteristicPolynomial.html)
on an
[`IntegerMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_matrix/struct.IntegerMatrix.html),
by value or by reference, returning an
[`IntegerPolynomial`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_polynomial/struct.IntegerPolynomial.html);
the Berkowitz and multimodular algorithms are internal choices behind it.
[`NaturalMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_matrix/struct.NaturalMatrix.html)
has it too, also returning an `IntegerPolynomial`, since the coefficients may be negative. Like
`fmpz_mat_charpoly`, it panics on a non-square matrix. FLINT's choice (`fmpz_mat/charpoly.c`) is
more specific than the manual's "depending on the size of the input": closed formulas up to
dimension 3, a shortcut for the zero matrix, then the Berkowitz algorithm up to dimension 8, or
whenever the bit length of the largest entry's bit size exceeds the dimension (that is, for
entries with at least $$2^n$$ bits), and the multimodular algorithm otherwise. Malachite makes the
same choices except for the last: measured on random matrices, Berkowitz's algorithm stays faster
up to a dimension that depends on the size of the entries, from 8 for entries of up to 16 bits, to
22 for entries of 33 to 128 bits, and back down to 10 above 512 bits, so Malachite uses those
cutoffs. Both algorithms give the same polynomial. The multimodular algorithm works modulo
word-sized primes with the characteristic polynomial of the reduced matrix computed as for
`nmod_mat_charpoly`, by Danilevsky's method above dimension 8 and Berkowitz's algorithm below.

`charpoly_bound` bounds the absolute values of the coefficients, which the multimodular algorithm
needs in order to know how many primes suffice. In Malachite it is
[`IntegerMatrix::characteristic_polynomial_bound`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_matrix/struct.IntegerMatrix.html#method.characteristic_polynomial_bound).
As with `det_bound` (see [Determinant](#determinant)), it computes with upper-bounding
floating-point magnitudes, rounding exactly as FLINT does, so the bound is the same as FLINT's: it
is rigorous but may exceed the one the manual's formula gives exactly, and it is $$1$$ for the
$$0 \times 0$$ matrix. Unlike FLINT, Malachite panics if the matrix is not square.

## [Minimal polynomial](https://flintlib.org/doc/fmpz_mat.html#minimal-polynomial) {#minimal-polynomial}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void fmpz_mat_minpoly_modular(fmpz_poly_t cp, const fmpz_mat_t mat)` | |
| ✗ | `void fmpz_mat_minpoly(fmpz_poly_t cp, const fmpz_mat_t mat)` | |

The minimal polynomial is the monic polynomial $$f$$ of least degree with $$f(A) = 0$$; it divides
the characteristic polynomial, and for the $$0 \times 0$$ matrix it is $$1$$. Malachite does not
compute it yet. `fmpz_mat_minpoly` throws on a non-square matrix and otherwise calls
`fmpz_mat_minpoly_modular`, which does not check. The modular method computes the minimal
polynomial modulo successive word-sized primes, discarding primes at which its degree drops,
and combines the residues by Chinese remaindering until either a coefficient bound is reached or
the result stops changing and passes an exact check: $$f(A)v = 0$$ for the generating vectors that
the modular computation found. So the result is proved, not probabilistic. (The manual's
description of the underscore form says "modular polynomial" where it means minimal
polynomial.)

Two defects in `fmpz_mat/minpoly_modular.c` matter only to readers of the code: the
coefficient-bound helper reads the bound `b` after calling `fmpz_clear(b)`, which frees it when it
does not fit in a word, and a work vector allocated with $$n + 1$$ entries is freed as if it had
$$n$$.

## [Rank](https://flintlib.org/doc/fmpz_mat.html#rank) {#rank}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `slong fmpz_mat_rank(const fmpz_mat_t A)` | [`Rank`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.Rank.html) |

The rank is the number of linearly independent rows, or equivalently columns; it is $$0$$ for
any matrix with no elements. In Malachite, it is [`Rank`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.Rank.html) on an [`IntegerMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_matrix/struct.IntegerMatrix.html), by value or by
reference, which returns a `u64` and makes the same choices as FLINT. `fmpz_mat/rank.c` row-reduces a
copy in one of three ways: when every entry fits in $$(\mathrm{FLINT\_BITS} - 4)/2$$ bits (30 on a
64-bit machine), by `fmpz_mat_rank_small_inplace`, which, declared in `fmpz_mat.h` but
undocumented, eliminates with word-sized integer quotients, in the manner of Euclid's
algorithm, and falls back to the general methods if the entries grow too large; otherwise by
fraction-free LU decomposition when the smaller dimension is below 25, and by reduced row echelon
form above that.

## [Column partitioning](https://flintlib.org/doc/fmpz_mat.html#column-partitioning) {#column-partitioning}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `int fmpz_mat_col_partition(slong * part, fmpz_mat_t M, int short_circuit)` | |

`fmpz_mat_col_partition` groups the columns of `M` into classes of equal columns, labeling each
column with its class in `part`, and returns the number of classes; with `short_circuit` set it
gives up, returning 0, as soon as there are more classes than rows. FLINT's only caller is the
van Hoeij step of integer polynomial factorization. Malachite does not have it yet; meanwhile,
collect each column from
[`columns`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_matrix/struct.IntegerMatrix.html#method.columns)
into a `Vec<&Integer>` and label them with a
[`HashMap`](https://doc.rust-lang.org/nightly/std/collections/struct.HashMap.html) from column
to label.

The manual says the labels lie in $$[0, p)$$, where $$p$$ is the number of classes, but
`fmpz_mat/col_partition.c` numbers the classes from 1, so they lie in $$[1, p]$$; the van Hoeij code
depends on this. When it returns 0 early, the function leaves `part` unset or only partly set. It
finds the classes by sorting the columns by a hash and comparing columns whose hashes agree, and
with `short_circuit` set it reads the hash of column 0 without checking that there is one.

## [Nonsingular solving](https://flintlib.org/doc/fmpz_mat.html#nonsingular-solving) {#nonsingular-solving}

| | FLINT | Malachite |
| :---: | --- | --- |
| ≈ | `int fmpz_mat_solve(fmpz_mat_t X, fmpz_t den, const fmpz_mat_t A, const fmpz_mat_t B)` | [`Solve`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.Solve.html) |
| ⚙ | `int fmpz_mat_solve_fflu(fmpz_mat_t X, fmpz_t den, const fmpz_mat_t A, const fmpz_mat_t B)` | internal |
| ⚙ | `int fmpz_mat_solve_fflu_precomp(fmpz_mat_t X, const slong * perm, const fmpz_mat_t FFLU, const fmpz_mat_t B)` | internal |
| ⚙ | `int fmpz_mat_solve_cramer(fmpz_mat_t X, fmpz_t den, const fmpz_mat_t A, const fmpz_mat_t B)` | internal |
| ✓ | `void fmpz_mat_solve_bound(fmpz_t N, fmpz_t D, const fmpz_mat_t A, const fmpz_mat_t B)` | [`solve_bounds`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_matrix/struct.IntegerMatrix.html#method.solve_bounds) |
| ✗ | `int fmpz_mat_solve_dixon(fmpz_mat_t X, fmpz_t M, const fmpz_mat_t A, const fmpz_mat_t B)` | |
| ⚙ | `int fmpz_mat_solve_dixon_den(fmpz_mat_t X, fmpz_t den, const fmpz_mat_t A, const fmpz_mat_t B)` | internal |
| ⚙ | `int fmpz_mat_solve_multi_mod_den(fmpz_mat_t X, fmpz_t den, const fmpz_mat_t A, const fmpz_mat_t B)` | internal |
| ⚙ | `int fmpz_mat_can_solve_multi_mod_den(fmpz_mat_t X, fmpz_t den, const fmpz_mat_t A, const fmpz_mat_t B)` | internal |
| ⚙ | `int fmpz_mat_can_solve_fflu(fmpz_mat_t X, fmpz_t den, const fmpz_mat_t A, const fmpz_mat_t B)` | internal |
| ≈ | `int fmpz_mat_can_solve(fmpz_mat_t X, fmpz_t den, const fmpz_mat_t A, const fmpz_mat_t B)` | [`Solve`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.Solve.html) |

These functions solve $$AX = B$$ over the rationals, returning an integer matrix $$X$$ and a
denominator `den` with $$AX = \mathit{den} \cdot B$$; the denominator divides $$\det A$$ but need
not be the smallest one. In Malachite, both `fmpz_mat_solve` and `fmpz_mat_can_solve` are
[`Solve`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.Solve.html)
on an [`IntegerMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_matrix/struct.IntegerMatrix.html), by value or by reference, with an [`IntegerMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_matrix/struct.IntegerMatrix.html) or an
[`IntegerVector`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/struct.IntegerVector.html)
right-hand side, also by value or by reference. It returns `Some((X, den))` if the system has a
solution and `None` otherwise, for any shape: when $$A$$ is square and nonsingular, the result is
`fmpz_mat_solve`'s, and otherwise it is `fmpz_mat_can_solve`'s. It is ≈ to both because it panics
if $$A$$ and $$B$$ have different heights, and because a square singular system that has solutions
gets one, where `fmpz_mat_solve` returns 0; for a square nonsingular system, the result can differ
from `fmpz_mat_can_solve`'s by a common factor of $$X$$ and `den`. The algorithm-specific forms
(`_fflu`, `_cramer`, `_dixon_den`, `_multi_mod_den`, `can_solve_fflu`, and `can_solve_multi_mod_den`)
are internal choices behind `Solve` and give FLINT's results; so does `solve_fflu_precomp`.
(`_fmpz_mat_solve_dixon_den` is omitted.)

The `solve` functions require a square nonsingular $$A$$ and return 0 if it is singular. The
`can_solve` functions accept an $$A$$ of any shape with as many rows as $$B$$, and return whether a
solution exists, computing one if so; `can_solve` uses the fraction-free version up to 15 rows
and the multimodular one above that. Three functions have a different output:
`solve_fflu_precomp` reuses a fraction-free LU decomposition from `fmpz_mat_fflu` (see [Row
reduction](#row-reduction)); its back substitution checks each division, but the divisions there
are always exact, since the pivot block of the first `rank` rows has determinant $$\pm$$`den`; `solve_bound` returns bounds
on the numerators and denominators of the reduced solution (in Malachite, the method
[`solve_bounds`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_matrix/struct.IntegerMatrix.html#method.solve_bounds),
which returns them as a pair of
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html)s and
panics unless $$A$$ is square with as many rows as $$B$$), and `solve_dixon` returns the
solution modulo an integer $$M$$ large enough for rational reconstruction to recover it. Malachite
has no counterpart of `fmpz_mat_solve_dixon`; its Dixon solver reconstructs the solution itself,
checking candidate reconstructions every few lifting steps, as `fmpz_mat_solve_dixon_den` does.

Two statements in the manual do not match FLINT 3.6.0. The description of `fmpz_mat_solve` names
only Cramer's rule and fraction-free LU decomposition, but `fmpz_mat/solve.c` uses Cramer's rule
up to dimension 3, fraction-free LU decomposition up to 15, Dixon's $$p$$-adic lifting
(`solve_dixon_den`) when $$B$$ has at most 32 columns, and the multimodular method
(`solve_multi_mod_den`) otherwise. And the chapter's introduction says that `den` is set to zero
when $$A$$ is singular; `solve_dixon_den` and `solve_multi_mod_den` instead leave `den`
unchanged, so `fmpz_mat_solve` (and [`fmpz_mat_inv`](#inverse)) do too above dimension 15. Test the
return value rather than `den`.

## [Row reduction](https://flintlib.org/doc/fmpz_mat.html#row-reduction) {#row-reduction}

| | FLINT | Malachite |
| :---: | --- | --- |
| ⚙ | `slong fmpz_mat_find_pivot_any(const fmpz_mat_t mat, slong start_row, slong end_row, slong c)` | internal |
| ≈ | `slong fmpz_mat_fflu(fmpz_mat_t B, fmpz_t den, slong * perm, const fmpz_mat_t A, int rank_check)` | [`FractionFreeLuDecomposition`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.FractionFreeLuDecomposition.html) |
| ≈ | `slong fmpz_mat_rref(fmpz_mat_t B, fmpz_t den, const fmpz_mat_t A)` | [`ReducedRowEchelonForm`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ReducedRowEchelonForm.html), [`ReducedRowEchelonFormAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ReducedRowEchelonFormAssign.html) |
| ⚙ | `slong fmpz_mat_rref_fflu(fmpz_mat_t B, fmpz_t den, const fmpz_mat_t A)` | internal |
| ⚙ | `slong fmpz_mat_rref_mul(fmpz_mat_t B, fmpz_t den, const fmpz_mat_t A)` | internal |
| ⚙ | `int fmpz_mat_is_in_rref_with_rank(const fmpz_mat_t A, const fmpz_t den, slong rank)` | internal |
| ⚙ | `int fmpz_mat_rref_upper_certify_lu_mod_p(fmpz_mat_t E, fmpz_t den, const fmpz_mat_t A, slong rank, const slong * P, const slong * pivs)` | internal |
| ⚙ | `int fmpz_mat_rank_certify_lu_mod_p(const fmpz_mat_t A, slong rank, const slong * P, const slong * pivs)` | internal |

The central functions are `fmpz_mat_fflu`, a fraction-free LU
decomposition (Nakos, Turner, and Williams) that also returns the rank and, through `perm`, the
row permutation, and `fmpz_mat_rref`, the reduced row echelon form. Both stay in integer
arithmetic by returning an integer matrix and a denominator `den`, which is $$\pm\det S$$ for a
suitable square submatrix $$S$$ of $$A$$ and is not in general the smallest denominator; this is
the same form as in [Nonsingular solving](#nonsingular-solving). In Malachite, `fmpz_mat_rref` is
[`ReducedRowEchelonForm`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ReducedRowEchelonForm.html) on an [`IntegerMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_matrix/struct.IntegerMatrix.html), by value or by reference, which returns `(B, den)` with
FLINT's results, and [`ReducedRowEchelonFormAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ReducedRowEchelonFormAssign.html), which replaces the matrix with `B` and
returns `den`; they are ≈ because the rank is not returned, but it is the number of nonzero rows of
`B`. `rref_fflu` and `rref_mul` are the internal choices behind them.

`fmpz_mat_fflu` is
[`FractionFreeLuDecomposition`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.FractionFreeLuDecomposition.html)
on an
[`IntegerMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_matrix/struct.IntegerMatrix.html),
by value or by reference, hence ≈: like
[`ModLuDecomposition`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModLuDecomposition.html),
it returns the permutation, $$L$$, and $$U$$ separately rather than packed into one matrix, with the
rank as the width of $$L$$, and it returns `den` as well. There is no rank check; Malachite's
internal callers use one. The pivots, and so all the results, are FLINT's: with pivots
$$p_0, \ldots, p_{r-1}$$, $$PA = LD^{-1}U$$ for
$$D = \operatorname{diag}(p_0, p_0p_1, \ldots, p_{r-2}p_{r-1})$$, and `den` is $$p_{r-1}$$. The manual
says that `den` is $$\pm\det(S)$$ with the sign decided by the parity of the permutation, but
`fmpz_mat_fflu` does not change the sign; `det_bareiss` does. `is_in_rref_with_rank` and
`find_pivot_any` are internal functions in Malachite; `is_in_rref_with_rank` accepts zero rows among
the first `rank` rows, as FLINT's does. `fmpz_mat_rref` uses the fraction-free method (`rref_fflu`) when the
matrix has at most 3 rows or at most 2 columns, or when it is wide: wider than tall with at most
20 rows, or, with 21 to 100 rows, wider than tall by a margin that grows with the height.
Otherwise, and so always above 100 rows, it uses the multimodular method (`rref_mul`, after
Stein), which finds the echelon structure modulo a prime and then certifies it over the
integers.

The two `certify` functions are that certification step, exposed separately: given the rank,
permutation, and pivot columns of an LU decomposition modulo a prime, they either confirm the
rank and (for `rref_upper_certify_lu_mod_p`) compute the corresponding rows of the reduced row
echelon form over $$\mathbb{Z}$$, or report that the prime was unlucky. Both are internal
functions in Malachite, used by the [solvers](#nonsingular-solving) to certify that a matrix is
singular. `is_in_rref_with_rank` checks a claimed result. `find_pivot_any` returns the first row in a range whose entry in a given
column is nonzero, or $$-1$$; `fmpz_mat.h` also declares an undocumented `find_pivot_smallest`,
used by the [rank](#rank) computation, and the description of `find_pivot_any` calls its
`end_row` parameter `stop_row`.

The description of `fmpz_mat_rref_mul` says that its certification step is implemented by
`fmpz_mat_compressed_rref_given_mod_p_structure`. No function of that name exists in FLINT 3.6.0;
the step is `fmpz_mat_rref_upper_certify_lu_mod_p`.

## [Strong echelon form and Howell form](https://flintlib.org/doc/fmpz_mat.html#strong-echelon-form-and-howell-form) {#strong-echelon-form-and-howell-form}

| | FLINT | Malachite |
| :---: | --- | --- |
| ≈ | `void fmpz_mat_strong_echelon_form_mod(fmpz_mat_t A, const fmpz_t mod)` | [`ModStrongEchelonForm`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModStrongEchelonForm.html), [`ModStrongEchelonFormAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModStrongEchelonFormAssign.html) |
| ≈ | `slong fmpz_mat_howell_form_mod(fmpz_mat_t A, const fmpz_t mod)` | [`ModHowellForm`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModHowellForm.html), [`ModHowellFormAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModHowellFormAssign.html) |

Over $$\mathbb{Z}/m\mathbb{Z}$$, where $$m$$ need not be prime, the row echelon form is not unique,
and the Howell form (Storjohann and Mulders) is the canonical echelon form of the row span that
takes its place; the strong echelon form (Fieker and Hofmann) holds the same rows, each in the row
whose index is the column of its pivot. Both functions transform `A` in place, so that its entries
reduced modulo `mod` form the result; the entries of `A` may be negative or larger than `mod`
beforehand. In Malachite these are
[`ModStrongEchelonForm`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModStrongEchelonForm.html)
and
[`ModHowellForm`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ModHowellForm.html)
(and their `Assign` versions) on a
[`NaturalMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_matrix/struct.NaturalMatrix.html)
with a [`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html)
modulus, whose elements must already be reduced: reduce an
[`IntegerMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_matrix/struct.IntegerMatrix.html)
first with [`Mod`](https://docs.rs/malachite-base/latest/malachite_base/num/modular/traits/trait.Mod.html), which gives a `NaturalMatrix`, hence ≈. When the modulus fits in a
word, they compute the forms as `nmod_mat_strong_echelon_form` and `nmod_mat_howell_form` do (see
[the word-sized page](/mapping/flint-word-sized-integer-matrices-mod-n/#strong-echelon-form-and-howell-form)),
which give the same results, since both forms are unique.
They panic if the modulus is 0 or if any element is not reduced. FLINT requires at least as many
rows as columns, and does not check it; Malachite gives a wider matrix zero rows until it is square,
which leaves its row span unchanged, so its forms are always $$\max(h, w) \times w$$. The Howell form of a
wide matrix can need more nonzero rows than the matrix has: modulo 4, the Howell form of
$$\begin{pmatrix} 2 & 1 \end{pmatrix}$$ is $$\begin{pmatrix} 2 & 1 \\ 0 & 2 \end{pmatrix}$$. The Howell
functions return the number of nonzero rows of the result, and Malachite's do not, hence ≈: it is
the number of leading nonzero rows of the result, since the zero rows come last.
FLINT's `fmpz_mat_howell_form_mod` moves the zero rows to the bottom, keeping the nonzero rows in
order, which the manual does not mention. `fmpz_mat_strong_echelon_form_mod` ends with a pass that
reduces each entry above a nonzero diagonal entry by that entry's row; Malachite omits it, because
the second phase has already left each such entry less than the diagonal entry below it, and no later
step changes either.

## [Nullspace](https://flintlib.org/doc/fmpz_mat.html#nullspace) {#nullspace}

| | FLINT | Malachite |
| :---: | --- | --- |
| ≈ | `slong fmpz_mat_nullspace(fmpz_mat_t B, const fmpz_mat_t A)` | [`Nullspace`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.Nullspace.html) |

`fmpz_mat_nullspace` returns the nullity $$k$$ of an $$m \times n$$ matrix $$A$$ and writes a basis
of its right nullspace, $$\{b : Ab = 0\}$$, into the first $$k$$ columns of `B`, which the caller
must allocate with $$n$$ rows and enough columns (the manual suggests $$n \times n$$); the other
columns are set to zero. In Malachite, this is [`Nullspace`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.Nullspace.html) on an [`IntegerMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_matrix/struct.IntegerMatrix.html), by value or by
reference, which returns a matrix with exactly $$k$$ columns, FLINT's first $$k$$, so the nullity is
its width; hence ≈.

The basis comes from the reduced row echelon form, as in `fmpz_mat/nullspace.c`: for each
non-pivot column, the basis vector has the reduced entries of that column in the pivot positions
and $$-\mathit{den}$$ in the column's own position. Its entries are integers but are not divided by
their GCD, so, as the manual warns, they are generally not minimal.

## [Hermite normal form](https://flintlib.org/doc/fmpz_mat.html#hermite-normal-form) {#hermite-normal-form}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpz_mat_hnf(fmpz_mat_t H, const fmpz_mat_t A)` | [`HermiteNormalForm`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.HermiteNormalForm.html), [`HermiteNormalFormAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.HermiteNormalFormAssign.html) |
| ✓ | `void fmpz_mat_hnf_transform(fmpz_mat_t H, fmpz_mat_t U, const fmpz_mat_t A)` | [`HermiteNormalFormAndTransform`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.HermiteNormalFormAndTransform.html) |
| ⚙ | `void fmpz_mat_hnf_classical(fmpz_mat_t H, const fmpz_mat_t A)` | internal |
| ⚙ | `void fmpz_mat_hnf_xgcd(fmpz_mat_t H, const fmpz_mat_t A)` | internal |
| ⚙ | `void fmpz_mat_hnf_modular(fmpz_mat_t H, const fmpz_mat_t A, const fmpz_t D)` | internal |
| ⚙ | `void fmpz_mat_hnf_modular_eldiv(fmpz_mat_t A, const fmpz_t D)` | internal |
| ⚙ | `void fmpz_mat_hnf_minors(fmpz_mat_t H, const fmpz_mat_t A)` | internal |
| ⚙ | `void fmpz_mat_hnf_pernet_stein(fmpz_mat_t H, const fmpz_mat_t A, flint_rand_t state)` | internal |
| ✓ | `int fmpz_mat_is_in_hnf(const fmpz_mat_t A)` | [`is_in_hermite_normal_form`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_matrix/struct.IntegerMatrix.html#method.is_in_hermite_normal_form) |

The (row) Hermite normal form $$H$$ of an integer matrix $$A$$ is the unique matrix with the same
row lattice in which the nonzero rows come first, each row's first nonzero entry (its pivot) is
positive and lies strictly to the right of the pivot of the row above, and the entries above
each pivot lie in $$[0, \text{pivot})$$; this is what `is_in_hnf` checks.

In Malachite, `fmpz_mat_hnf` is [`HermiteNormalForm`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.HermiteNormalForm.html) on an
[`IntegerMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_matrix/struct.IntegerMatrix.html), by value or by reference, or
[`HermiteNormalFormAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.HermiteNormalFormAssign.html) in place, and `hnf_transform` is
[`HermiteNormalFormAndTransform`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.HermiteNormalFormAndTransform.html), which returns the
pair $$(H, U)$$. The algorithm-specific functions are internal choices behind them, and give FLINT's
results.

`fmpz_mat_hnf` uses the classical algorithm (`hnf_classical`, Cohen's Algorithm 2.4.4) for
matrices with fewer rows than a cutoff that shrinks as the entries grow, from 52 rows for entries
of at most 2 bits to 3 rows for entries of at most 512 bits, and the randomized algorithm of
Pernet and Stein otherwise; the result does not depend on the random choices. Malachite makes the
same kind of choice, with cutoffs measured for its own arithmetic, from 60 rows for entries of at
most 2 bits to 2 rows for entries of more than 128 bits, and its Pernet–Stein implementation is
deterministic: it tries primes in increasing order rather than at random. Three of the others
assume, without checking, that $$A$$ has full column rank: `hnf_minors` (Kannan and Bachem),
`hnf_modular` (Domich, Kannan, and Trotter), which also assumes that `D` is a positive multiple of
the determinant of the lattice, and `hnf_modular_eldiv` (Fieker and Hofmann), which works in place
and also assumes that `D` is a positive multiple of the largest elementary divisor.

`hnf_transform` also returns a unimodular $$U$$ with $$UA = H$$, which is unique only when $$A$$ has
full row rank. The manual says that `hnf_transform` chooses its algorithm "as per
`fmpz_mat_hnf`". `fmpz_mat/hnf_transform.c` instead appends an identity matrix to $$A$$ and calls
`fmpz_mat_hnf` when $$A$$ has more columns than rows or is found, modulo a random prime, not to have
full column rank, and otherwise calls `fmpz_mat_hnf_minors_transform`, which `fmpz_mat.h` declares
but the manual does not document. Malachite makes the same choice but confirms a rank deficiency
found modulo its prime over the integers, so its $$U$$ is FLINT's except when FLINT's random prime
happens to lower the rank; its extended GCDs follow `fmpz_xgcd`'s choice of cofactors, so that
$$U$$ is the same even when it is not unique.

## [Smith normal form](https://flintlib.org/doc/fmpz_mat.html#smith-normal-form) {#smith-normal-form}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpz_mat_snf(fmpz_mat_t S, const fmpz_mat_t A)` | [`SmithNormalForm`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.SmithNormalForm.html), [`SmithNormalFormAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.SmithNormalFormAssign.html) |
| ⚙ | `void fmpz_mat_snf_diagonal(fmpz_mat_t S, const fmpz_mat_t A)` | internal |
| ⚙ | `void fmpz_mat_snf_kannan_bachem(fmpz_mat_t S, const fmpz_mat_t A)` | internal |
| ⚙ | `void fmpz_mat_snf_iliopoulos(fmpz_mat_t S, const fmpz_mat_t A, const fmpz_t mod)` | internal |
| ✓ | `int fmpz_mat_is_in_snf(const fmpz_mat_t A)` | [`is_in_smith_normal_form`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_matrix/struct.IntegerMatrix.html#method.is_in_smith_normal_form) |
| ✓ | `int fmpz_mat_is_diagonal(const fmpz_mat_t A)` | [`is_diagonal`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.is_diagonal) |
| ≈ | `void fmpz_mat_snf_transform(fmpz_mat_t S, fmpz_mat_t U, fmpz_mat_t V, const fmpz_mat_t A)` | [`SmithNormalFormAndTransforms`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.SmithNormalFormAndTransforms.html) |
| ✓ | `slong fmpz_mat_elementary_divisors(fmpz * ed, const fmpz_mat_t A)` | [`ElementaryDivisors`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ElementaryDivisors.html) |

The Smith normal form of an $$m \times n$$ integer matrix $$A$$ is the unique diagonal matrix
$$S = UAV$$, with $$U$$ and $$V$$ unimodular, whose diagonal entries $$d_1 \mid d_2 \mid \cdots$$
are nonnegative and each divide the next; the nonzero ones are the elementary divisors, and their
number is the rank.

In Malachite, `fmpz_mat_snf` is [`SmithNormalForm`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.SmithNormalForm.html) on an
[`IntegerMatrix`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_matrix/struct.IntegerMatrix.html), by value or by reference, or
[`SmithNormalFormAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.SmithNormalFormAssign.html) in place;
`fmpz_mat_elementary_divisors` is [`ElementaryDivisors`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.ElementaryDivisors.html), which
returns a `Vec` of [`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html)s,
one for each unit of rank, rather than filling a buffer and returning the rank. The
algorithm-specific functions are internal choices behind them, and give FLINT's results.

In FLINT 3.6.0, `fmpz_mat_snf` uses `snf_diagonal` when $$A$$ is already diagonal and otherwise
alternates row and column Hermite normal forms until the matrix is diagonal, then fixes the
divisibility chain with GCDs and LCMs of pairs of diagonal entries and makes the diagonal
nonnegative; `snf_transform` does the same while accumulating $$U$$ and $$V$$, either of which may
be `NULL`. The alternation has an iteration bound, and exceeding it throws, with a message asking
for a bug report (Malachite panics). Malachite makes the same choices, except that matrices whose
smaller dimension is at most 4 are reduced by the algorithm of Kannan and Bachem, whose
intermediate entries grow quickly in larger matrices but which is faster for these.
`snf_kannan_bachem` and `snf_iliopoulos` are alternative algorithms; `snf_iliopoulos` assumes,
without checking, that $$A$$ is square and nonsingular and that `mod` is a positive multiple of the
largest elementary divisor, as does `snf_diagonal` that $$A$$ is diagonal.

[`SmithNormalFormAndTransforms`](https://docs.rs/malachite-base/latest/malachite_base/num/linear_algebra/traits/trait.SmithNormalFormAndTransforms.html) always returns the
triple $$(S, U, V)$$; there is no way to skip computing $$U$$ or $$V$$, as passing `NULL` to
`snf_transform` does. The transforms are not unique, and Malachite's are FLINT's, since its
Hermite normal form transforms and extended GCDs make FLINT's choices.

`elementary_divisors` uses Lübeck's algorithm, which factors the pivots of the Hermite normal
form and finds the powers of each prime factor from nullspaces modulo the prime, and falls back to
the full Smith normal form when a pivot is too large to factor cheaply. Malachite uses Lübeck's
algorithm only when the rank is at least 32 and every pivot fits in a `u64`, and otherwise finds
the Smith normal form from the Hermite normal form.

[`is_diagonal`](https://docs.rs/malachite-base/latest/malachite_base/matrix/trait.Matrix.html#tymethod.is_diagonal)
agrees with `fmpz_mat_is_diagonal`: neither requires the matrix to be square, and a matrix with no
elements is diagonal. (`_fmpz_mat_snf_iter_bound` is omitted.)

## [Special matrices](https://flintlib.org/doc/fmpz_mat.html#special-matrices) {#special-matrices}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void fmpz_mat_gram(fmpz_mat_t B, const fmpz_mat_t A)` | |
| ✗ | `int fmpz_mat_is_hadamard(const fmpz_mat_t H)` | |
| ✗ | `int fmpz_mat_hadamard(fmpz_mat_t H)` | |

`fmpz_mat_gram` sets `B` to the Gram matrix $$AA^T$$ of the rows of an $$m \times n$$ matrix $$A$$:
its $$(i, j)$$ entry is the dot product of rows $$i$$ and $$j$$. It throws unless `B` is
$$m \times m$$. Malachite does not have it yet; meanwhile, entry $$(i, j)$$ is
`a.row(i).dot(&a.row(j))`, with
[`row`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_matrix/struct.IntegerMatrix.html#method.row)
and [`Dot`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Dot.html).

A Hadamard matrix of order $$n$$ has entries $$\pm 1$$ and satisfies $$HH^T = nI$$, which is what
`fmpz_mat_is_hadamard` checks; such a matrix can exist only for $$n = 1$$, $$n = 2$$, or $$n$$ a
multiple of 4. `fmpz_mat_hadamard` fills a square `H` with one, or returns 0 if its construction
does not reach that order: it uses Paley's constructions from the quadratic characters of a
finite field of odd prime-power order $$q$$, giving orders $$q + 1$$ and $$2(q + 1)$$, followed by
repeated doubling (Sylvester's construction). It returns 1 for the $$0 \times 0$$ matrix. Malachite
has neither function yet; the Paley step needs arithmetic in fields of prime-power order, which
Malachite does not have.

## [Conversions](https://flintlib.org/doc/fmpz_mat.html#conversions) {#conversions}

| | FLINT | Malachite |
| :---: | --- | --- |
| — | `int fmpz_mat_get_d_mat(d_mat_t B, const fmpz_mat_t A)` | |
| — | `int fmpz_mat_get_d_mat_transpose(d_mat_t B, const fmpz_mat_t A)` | |

These convert every element to a [`f64`](https://doc.rust-lang.org/nightly/core/primitive.f64.html),
rounding toward zero, into FLINT's `d_mat_t` (a matrix of doubles), and return $$-1$$ if some
element's magnitude exceeds the largest finite double; the elements converted before that one
are left in `B`. Malachite has no matrix of floating-point numbers, hence —. For a single element,
the conversion is
[`RoundingFrom`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.RoundingFrom.html)
with `Down`, `f64::rounding_from(&x, Down).0`, which, when $$|x|$$ is too large, returns the finite
double of largest magnitude with the sign of $$x$$ rather than an error, so test for that case
separately if it matters.

## [Cholesky Decomposition](https://flintlib.org/doc/fmpz_mat.html#cholesky-decomposition) {#cholesky-decomposition}

| | FLINT | Malachite |
| :---: | --- | --- |
| — | `void fmpz_mat_chol_d(d_mat_t R, const fmpz_mat_t A)` | |
| ✗ | `int fmpz_mat_is_spd(const fmpz_mat_t A)` | |

`fmpz_mat_chol_d` computes, in double-precision arithmetic, the lower triangular $$R$$ with
$$A = RR^T$$ for a symmetric positive definite $$A$$; it throws on a shape mismatch, and if $$A$$ is
not positive definite, which it does not check, the square roots it takes produce NaNs. Its output
is a matrix of doubles, which Malachite does not have (see [Conversions](#conversions)), hence —.

`fmpz_mat_is_spd` decides exactly whether $$A$$ is symmetric and positive definite (and so square).
It first tries an $$LDL^T$$ decomposition in ball arithmetic, which can only confirm positive
definiteness, and if that fails, computes the [characteristic
polynomial](#characteristic-polynomial) and checks with Descartes' rule of signs that all its
roots are positive. Malachite does not have it yet. The manual gives its return type as `void`;
`fmpz_mat.h` declares `int`.

## [LLL](https://flintlib.org/doc/fmpz_mat.html#lll) {#lll}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `int fmpz_mat_is_reduced(const fmpz_mat_t A, double delta, double eta)` | |
| ✗ | `int fmpz_mat_is_reduced_gram(const fmpz_mat_t A, double delta, double eta)` | |
| ✗ | `int fmpz_mat_is_reduced_with_removal(const fmpz_mat_t A, double delta, double eta, const fmpz_t gs_B, int newd)` | |
| ✗ | `int fmpz_mat_is_reduced_gram_with_removal(const fmpz_mat_t A, double delta, double eta, const fmpz_t gs_B, int newd)` | |

A basis, given as the rows of $$A$$, is $$(\delta, \eta)$$-LLL-reduced when its Gram–Schmidt
coefficients satisfy $$|\mu_{ij}| \le \eta$$ (size reduction) and consecutive Gram–Schmidt lengths
satisfy Lovász's condition with factor $$\delta$$. These functions decide that exactly; the `_gram`
forms take the Gram matrix of the basis instead of the basis, and the `_with_removal` forms check
only the first `newd` vectors and require the squared Gram–Schmidt lengths of the others to exceed
`gs_B`. `is_reduced` and `is_reduced_with_removal` work in ball arithmetic at increasing
precision and switch to exact rational arithmetic when the precision would exceed a bound, so
their answers are exact; the `_gram` forms use rational arithmetic throughout. Since `delta` and
`eta` are `double`s, converted exactly to rationals, a value such as 0.99 stands for the double
nearest it, not for $$99/100$$.

FLINT's main LLL implementation is the separate `fmpz_lll` module, a floating-point algorithm
in the style of Nguyen and Stehlé; this chapter has only these checks and the two exact
algorithms below. Malachite has no lattice reduction yet. When it does, the exact algorithms need
only integers and rationals, while a floating-point one would build on
[`Float`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html).

## [Classical LLL](https://flintlib.org/doc/fmpz_mat.html#classical-lll) {#classical-lll}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void fmpz_mat_lll_original(fmpz_mat_t A, const fmpq_t delta, const fmpq_t eta)` | |

`fmpz_mat_lll_original` is the algorithm of Lenstra, Lenstra, and Lovász, run entirely in exact
rational arithmetic: it computes the Gram–Schmidt coefficients as rationals and replaces the rows
of $$A$$, which must be linearly independent (this is not checked), by a $$(\delta, \eta)$$-reduced
basis of the same lattice. Unlike the checks above, it takes `delta` and `eta` as exact
rationals.

## [Modified LLL](https://flintlib.org/doc/fmpz_mat.html#modified-lll) {#modified-lll}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void fmpz_mat_lll_storjohann(fmpz_mat_t A, const fmpq_t delta, const fmpq_t eta)` | |

`fmpz_mat_lll_storjohann` computes the same kind of reduced basis by Storjohann's modified
algorithm, which keeps the Gram–Schmidt data as integers (from the fraction-free Gram matrix of
the rows) and reduces it modulo a bound, improving the dependence on the dimension. Its only
floating-point step computes that bound, $$2\lceil\sqrt{n \cdot \max}\,\rceil + 1$$, in double
precision; the rest is exact.
