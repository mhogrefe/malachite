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
    CheckedUnsignedStirlingNumberOfTheFirstKind, Factorial, UnsignedStirlingNumberOfTheFirstKind,
};
use malachite_base::polynomial::Polynomial;
use malachite_base::polynomial::special::traits::RisingFactorialPolynomialTruncated;
use malachite_base::test_util::generators::{
    unsigned_gen_var_5, unsigned_pair_gen_var_28, unsigned_pair_gen_var_55,
};
use malachite_nz::natural::Natural;
use malachite_nz::natural::number_theory::unsigned_stirling_number_of_the_first_kind::{
    stirling_1u_by_ogf, stirling_1u_by_reversed_ogf, stirling_1u_by_small_difference,
};
use malachite_nz::natural_polynomial::NaturalPolynomial;

// Checks [n, k] against the default dispatch; when 2 <= k < n <= 10000, against both ordinary
// generating functions, which hold about min(k, n - k) coefficients; and, when 1 <= n - k <= 300,
// against the second-order Eulerian numbers.
fn verify(n: u64, k: u64) -> Natural {
    let s = Natural::unsigned_stirling_number_of_the_first_kind(n, k);
    assert!(s.is_valid());
    if k >= 2 && k < n && n <= 10000 {
        assert_eq!(stirling_1u_by_ogf(n, k), s);
        assert_eq!(stirling_1u_by_reversed_ogf(n, k), s);
    }
    if k >= 1 && k < n && n - k <= 300 {
        assert_eq!(stirling_1u_by_small_difference(n, k), s);
    }
    s
}

#[test]
fn test_unsigned_stirling_number_of_the_first_kind() {
    let test = |n, k, out: &str| {
        assert_eq!(verify(n, k).to_string(), out);
    };
    // (expected values computed independently with python)
    // - k >= n
    test(3, 5, "0");
    test(0, 1, "0");
    test(0, 0, "1");
    test(5, 5, "1");
    // - k == 0 < n
    test(5, 0, "0");
    // - k == 1: [n, 1] = (n - 1)!
    test(5, 1, "24");
    test(30, 1, "8841761993739701954543616000000");
    // - n - k <= min(0.3n, 3.3n^(2/3)): the second-order Eulerian numbers, including n - k = 1 and
    //   the edge of the range at n = 100
    test(6, 5, "15");
    test(5, 4, "10");
    test(200, 199, "19900");
    test(20, 15, "973941900");
    test(
        100,
        70,
        "3831094491887515958025011340641050277364399396127460677965145835855648483400",
    );
    test(140, 130, "1348805876271310396860808336492680");
    test(
        141,
        122,
        "11699300711115338030224241605452950190045699558871649715000",
    );
    test(
        141,
        123,
        "27044113374677329418654741552742135713757538315161881500",
    );
    test(141, 139, "48246205");
    test(
        200,
        174,
        "1504747839735873692487446833355979814497485320600194254883289200200735625462369700200",
    );
    test(
        200,
        175,
        "2350565736456619022666529367008688770258033003219433945785192370053076231556650000",
    );
    test(300, 298, "1001283725");
    test(
        5000,
        4968,
        "417443332688324798613065796671920733020215061533970184358732615259290563317882295087164956\
        076103434044198213755967363687167845073220768817222956241177371933619434684940909911944766\
        431904134250",
    );
    test(
        u64::MAX,
        u64::MAX - 2,
        "14474011154664524422192363201981531115617445066701938474182899447357142728705",
    );
    test(
        u64::MAX,
        u64::MAX - 3,
        "820875129091551649677316846524412140452135980607530297299872870947888676140969258974806145\
        263421353760203498061825",
    );
    test(
        u64::MAX - 1,
        u64::MAX - 4,
        "820875129091551649410318467034298380345745082731863275292490019722127914675568672407230483\
        583208362632880823730191",
    );
    test(
        1000000000000000000,
        999999999999999995,
        "260416666666666661892361111111111146701388888888888750173611111111111412065972222222221870\
        225694444444444628472222222222222217708333333333333312500000000000000000000000000000000",
    );
    // - n - k too large for the Eulerian numbers, and k < n / 2: the ordinary generating function,
    //   (1 + x)(2 + x)...(n - 1 + x)
    test(10, 3, "1172700");
    test(10, 4, "723680");
    test(
        100,
        49,
        "121881597632336812519396125882028832517629189429434884328272143726700510566535603153959432\
        487918248137703001513750",
    );
    test(
        300,
        100,
        "376909250555303638611862421669595629299790553420938832632592924942890660751370250214457503\
        296322416916733897209447624680799370645144096776841719390488412801434329444287898264317052\
        120934511082840851466159631369481514328168584553866824352230755557700949439903033856394647\
        932264234370267124457115577994120519487925645526965247828284936443913270442392475687447437\
        060939940000305840240973447750195430127631008926094955368144735285714667403646205820279353\
        418227349682499248817684031195125487525657199631138816",
    );
    // - a product of 16 factors whose truncation the basecase expands whole in double words
    test(17, 3, "102992244837120");
    // - n - k too large for the Eulerian numbers, and n / 2 <= k: the reversed generating function,
    //   (1 + x)(1 + 2x)...(1 + (n - 1)x), including the edge of the range at n = 100
    test(5, 2, "50");
    test(10, 5, "269325");
    test(
        100,
        50,
        "318322278235296438474435412072968606417560943939705506371757866876922711307183638219873969\
        7421125692626030268475",
    );
    test(
        100,
        69,
        "378930257375323871254013261365519243906627318364048765487272900598767417730000",
    );
    test(
        141,
        70,
        "121521542962047757319509018500499265304128948008666309102309998508205173957177615171152143\
        96912019286989256086666925393717635655140975339916445271942733991682402196407750",
    );
    test(
        141,
        71,
        "218949888603059859288730491308432030121884924518788348219359329160087487774019846563746232\
        591015446640938556358462238521350251964749212149666512564713156547678290351575",
    );
    // - a product of 19 factors whose truncation the basecase expands whole in double words
    test(20, 10, "381922055502195");
    // - extreme arguments with closed forms
    test(u64::MAX, u64::MAX, "1");
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
fn test_unsigned_stirling_number_of_the_first_kind_large() {
    // the values here run to thousands of digits, so they are checked against the coefficients of
    // the rising factorial polynomial
    let test = |n, k| {
        assert_eq!(
            verify(n, k),
            *NaturalPolynomial::rising_factorial_polynomial_truncated(n, k + 1).coefficient(k)
        );
    };
    // - the ordinary generating function
    test(1000, 300);
    test(3000, 5);
    // - the reversed generating function
    test(1000, 600);
    // - the second-order Eulerian numbers
    test(1000, 900);
    test(3000, 2990);
}

#[test]
fn unsigned_stirling_number_of_the_first_kind_properties() {
    unsigned_pair_gen_var_28::<u8, u8>().test_properties(|(n, k)| {
        let (n, k) = (u64::from(n), u64::from(k));
        let s = verify(n, k);
        assert_eq!(s == 0u32, k > n || k == 0 && n != 0);
        if n != 0 && k != 0 {
            // [n, k] = (n - 1)[n - 1, k] + [n - 1, k - 1]
            assert_eq!(
                Natural::unsigned_stirling_number_of_the_first_kind(n - 1, k)
                    * Natural::from(n - 1)
                    + Natural::unsigned_stirling_number_of_the_first_kind(n - 1, k - 1),
                s
            );
        }
        assert_eq!(
            *NaturalPolynomial::rising_factorial_polynomial_truncated(n, k + 1).coefficient(k),
            s
        );
        // the primitive versions return a value exactly when it fits
        assert_eq!(
            u8::checked_unsigned_stirling_number_of_the_first_kind(n, k).map(Natural::from),
            if s.significant_bits() <= u8::WIDTH {
                Some(s.clone())
            } else {
                None
            }
        );
        assert_eq!(
            u64::checked_unsigned_stirling_number_of_the_first_kind(n, k).map(Natural::from),
            if s.significant_bits() <= u64::WIDTH {
                Some(s.clone())
            } else {
                None
            }
        );
        assert_eq!(
            u128::checked_unsigned_stirling_number_of_the_first_kind(n, k).map(Natural::from),
            if s.significant_bits() <= u128::WIDTH {
                Some(s)
            } else {
                None
            }
        );
    });

    unsigned_gen_var_5::<u8>().test_properties(|n| {
        let n = u64::from(n);
        // the row of [n, k] sums to n!
        let sum: Natural = (0..=n)
            .map(|k| Natural::unsigned_stirling_number_of_the_first_kind(n, k))
            .sum();
        assert_eq!(sum, Natural::factorial(n));
    });
}

fn primitive_agreement_helper<T: PrimitiveUnsigned>()
where
    Natural: From<T>,
{
    unsigned_pair_gen_var_55::<T>().test_properties(|(n, k)| {
        assert_eq!(
            Natural::from(T::unsigned_stirling_number_of_the_first_kind(n, k)),
            Natural::unsigned_stirling_number_of_the_first_kind(n, k)
        );
    });
}

#[test]
fn unsigned_stirling_number_of_the_first_kind_primitive_agreement() {
    primitive_agreement_helper::<u8>();
    primitive_agreement_helper::<u16>();
    primitive_agreement_helper::<u32>();
    primitive_agreement_helper::<u64>();
    primitive_agreement_helper::<u128>();
    primitive_agreement_helper::<usize>();
}
