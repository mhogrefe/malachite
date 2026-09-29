// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::arithmetic::vec::dot_general::vec_dot_general;
use malachite_nz::test_util::generators::{
    integer_vec_integer_pair_gen, integer_vec_integer_vec_integer_triple_gen_var_1,
};
use malachite_nz::test_util::integer_polynomial::arithmetic::vec::dot_general::*;

fn parse(xs: &[&str]) -> Vec<Integer> {
    xs.iter().map(|x| Integer::from_str(x).unwrap()).collect()
}

#[test]
fn test_vec_dot_general() {
    let test = |initial: Option<&str>, subtract, xs: &[&str], ys: &[&str], reverse, out| {
        let initial = initial.map(|x| Integer::from_str(x).unwrap());
        let xs = parse(xs);
        let ys = parse(ys);
        let result = vec_dot_general(initial.as_ref(), subtract, &xs, &ys, reverse);
        assert!(result.is_valid());
        assert_eq!(result.to_string(), out);
        assert_eq!(
            vec_dot_general_naive(initial.as_ref(), subtract, &xs, &ys, reverse),
            result
        );
    };
    // - len <= 1 && initial.is_none() && len == 0
    test(None, false, &[], &[], false, "0");
    // - len <= 1 && initial.is_none() && len == 1
    test(None, false, &["3"], &["4"], false, "12");
    // - len <= 1 && initial.is_some()
    test(Some("5"), true, &[], &[], false, "5");
    // - len == 1 && subtract
    test(Some("1"), true, &["3"], &["4"], false, "-11");
    // - len == 1 && !subtract
    test(Some("1"), false, &["3"], &["4"], false, "13");
    // - is_small(ca) && is_small(cb)
    // - posn == 0 && negn == 0
    test(None, false, &["1", "2", "3"], &["4", "5", "6"], true, "28");
    // - an <= 2 && aneg ^ subtract
    // - an <= 2
    test(
        Some("-100"),
        false,
        &["1", "2", "3"],
        &["4", "5", "6"],
        false,
        "-68",
    );
    // - an > 2 && aneg ^ subtract
    // - s2 >> (Limb::WIDTH - 1) == 0
    // - negn > posn
    // - posn != 0 && negn != 0
    // - *sn == 0 in mpn_add
    test(
        Some("2014116069553517468766254461327952113019"),
        true,
        &["149365648", "0"],
        &["-24", "66955"],
        true,
        "2014116069553517468766254451327175151179",
    );
    // - an > 2 && !(aneg ^ subtract)
    // - negn == 0
    // - *sn >= bn in mpn_add
    test(
        Some("-11150372599224746770903368909310353045716991"),
        true,
        &["16383", "8"],
        &["-23", "8192"],
        true,
        "-11150372599224746770903368909310353179926343",
    );
    // - !(ptr::eq(ap.as_ptr(), bp.as_ptr()) && an == bn)
    // - posn == negn && pos >= neg
    test(
        None,
        false,
        &["18446744073709551616", "18446744073709551616"],
        &["18446744073709551616", "-18446744073709551616"],
        false,
        "0",
    );
    // Small products whose sum needs three limbs.
    test(
        None,
        false,
        &["4611686018427387903", "4611686018427387903", "4611686018427387903"],
        &["4611686018427387903", "4611686018427387903", "4611686018427387903"],
        false,
        "63802943797675961871712622782892212227",
    );
    test(
        None,
        false,
        &["-4611686018427387903", "-4611686018427387903"],
        &["4611686018427387903", "4611686018427387903"],
        false,
        "-42535295865117307914475081855261474818",
    );

    // A vector against itself, reversed, so that the middle element meets itself.
    let test_self = |xs: &[&str], out| {
        let xs = parse(xs);
        let result = vec_dot_general(None, false, &xs, &xs, true);
        assert!(result.is_valid());
        assert_eq!(result.to_string(), out);
        assert_eq!(vec_dot_general_naive(None, false, &xs, &xs, true), result);
    };
    // - bn == 1 && aneg ^ bneg
    // - posn == 0
    // - *sn >= an in mpn_addmul_1
    // - *sn > an in mpn_addmul_1
    // - *sn < an in mpn_addmul_1
    test_self(&["3", "-6234214638293007514"], "-37405287829758045084");
    // - bn == 1 && !(aneg ^ bneg)
    // - *sn < bn in mpn_add
    test_self(&["2", "8473997346921581167"], "33895989387686324668");
    // - ptr::eq(ap.as_ptr(), bp.as_ptr()) && an == bn
    test_self(
        &["7", "-36902495346673827855", "8"],
        "1361794162811283518282739369638173901137",
    );
    test_self(
        &["79246644863980482951", "-27053603762652800444"],
        "-4287814659339585233620934481079494460488",
    );
    // - s2 >> (Limb::WIDTH - 1) != 0
    // - posn > negn
    test_self(
        &["-64", "-5746593124524752896", "7"],
        "33023332538835162144180739809860385920",
    );
    test_self(
        &["2", "1", "-36285626321711999153"],
        "-145142505286847996611",
    );
    // - posn == negn && pos < neg
    test_self(
        &["128", "1", "-8040920777294848189", "50"],
        "-16081841554589683578",
    );
    test_self(&["6", "9135964658822477854"], "109631575905869734248");
    // - *sn < an && *sn != 0 in mpn_addmul_1
    test_self(
        &["7010074098069975854", "7038195", "-906406899259263184314", "-1"],
        "-12758937026684247891185698168",
    );
}

#[test]
fn vec_dot_general_properties() {
    integer_vec_integer_vec_integer_triple_gen_var_1().test_properties(|(xs, ys, initial)| {
        for initial in [None, Some(&initial)] {
            for subtract in [false, true] {
                for reverse in [false, true] {
                    let result = vec_dot_general(initial, subtract, &xs, &ys, reverse);
                    assert!(result.is_valid());
                    assert_eq!(
                        vec_dot_general_naive(initial, subtract, &xs, &ys, reverse),
                        result
                    );
                }
            }
        }
        let plain = vec_dot_general(None, false, &xs, &ys, false);
        // The initial value is added, and subtracting negates the dot product.
        assert_eq!(
            vec_dot_general(Some(&initial), false, &xs, &ys, false),
            &initial + &plain
        );
        assert_eq!(
            vec_dot_general(Some(&initial), true, &xs, &ys, false),
            &initial - &plain
        );
        // The dot product is symmetric, and reversing reads ys backwards.
        assert_eq!(vec_dot_general(None, false, &ys, &xs, false), plain);
        let mut reversed = ys.clone();
        reversed.reverse();
        assert_eq!(vec_dot_general(None, false, &xs, &reversed, true), plain);
    });

    integer_vec_integer_pair_gen().test_properties(|(xs, _)| {
        // A vector against itself, reversed, takes the squaring path for its middle element.
        let result = vec_dot_general(None, false, &xs, &xs, true);
        assert_eq!(vec_dot_general_naive(None, false, &xs, &xs, true), result);
        let square = vec_dot_general(None, false, &xs, &xs, false);
        assert!(square >= 0u32);
        assert_eq!(square == 0u32, xs.iter().all(|x| *x == 0u32));
    });
}
