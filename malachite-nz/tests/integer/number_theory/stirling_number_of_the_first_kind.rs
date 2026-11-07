// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Parity, PowerOf2, UnsignedAbs};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::num::number_theory::traits::{
    CheckedStirlingNumberOfTheFirstKind, StirlingNumberOfTheFirstKind,
    UnsignedStirlingNumberOfTheFirstKind,
};
use malachite_base::polynomial::Polynomial;
use malachite_base::polynomial::special::traits::FallingFactorialPolynomialTruncated;
use malachite_base::test_util::generators::{unsigned_pair_gen_var_28, unsigned_pair_gen_var_56};
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::natural::Natural;

#[test]
fn test_stirling_number_of_the_first_kind() {
    let test = |n, k, out: &str| {
        let s = Integer::stirling_number_of_the_first_kind(n, k);
        assert!(s.is_valid());
        assert_eq!(s.to_string(), out);
    };
    // (expected values computed independently with python)
    // - n + k even
    test(0, 0, "1");
    test(5, 5, "1");
    test(5, 1, "24");
    test(5, 3, "35");
    test(30, 2, "35027999979859805266492784640000");
    test(
        100,
        50,
        "318322278235296438474435412072968606417560943939705506371757866876922711307183638219873969\
        7421125692626030268475",
    );
    // - n + k odd
    test(5, 2, "-50");
    test(5, 4, "-10");
    test(6, 1, "-120");
    test(30, 1, "-8841761993739701954543616000000");
    test(
        100,
        49,
        "-12188159763233681251939612588202883251762918942943488432827214372670051056653560315395943\
        2487918248137703001513750",
    );
    // - zero values
    test(3, 5, "0");
    test(5, 0, "0");
    // - extreme arguments, where n - k is small
    test(
        u64::MAX,
        u64::MAX - 1,
        "-170141183460469231704017187605319778305",
    );
    test(
        u64::MAX,
        u64::MAX - 2,
        "14474011154664524422192363201981531115617445066701938474182899447357142728705",
    );
    test(
        u64::MAX,
        u64::MAX - 3,
        "-82087512909155164967731684652441214045213598060753029729987287094788867614096925897480614\
        5263421353760203498061825",
    );
}

#[test]
fn stirling_number_of_the_first_kind_properties() {
    unsigned_pair_gen_var_28::<u8, u8>().test_properties(|(n, k)| {
        let (n, k) = (u64::from(n), u64::from(k));
        let s = Integer::stirling_number_of_the_first_kind(n, k);
        assert!(s.is_valid());
        // the magnitude is the unsigned Stirling number of the first kind, and the sign is (-1)^(n
        // + k)
        assert_eq!(
            (&s).unsigned_abs(),
            Natural::unsigned_stirling_number_of_the_first_kind(n, k)
        );
        if s != 0u32 {
            assert_eq!(s > 0u32, n.even() == k.even());
        }
        assert_eq!(
            *IntegerPolynomial::falling_factorial_polynomial_truncated(n, k + 1).coefficient(k),
            s
        );
        if n != 0 && k != 0 {
            // s(n, k) = s(n - 1, k - 1) - (n - 1)s(n - 1, k)
            assert_eq!(
                Integer::stirling_number_of_the_first_kind(n - 1, k - 1)
                    - Integer::stirling_number_of_the_first_kind(n - 1, k) * Integer::from(n - 1),
                s
            );
        }
        // the primitive versions return a value exactly when it fits
        let fits =
            |width: u64| s.significant_bits() < width || s == -Integer::power_of_2(width - 1);
        assert_eq!(
            i8::checked_stirling_number_of_the_first_kind(n, k).map(Integer::from),
            if fits(i8::WIDTH) {
                Some(s.clone())
            } else {
                None
            }
        );
        assert_eq!(
            i128::checked_stirling_number_of_the_first_kind(n, k).map(Integer::from),
            if fits(i128::WIDTH) {
                Some(s.clone())
            } else {
                None
            }
        );
    });
}

fn primitive_agreement_helper<T: PrimitiveSigned>()
where
    Integer: From<T>,
{
    unsigned_pair_gen_var_56::<T>().test_properties(|(n, k)| {
        assert_eq!(
            Integer::from(T::stirling_number_of_the_first_kind(n, k)),
            Integer::stirling_number_of_the_first_kind(n, k)
        );
    });
}

#[test]
fn stirling_number_of_the_first_kind_primitive_agreement() {
    primitive_agreement_helper::<i8>();
    primitive_agreement_helper::<i16>();
    primitive_agreement_helper::<i32>();
    primitive_agreement_helper::<i64>();
    primitive_agreement_helper::<i128>();
    primitive_agreement_helper::<isize>();
}
