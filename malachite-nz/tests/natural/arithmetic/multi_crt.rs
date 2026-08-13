// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Crt;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::strings::ToDebugString;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::natural::arithmetic::multi_crt::MultiCrt;
use malachite_nz::test_util::generators::{
    natural_quadruple_gen_var_5, natural_vec_pair_gen_var_1,
};
use malachite_nz::test_util::integer::arithmetic::crt::balanced_to_canonical;
use malachite_nz::test_util::natural::arithmetic::crt::multi_crt_simple;
use std::panic::catch_unwind;
use std::str::FromStr;

fn n(s: &str) -> Natural {
    Natural::from_str(s).unwrap()
}

fn ns(ss: &[&str]) -> Vec<Natural> {
    ss.iter().copied().map(n).collect()
}

#[test]
fn test_multi_crt() {
    let test = |ms: &[&str], vs: &[&str], out: &str, out_balanced: &str| {
        let ms = ns(ms);
        let vs = ns(vs);
        let crt = MultiCrt::new(&ms);
        match crt {
            None => {
                assert_eq!("None", out);
                assert_eq!("None", out_balanced);
                assert_eq!(Natural::multi_crt(&ms, &vs), None);
                assert_eq!(Integer::multi_balanced_crt(&ms, &vs), None);
            }
            Some(crt) => {
                assert_eq!(crt.moduli_count(), ms.len());
                assert_eq!(
                    crt.modulus(),
                    &ms.iter().fold(Natural::ONE, |acc, m| acc * m)
                );
                assert_eq!(Some(crt.apply(&vs)).to_debug_string(), out);
                assert_eq!(Natural::multi_crt(&ms, &vs).to_debug_string(), out);
                assert_eq!(
                    Some(crt.apply_balanced(&vs)).to_debug_string(),
                    out_balanced
                );
                assert_eq!(
                    Integer::multi_balanced_crt(&ms, &vs).to_debug_string(),
                    out_balanced
                );
                // The context is reusable.
                assert_eq!(Some(crt.apply(&vs)).to_debug_string(), out);
            }
        }
        if !ms.iter().any(|m| *m == 1u32) {
            assert_eq!(multi_crt_simple(&ms, &vs).to_debug_string(), out);
        }
    };
    // - a single modulus; the balanced representative of 3 mod 5 is -2
    test(&["5"], &["3"], "Some(3)", "Some(-2)");
    // - a single modulus of 1 is usable, unlike 1 among many
    test(&["1"], &["0"], "Some(0)", "Some(0)");
    // - a single zero modulus is not usable
    test(&["0"], &["0"], "None", "None");
    // - two moduli, matching the pair form; also the negative numerator in the exact division of
    //   the partial-fraction fill, which the coverage pass showed fires on most nodes
    test(&["3", "5"], &["2", "3"], "Some(8)", "Some(-7)");
    // - moduli in descending order exercise the pair-ordering swap in the partial-fraction fill
    test(&["5", "3"], &["2", "1"], "Some(7)", "Some(7)");
    // - three moduli
    test(&["3", "5", "7"], &["2", "3", "2"], "Some(23)", "Some(23)");
    // - equal residues short-circuit to themselves
    test(&["3", "5", "7"], &["1", "1", "1"], "Some(1)", "Some(1)");
    // - a balanced tie at exactly half the product stays positive
    test(&["2", "7"], &["1", "0"], "Some(7)", "Some(7)");
    // - a modulus of 1 among two or more is rejected
    test(&["3", "1"], &["2", "0"], "None", "None");
    test(&["1", "3"], &["0", "2"], "None", "None");
    // - a zero modulus among two or more is rejected
    test(&["0", "3"], &["0", "2"], "None", "None");
    // - moduli that are not coprime are rejected
    test(&["4", "6"], &["1", "3"], "None", "None");
    // - equal moduli are rejected
    test(&["5", "5"], &["1", "1"], "None", "None");
    // - ten moduli, exercising deeper trees and working-slot reuse
    test(
        &["3", "5", "7", "11", "13", "17", "19", "23", "29", "31"],
        &["2", "3", "5", "7", "11", "0", "1", "2", "3", "4"],
        "Some(45450662528)",
        "Some(45450662528)",
    );
    // - three multi-limb pairwise-coprime moduli (all beyond 2^64, so none is a single limb on
    //   64-bit builds)
    test(
        &[
            "98765432123456789012345678990",
            "12345678987654321012345678901",
            "36925814703692581470369258147",
        ],
        &["123456789", "987654321", "555555555"],
        "Some(24173524061642602673292786027546845360349606664511198754352958814553641734077176362\
        349)",
        "Some(-2085109369671423973351172270070369913577585151981584410238612520587921240985076358\
        6181)",
    );
}

#[test]
fn multi_crt_fail() {
    assert_panic!(MultiCrt::new(&[]));
    assert_panic!(Natural::multi_crt(&[], &[]));
    // one value per modulus
    assert_panic!({
        let crt = MultiCrt::new(&[Natural::from(3u32), Natural::from(5u32)]).unwrap();
        crt.apply(&[Natural::ONE])
    });
    // values must be reduced
    assert_panic!({
        let crt = MultiCrt::new(&[Natural::from(3u32), Natural::from(5u32)]).unwrap();
        crt.apply(&[Natural::from(3u32), Natural::ZERO])
    });
    assert_panic!({
        let crt = MultiCrt::new(&[Natural::from(3u32), Natural::from(5u32)]).unwrap();
        crt.apply_balanced(&[Natural::ZERO, Natural::from(5u32)])
    });
}

#[test]
fn multi_crt_properties() {
    natural_vec_pair_gen_var_1().test_properties(|(ms, vs)| {
        let crt = MultiCrt::new(&ms).unwrap();
        let p = ms.iter().fold(Natural::ONE, |acc, m| acc * m);
        assert_eq!(crt.modulus(), &p);
        assert_eq!(crt.moduli_count(), ms.len());

        let x = crt.apply(&vs);
        assert!(x < p);
        for (v, m) in vs.iter().zip(ms.iter()) {
            assert_eq!(&x % m, *v);
        }
        assert_eq!(Natural::multi_crt(&ms, &vs), Some(x.clone()));
        assert_eq!(multi_crt_simple(&ms, &vs), Some(x.clone()));
        assert_eq!(crt.apply(&vs), x);

        let y = crt.apply_balanced(&vs);
        assert_eq!(Integer::multi_balanced_crt(&ms, &vs), Some(y.clone()));
        let doubled = y.unsigned_abs_ref() << 1u64;
        if y >= 0u32 {
            assert!(doubled <= p);
        } else {
            assert!(doubled < p);
        }
        assert_eq!(balanced_to_canonical(&y, &p), x);

        // Appending a modulus sharing a factor with an existing one makes the list unusable.
        let mut bad = ms.clone();
        bad.push(ms[0].clone());
        assert!(MultiCrt::new(&bad).is_none());
    });

    natural_quadruple_gen_var_5().test_properties(|(r1, m1, r2, m2)| {
        // The two-modulus context agrees with the pair combination.
        let pair = (&r1).crt(&m1, &r2, &m2);
        if m1 != 1u32 && m2 != 1u32 {
            assert_eq!(Natural::multi_crt(&[m1, m2], &[r1, r2]), pair);
        }
    });
}
