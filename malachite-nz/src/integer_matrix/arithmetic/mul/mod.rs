// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_matrix::IntegerMatrix;
use crate::integer_matrix::arithmetic::mul::classical::{
    matrix_mul_classical, matrix_mul_classical_assign,
};
use crate::integer_matrix::arithmetic::mul::double_word::{
    DoubleWordRows, double_words, matrix_mul_double_word_internal,
};
use crate::integer_matrix::arithmetic::mul::multi_mod::matrix_mul_multi_mod;
use crate::integer_matrix::arithmetic::mul::small::{
    matrix_mul_small_1, matrix_mul_small_2a, matrix_mul_small_2b, matrix_mul_small_internal,
    small_words, small_words_transposed,
};
use crate::integer_matrix::arithmetic::mul::strassen::matrix_mul_strassen;
use crate::integer_matrix::arithmetic::mul::waksman::matrix_mul_waksman;
use crate::integer_polynomial::arithmetic::coefficient::{
    PolynomialCoefficient, SMALL_FMPZ_BITCOUNT_MAX,
};
use crate::integer_vector::arithmetic::max_bits::vec_max_bits;
use crate::natural::TWICE_WIDTH;
use crate::platform::SignedLimb;
use alloc::vec;
use alloc::vec::Vec;
use core::ops::{Mul, MulAssign};
use malachite_base::fail_on_untested_path;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;

/// The classical algorithm.
pub mod classical;
/// FLINT's algorithm for elements that fit in two limbs.
pub mod double_word;
/// The multimodular algorithm.
pub mod multi_mod;
/// FLINT's algorithms for elements that fit in small `fmpz`s.
pub mod small;
/// Strassen's algorithm.
pub mod strassen;
/// Waksman's algorithm.
pub mod waksman;

// Above this smallest dimension, a product whose elements fit in small `fmpz`s uses Strassen's
// algorithm: `1000 + 300 * flint_get_num_threads()` in `fmpz_mat/mul.c`, FLINT 3.6.0, with one
// thread.
const STRASSEN_SMALL_DIMENSION_MIN: usize = 1300;

// Above this smallest dimension, a product whose elements fit in small `fmpz`s but whose result
// does not uses the multimodular algorithm: `4000 + 300 * flint_get_num_threads()` in
// `fmpz_mat/mul.c`, FLINT 3.6.0, with one thread.
const MULTI_MOD_SMALL_DIMENSION_MIN: usize = 4300;

// The shape of a matrix product: the first factor is `ar` by `br`, and the second is `br` by `bc`.
crate_test_struct! {
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
ProductShape {
    pub ar: usize,
    pub br: usize,
    pub bc: usize,
}}

// How the algorithms for small elements sum each dot product.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SmallMulKind {
    // In one word, without blocking.
    One,
    // In a double word, without blocking.
    TwoA,
    // In three words, without blocking.
    TwoB,
    // With blocking, in as many words as needed for products with the given number of significant
    // bits.
    Internal(u64),
}

// An algorithm that reads the elements of both matrices as words.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum WordMulAlgorithm {
    // For elements that fit in small `fmpz`s.
    Small(SmallMulKind),
    // For elements that fit in double words: whether some element is negative, and the number of
    // significant bits that every element of the product fits in.
    DoubleWord(bool, u64),
}

// The algorithm that `matrix_mul` uses for a product.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MatrixMulAlgorithm {
    // The product is a zero matrix.
    Zero,
    Classical,
    Waksman,
    Strassen,
    // Whether some element is negative, and the number of significant bits that every element of
    // the product fits in.
    MultiMod(bool, u64),
    Word(WordMulAlgorithm),
}

// Chooses the algorithm for the product of the matrices with elements `xs` and `ys`.
//
// This is the choice that `fmpz_mat_mul` from `fmpz_mat/mul.c`, FLINT 3.6.0, makes with one thread
// and without BLAS.
pub(crate) fn choose_matrix_mul_algorithm<C: PolynomialCoefficient>(
    xs: &[C],
    ys: &[C],
    shape: ProductShape,
) -> MatrixMulAlgorithm {
    let ProductShape { ar, br, bc } = shape;
    if ar == 0 || br == 0 || bc == 0 {
        return MatrixMulAlgorithm::Zero;
    }
    // The classical algorithm handles an inner dimension of 1 or 2 directly.
    if br <= 2 {
        return MatrixMulAlgorithm::Classical;
    }
    let (a_bits, a_negative) = vec_max_bits(xs);
    let (b_bits, b_negative) = vec_max_bits(ys);
    let sign = a_negative || b_negative;
    if a_bits == 0 || b_bits == 0 {
        return MatrixMulAlgorithm::Zero;
    }
    let c_bits = a_bits + b_bits + u64::exact_from(br).significant_bits();
    let dim = ar.min(bc).min(br);
    if a_bits <= SMALL_FMPZ_BITCOUNT_MAX && b_bits <= SMALL_FMPZ_BITCOUNT_MAX {
        // Strassen's algorithm and the multimodular one are effective at large dimensions.
        let kind = if ar < 9 || ar + br < 20 {
            if c_bits <= SMALL_FMPZ_BITCOUNT_MAX {
                SmallMulKind::One
            } else if c_bits < TWICE_WIDTH {
                SmallMulKind::TwoA
            } else {
                SmallMulKind::TwoB
            }
        } else {
            // With one thread, FLINT's limit is 300. Strassen's algorithm avoids large intermediate
            // values when the product fits in a small `fmpz`; otherwise the multimodular algorithm
            // is used at even larger dimensions.
            if c_bits <= SMALL_FMPZ_BITCOUNT_MAX {
                if dim > STRASSEN_SMALL_DIMENSION_MIN {
                    return MatrixMulAlgorithm::Strassen;
                }
            } else if dim > MULTI_MOD_SMALL_DIMENSION_MIN {
                // Every dimension would have to exceed 4300, too large for the tests.
                fail_on_untested_path("choose_matrix_mul_algorithm, small elements, multimodular");
                return MatrixMulAlgorithm::MultiMod(sign, c_bits);
            }
            SmallMulKind::Internal(c_bits)
        };
        MatrixMulAlgorithm::Word(WordMulAlgorithm::Small(kind))
    } else if a_bits + u64::from(sign) <= TWICE_WIDTH && b_bits + u64::from(sign) <= TWICE_WIDTH {
        // The double-word algorithm's cost hardly depends on `c_bits`, so it does better as
        // `c_bits` grows, but the multimodular algorithm eventually wins at large dimensions. The
        // double-word algorithm handles unsigned elements better than signed ones.
        let dim = if sign { dim << 1 } else { dim };
        if dim > 300 {
            // FLINT computes `(c_bits - 2 * FLINT_BITS) / 8` in unsigned arithmetic, so when
            // `c_bits` is less than `2 * FLINT_BITS` the difference wraps around and the squared
            // limit overflows; the evident intent, a limit of 0, is used instead.
            let limit = usize::exact_from(c_bits.saturating_sub(TWICE_WIDTH) >> 3).pow(2);
            if dim - 300 > limit {
                return MatrixMulAlgorithm::MultiMod(sign, c_bits);
            }
        }
        MatrixMulAlgorithm::Word(WordMulAlgorithm::DoubleWord(sign, c_bits))
    } else {
        // Waksman's algorithm wants elements of similar sizes.
        if u64::exact_from(dim) >= 3 * c_bits.significant_bits() {
            return MatrixMulAlgorithm::MultiMod(sign, c_bits);
        }
        let min_bits = a_bits.min(b_bits);
        let max_bits = a_bits.max(b_bits);
        if dim < 20
            && ((dim == 2 && min_bits >= 5000 && max_bits * 10 <= min_bits * 11)
                || (max_bits * 5 <= min_bits << 3
                    && ((dim == 3 && min_bits >= 3000)
                        || (dim >= 4 && min_bits >= 1000)
                        || (dim >= 12 && min_bits >= 500))))
        {
            MatrixMulAlgorithm::Waksman
        } else if a_bits >= 500 && b_bits >= 500 && dim >= 8 {
            MatrixMulAlgorithm::Strassen
        } else {
            MatrixMulAlgorithm::Classical
        }
    }
}

// A word algorithm, together with the first matrix in the form that it reads.
enum WordMul<'a, C> {
    // The algorithms for small elements, with the first matrix as signed words.
    Small(SmallMulKind, Vec<SignedLimb>),
    // The algorithm for double-word elements: whether some element is negative, the number of
    // significant bits that every element of the product fits in, and the rows of the first matrix.
    DoubleWord(bool, u64, DoubleWordRows<'a, C>),
}

impl<'a, C: PolynomialCoefficient> WordMul<'a, C> {
    // Prepares to multiply the matrix with elements `xs`, reading its elements where the algorithm
    // allows, as FLINT does: the double-word algorithm converts one row at a time.
    fn borrowing(algorithm: WordMulAlgorithm, xs: &'a [C]) -> Self {
        match algorithm {
            WordMulAlgorithm::Small(kind) => WordMul::Small(kind, small_words(xs)),
            WordMulAlgorithm::DoubleWord(sign, c_bits) => {
                WordMul::DoubleWord(sign, c_bits, DoubleWordRows::Elements(xs))
            }
        }
    }

    // Prepares to multiply the matrix with elements `xs`, converting all of them first, so that the
    // product may be written over `xs`.
    fn converting(algorithm: WordMulAlgorithm, xs: &[C]) -> Self {
        match algorithm {
            WordMulAlgorithm::Small(kind) => WordMul::Small(kind, small_words(xs)),
            WordMulAlgorithm::DoubleWord(sign, c_bits) => WordMul::DoubleWord(
                sign,
                c_bits,
                DoubleWordRows::Converted(double_words(xs, sign)),
            ),
        }
    }

    // Sets `out` to the product of the prepared first matrix and the matrix with elements `ys`.
    fn run(&self, out: &mut [C], ys: &[C], shape: ProductShape) {
        match self {
            WordMul::Small(SmallMulKind::One, a_words) => {
                matrix_mul_small_1(out, a_words, &small_words_transposed(ys, shape), shape);
            }
            WordMul::Small(SmallMulKind::TwoA, a_words) => {
                matrix_mul_small_2a(out, a_words, &small_words_transposed(ys, shape), shape);
            }
            WordMul::Small(SmallMulKind::TwoB, a_words) => {
                matrix_mul_small_2b(out, a_words, &small_words_transposed(ys, shape), shape);
            }
            &WordMul::Small(SmallMulKind::Internal(c_bits), ref a_words) => {
                matrix_mul_small_internal(out, a_words, ys, shape, c_bits);
            }
            &WordMul::DoubleWord(sign, c_bits, ref rows) => {
                matrix_mul_double_word_internal(out, rows, ys, shape, sign, c_bits);
            }
        }
    }
}

// Returns the elements of the product of the matrices with elements `xs` and `ys`, both in
// row-major order, choosing an algorithm.
//
// This is equivalent to `fmpz_mat_mul` from `fmpz_mat/mul.c`, FLINT 3.6.0, with one thread and
// without BLAS.
crate_test_fn! {matrix_mul<C: PolynomialCoefficient>(
    xs: &[C],
    ys: &[C],
    shape: ProductShape,
) -> Vec<C> {
    match choose_matrix_mul_algorithm(xs, ys, shape) {
        MatrixMulAlgorithm::Zero => vec![C::ZERO; shape.ar * shape.bc],
        MatrixMulAlgorithm::Classical => matrix_mul_classical(xs, ys, shape),
        MatrixMulAlgorithm::Waksman => {
            C::from_integers(matrix_mul_waksman(&C::as_integers(xs), &C::as_integers(ys), shape))
        }
        MatrixMulAlgorithm::Strassen => {
            C::from_integers(matrix_mul_strassen(&C::as_integers(xs), &C::as_integers(ys), shape))
        }
        MatrixMulAlgorithm::MultiMod(sign, c_bits) => {
            matrix_mul_multi_mod(xs, ys, shape, sign, c_bits)
        }
        MatrixMulAlgorithm::Word(algorithm) => {
            let mut out = vec![C::ZERO; shape.ar * shape.bc];
            WordMul::borrowing(algorithm, xs).run(&mut out, ys, shape);
            out
        }
    }
}}

// Replaces the elements `xs` of the first factor of a product with those of the product, choosing
// an algorithm. The classical algorithm reuses the storage of `xs`, a row at a time; the algorithms
// for small elements read the first matrix as words, which are copied out first, so the product can
// be written directly into `xs`; Waksman's and Strassen's algorithms compute the product
// separately.
//
// This is equivalent to `fmpz_mat_mul` from `fmpz_mat/mul.c`, FLINT 3.6.0, with one thread, without
// BLAS, and with the output the same as the first input.
crate_test_fn! {matrix_mul_assign<C: PolynomialCoefficient>(
    xs: &mut Vec<C>,
    ys: &[C],
    shape: ProductShape,
) {
    match choose_matrix_mul_algorithm(xs, ys, shape) {
        MatrixMulAlgorithm::Zero => {
            xs.clear();
            xs.resize(shape.ar * shape.bc, C::ZERO);
        }
        MatrixMulAlgorithm::Classical => matrix_mul_classical_assign(xs, ys, shape),
        MatrixMulAlgorithm::Waksman => {
            let product = matrix_mul_waksman(&C::as_integers(xs), &C::as_integers(ys), shape);
            // Keep the storage of `xs`.
            xs.clear();
            xs.append(&mut C::from_integers(product));
        }
        MatrixMulAlgorithm::Strassen => {
            let product = matrix_mul_strassen(&C::as_integers(xs), &C::as_integers(ys), shape);
            // Keep the storage of `xs`.
            xs.clear();
            xs.append(&mut C::from_integers(product));
        }
        MatrixMulAlgorithm::MultiMod(sign, c_bits) => {
            let mut product = matrix_mul_multi_mod(xs, ys, shape, sign, c_bits);
            // Keep the storage of `xs`.
            xs.clear();
            xs.append(&mut product);
        }
        MatrixMulAlgorithm::Word(algorithm) => {
            let product = WordMul::converting(algorithm, xs);
            xs.resize(shape.ar * shape.bc, C::ZERO);
            product.run(xs, ys, shape);
        }
    }
}}

fn assert_mul_compatible(a: &IntegerMatrix, b: &IntegerMatrix) {
    assert_eq!(
        a.width, b.height,
        "the width of the first matrix must equal the height of the second"
    );
}

const fn product_shape(a: &IntegerMatrix, b: &IntegerMatrix) -> ProductShape {
    ProductShape {
        ar: a.height,
        br: b.height,
        bc: b.width,
    }
}

fn mul_helper(a: &IntegerMatrix, b: &IntegerMatrix) -> IntegerMatrix {
    assert_mul_compatible(a, b);
    IntegerMatrix {
        elements: matrix_mul(&a.elements, &b.elements, product_shape(a, b)),
        height: a.height,
        width: b.width,
    }
}

fn mul_assign_helper(a: &mut IntegerMatrix, b: &IntegerMatrix) {
    assert_mul_compatible(a, b);
    let shape = product_shape(a, b);
    matrix_mul_assign(&mut a.elements, &b.elements, shape);
    a.width = b.width;
}

impl Mul<Self> for IntegerMatrix {
    type Output = Self;

    /// Multiplies two [`IntegerMatrix`]s, taking both by value.
    ///
    /// The product $AB$ of an $h \times k$ matrix $A$ and a $k \times w$ matrix $B$ is the $h
    /// \times w$ matrix whose element in row $i$ and column $j$ is the dot product of row $i$ of
    /// $A$ with column $j$ of $B$:
    ///
    /// $$
    /// f(A, B)_{ij} = \sum_{l=0}^{k-1} A_{il}B_{lj}.
    /// $$
    ///
    /// When $k$ is 0, the product is the $h \times w$ zero matrix.
    ///
    /// # Worst-case complexity
    /// $T(n, k, m) = O(nkm \log m \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of elements of the product,
    /// $k$ is the width of the first matrix, and $m$ is the largest number of significant bits of
    /// any element of either matrix.
    ///
    /// # Panics
    /// Panics if the width of the first matrix is not the height of the second.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[1, -2, 3; 4, 5, -6]").unwrap();
    /// let b = IntegerMatrix::from_str("[1, 0; 2, -1; 0, 3]").unwrap();
    /// assert_eq!((a * b).to_string(), "[-3, 11; 14, -23]");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_mul` from `fmpz_mat/mul.c`, FLINT 3.6.0, except that it
    /// panics if the dimensions are incompatible, which FLINT checks only in debug builds.
    #[inline]
    fn mul(self, other: Self) -> Self {
        mul_helper(&self, &other)
    }
}

impl Mul<&Self> for IntegerMatrix {
    type Output = Self;

    /// Multiplies two [`IntegerMatrix`]s, taking the first by value and the second by reference.
    ///
    /// The product $AB$ of an $h \times k$ matrix $A$ and a $k \times w$ matrix $B$ is the $h
    /// \times w$ matrix whose element in row $i$ and column $j$ is the dot product of row $i$ of
    /// $A$ with column $j$ of $B$:
    ///
    /// $$
    /// f(A, B)_{ij} = \sum_{l=0}^{k-1} A_{il}B_{lj}.
    /// $$
    ///
    /// When $k$ is 0, the product is the $h \times w$ zero matrix.
    ///
    /// # Worst-case complexity
    /// $T(n, k, m) = O(nkm \log m \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of elements of the product,
    /// $k$ is the width of the first matrix, and $m$ is the largest number of significant bits of
    /// any element of either matrix.
    ///
    /// # Panics
    /// Panics if the width of the first matrix is not the height of the second.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[1, -2, 3; 4, 5, -6]").unwrap();
    /// let b = IntegerMatrix::from_str("[1, 0; 2, -1; 0, 3]").unwrap();
    /// assert_eq!((a * &b).to_string(), "[-3, 11; 14, -23]");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_mul` from `fmpz_mat/mul.c`, FLINT 3.6.0, except that it
    /// panics if the dimensions are incompatible, which FLINT checks only in debug builds.
    #[inline]
    fn mul(self, other: &Self) -> Self {
        mul_helper(&self, other)
    }
}

impl Mul<IntegerMatrix> for &IntegerMatrix {
    type Output = IntegerMatrix;

    /// Multiplies two [`IntegerMatrix`]s, taking the first by reference and the second by value.
    ///
    /// The product $AB$ of an $h \times k$ matrix $A$ and a $k \times w$ matrix $B$ is the $h
    /// \times w$ matrix whose element in row $i$ and column $j$ is the dot product of row $i$ of
    /// $A$ with column $j$ of $B$:
    ///
    /// $$
    /// f(A, B)_{ij} = \sum_{l=0}^{k-1} A_{il}B_{lj}.
    /// $$
    ///
    /// When $k$ is 0, the product is the $h \times w$ zero matrix.
    ///
    /// # Worst-case complexity
    /// $T(n, k, m) = O(nkm \log m \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of elements of the product,
    /// $k$ is the width of the first matrix, and $m$ is the largest number of significant bits of
    /// any element of either matrix.
    ///
    /// # Panics
    /// Panics if the width of the first matrix is not the height of the second.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[1, -2, 3; 4, 5, -6]").unwrap();
    /// let b = IntegerMatrix::from_str("[1, 0; 2, -1; 0, 3]").unwrap();
    /// assert_eq!((&a * b).to_string(), "[-3, 11; 14, -23]");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_mul` from `fmpz_mat/mul.c`, FLINT 3.6.0, except that it
    /// panics if the dimensions are incompatible, which FLINT checks only in debug builds.
    #[inline]
    fn mul(self, other: IntegerMatrix) -> IntegerMatrix {
        mul_helper(self, &other)
    }
}

impl Mul<&IntegerMatrix> for &IntegerMatrix {
    type Output = IntegerMatrix;

    /// Multiplies two [`IntegerMatrix`]s, taking both by reference.
    ///
    /// The product $AB$ of an $h \times k$ matrix $A$ and a $k \times w$ matrix $B$ is the $h
    /// \times w$ matrix whose element in row $i$ and column $j$ is the dot product of row $i$ of
    /// $A$ with column $j$ of $B$:
    ///
    /// $$
    /// f(A, B)_{ij} = \sum_{l=0}^{k-1} A_{il}B_{lj}.
    /// $$
    ///
    /// When $k$ is 0, the product is the $h \times w$ zero matrix.
    ///
    /// # Worst-case complexity
    /// $T(n, k, m) = O(nkm \log m \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of elements of the product,
    /// $k$ is the width of the first matrix, and $m$ is the largest number of significant bits of
    /// any element of either matrix.
    ///
    /// # Panics
    /// Panics if the width of the first matrix is not the height of the second.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[1, -2, 3; 4, 5, -6]").unwrap();
    /// let b = IntegerMatrix::from_str("[1, 0; 2, -1; 0, 3]").unwrap();
    /// assert_eq!((&a * &b).to_string(), "[-3, 11; 14, -23]");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_mul` from `fmpz_mat/mul.c`, FLINT 3.6.0, except that it
    /// panics if the dimensions are incompatible, which FLINT checks only in debug builds.
    #[inline]
    fn mul(self, other: &IntegerMatrix) -> IntegerMatrix {
        mul_helper(self, other)
    }
}

impl MulAssign<Self> for IntegerMatrix {
    /// Multiplies an [`IntegerMatrix`] by another [`IntegerMatrix`] in place, taking the second by
    /// value.
    ///
    /// The product is written back into the storage of `self`. When every element fits in two limbs
    /// and the matrices are not too large, the elements of `self` are first copied out as words,
    /// and the product is written directly. For large matrices, Waksman's algorithm, Strassen's
    /// algorithm, or a multimodular algorithm computes the product separately, and it is then moved
    /// into the storage of `self`. Otherwise the product is computed one row at a time, so when it
    /// is no wider than `self`, and in particular when the second matrix is square, the only
    /// additional memory is one row.
    ///
    /// The product $AB$ of an $h \times k$ matrix $A$ and a $k \times w$ matrix $B$ is the $h
    /// \times w$ matrix whose element in row $i$ and column $j$ is the dot product of row $i$ of
    /// $A$ with column $j$ of $B$:
    ///
    /// $$
    /// f(A, B)_{ij} = \sum_{l=0}^{k-1} A_{il}B_{lj}.
    /// $$
    ///
    /// When $k$ is 0, the product is the $h \times w$ zero matrix.
    ///
    /// # Worst-case complexity
    /// $T(n, k, m) = O(nkm \log m \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of elements of the product,
    /// $k$ is the width of the first matrix, and $m$ is the largest number of significant bits of
    /// any element of either matrix.
    ///
    /// # Panics
    /// Panics if the width of the first matrix is not the height of the second.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let mut a = IntegerMatrix::from_str("[1, -2; 3, 4]").unwrap();
    /// a *= IntegerMatrix::from_str("[0, 1; 1, 0]").unwrap();
    /// assert_eq!(a.to_string(), "[-2, 1; 4, 3]");
    ///
    /// let mut a = IntegerMatrix::from_str("[1, -2, 3; 4, 5, -6]").unwrap();
    /// a *= IntegerMatrix::from_str("[1, 0; 2, -1; 0, 3]").unwrap();
    /// assert_eq!(a.to_string(), "[-3, 11; 14, -23]");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_mul` from `fmpz_mat/mul.c`, FLINT 3.6.0, except that it
    /// panics if the dimensions are incompatible, which FLINT checks only in debug builds.
    #[inline]
    fn mul_assign(&mut self, other: Self) {
        mul_assign_helper(self, &other);
    }
}

impl MulAssign<&Self> for IntegerMatrix {
    /// Multiplies an [`IntegerMatrix`] by another [`IntegerMatrix`] in place, taking the second by
    /// reference.
    ///
    /// The product is written back into the storage of `self`. When every element fits in two limbs
    /// and the matrices are not too large, the elements of `self` are first copied out as words,
    /// and the product is written directly. For large matrices, Waksman's algorithm, Strassen's
    /// algorithm, or a multimodular algorithm computes the product separately, and it is then moved
    /// into the storage of `self`. Otherwise the product is computed one row at a time, so when it
    /// is no wider than `self`, and in particular when the second matrix is square, the only
    /// additional memory is one row.
    ///
    /// The product $AB$ of an $h \times k$ matrix $A$ and a $k \times w$ matrix $B$ is the $h
    /// \times w$ matrix whose element in row $i$ and column $j$ is the dot product of row $i$ of
    /// $A$ with column $j$ of $B$:
    ///
    /// $$
    /// f(A, B)_{ij} = \sum_{l=0}^{k-1} A_{il}B_{lj}.
    /// $$
    ///
    /// When $k$ is 0, the product is the $h \times w$ zero matrix.
    ///
    /// # Worst-case complexity
    /// $T(n, k, m) = O(nkm \log m \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of elements of the product,
    /// $k$ is the width of the first matrix, and $m$ is the largest number of significant bits of
    /// any element of either matrix.
    ///
    /// # Panics
    /// Panics if the width of the first matrix is not the height of the second.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let mut a = IntegerMatrix::from_str("[1, -2; 3, 4]").unwrap();
    /// a *= &IntegerMatrix::from_str("[0, 1; 1, 0]").unwrap();
    /// assert_eq!(a.to_string(), "[-2, 1; 4, 3]");
    ///
    /// let mut a = IntegerMatrix::from_str("[1, -2, 3; 4, 5, -6]").unwrap();
    /// a *= &IntegerMatrix::from_str("[1, 0; 2, -1; 0, 3]").unwrap();
    /// assert_eq!(a.to_string(), "[-3, 11; 14, -23]");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_mul` from `fmpz_mat/mul.c`, FLINT 3.6.0, except that it
    /// panics if the dimensions are incompatible, which FLINT checks only in debug builds.
    #[inline]
    fn mul_assign(&mut self, other: &Self) {
        mul_assign_helper(self, other);
    }
}
