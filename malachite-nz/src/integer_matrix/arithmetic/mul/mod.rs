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
use crate::integer_matrix::arithmetic::mul::small::{
    matrix_mul_small_1, matrix_mul_small_2a, matrix_mul_small_2b, matrix_mul_small_internal,
    small_words, small_words_transposed,
};
use crate::integer_polynomial::arithmetic::coefficient::{
    PolynomialCoefficient, SMALL_FMPZ_BITCOUNT_MAX,
};
use crate::integer_vector::arithmetic::max_bits::vec_max_bits;
use crate::natural::TWICE_WIDTH;
use crate::platform::SignedLimb;
use alloc::vec;
use alloc::vec::Vec;
use core::ops::{Mul, MulAssign};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;

/// The classical algorithm.
pub mod classical;
/// FLINT's algorithm for elements that fit in two limbs.
pub mod double_word;
/// FLINT's algorithms for elements that fit in small `fmpz`s.
pub mod small;

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
    Word(WordMulAlgorithm),
}

// Chooses the algorithm for the product of the matrices with elements `xs` and `ys`.
//
// This is the choice that `fmpz_mat_mul` from `fmpz_mat/mul.c`, FLINT 3.6.0, makes with one thread
// and without BLAS, restricted to the algorithms ported so far: where FLINT would choose Strassen's
// algorithm or the multimodular one, this falls back to the algorithm FLINT uses for smaller
// dimensions.
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
    MatrixMulAlgorithm::Word(
        if a_bits <= SMALL_FMPZ_BITCOUNT_MAX && b_bits <= SMALL_FMPZ_BITCOUNT_MAX {
            WordMulAlgorithm::Small(if ar < 9 || ar + br < 20 {
                if c_bits <= SMALL_FMPZ_BITCOUNT_MAX {
                    SmallMulKind::One
                } else if c_bits < TWICE_WIDTH {
                    SmallMulKind::TwoA
                } else {
                    SmallMulKind::TwoB
                }
            } else {
                // FLINT uses Strassen's algorithm or the multimodular one for large dimensions.
                SmallMulKind::Internal(c_bits)
            })
        } else if a_bits + u64::from(sign) <= TWICE_WIDTH && b_bits + u64::from(sign) <= TWICE_WIDTH
        {
            // FLINT uses the multimodular algorithm for large dimensions.
            WordMulAlgorithm::DoubleWord(sign, c_bits)
        } else {
            // FLINT uses the multimodular algorithm, Waksman's, or Strassen's for some dimensions
            // and sizes.
            return MatrixMulAlgorithm::Classical;
        },
    )
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
// without BLAS, restricted to the algorithms ported so far.
crate_test_fn! {matrix_mul<C: PolynomialCoefficient>(
    xs: &[C],
    ys: &[C],
    shape: ProductShape,
) -> Vec<C> {
    match choose_matrix_mul_algorithm(xs, ys, shape) {
        MatrixMulAlgorithm::Zero => vec![C::ZERO; shape.ar * shape.bc],
        MatrixMulAlgorithm::Classical => matrix_mul_classical(xs, ys, shape),
        MatrixMulAlgorithm::Word(algorithm) => {
            let mut out = vec![C::ZERO; shape.ar * shape.bc];
            WordMul::borrowing(algorithm, xs).run(&mut out, ys, shape);
            out
        }
    }
}}

// Replaces the elements `xs` of the first factor of a product with those of the product, choosing
// an algorithm and reusing the storage of `xs`. The algorithms for small elements read the first
// matrix as words, which are copied out first, so the product can be written directly into `xs`.
//
// This is equivalent to `fmpz_mat_mul` from `fmpz_mat/mul.c`, FLINT 3.6.0, with one thread, without
// BLAS, and with the output the same as the first input, restricted to the algorithms ported so
// far.
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
    /// The product is written back into the storage of `self`. When every element fits in two
    /// limbs, the elements of `self` are first copied out as words, and the product is written
    /// directly; otherwise the product is computed one row at a time, so when it is no wider than
    /// `self`, and in particular when the second matrix is square, the only additional memory is
    /// one row.
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
    /// The product is written back into the storage of `self`. When every element fits in two
    /// limbs, the elements of `self` are first copied out as words, and the product is written
    /// directly; otherwise the product is computed one row at a time, so when it is no wider than
    /// `self`, and in particular when the second matrix is square, the only additional memory is
    /// one row.
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
