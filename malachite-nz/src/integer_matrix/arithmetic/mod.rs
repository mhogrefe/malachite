// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Implementations of [`Add`](core::ops::Add) and [`AddAssign`](core::ops::AddAssign), for adding
/// matrices of the same shape.
pub mod add;
crate_test_mod! {
    /// Adding a multiple of a matrix of words to an `IntegerMatrix`. Internal machinery for
    /// [`Solve`](malachite_base::num::arithmetic::traits::Solve), public only under `test_build`.
    add_mul_unsigned_matrix
}
/// Adding a multiple of one matrix to another, by implementations of
/// [`AddMul`](malachite_base::num::arithmetic::traits::AddMul) and
/// [`AddMulAssign`](malachite_base::num::arithmetic::traits::AddMulAssign).
///
/// # add_mul
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::AddMul;
/// use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
/// use malachite_nz::integer::Integer;
/// use malachite_nz::integer_matrix::IntegerMatrix;
///
/// assert_eq!(
///     IntegerMatrix::from_str("[]_{0,0}")
///         .unwrap()
///         .add_mul(
///             IntegerMatrix::from_str("[]_{0,0}").unwrap(),
///             Integer::from(5)
///         )
///         .to_string(),
///     "[]_{0,0}"
/// );
/// assert_eq!(
///     IntegerMatrix::from_str("[1, -2, 3]")
///         .unwrap()
///         .add_mul(
///             IntegerMatrix::from_str("[4, 5, -6]").unwrap(),
///             Integer::ZERO
///         )
///         .to_string(),
///     "[1, -2, 3]"
/// );
/// assert_eq!(
///     IntegerMatrix::from_str("[1, -2, 3]")
///         .unwrap()
///         .add_mul(IntegerMatrix::from_str("[4, 5, -6]").unwrap(), Integer::ONE)
///         .to_string(),
///     "[5, 3, -3]"
/// );
/// assert_eq!(
///     IntegerMatrix::from_str("[1, -2, 3]")
///         .unwrap()
///         .add_mul(
///             IntegerMatrix::from_str("[4, 5, -6]").unwrap(),
///             Integer::NEGATIVE_ONE
///         )
///         .to_string(),
///     "[-3, -7, 9]"
/// );
/// assert_eq!(
///     (&IntegerMatrix::from_str("[1, -2, 3]").unwrap())
///         .add_mul(
///             &IntegerMatrix::from_str("[4, 5, -6]").unwrap(),
///             &Integer::from(10)
///         )
///         .to_string(),
///     "[41, 48, -57]"
/// );
/// assert_eq!(
///     IntegerMatrix::from_str("[1, 0]")
///         .unwrap()
///         .add_mul(
///             &IntegerMatrix::from_str("[18446744073709551615, -1]").unwrap(),
///             &Integer::from_str("18446744073709551617").unwrap()
///         )
///         .to_string(),
///     "[340282366920938463463374607431768211456, -18446744073709551617]"
/// );
/// ```
///
/// # add_mul_assign
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::AddMulAssign;
/// use malachite_nz::integer::Integer;
/// use malachite_nz::integer_matrix::IntegerMatrix;
///
/// let mut a = IntegerMatrix::from_str("[1, -2, 3]").unwrap();
/// a.add_mul_assign(
///     IntegerMatrix::from_str("[4, 5, -6]").unwrap(),
///     Integer::from(10),
/// );
/// assert_eq!(a.to_string(), "[41, 48, -57]");
/// ```
pub mod add_mul;
/// Implementations of [`BalancedMod`](malachite_base::num::arithmetic::traits::BalancedMod) and
/// [`BalancedModAssign`](malachite_base::num::arithmetic::traits::BalancedModAssign), which reduce
/// every element of an [`IntegerMatrix`](super::IntegerMatrix) to the representative closest to
/// zero modulo a number.
pub mod balanced_mod;
/// Implementations of
/// [`BalancedModPowerOf2`](malachite_base::num::arithmetic::traits::BalancedModPowerOf2) and
/// [`BalancedModPowerOf2Assign`](
/// malachite_base::num::arithmetic::traits::BalancedModPowerOf2Assign), which reduce every element
/// of an [`IntegerMatrix`](super::IntegerMatrix) to the representative closest to zero modulo a
/// power of 2.
pub mod balanced_mod_power_of_2;
crate_test_mod! {
    /// Solving a linear system of any shape by Dixon's p-adic lifting. Internal machinery for
    /// malachite-q's `RationalMatrix` solvers, public only under `test_build`.
    can_solve_dixon
}
crate_test_mod! {
    /// Solving a linear system of any shape with the multimodular algorithm. Internal machinery for
    /// [`Solve`](malachite_base::num::arithmetic::traits::Solve), public only under `test_build`.
    can_solve_multi_mod
}
crate_test_mod! {
    /// Certifying, from an LU decomposition modulo a prime, the rank of an integer matrix. Internal
    /// machinery for [`Solve`](malachite_base::num::arithmetic::traits::Solve), public only under
    /// `test_build`.
    certify_lu_mod_p
}
/// Implementations of [`CanonicalPrimitivePart`](
/// malachite_base::num::arithmetic::traits::CanonicalPrimitivePart),
/// [`CanonicalPrimitivePartAssign`](
/// malachite_base::num::arithmetic::traits::CanonicalPrimitivePartAssign), and
/// [`ContentAndCanonicalPrimitivePart`](
/// malachite_base::num::arithmetic::traits::ContentAndCanonicalPrimitivePart), which compute the
/// primitive part of a matrix with its first nonzero element, in row-major order, made positive.
pub mod canonical_primitive_part;
/// Implementations of [`Content`](malachite_base::num::arithmetic::traits::Content),
/// [`PrimitivePart`](malachite_base::num::arithmetic::traits::PrimitivePart),
/// [`PrimitivePartAssign`](malachite_base::num::arithmetic::traits::PrimitivePartAssign), and
/// [`ContentAndPrimitivePart`](malachite_base::num::arithmetic::traits::ContentAndPrimitivePart),
/// which compute the GCD of a matrix's elements and the matrix divided by it.
pub mod content;
/// Implementations of [`ContentChained`](malachite_base::num::arithmetic::traits::ContentChained),
/// which computes the GCD of a number and a matrix's elements.
pub mod content_chained;
/// Implementations of [`BalancedCrt`](malachite_base::num::arithmetic::traits::BalancedCrt), which
/// combine a matrix of residues modulo any number with one modulo a word-sized number into balanced
/// representatives by the Chinese remainder theorem.
pub mod crt;
/// Implementations of [`Determinant`](malachite_base::num::arithmetic::traits::Determinant), for
/// computing the determinants of square integer matrices.
pub mod determinant;
/// The method [`determinant_bound`](super::IntegerMatrix::determinant_bound), an upper bound for
/// the absolute value of the determinant from Hadamard's inequality.
pub mod determinant_bound;
/// Implementations of
/// [`DeterminantSign`](malachite_base::num::arithmetic::traits::DeterminantSign), for determining
/// the signs of the determinants of square integer matrices.
pub mod determinant_sign;
crate_test_mod! {
    /// A divisor of the determinant of a square integer matrix, found by solving a linear system.
    /// Internal machinery for
    /// [`Determinant`](malachite_base::num::arithmetic::traits::Determinant), public only under
    /// `test_build`.
    determinant_divisor
}
/// Exact division of a matrix by a scalar, by implementations of
/// [`DivExact`](malachite_base::num::arithmetic::traits::DivExact) and
/// [`DivExactAssign`](malachite_base::num::arithmetic::traits::DivExactAssign).
///
/// # div_exact
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::DivExact;
/// use malachite_base::num::basic::traits::{NegativeOne, One};
/// use malachite_nz::integer::Integer;
/// use malachite_nz::integer_matrix::IntegerMatrix;
///
/// assert_eq!(
///     IntegerMatrix::from_str("[]_{2,0}")
///         .unwrap()
///         .div_exact(Integer::from(5))
///         .to_string(),
///     "[]_{2,0}"
/// );
/// assert_eq!(
///     IntegerMatrix::from_str("[0, -6; -255, -3]")
///         .unwrap()
///         .div_exact(Integer::ONE)
///         .to_string(),
///     "[0, -6; -255, -3]"
/// );
/// assert_eq!(
///     IntegerMatrix::from_str("[0, -6; -255, -3]")
///         .unwrap()
///         .div_exact(Integer::NEGATIVE_ONE)
///         .to_string(),
///     "[0, 6; 255, 3]"
/// );
/// assert_eq!(
///     (&IntegerMatrix::from_str("[0, -6; -255, -3]").unwrap())
///         .div_exact(&Integer::from(-3))
///         .to_string(),
///     "[0, 2; 85, 1]"
/// );
/// assert_eq!(
///     (&IntegerMatrix::from_str("[-3802951800684688204490109616128, 6; 0, 3]").unwrap())
///         .div_exact(Integer::from(3))
///         .to_string(),
///     "[-1267650600228229401496703205376, 2; 0, 1]"
/// );
/// ```
///
/// # div_exact_assign
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::DivExactAssign;
/// use malachite_nz::integer::Integer;
/// use malachite_nz::integer_matrix::IntegerMatrix;
///
/// let mut a = IntegerMatrix::from_str("[0, -6; -255, -3]").unwrap();
/// a.div_exact_assign(Integer::from(-3));
/// assert_eq!(a.to_string(), "[0, 2; 85, 1]");
/// ```
pub mod div_exact;
/// Implementations of
/// [`ElementaryDivisors`](malachite_base::num::arithmetic::traits::ElementaryDivisors), for
/// computing the elementary divisors of integer matrices.
pub mod elementary_divisors;
/// Left-shifting a matrix (multiplying it by a power of 2) and rounding every element, by
/// implementations of
/// [`EntrywiseShlRound`](malachite_base::num::arithmetic::traits::EntrywiseShlRound) and
/// [`EntrywiseShlRoundAssign`](malachite_base::num::arithmetic::traits::EntrywiseShlRoundAssign).
///
/// # entrywise_shl_round
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::EntrywiseShlRound;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use malachite_nz::integer_matrix::IntegerMatrix;
///
/// assert_eq!(
///     (IntegerMatrix::from_str("[-1, 2; -3, 5]")
///         .unwrap()
///         .entrywise_shl_round(-1i8, Floor))
///     .to_string(),
///     "[-1, 1; -2, 2]"
/// );
/// assert_eq!(
///     (IntegerMatrix::from_str("[-1, 2; -3, 5]")
///         .unwrap()
///         .entrywise_shl_round(-1i16, Down))
///     .to_string(),
///     "[0, 1; -1, 2]"
/// );
/// assert_eq!(
///     (&IntegerMatrix::from_str("[-1, 2; -3, 5]")
///         .unwrap()
///         .entrywise_shl_round(-1i32, Nearest))
///         .to_string(),
///     "[0, 1; -2, 2]"
/// );
/// assert_eq!(
///     (IntegerMatrix::from_str("[4, -8, 12]")
///         .unwrap()
///         .entrywise_shl_round(-2i64, Exact))
///     .to_string(),
///     "[1, -2, 3]"
/// );
/// assert_eq!(
///     (&IntegerMatrix::from_str("[1, -2, 3]")
///         .unwrap()
///         .entrywise_shl_round(2i8, Exact))
///         .to_string(),
///     "[4, -8, 12]"
/// );
/// ```
///
/// # entrywise_shl_round_assign
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::EntrywiseShlRoundAssign;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use malachite_nz::integer_matrix::IntegerMatrix;
///
/// let mut a = IntegerMatrix::from_str("[-1, 2; -3, 5]").unwrap();
/// a.entrywise_shl_round_assign(-1i8, Down);
/// assert_eq!(a.to_string(), "[0, 1; -1, 2]");
/// let mut a = IntegerMatrix::from_str("[1, -2, 3]").unwrap();
/// a.entrywise_shl_round_assign(2i32, Exact);
/// assert_eq!(a.to_string(), "[4, -8, 12]");
/// ```
pub mod entrywise_shl_round;
/// Right-shifting a matrix (dividing it by a power of 2) and rounding every element, by
/// implementations of
/// [`EntrywiseShrRound`](malachite_base::num::arithmetic::traits::EntrywiseShrRound) and
/// [`EntrywiseShrRoundAssign`](malachite_base::num::arithmetic::traits::EntrywiseShrRoundAssign).
///
/// # entrywise_shr_round
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::EntrywiseShrRound;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use malachite_nz::integer_matrix::IntegerMatrix;
///
/// assert_eq!(
///     (IntegerMatrix::from_str("[-1, 2; -3, 5]")
///         .unwrap()
///         .entrywise_shr_round(1u8, Floor))
///     .to_string(),
///     "[-1, 1; -2, 2]"
/// );
/// assert_eq!(
///     (IntegerMatrix::from_str("[-1, 2; -3, 5]")
///         .unwrap()
///         .entrywise_shr_round(1u16, Down))
///     .to_string(),
///     "[0, 1; -1, 2]"
/// );
/// assert_eq!(
///     (IntegerMatrix::from_str("[-1, 2; -3, 5]")
///         .unwrap()
///         .entrywise_shr_round(1u32, Ceiling))
///     .to_string(),
///     "[0, 1; -1, 3]"
/// );
/// assert_eq!(
///     (&IntegerMatrix::from_str("[-1, 2; -3, 5]")
///         .unwrap()
///         .entrywise_shr_round(1u64, Up))
///         .to_string(),
///     "[-1, 1; -2, 3]"
/// );
/// assert_eq!(
///     (IntegerMatrix::from_str("[-1, 2; -3, 5]")
///         .unwrap()
///         .entrywise_shr_round(1u8, Nearest))
///     .to_string(),
///     "[0, 1; -2, 2]"
/// );
/// assert_eq!(
///     (IntegerMatrix::from_str("[4, -8, 12]")
///         .unwrap()
///         .entrywise_shr_round(2u16, Exact))
///     .to_string(),
///     "[1, -2, 3]"
/// );
/// assert_eq!(
///     (IntegerMatrix::from_str("[-1, 2; -3, 5]")
///         .unwrap()
///         .entrywise_shr_round(1i8, Down))
///     .to_string(),
///     "[0, 1; -1, 2]"
/// );
/// assert_eq!(
///     (&IntegerMatrix::from_str("[1, -2, 3]")
///         .unwrap()
///         .entrywise_shr_round(-2i16, Exact))
///         .to_string(),
///     "[4, -8, 12]"
/// );
/// ```
///
/// # entrywise_shr_round_assign
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::EntrywiseShrRoundAssign;
/// use malachite_base::rounding_modes::RoundingMode::*;
/// use malachite_nz::integer_matrix::IntegerMatrix;
///
/// let mut a = IntegerMatrix::from_str("[-1, 2; -3, 5]").unwrap();
/// a.entrywise_shr_round_assign(1u8, Down);
/// assert_eq!(a.to_string(), "[0, 1; -1, 2]");
/// let mut a = IntegerMatrix::from_str("[-1, 2; -3, 5]").unwrap();
/// a.entrywise_shr_round_assign(1i32, Nearest);
/// assert_eq!(a.to_string(), "[0, 1; -2, 2]");
/// ```
pub mod entrywise_shr_round;
/// Implementations of [`FractionFreeLuDecomposition`][fflu], for computing a fraction-free LU
/// decomposition of a matrix.
///
/// [fflu]: malachite_base::num::arithmetic::traits::FractionFreeLuDecomposition
pub mod fraction_free_lu_decomposition;
#[doc(hidden)]
pub mod fraction_matrix;
/// Implementations of
/// [`HermiteNormalForm`](malachite_base::num::arithmetic::traits::HermiteNormalForm) and
/// [`HermiteNormalFormAssign`](malachite_base::num::arithmetic::traits::HermiteNormalFormAssign),
/// for computing the Hermite normal forms of integer matrices.
pub mod hermite_normal_form;
/// Implementations of [`HermiteNormalFormAndTransform`][hnft], for computing the Hermite normal
/// forms of integer matrices together with transformation matrices.
///
/// [hnft]: malachite_base::num::arithmetic::traits::HermiteNormalFormAndTransform
pub mod hermite_normal_form_and_transform;
crate_test_mod! {
    /// The algorithm of Pernet and Stein for computing Hermite normal forms. Internal machinery for
    /// [`HermiteNormalForm`](malachite_base::num::arithmetic::traits::HermiteNormalForm), public
    /// only under `test_build`.
    hermite_normal_form_pernet_stein
}
/// Implementations of [`Inverse`](malachite_base::num::arithmetic::traits::Inverse), for inverting
/// square integer matrices.
pub mod inverse;
/// The method [`is_in_hermite_normal_form`](super::IntegerMatrix::is_in_hermite_normal_form), which
/// checks whether a matrix is in Hermite normal form.
pub mod is_in_hermite_normal_form;
/// The method [`is_in_smith_normal_form`](super::IntegerMatrix::is_in_smith_normal_form), which
/// checks whether a matrix is in Smith normal form.
pub mod is_in_smith_normal_form;
crate_test_mod! {
    /// A check of whether a matrix is in fraction-free reduced row echelon form with a given
    /// denominator and rank. Internal machinery for the rank certificates of
    /// [`Solve`](malachite_base::num::arithmetic::traits::Solve) and a future reduced row echelon
    /// form, public only under `test_build`.
    is_in_reduced_row_echelon_form_with_rank
}
/// Implementations of
/// [`KroneckerProduct`](malachite_base::num::arithmetic::traits::KroneckerProduct) and
/// [`KroneckerProductAssign`](malachite_base::num::arithmetic::traits::KroneckerProductAssign), for
/// computing the Kronecker product of two [`IntegerMatrix`](super::IntegerMatrix)s.
pub mod kronecker_product;
/// Implementations of [`Mod`](malachite_base::num::arithmetic::traits::Mod),
/// [`Rem`](core::ops::Rem), and [`RemAssign`](core::ops::RemAssign), for reducing every element of
/// an [`IntegerMatrix`](super::IntegerMatrix) modulo a number.
pub mod mod_op;
/// Implementations of [`ModPowerOf2`](malachite_base::num::arithmetic::traits::ModPowerOf2),
/// [`RemPowerOf2`](malachite_base::num::arithmetic::traits::RemPowerOf2), and
/// [`RemPowerOf2Assign`](malachite_base::num::arithmetic::traits::RemPowerOf2Assign), for reducing
/// every element of an [`IntegerMatrix`](super::IntegerMatrix) modulo a power of 2.
pub mod mod_power_of_2;

/// Implementations of [`Mul`](core::ops::Mul) and [`MulAssign`](core::ops::MulAssign), for
/// multiplying two matrices.
pub mod mul;
/// Implementations of [`Mul`](core::ops::Mul) for multiplying an
/// [`IntegerMatrix`](super::IntegerMatrix) by an
/// [`IntegerVector`](crate::integer_vector::IntegerVector) on the right.
pub mod mul_vector;
/// [`IntegerMatrix::multi_balanced_crt`](super::IntegerMatrix::multi_balanced_crt), which combines
/// matrices of residues modulo many word-sized moduli into balanced representatives by the Chinese
/// remainder theorem.
pub mod multi_crt;
/// [`IntegerMatrix::multi_mod`](super::IntegerMatrix::multi_mod), which reduces a matrix modulo
/// many word-sized moduli.
pub mod multi_mod;
/// Implementations of [`Neg`](core::ops::Neg) and
/// [`NegAssign`](malachite_base::num::arithmetic::traits::NegAssign), for negating a matrix.
pub mod neg;

/// Implementations of [`Nullspace`](malachite_base::num::arithmetic::traits::Nullspace), for
/// finding bases of the right nullspaces of integer matrices.
pub mod nullspace;
/// Implementations of [`Permanent`](malachite_base::num::arithmetic::traits::Permanent), for
/// computing the permanents of square integer matrices.
pub mod permanent;
/// Implementations of [`Pow`](malachite_base::num::arithmetic::traits::Pow) and
/// [`PowAssign`](malachite_base::num::arithmetic::traits::PowAssign), for raising a square matrix
/// to a power.
pub mod pow;
/// Implementations of [`Rank`](malachite_base::num::arithmetic::traits::Rank), for finding the
/// ranks of integer matrices.
pub mod rank;
/// Implementations of
/// [`ReducedRowEchelonForm`](malachite_base::num::arithmetic::traits::ReducedRowEchelonForm) and
/// [`ReducedRowEchelonFormAssign`](
/// malachite_base::num::arithmetic::traits::ReducedRowEchelonFormAssign), for finding the
/// fraction-free reduced row echelon forms of integer matrices.
pub mod reduced_row_echelon_form;
/// Implementations of [`Mul`](core::ops::Mul) and [`MulAssign`](core::ops::MulAssign), for
/// multiplying an [`IntegerMatrix`](super::IntegerMatrix) by a scalar on either side.
pub mod scalar_mul;
/// Left-shifting a matrix (multiplying it by a power of 2), by implementations of
/// [`Shl`](core::ops::Shl) and [`ShlAssign`](core::ops::ShlAssign).
///
/// # shl
/// ```
/// use core::str::FromStr;
/// use malachite_nz::integer_matrix::IntegerMatrix;
///
/// assert_eq!(
///     (IntegerMatrix::from_str("[]_{0,0}").unwrap() << 10u8).to_string(),
///     "[]_{0,0}"
/// );
/// assert_eq!(
///     (IntegerMatrix::from_str("[1, -2, 3]").unwrap() << 0u16).to_string(),
///     "[1, -2, 3]"
/// );
/// assert_eq!(
///     (IntegerMatrix::from_str("[1, -2, 3]").unwrap() << 2u32).to_string(),
///     "[4, -8, 12]"
/// );
/// assert_eq!(
///     (&IntegerMatrix::from_str("[-3, 0]").unwrap() << 100u64).to_string(),
///     "[-3802951800684688204490109616128, 0]"
/// );
///
/// assert_eq!(
///     (IntegerMatrix::from_str("[1, -2, 3]").unwrap() << 2i8).to_string(),
///     "[4, -8, 12]"
/// );
///
/// assert_eq!(
///     (&IntegerMatrix::from_str("[-1, 2; -3, 5]").unwrap() << -1i64).to_string(),
///     "[-1, 1; -2, 2]"
/// );
/// ```
///
/// # shl_assign
/// ```
/// use core::str::FromStr;
/// use malachite_nz::integer_matrix::IntegerMatrix;
///
/// let mut a = IntegerMatrix::from_str("[1, -2, 3]").unwrap();
/// a <<= 2u32;
/// assert_eq!(a.to_string(), "[4, -8, 12]");
///
/// let mut a = IntegerMatrix::from_str("[-1, 2; -3, 5]").unwrap();
/// a <<= -1i32;
/// assert_eq!(a.to_string(), "[-1, 1; -2, 2]");
/// ```
pub mod shl;
/// Right-shifting a matrix (dividing it by a power of 2 and taking the floor), by implementations
/// of [`Shr`](core::ops::Shr) and [`ShrAssign`](core::ops::ShrAssign).
///
/// # shr
/// ```
/// use core::str::FromStr;
/// use malachite_nz::integer_matrix::IntegerMatrix;
///
/// assert_eq!(
///     (IntegerMatrix::from_str("[]_{0,0}").unwrap() >> 10u8).to_string(),
///     "[]_{0,0}"
/// );
/// assert_eq!(
///     (IntegerMatrix::from_str("[1, -2, 3]").unwrap() >> 0u16).to_string(),
///     "[1, -2, 3]"
/// );
/// assert_eq!(
///     (IntegerMatrix::from_str("[4, -8, 12]").unwrap() >> 2u32).to_string(),
///     "[1, -2, 3]"
/// );
/// assert_eq!(
///     (IntegerMatrix::from_str("[-1, 2; -3, 5]").unwrap() >> 1u8).to_string(),
///     "[-1, 1; -2, 2]"
/// );
/// assert_eq!(
///     (&IntegerMatrix::from_str("[-3802951800684688204490109616128, 0]").unwrap() >> 100u64)
///         .to_string(),
///     "[-3, 0]"
/// );
/// assert_eq!(
///     (IntegerMatrix::from_str("[1, -2, 3]").unwrap() >> -2i8).to_string(),
///     "[4, -8, 12]"
/// );
/// assert_eq!(
///     (&IntegerMatrix::from_str("[-1, 2; -3, 5]").unwrap() >> 1i64).to_string(),
///     "[-1, 1; -2, 2]"
/// );
/// ```
///
/// # shr_assign
/// ```
/// use core::str::FromStr;
/// use malachite_nz::integer_matrix::IntegerMatrix;
///
/// let mut a = IntegerMatrix::from_str("[-1, 2; -3, 5]").unwrap();
/// a >>= 1u32;
/// assert_eq!(a.to_string(), "[-1, 1; -2, 2]");
/// let mut a = IntegerMatrix::from_str("[1, -2, 3]").unwrap();
/// a >>= -2i32;
/// assert_eq!(a.to_string(), "[4, -8, 12]");
/// ```
pub mod shr;
/// Implementations of
/// [`SimilarityTransform`](malachite_base::num::arithmetic::traits::SimilarityTransform) and
/// [`SimilarityTransformAssign`](
/// malachite_base::num::arithmetic::traits::SimilarityTransformAssign),
/// for applying the similarity transform of FLINT's `fmpz_mat_similarity` to a square matrix.
pub mod similarity_transform;
/// Implementations of [`SmithNormalForm`](malachite_base::num::arithmetic::traits::SmithNormalForm)
/// and [`SmithNormalFormAssign`](malachite_base::num::arithmetic::traits::SmithNormalFormAssign),
/// for computing the Smith normal forms of integer matrices.
pub mod smith_normal_form;
/// Implementations of [`SmithNormalFormAndTransforms`](
/// malachite_base::num::arithmetic::traits::SmithNormalFormAndTransforms), for computing the Smith
/// normal forms of integer matrices together with transformation matrices.
pub mod smith_normal_form_and_transforms;
/// Implementations of [`Solve`](malachite_base::num::arithmetic::traits::Solve), for solving linear
/// systems over the rationals with integer matrices.
pub mod solve;
/// The method [`solve_bounds`](super::IntegerMatrix::solve_bounds), upper bounds for the numerators
/// and denominators of the solution of a linear system.
pub mod solve_bounds;
crate_test_mod! {
    /// Solving a square linear system by Dixon's p-adic lifting. Internal machinery for
    /// [`Solve`](malachite_base::num::arithmetic::traits::Solve), public only under `test_build`.
    solve_dixon
}
crate_test_mod! {
    /// Solving a square linear system with the multimodular algorithm. Internal machinery for
    /// [`Solve`](malachite_base::num::arithmetic::traits::Solve), public only under `test_build`.
    solve_multi_mod
}
/// Implementations of [`Square`](malachite_base::num::arithmetic::traits::Square) and
/// [`SquareAssign`](malachite_base::num::arithmetic::traits::SquareAssign), for squaring a square
/// matrix.
pub mod square;
/// Implementations of [`Sub`](core::ops::Sub) and [`SubAssign`](core::ops::SubAssign), for
/// subtracting matrices of the same shape.
pub mod sub;
/// Subtracting a multiple of one matrix from another, by implementations of
/// [`SubMul`](malachite_base::num::arithmetic::traits::SubMul) and
/// [`SubMulAssign`](malachite_base::num::arithmetic::traits::SubMulAssign).
///
/// # sub_mul
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::SubMul;
/// use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
/// use malachite_nz::integer::Integer;
/// use malachite_nz::integer_matrix::IntegerMatrix;
///
/// assert_eq!(
///     IntegerMatrix::from_str("[]_{0,0}")
///         .unwrap()
///         .sub_mul(
///             IntegerMatrix::from_str("[]_{0,0}").unwrap(),
///             Integer::from(5)
///         )
///         .to_string(),
///     "[]_{0,0}"
/// );
/// assert_eq!(
///     IntegerMatrix::from_str("[1, -2, 3]")
///         .unwrap()
///         .sub_mul(
///             IntegerMatrix::from_str("[4, 5, -6]").unwrap(),
///             Integer::ZERO
///         )
///         .to_string(),
///     "[1, -2, 3]"
/// );
/// assert_eq!(
///     IntegerMatrix::from_str("[1, -2, 3]")
///         .unwrap()
///         .sub_mul(IntegerMatrix::from_str("[4, 5, -6]").unwrap(), Integer::ONE)
///         .to_string(),
///     "[-3, -7, 9]"
/// );
/// assert_eq!(
///     IntegerMatrix::from_str("[1, -2, 3]")
///         .unwrap()
///         .sub_mul(
///             IntegerMatrix::from_str("[4, 5, -6]").unwrap(),
///             Integer::NEGATIVE_ONE
///         )
///         .to_string(),
///     "[5, 3, -3]"
/// );
/// assert_eq!(
///     (&IntegerMatrix::from_str("[1, -2, 3]").unwrap())
///         .sub_mul(
///             &IntegerMatrix::from_str("[4, 5, -6]").unwrap(),
///             &Integer::from(10)
///         )
///         .to_string(),
///     "[-39, -52, 63]"
/// );
/// assert_eq!(
///     IntegerMatrix::from_str("[1, 0]")
///         .unwrap()
///         .sub_mul(
///             &IntegerMatrix::from_str("[18446744073709551615, -1]").unwrap(),
///             &Integer::from_str("18446744073709551617").unwrap()
///         )
///         .to_string(),
///     "[-340282366920938463463374607431768211454, 18446744073709551617]"
/// );
/// ```
///
/// # sub_mul_assign
/// ```
/// use core::str::FromStr;
/// use malachite_base::num::arithmetic::traits::SubMulAssign;
/// use malachite_nz::integer::Integer;
/// use malachite_nz::integer_matrix::IntegerMatrix;
///
/// let mut a = IntegerMatrix::from_str("[1, -2, 3]").unwrap();
/// a.sub_mul_assign(
///     IntegerMatrix::from_str("[4, 5, -6]").unwrap(),
///     Integer::from(10),
/// );
/// assert_eq!(a.to_string(), "[-39, -52, 63]");
/// ```
pub mod sub_mul;
/// The method [`submatrix_determinant_bound`](super::IntegerMatrix::submatrix_determinant_bound),
/// an upper bound for the absolute values of the determinants of the square submatrices of a
/// matrix.
pub mod submatrix_determinant_bound;

/// An implementation of [`Trace`](malachite_base::num::arithmetic::traits::Trace), the sum of the
/// diagonal elements of a square matrix.
pub mod trace;
/// Implementations of [`Mul`](core::ops::Mul) for multiplying an
/// [`IntegerVector`](crate::integer_vector::IntegerVector) by an
/// [`IntegerMatrix`](super::IntegerMatrix) on the right.
pub mod vector_mul;
crate_test_mod! {
    /// LU decompositions of integer matrices modulo word-sized primes, and the solving of systems
    /// with them. Internal machinery for [`Solve`](malachite_base::num::arithmetic::traits::Solve),
    /// public only under `test_build`.
    word_lu
}
