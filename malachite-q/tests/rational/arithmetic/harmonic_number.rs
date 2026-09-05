// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::generators::unsigned_gen_var_5;
use malachite_q::Rational;
use malachite_q::test_util::rational::arithmetic::harmonic_number::harmonic_number_naive;

#[test]
fn test_harmonic_number() {
    let test = |n, out| {
        assert_eq!(Rational::harmonic_number(n).to_string(), out);
    };
    // - values served from the table, including both edges
    test(0, "0");
    test(1, "1");
    test(2, "3/2");
    test(46, "5943339269060627227/1345655451257488800");
    // - the first computed value, one past the table
    test(47, "280682601097106968469/63245806209101973600");
    test(48, "282000222059796592919/63245806209101973600");
    // - a value large enough for word-accumulator flushes, but below the balanced split
    test(
        100,
        "14466636279520351160221518043104131447711/2788815009188499086581352357412492142272",
    );
}

#[test]
fn test_harmonic_number_large() {
    // - a value deep enough for several levels of balanced splitting; the expected value was
    //   computed independently with python fractions
    let h = Rational::harmonic_number(1000);
    assert_eq!(
        h.to_numerator().to_string(),
        "533629132822947850455910456240429804096524722803842600971013492484562688894971017575060979\
        019850356914090887315504680983784421721178850094643023443265660225021002784256328520814055\
        449412104425101426727702947747127089179639677796104532246924268664688882815820719848971051\
        107968732493191555293970175089315645199760857344730141832840117244122806490743077037366831\
        70055800293659235088589360235285852808160759574737836655413175508131522517"
    );
    assert_eq!(
        h.to_denominator().to_string(),
        "712886527466509305316638415571427292066835886188589304045200199115432408758111149947644415\
        191387158691171781701957525651298026406762100925146587100430513107268626814320019660997486\
        274593718834370501543445252373974529896314567498212823695623282379401106880926231770886197\
        954079124775455804932647573782992335275179673524804246363805113703433121478174685087845348\
        5678021888075373249921995672056932029099390891687487672697950931603520000"
    );
}

#[test]
fn test_harmonic_number_table_is_correct() {
    // The table holds the harmonic numbers 0 through 46; check every entry against term-by-term
    // summation, so that a corrupted table entry cannot hide behind the table lookup.
    for n in 0..47 {
        assert_eq!(
            Rational::harmonic_number(n),
            harmonic_number_naive(n),
            "table entry {n}"
        );
    }
}

#[test]
#[should_panic]
fn harmonic_number_fail() {
    Rational::harmonic_number(1 << 63);
}

#[test]
fn harmonic_number_properties() {
    // The naive reference costs n bignum additions with denominators of Theta(n) bits, so the
    // term-by-term cross-check runs only at u8 scale; the u16-scale pass exercises the fast path
    // alone, checked through the defining recurrence, which costs two fast evaluations.
    unsigned_gen_var_5::<u8>().test_properties(|n| {
        let n = u64::from(n);
        assert_eq!(Rational::harmonic_number(n), harmonic_number_naive(n));
    });
    unsigned_gen_var_5::<u16>().test_properties(|n| {
        let n = u64::from(n);
        let h = Rational::harmonic_number(n);
        assert!(h.is_valid());
        if n != 0 {
            // the defining recurrence
            assert_eq!(
                &h - Rational::harmonic_number(n - 1),
                Rational::from_unsigneds(1u64, n)
            );
            assert!(h >= 1u32);
        }
    });
}
