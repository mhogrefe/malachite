use core::cmp::Ordering::*;
use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{Determinant, DeterminantSign, Sign};
use malachite_base::num::basic::traits::Zero;
use malachite_nz::integer::Integer;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_q::rational_matrix::RationalMatrix;

fn main() {
    let a = IntegerMatrix::from_str("[1, 2; 3, 4]").unwrap();
    // The sign of the determinant: flagged.
    let _ = a.clone().determinant().sign();
    let _ = (&a).determinant().sign();
    let _ = Determinant::determinant(&a).sign();
    // The determinant compared with zero, by an operator: flagged.
    let _ = a.clone().determinant() > 0u32;
    let _ = (&a).determinant() == Integer::ZERO;
    let _ = 0u32 != (&a).determinant();
    // The determinant compared with zero, by a method: flagged.
    let _ = (&a).determinant().cmp(&Integer::ZERO);
    let _ = (&a).determinant().partial_cmp(&0u32);
    // Other matrix types that implement `DeterminantSign`: flagged.
    let n = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    let _ = (&n).determinant().sign();
    let q = RationalMatrix::from_str("[1/2, 2; 3, 4]").unwrap();
    let _ = (&q).determinant() < 0u32;
    // The determinant used for more than its sign: fine.
    let d = (&a).determinant();
    let _ = (d.to_string(), d.sign());
    // The determinant compared with a nonzero value: fine.
    let _ = (&a).determinant() > 1u32;
    // The suggested form: fine.
    let _ = a.determinant_sign() == Greater;
}
