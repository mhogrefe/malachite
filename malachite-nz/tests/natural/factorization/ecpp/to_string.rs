// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_nz::natural::factorization::ecpp::EcppCertificate;
use malachite_nz::test_util::generators::ecpp_certificate_natural_pair_gen;
use malachite_nz::test_util::natural::factorization::ecpp::ecpp_flint_fixture_strings;

#[test]
fn test_to_string() {
    let test = |s: &str, out: &str, out_pari: &str| {
        let cert = EcppCertificate::from_str(s).unwrap();
        assert_eq!(cert.to_string(), out);
        assert_eq!(cert.to_pari_string(), out_pari);
    };
    let flint = ecpp_flint_fixture_strings().swap_remove(0);
    test(
        &flint,
        &flint,
        "[[18446744073709551629, 4649078064, 89474922, 4837558977367648749, [7685973744855724010, 1\
            2593611317027034729]]]",
    );
    test(
        "[[68818587741598470525044465741, -523883721310323, 6684315, 13709682389390639021241088892\
            , [4543650699581189310901458620, 26860660642440259748266721942]], [10295533310683143210\
            451, 73075650452, 33011640000, 10256222990173086439768, [7414706640842905801292, 325414\
            1724577753829968]]]",
        "ecpp certificate, 2 steps\n[0] n = 68818587741598470525044465741\n    D = 0\n    a = 13709\
            682389390639021241088892\n    b = 32079317506793249522508881175\n    m = 68818587741598\
            994408765776065\n    q = 10295533310683143210451\n    x = 4543650699581189310901458620\
            \n    y = 26860660642440259748266721942\n[1] n = 10295533310683143210451\n    D = 0\
            \n    a = 10256222990173086439768\n    b = 10269326430343105363329\n    m = 10295533310\
            610067560000\n    q = 311875850779\n    x = 7414706640842905801292\n    y = 32541417245\
            77753829968\n",
        "[[68818587741598470525044465741, -523883721310323, 6684315, 13709682389390639021241088892\
            , [4543650699581189310901458620, 26860660642440259748266721942]], [10295533310683143210\
            451, 73075650452, 33011640000, 10256222990173086439768, [7414706640842905801292, 325414\
            1724577753829968]]]",
    );
    assert_eq!(
        EcppCertificate::default().to_string(),
        "ecpp certificate, 0 steps\n"
    );
    assert_eq!(EcppCertificate::default().to_pari_string(), "[]");
}

#[test]
fn to_string_properties() {
    // Printing in FLINT's format and reading back gives the same certificate; in the PARI/GP
    // format, the same one up to D and b, which that format derives
    ecpp_certificate_natural_pair_gen().test_properties(|(cert, _)| {
        let flint = cert.to_string();
        assert_eq!(EcppCertificate::from_str(&flint).as_ref(), Ok(&cert));
        let pari = cert.to_pari_string();
        let cert_2 = EcppCertificate::from_str(&pari).unwrap();
        assert_eq!(cert_2.to_pari_string(), pari);
        assert!(cert_2.steps.iter().all(|s| s.d == 0));
    });
}
