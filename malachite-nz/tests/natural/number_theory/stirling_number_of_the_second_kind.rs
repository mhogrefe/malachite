// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::num::number_theory::traits::{
    BellNumber, CheckedStirlingNumberOfTheSecondKind, StirlingNumberOfTheSecondKind,
};
use malachite_base::polynomial::Polynomial;
use malachite_base::polynomial::special::traits::TouchardPolynomialTruncated;
use malachite_base::test_util::generators::{
    unsigned_gen_var_5, unsigned_pair_gen_var_28, unsigned_pair_gen_var_54,
};
use malachite_nz::natural::Natural;
use malachite_nz::natural::number_theory::stirling_number_of_the_second_kind::{
    stirling_2_by_egf, stirling_2_by_multi_mod, stirling_2_by_powsum,
};
use malachite_nz::natural_polynomial::NaturalPolynomial;

// Checks S(n, k) against the default dispatch and, when 2 <= k <= n - 2, against each of the three
// general algorithms.
fn verify(n: u64, k: u64) -> Natural {
    let s = Natural::stirling_number_of_the_second_kind(n, k);
    assert!(s.is_valid());
    if k >= 2 && n >= 2 && k <= n - 2 {
        assert_eq!(stirling_2_by_powsum(n, k), s);
        assert_eq!(stirling_2_by_egf(n, k), s);
        assert_eq!(stirling_2_by_multi_mod(n, k), s);
    }
    s
}

#[test]
fn test_stirling_number_of_the_second_kind() {
    let test = |n, k, out: &str| {
        assert_eq!(verify(n, k).to_string(), out);
    };
    // (expected values computed independently with python)
    // - k > n
    test(3, 5, "0");
    test(0, 1, "0");
    // - k == n
    test(0, 0, "1");
    test(5, 5, "1");
    // - k == 0 < n
    test(5, 0, "0");
    // - k == 1 < n
    test(5, 1, "1");
    // - k == n - 1, with n even and odd
    test(6, 5, "15");
    test(5, 4, "10");
    // - k == 2 < n - 1
    test(5, 2, "15");
    test(100, 2, "633825300114114700748351602687");
    // - n <= 26: the one-word triangle, including both edges of the range
    test(5, 3, "25");
    test(10, 5, "42525");
    test(26, 3, "423610750290");
    test(26, 13, "1850568574253550060");
    test(26, 24, "47450");
    // - 26 < n <= 43: the two-word triangle, including both edges of the range
    test(27, 3, "1270865805301");
    test(27, 13, "29206898819153109600");
    test(43, 21, "517331687359579810443886364399481286");
    test(43, 41, "382571");
    // - 43 < n < 200: the power sum when k <= 0.9n, including k = 0.9n exactly
    test(44, 3, "164128474901175516606");
    test(44, 39, "3327037093380");
    test(
        100,
        50,
        "430983237009366340421514301547258695943520289614340613912441741131280319058853783145598261\
        659992013900",
    );
    test(100, 90, "692829506156717558646880139550");
    test(
        199,
        179,
        "2223812842617126351423145443474821076696412053331578777012003405258",
    );
    // - 43 < n < 200: the exponential generating function when k > 0.9n
    test(44, 40, "22810647783");
    test(44, 42, "420497");
    test(100, 91, "1807667046447663759097479740");
    test(
        199,
        180,
        "2962489312062730172038282351031162364558010185240697850169160500",
    );
    test(199, 197, "191467452");
    // - n >= 200: the power sum below the low cutoff (0.8n at n = 200)
    test(
        200,
        3,
        "442689981459791282231302203392966036680698866460706115581147807859341138891155336076676988\
        56646",
    );
    test(
        200,
        160,
        "123021820054213140368587083275260707681718981483945239147131474350081670678478464381419873\
        328027773599645565088723381658",
    );
    // - n >= 200: multi-mod between the cutoffs (161 through 189 at n = 200)
    test(
        200,
        161,
        "449228683724543160244139588784532627745782983462475092635880129199427872615651828277904842\
        871262221503491373209078365",
    );
    test(200, 189, "2289727105691680085214347215800042258000");
    // - n >= 200: the exponential generating function above the high cutoff
    test(200, 190, "1452971373681263576920687191333103600");
    test(200, 198, "195368250");
    // - extreme arguments with closed forms
    test(u64::MAX, u64::MAX, "1");
    test(u64::MAX, 1, "1");
    test(u64::MAX - 1, u64::MAX, "0");
    test(
        u64::MAX,
        u64::MAX - 1,
        "170141183460469231704017187605319778305",
    );
    test(
        u64::MAX - 1,
        u64::MAX - 2,
        "170141183460469231685570443531610226691",
    );
}

#[test]
fn test_stirling_number_of_the_second_kind_large() {
    // the values here run to thousands of digits, so they are checked against the coefficients of
    // the Touchard polynomial, which are computed by different algorithms. Only the dispatched
    // algorithm runs: the exponential generating function, for example, would form (n - k + 1)!^k,
    // which is far too large away from k close to n
    let test = |n, k| {
        assert_eq!(
            Natural::stirling_number_of_the_second_kind(n, k),
            *NaturalPolynomial::touchard_polynomial_truncated(n, k + 1).coefficient(k)
        );
    };
    // - 200 <= n < 3000: the power sum, multi-mod, and the exponential generating function
    test(1000, 500);
    test(1000, 800);
    test(1000, 990);
    // - n >= 3000: the low cutoff is 1500 / n
    test(3000, 1500);
    test(3000, 1501);
    test(3000, 2900);
}

#[test]
fn stirling_number_of_the_second_kind_properties() {
    unsigned_pair_gen_var_28::<u8, u8>().test_properties(|(n, k)| {
        let (n, k) = (u64::from(n), u64::from(k));
        let s = verify(n, k);
        assert_eq!(s == 0u32, k > n || k == 0 && n != 0);
        if n != 0 && k != 0 {
            // S(n, k) = kS(n - 1, k) + S(n - 1, k - 1)
            assert_eq!(
                Natural::stirling_number_of_the_second_kind(n - 1, k) * Natural::from(k)
                    + Natural::stirling_number_of_the_second_kind(n - 1, k - 1),
                s
            );
        }
        assert_eq!(
            *NaturalPolynomial::touchard_polynomial_truncated(n, k + 1).coefficient(k),
            s
        );
        // the primitive versions return a value exactly when it fits
        assert_eq!(
            u8::checked_stirling_number_of_the_second_kind(n, k).map(Natural::from),
            if s.significant_bits() <= u8::WIDTH {
                Some(s.clone())
            } else {
                None
            }
        );
        assert_eq!(
            u64::checked_stirling_number_of_the_second_kind(n, k).map(Natural::from),
            if s.significant_bits() <= u64::WIDTH {
                Some(s.clone())
            } else {
                None
            }
        );
        assert_eq!(
            u128::checked_stirling_number_of_the_second_kind(n, k).map(Natural::from),
            if s.significant_bits() <= u128::WIDTH {
                Some(s)
            } else {
                None
            }
        );
    });

    unsigned_gen_var_5::<u8>().test_properties(|n| {
        let n = u64::from(n);
        // the row of S(n, k) sums to the Bell number B(n)
        let sum: Natural = (0..=n)
            .map(|k| Natural::stirling_number_of_the_second_kind(n, k))
            .sum();
        assert_eq!(sum, Natural::bell_number(n));
    });
}

fn primitive_agreement_helper<T: PrimitiveUnsigned>()
where
    Natural: From<T>,
{
    unsigned_pair_gen_var_54::<T>().test_properties(|(n, k)| {
        assert_eq!(
            Natural::from(T::stirling_number_of_the_second_kind(n, k)),
            Natural::stirling_number_of_the_second_kind(n, k)
        );
    });
}

#[test]
fn stirling_number_of_the_second_kind_primitive_agreement() {
    primitive_agreement_helper::<u8>();
    primitive_agreement_helper::<u16>();
    primitive_agreement_helper::<u32>();
    primitive_agreement_helper::<u64>();
    primitive_agreement_helper::<u128>();
    primitive_agreement_helper::<usize>();
}
