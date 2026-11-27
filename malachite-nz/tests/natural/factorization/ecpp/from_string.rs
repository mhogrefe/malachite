// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::oracle_row;
use malachite_nz::natural::factorization::ecpp::EcppCertificate;
use malachite_nz::test_util::generators::ecpp_string_gen;

#[test]
fn test_from_str() {
    let test = |s: &str, out: &str| {
        let result = EcppCertificate::from_str(s).ok();
        oracle_row!(
            "flint",
            "ecpp_cert_set_str",
            "ecpp_cert_set_str({s:?}) = {result:?}"
        );
        assert_eq!(format!("{result:?}"), out);
    };
    // - a Pari-format certificate
    test(
        "[[18446744073709551629, 4649078064, 89474922, 4837558977367648749, [7685973744855724010, 1\
            2593611317027034729]]]",
        "Some(EcppCertificate { steps: [EcppStep { n: 18446744073709551629, d: 0, a: 48375589773676\
            48749, b: 5069635411140069722, m: 18446744069060473566, q: 206166640403, x: 76859737448\
            55724010, y: 12593611317027034729 }] })",
    );
    // - a Flint-format certificate
    test(
        "ecpp certificate, 1 steps\n[0] n = 18446744073709551629\n    D = -20\n    a = 483755897736\
            7648749\n    b = 5069635411140069722\n    m = 18446744069060473566\n    q = 20616664040\
            3\n    x = 7685973744855724010\n    y = 12593611317027034729\n",
        "Some(EcppCertificate { steps: [EcppStep { n: 18446744073709551629, d: -20, a: 483755897736\
            7648749, b: 5069635411140069722, m: 18446744069060473566, q: 206166640403, x: 768597374\
            4855724010, y: 12593611317027034729 }] })",
    );
    // - the empty string
    test("", "None");
    // - whitespace only
    test(" \n\t\r", "None");
    // - Pari: no closing bracket
    test("[", "None");
    // - Pari: no steps
    test("[]", "None");
    // - Flint: no steps
    test("ecpp certificate, 0 steps\n", "None");
    // - Pari: truncated
    test("[[12, 3, 4", "None");
    // - Flint: a field that is not a number
    test("ecpp certificate, 1 steps\n[0] n = x", "None");
    // - Pari: whitespace around every token, and text after the end
    test(
        " [ [ 18446744073709551629 ,\n4649078064 , 89474922 , 4837558977367648749 , [ 7685973744855\
            724010 , 12593611317027034729 ] ] ] junk",
        "Some(EcppCertificate { steps: [EcppStep { n: 18446744073709551629, d: 0, a: 48375589773676\
            48749, b: 5069635411140069722, m: 18446744069060473566, q: 206166640403, x: 76859737448\
            55724010, y: 12593611317027034729 }] })",
    );
    // - Pari: a leading +
    test(
        "[[18446744073709551629, 4649078064, +89474922, 4837558977367648749, [7685973744855724010, \
            12593611317027034729]]]",
        "None",
    );
    // - Pari: a, x, and y negative and unreduced
    test(
        "[[18446744073709551629, 4649078064, 89474922, -13609185096341902880, [26132717818565275639\
            , -5853132756682516900]]]",
        "Some(EcppCertificate { steps: [EcppStep { n: 18446744073709551629, d: 0, a: 48375589773676\
            48749, b: 5069635411140069722, m: 18446744069060473566, q: 206166640403, x: 76859737448\
            55724010, y: 12593611317027034729 }] })",
    );
    // - Pari: N < 3
    test("[[2, 0, 1, 0, [0, 0]]]", "None");
    // - Pari: s = 0
    test(
        "[[18446744073709551629, 4649078064, 0, 4837558977367648749, [7685973744855724010, 12593611\
            317027034729]]]",
        "None",
    );
    // - Pari: s does not divide m
    test(
        "[[18446744073709551629, 4649078064, 89474923, 4837558977367648749, [7685973744855724010, 1\
            2593611317027034729]]]",
        "None",
    );
    // - Pari: m < 0
    test(
        "[[18446744073709551629, 18446744073709551631, 89474922, 4837558977367648749, [768597374485\
            5724010, 12593611317027034729]]]",
        "None",
    );
    // - Pari: no comma between steps
    test(
        "[[18446744073709551629, 4649078064, 89474922, 4837558977367648749, [7685973744855724010, 1\
            2593611317027034729]] [18446744073709551629, 4649078064, 89474922, 4837558977367648749\
            , [7685973744855724010, 12593611317027034729]]]",
        "None",
    );
    // - Pari: no bracket after a comma
    test(
        "[[18446744073709551629, 4649078064, 89474922, 4837558977367648749, [7685973744855724010, 1\
            2593611317027034729]], 5]",
        "None",
    );
    // - Pari: no comma after N
    test(
        "[[18446744073709551629 4649078064, 89474922, 4837558977367648749, [7685973744855724010, 12\
            593611317027034729]]]",
        "None",
    );
    // - Pari: no bracket around the point
    test(
        "[[18446744073709551629, 4649078064, 89474922, 4837558977367648749, 7685973744855724010, 12\
            593611317027034729]]]",
        "None",
    );
    // - Pari: a missing field
    test("[[18446744073709551629, 4649078064, 89474922]]", "None");
    // - Flint: a negative field
    test(
        "ecpp certificate, 1 steps\n[0] n = 18446744073709551629\n    D = -20\n    a = -48375589773\
            67648749\n    b = 5069635411140069722\n    m = 18446744069060473566\n    q = 2061666404\
            03\n    x = 7685973744855724010\n    y = 12593611317027034729\n",
        "None",
    );
    // - Flint: D too large for 64 bits
    test(
        "ecpp certificate, 1 steps\n[0] n = 18446744073709551629\n    D = 9223372036854775808\n    \
            a = 4837558977367648749\n    b = 5069635411140069722\n    m = 18446744069060473566\
            \n    q = 206166640403\n    x = 7685973744855724010\n    y = 12593611317027034729\n",
        "None",
    );
    // - Flint: a missing label
    test(
        "ecpp certificate, 1 steps\n[0] n = 18446744073709551629\n    D = -20\n    a = 483755897736\
            7648749\n    b = 5069635411140069722\n    m = 18446744069060473566\n    q = 20616664040\
            3\n    X = 7685973744855724010\n    y = 12593611317027034729\n",
        "None",
    );
    // - Flint: unreduced fields are kept
    test(
        "ecpp certificate, 1 steps\n[0] n = 18446744073709551629\n    D = -20\n    a = 232843030510\
            77200378\n    b = 5069635411140069722\n    m = 18446744069060473566\n    q = 2061666404\
            03\n    x = 7685973744855724010\n    y = 12593611317027034729\n",
        "Some(EcppCertificate { steps: [EcppStep { n: 18446744073709551629, d: -20, a: 232843030510\
            77200378, b: 5069635411140069722, m: 18446744069060473566, q: 206166640403, x: 76859737\
            44855724010, y: 12593611317027034729 }] })",
    );
    // - Flint: two steps
    test(
        "ecpp certificate, 2 steps\n[0] n = 18446744073709551629\n    D = -20\n    a = 483755897736\
            7648749\n    b = 5069635411140069722\n    m = 18446744069060473566\n    q = 20616664040\
            3\n    x = 7685973744855724010\n    y = 12593611317027034729\n[1] n = 18446744073709551\
            629\n    D = -20\n    a = 4837558977367648749\n    b = 5069635411140069722\n    m = 184\
            46744069060473566\n    q = 206166640403\n    x = 7685973744855724010\n    y = 125936113\
            17027034729\n",
        "Some(EcppCertificate { steps: [EcppStep { n: 18446744073709551629, d: -20, a: 483755897736\
            7648749, b: 5069635411140069722, m: 18446744069060473566, q: 206166640403, x: 768597374\
            4855724010, y: 12593611317027034729 }, EcppStep { n: 18446744073709551629, d: -20, a: 4\
            837558977367648749, b: 5069635411140069722, m: 18446744069060473566, q: 206166640403, x\
            : 7685973744855724010, y: 12593611317027034729 }] })",
    );
}

#[test]
fn from_str_properties() {
    ecpp_string_gen().test_properties(|s| {
        if let Ok(cert) = EcppCertificate::from_str(&s) {
            assert!(!cert.steps.is_empty());
            assert_eq!(EcppCertificate::from_str(&cert.to_string()), Ok(cert));
        }
    });
}
