// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::iterators::comparison::{is_strictly_ascending, is_strictly_descending};
use malachite_base::num::arithmetic::traits::{Abs, Floor, Parity};
use malachite_base::num::conversion::traits::IsInteger;
use malachite_base::strings::ToDebugString;
use malachite_nz::test_util::generators::integer_gen;
use malachite_q::conversion::traits::Convergents;
use malachite_q::test_util::conversion::continued_fraction::convergents::convergents_alt;
use malachite_q::test_util::generators::rational_gen;
use malachite_q::Rational;
use std::str::FromStr;

#[test]
fn test_convergents() {
    let test = |x: &str, out: &str| {
        let x = Rational::from_str(x).unwrap();
        let convergents = x.clone().convergents().collect_vec();
        assert!(convergents.iter().all(Rational::is_valid));
        assert_eq!((&x).convergents().collect_vec(), convergents);
        assert_eq!(convergents_alt(x).collect_vec(), convergents);
        assert_eq!(convergents.to_debug_string(), out);
    };
    test("0", "[0]");
    test("123", "[123]");
    test("-123", "[-123]");
    test("1/2", "[0, 1/2]");
    test("22/7", "[3, 22/7]");
    test("-22/7", "[-4, -3, -22/7]");
    test("99/100", "[0, 1, 99/100]");
    test(
        "936851431250/1397",
        "[670616629, 1341233259/2, 2011849888/3, 3353083147/5, 5364933035/8, 8718016182/13, \
        232033353767/346, 936851431250/1397]",
    );
    test(
        "6369051672525773/4503599627370496",
        "[1, 3/2, 7/5, 17/12, 41/29, 99/70, 239/169, 577/408, 1393/985, 3363/2378, 8119/5741, \
        19601/13860, 47321/33461, 114243/80782, 275807/195025, 665857/470832, 1607521/1136689, \
        3880899/2744210, 9369319/6625109, 22619537/15994428, 54608393/38613965, \
        77227930/54608393, 131836323/93222358, 209064253/147830751, 549964829/388883860, \
        4058818056/2870017771, 4608782885/3258901631, 13276383826/9387821033, \
        442729449143/313056995720, 898735282112/635501812473, 6733876423927/4761569683031, \
        34568117401747/24443350227628, 75870111227421/53648270138287, \
        110438228629168/78091620365915, 186308339856589/131739890504202, \
        3091371666334592/2185929868433147, 6369051672525773/4503599627370496]",
    );
    test(
        "884279719003555/281474976710656",
        "[3, 22/7, 333/106, 355/113, 103993/33102, 104348/33215, 208341/66317, 312689/99532, \
        833719/265381, 1146408/364913, 4272943/1360120, 5419351/1725033, 80143857/25510582, \
        245850922/78256779, 817696623/260280919, 1881244168/598818617, 2698940791/859099536, \
        9978066541/3176117225, 32633140414/10387451211, 238410049439/75888275702, \
        509453239292/162164002615, 747863288731/238052278317, 1257316528023/400216280932, \
        4519812872800/1438701121113, 10296942273623/3277618523158, \
        436991388364966/139098679093749, 884279719003555/281474976710656]",
    );
    test(
        "6121026514868073/2251799813685248",
        "[2, 3, 8/3, 11/4, 19/7, 87/32, 106/39, 193/71, 1264/465, 1457/536, 2721/1001, \
        23225/8544, 25946/9545, 49171/18089, 517656/190435, 566827/208524, 1084483/398959, \
        13580623/4996032, 14665106/5394991, 28245729/10391023, 325368125/119696244, \
        353613854/130087267, 678981979/249783511, 1032595833/379870778, 12037536142/4428362069, \
        61220276543/22521681123, 73257812685/26950043192, 134478089228/49471724315, \
        342213991141/125893491822, 476692080369/175365216137, 2248982312617/827354356370, \
        4974656705603/1830073928877, 7223639018220/2657428285247, 12198295723823/4487502214124, \
        117008300532627/43044948212363, 2001339404778482/736251621824295, \
        6121026514868073/2251799813685248]",
    );
}

#[test]
fn convergents_properties() {
    rational_gen().test_properties(|x| {
        let convergents = x.clone().convergents().collect_vec();
        assert!(convergents.iter().all(Rational::is_valid));
        assert_eq!(convergents[0], (&x).floor());
        assert_eq!(*convergents.last().unwrap(), x);
        assert_eq!((&x).convergents().collect_vec(), convergents);
        assert_eq!(convergents_alt(x.clone()).collect_vec(), convergents);

        // The denominators of the convergents are strictly increasing, with the single exception
        // that the first two convergents may both be integers.
        if let Some(i) = convergents.iter().position(|x| !x.is_integer()) {
            assert!(i == 1 || i == 2);
            assert!(is_strictly_ascending(
                convergents[i - 1..].iter().map(Rational::denominator_ref)
            ));
        }
        assert!(is_strictly_descending(
            convergents.iter().map(|c| (c - &x).abs())
        ));
        for (i, c) in convergents.iter().enumerate() {
            if i.even() {
                assert!(*c <= x);
            } else {
                assert!(*c >= x);
            }
        }
    });

    integer_gen().test_properties(|x| {
        let convergents = Rational::from(&x).convergents().collect_vec();
        assert_eq!(convergents, &[x]);
    });
}
