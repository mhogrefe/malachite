// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Abs, FloorLogBase, Pow, PowerOf2, RoundToMultiple};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::conversion::string::options::{
    FromSciStringOptions, SciSizeOptions, ToSciOptions,
};
use malachite_base::num::conversion::traits::{ExactFrom, FromSciString, ToSci};
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_base::strings::string_is_subset;
use malachite_base::test_util::num::conversion::string::from_sci_string::DECIMAL_SCI_STRING_CHARS;
use malachite_nz::test_util::generators::{integer_gen, integer_to_sci_options_pair_gen_var_1};
use malachite_q::conversion::string::to_sci::floor_log_base_of_abs;
use malachite_q::test_util::generators::{rational_gen, rational_to_sci_options_pair_gen_var_1};
use malachite_q::Rational;
use std::collections::HashMap;
use std::str::FromStr;

#[test]
pub fn test_to_sci() {
    assert_eq!(
        Rational::power_of_2(1000000u64).to_sci().to_string(),
        "9.900656229295898e301029"
    );
    assert_eq!(
        (-Rational::power_of_2(1000000u64)).to_sci().to_string(),
        "-9.900656229295898e301029"
    );
    assert_eq!(
        Rational::power_of_2(-1000000i64).to_sci().to_string(),
        "1.01003405919803e-301030"
    );
    assert_eq!(
        (-Rational::power_of_2(-1000000i64)).to_sci().to_string(),
        "-1.01003405919803e-301030"
    );

    fn test(s: &str, out: &str) {
        let x = Rational::from_str(s).unwrap();
        assert_eq!(x.to_sci().to_string(), out);
        assert_eq!(
            x.to_sci_with_options(ToSciOptions::default()).to_string(),
            out
        );
    }
    test("1/2", "0.5");
    test("1/3", "0.3333333333333333");
    test("1/4", "0.25");
    test("1/5", "0.2");
    test("1/6", "0.1666666666666667");
    test("1/7", "0.1428571428571429");
    test("1/8", "0.125");
    test("1/9", "0.1111111111111111");
    test("1/10", "0.1");
    // Still 16 significant digits
    test("1/11", "0.09090909090909091");

    test("1/137", "0.007299270072992701");
    test("22/7", "3.142857142857143");
    test("245850922/78256779", "3.141592653589793");
    test("936851431250/1397", "670616629.3843951");
    test("1/123456789", "8.100000073710001e-9");

    test("0", "0");
    test("1", "1");
    test("10", "10");
    test("100", "100");
    test("1000", "1000");
    test("10000", "10000");
    test("100000", "100000");
    test("1000000", "1000000");
    test("10000000", "10000000");
    test("100000000", "100000000");
    test("1000000000", "1000000000");
    test("10000000000", "10000000000");
    test("100000000000", "100000000000");
    test("1000000000000", "1000000000000");
    test("10000000000000", "10000000000000");
    test("100000000000000", "100000000000000");
    test("1000000000000000", "1000000000000000");
    test("10000000000000000", "1e16");
    test("100000000000000000", "1e17");
    test("1/10", "0.1");
    test("1/100", "0.01");
    test("1/1000", "0.001");
    test("1/10000", "0.0001");
    test("1/100000", "0.00001");
    test("1/1000000", "1e-6");
    test("1/10000000", "1e-7");
    test("1/100000000", "1e-8");

    test("999999999999999", "999999999999999");
    test("9999999999999999", "9999999999999999");
    test("99999999999999999", "1e17");
    test("999999999999999999", "1e18");

    test("-1", "-1");
    test("-10", "-10");
    test("-100", "-100");
    test("-1000", "-1000");
    test("-10000", "-10000");
    test("-100000", "-100000");
    test("-1000000", "-1000000");
    test("-10000000", "-10000000");
    test("-100000000", "-100000000");
    test("-1000000000", "-1000000000");
    test("-10000000000", "-10000000000");
    test("-100000000000", "-100000000000");
    test("-1000000000000", "-1000000000000");
    test("-10000000000000", "-10000000000000");
    test("-100000000000000", "-100000000000000");
    test("-1000000000000000", "-1000000000000000");
    test("-10000000000000000", "-1e16");
    test("-100000000000000000", "-1e17");
    test("-1/10", "-0.1");
    test("-1/100", "-0.01");
    test("-1/1000", "-0.001");
    test("-1/10000", "-0.0001");
    test("-1/100000", "-0.00001");
    test("-1/1000000", "-1e-6");
    test("-1/10000000", "-1e-7");
    test("-1/100000000", "-1e-8");
}

#[test]
pub fn test_to_sci_with_options() {
    fn test_i(x: &Rational, options: ToSciOptions, out: &str) {
        assert_eq!(x.to_sci_with_options(options).to_string(), out);
    }
    fn test(s: &str, options: ToSciOptions, out: &str) {
        test_i(&Rational::from_str(s).unwrap(), options, out);
    }
    // For tests with the default options, see `test_to_sci`

    let mut options = ToSciOptions::default();
    options.set_include_trailing_zeros(true);
    test("0", options, "0.000000000000000");
    test("1", options, "1.000000000000000");
    test("10", options, "10.00000000000000");
    test("100", options, "100.0000000000000");
    test("1000", options, "1000.000000000000");
    test("10000", options, "10000.00000000000");
    test("100000", options, "100000.0000000000");
    test("1000000", options, "1000000.000000000");
    test("10000000", options, "10000000.00000000");
    test("100000000", options, "100000000.0000000");
    test("1000000000", options, "1000000000.000000");
    test("10000000000", options, "10000000000.00000");
    test("100000000000", options, "100000000000.0000");
    test("1000000000000", options, "1000000000000.000");
    test("10000000000000", options, "10000000000000.00");
    test("100000000000000", options, "100000000000000.0");
    test("1000000000000000", options, "1000000000000000");
    test("10000000000000000", options, "1.000000000000000e16");
    test("100000000000000000", options, "1.000000000000000e17");
    test_i(&Rational::from(u64::MAX), options, "1.844674407370955e19");
    test_i(&Rational::from(u128::MAX), options, "3.402823669209385e38");

    test("999999999999999", options, "999999999999999.0");
    test("9999999999999999", options, "9999999999999999");
    test("99999999999999999", options, "1.000000000000000e17");
    test("999999999999999999", options, "1.000000000000000e18");

    options = ToSciOptions::default();
    options.set_base(2);
    test_i(&Rational::from(u128::MAX), options, "1e128");
    options.set_base(3);
    test_i(&Rational::from(u128::MAX), options, "2.022011021210021e80");
    options.set_base(4);
    test_i(&Rational::from(u128::MAX), options, "1e64");
    options.set_base(5);
    test_i(&Rational::from(u128::MAX), options, "1.103111044120131e55");
    options.set_base(8);
    test_i(&Rational::from(u128::MAX), options, "4e42");
    // When base >= 15, there is a mandatory sign after the exponent indicator "e", to distinguish
    // it from the digit "e"
    options.set_base(16);
    test_i(&Rational::from(u128::MAX), options, "1e+32");
    options.set_base(32);
    test_i(&Rational::from(u128::MAX), options, "8e+25");
    options.set_base(36);
    test_i(&Rational::from(u128::MAX), options, "f.5lxx1zz5pnorynqe+24");

    // The sign can be forced in other cases too
    options.set_base(3);
    options.set_force_exponent_plus_sign(true);
    test_i(&Rational::from(u128::MAX), options, "2.022011021210021e+80");

    // The digits can be uppercase, and so can the exponent indicator
    options = ToSciOptions::default();
    options.set_base(36);
    options.set_uppercase();
    test_i(&Rational::from(u128::MAX), options, "F.5LXX1ZZ5PNORYNQe+24");

    options.set_lowercase();
    options.set_e_uppercase();
    test_i(&Rational::from(u128::MAX), options, "f.5lxx1zz5pnorynqE+24");

    options.set_uppercase();
    test_i(&Rational::from(u128::MAX), options, "F.5LXX1ZZ5PNORYNQE+24");

    options = ToSciOptions::default();
    options.set_size_complete();
    options.set_base(2);
    test_i(
        &Rational::from(u128::MAX),
        options,
        "11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111\
        111111111111111111111111111111111111111",
    );
    options.set_base(3);
    test_i(
        &Rational::from(u128::MAX),
        options,
        "202201102121002021012000211012011021221022212021111001022110211020010021100121010",
    );
    options.set_base(4);
    test_i(
        &Rational::from(u128::MAX),
        options,
        "3333333333333333333333333333333333333333333333333333333333333333",
    );
    options.set_base(5);
    test_i(
        &Rational::from(u128::MAX),
        options,
        "11031110441201303134210404233413032443021130230130231310",
    );
    options.set_base(8);
    test_i(
        &Rational::from(u128::MAX),
        options,
        "3777777777777777777777777777777777777777777",
    );
    options.set_base(16);
    test_i(
        &Rational::from(u128::MAX),
        options,
        "ffffffffffffffffffffffffffffffff",
    );
    options.set_base(32);
    test_i(
        &Rational::from(u128::MAX),
        options,
        "7vvvvvvvvvvvvvvvvvvvvvvvvv",
    );
    options.set_base(36);
    test_i(
        &Rational::from(u128::MAX),
        options,
        "f5lxx1zz5pnorynqglhzmsp33",
    );

    options = ToSciOptions::default();
    options.set_precision(4);
    options.set_include_trailing_zeros(true);
    test("0", options, "0.000");
    test("1", options, "1.000");
    test("10", options, "10.00");
    test("100", options, "100.0");
    test("1000", options, "1000");
    test("10000", options, "1.000e4");
    test("9", options, "9.000");
    test("99", options, "99.00");
    test("999", options, "999.0");
    test("9999", options, "9999");
    test("99999", options, "1.000e5");

    options.set_include_trailing_zeros(false);
    test("0", options, "0");
    test("1", options, "1");
    test("10", options, "10");
    test("100", options, "100");
    test("1000", options, "1000");
    test("10000", options, "1e4");
    test("9", options, "9");
    test("99", options, "99");
    test("999", options, "999");
    test("9999", options, "9999");
    test("99999", options, "1e5");

    options = ToSciOptions::default();
    options.set_precision(1);
    options.set_include_trailing_zeros(true); // doesn't matter when precision is 1
    test("0", options, "0");
    test("1", options, "1");
    test("10", options, "1e1");
    test("100", options, "1e2");
    test("1000", options, "1e3");
    test("10000", options, "1e4");
    test("9", options, "9");
    test("99", options, "1e2");
    test("999", options, "1e3");
    test("9999", options, "1e4");
    test("99999", options, "1e5");

    options.set_include_trailing_zeros(false);
    test("0", options, "0");
    test("1", options, "1");
    test("10", options, "1e1");
    test("100", options, "1e2");
    test("1000", options, "1e3");
    test("10000", options, "1e4");
    test("9", options, "9");
    test("99", options, "1e2");
    test("999", options, "1e3");
    test("9999", options, "1e4");
    test("99999", options, "1e5");

    options = ToSciOptions::default();
    options.set_scale(2);
    options.set_include_trailing_zeros(true);
    test("0", options, "0.00");
    test("1", options, "1.00");
    test("10", options, "10.00");
    test("100", options, "100.00");
    test("1000", options, "1000.00");
    test("10000", options, "10000.00");
    test("9", options, "9.00");
    test("99", options, "99.00");
    test("999", options, "999.00");
    test("9999", options, "9999.00");
    test("99999", options, "99999.00");

    options.set_include_trailing_zeros(false);
    test("0", options, "0");
    test("1", options, "1");
    test("10", options, "10");
    test("100", options, "100");
    test("1000", options, "1000");
    test("10000", options, "10000");
    test("9", options, "9");
    test("99", options, "99");
    test("999", options, "999");
    test("9999", options, "9999");
    test("99999", options, "99999");

    options = ToSciOptions::default();
    options.set_scale(0);
    options.set_include_trailing_zeros(true); // doesn't matter when scale is 0
    test("0", options, "0");
    test("1", options, "1");
    test("10", options, "10");
    test("100", options, "100");
    test("1000", options, "1000");
    test("10000", options, "10000");
    test("9", options, "9");
    test("99", options, "99");
    test("999", options, "999");
    test("9999", options, "9999");
    test("99999", options, "99999");

    options.set_include_trailing_zeros(false);
    test("0", options, "0");
    test("1", options, "1");
    test("10", options, "10");
    test("100", options, "100");
    test("1000", options, "1000");
    test("10000", options, "10000");
    test("9", options, "9");
    test("99", options, "99");
    test("999", options, "999");
    test("9999", options, "9999");
    test("99999", options, "99999");

    options = ToSciOptions::default();
    options.set_precision(2);
    options.set_rounding_mode(Nearest); // This is the default
    test("123", options, "1.2e2");
    options.set_rounding_mode(Down);
    test("123", options, "1.2e2");
    options.set_rounding_mode(Floor);
    test("123", options, "1.2e2");
    options.set_rounding_mode(Up);
    test("123", options, "1.3e2");
    options.set_rounding_mode(Ceiling);
    test("123", options, "1.3e2");

    options.set_rounding_mode(Nearest);
    test("135", options, "1.4e2");
    options.set_rounding_mode(Down);
    test("135", options, "1.3e2");
    options.set_rounding_mode(Floor);
    test("135", options, "1.3e2");
    options.set_rounding_mode(Up);
    test("135", options, "1.4e2");
    options.set_rounding_mode(Ceiling);
    test("135", options, "1.4e2");

    options.set_rounding_mode(Exact);
    test("140", options, "1.4e2");

    options.set_rounding_mode(Nearest);
    test("999", options, "1e3");
    options.set_rounding_mode(Down);
    test("999", options, "9.9e2");
    options.set_rounding_mode(Floor);
    test("999", options, "9.9e2");
    options.set_rounding_mode(Up);
    test("999", options, "1e3");
    options.set_rounding_mode(Ceiling);
    test("999", options, "1e3");

    let mut options = ToSciOptions::default();
    options.set_include_trailing_zeros(true);
    test_i(&Rational::from(i64::MAX), options, "9.223372036854776e18");
    test_i(&Rational::from(i128::MAX), options, "1.701411834604692e38");
    test("-1", options, "-1.000000000000000");
    test("-10", options, "-10.00000000000000");
    test("-100", options, "-100.0000000000000");
    test("-1000", options, "-1000.000000000000");
    test("-10000", options, "-10000.00000000000");
    test("-100000", options, "-100000.0000000000");
    test("-1000000", options, "-1000000.000000000");
    test("-10000000", options, "-10000000.00000000");
    test("-100000000", options, "-100000000.0000000");
    test("-1000000000", options, "-1000000000.000000");
    test("-10000000000", options, "-10000000000.00000");
    test("-100000000000", options, "-100000000000.0000");
    test("-1000000000000", options, "-1000000000000.000");
    test("-10000000000000", options, "-10000000000000.00");
    test("-100000000000000", options, "-100000000000000.0");
    test("-1000000000000000", options, "-1000000000000000");
    test("-10000000000000000", options, "-1.000000000000000e16");
    test("-100000000000000000", options, "-1.000000000000000e17");
    test_i(&Rational::from(i64::MIN), options, "-9.223372036854776e18");
    test_i(&Rational::from(i128::MIN), options, "-1.701411834604692e38");

    test("-999999999999999", options, "-999999999999999.0");
    test("-9999999999999999", options, "-9999999999999999");
    test("-99999999999999999", options, "-1.000000000000000e17");
    test("-999999999999999999", options, "-1.000000000000000e18");

    options = ToSciOptions::default();
    options.set_base(2);
    test_i(&Rational::from(i128::MAX), options, "1e127");
    test_i(&Rational::from(i128::MIN), options, "-1e127");
    options.set_base(3);
    test_i(&Rational::from(i128::MAX), options, "1.01100201022001e80");
    test_i(&Rational::from(i128::MIN), options, "-1.01100201022001e80");
    options.set_base(4);
    test_i(&Rational::from(i128::MAX), options, "2e63");
    test_i(&Rational::from(i128::MIN), options, "-2e63");
    options.set_base(5);
    test_i(&Rational::from(i128::MAX), options, "3.013030220323124e54");
    test_i(&Rational::from(i128::MIN), options, "-3.013030220323124e54");
    options.set_base(8);
    test_i(&Rational::from(i128::MAX), options, "2e42");
    test_i(&Rational::from(i128::MIN), options, "-2e42");
    // When base >= 15, there is a mandatory sign after the exponent indicator "e", to distinguish
    // it from the digit "e"
    options.set_base(16);
    test_i(&Rational::from(i128::MAX), options, "8e+31");
    test_i(&Rational::from(i128::MIN), options, "-8e+31");
    options.set_base(32);
    test_i(&Rational::from(i128::MAX), options, "4e+25");
    test_i(&Rational::from(i128::MIN), options, "-4e+25");
    options.set_base(36);
    test_i(&Rational::from(i128::MAX), options, "7.ksyyizzkutudzbve+24");
    test_i(
        &Rational::from(i128::MIN),
        options,
        "-7.ksyyizzkutudzbve+24",
    );

    // The sign can be forced in other cases too
    options.set_base(3);
    options.set_force_exponent_plus_sign(true);
    test_i(&Rational::from(i128::MAX), options, "1.01100201022001e+80");
    test_i(&Rational::from(i128::MIN), options, "-1.01100201022001e+80");

    // The digits can be uppercase, and so can the exponent indicator
    options = ToSciOptions::default();
    options.set_base(36);
    options.set_uppercase();
    test_i(&Rational::from(i128::MAX), options, "7.KSYYIZZKUTUDZBVe+24");
    test_i(
        &Rational::from(i128::MIN),
        options,
        "-7.KSYYIZZKUTUDZBVe+24",
    );

    options.set_lowercase();
    options.set_e_uppercase();
    test_i(&Rational::from(i128::MAX), options, "7.ksyyizzkutudzbvE+24");
    test_i(
        &Rational::from(i128::MIN),
        options,
        "-7.ksyyizzkutudzbvE+24",
    );

    options.set_uppercase();
    test_i(&Rational::from(i128::MAX), options, "7.KSYYIZZKUTUDZBVE+24");
    test_i(
        &Rational::from(i128::MIN),
        options,
        "-7.KSYYIZZKUTUDZBVE+24",
    );

    options = ToSciOptions::default();
    options.set_size_complete();
    options.set_base(2);
    test_i(
        &Rational::from(i128::MAX),
        options,
        "11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111\
        11111111111111111111111111111111111111",
    );
    test_i(
        &Rational::from(i128::MIN),
        options,
        "-1000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        0000000000000000000000000000000000000000",
    );
    options.set_base(3);
    test_i(
        &Rational::from(i128::MAX),
        options,
        "101100201022001010121000102002120122110122221010202000122201220121120010200022001",
    );
    test_i(
        &Rational::from(i128::MIN),
        options,
        "-101100201022001010121000102002120122110122221010202000122201220121120010200022002",
    );
    options.set_base(4);
    test_i(
        &Rational::from(i128::MAX),
        options,
        "1333333333333333333333333333333333333333333333333333333333333333",
    );
    test_i(
        &Rational::from(i128::MIN),
        options,
        "-2000000000000000000000000000000000000000000000000000000000000000",
    );
    options.set_base(5);
    test_i(
        &Rational::from(i128::MAX),
        options,
        "3013030220323124042102424341431241221233040112312340402",
    );
    test_i(
        &Rational::from(i128::MIN),
        options,
        "-3013030220323124042102424341431241221233040112312340403",
    );
    options.set_base(8);
    test_i(
        &Rational::from(i128::MAX),
        options,
        "1777777777777777777777777777777777777777777",
    );
    test_i(
        &Rational::from(i128::MIN),
        options,
        "-2000000000000000000000000000000000000000000",
    );
    options.set_base(16);
    test_i(
        &Rational::from(i128::MAX),
        options,
        "7fffffffffffffffffffffffffffffff",
    );
    test_i(
        &Rational::from(i128::MIN),
        options,
        "-80000000000000000000000000000000",
    );
    options.set_base(32);
    test_i(
        &Rational::from(i128::MAX),
        options,
        "3vvvvvvvvvvvvvvvvvvvvvvvvv",
    );
    test_i(
        &Rational::from(i128::MIN),
        options,
        "-40000000000000000000000000",
    );
    options.set_base(36);
    test_i(
        &Rational::from(i128::MAX),
        options,
        "7ksyyizzkutudzbv8aqztecjj",
    );
    test_i(
        &Rational::from(i128::MIN),
        options,
        "-7ksyyizzkutudzbv8aqztecjk",
    );

    options = ToSciOptions::default();
    options.set_precision(4);
    options.set_include_trailing_zeros(true);
    test("-1", options, "-1.000");
    test("-10", options, "-10.00");
    test("-100", options, "-100.0");
    test("-1000", options, "-1000");
    test("-10000", options, "-1.000e4");
    test("-9", options, "-9.000");
    test("-99", options, "-99.00");
    test("-999", options, "-999.0");
    test("-9999", options, "-9999");
    test("-99999", options, "-1.000e5");

    options.set_include_trailing_zeros(false);
    test("-1", options, "-1");
    test("-10", options, "-10");
    test("-100", options, "-100");
    test("-1000", options, "-1000");
    test("-10000", options, "-1e4");
    test("-9", options, "-9");
    test("-99", options, "-99");
    test("-999", options, "-999");
    test("-9999", options, "-9999");
    test("-99999", options, "-1e5");

    options = ToSciOptions::default();
    options.set_precision(1);
    options.set_include_trailing_zeros(true); // doesn't matter when precision is 1
    test("-1", options, "-1");
    test("-10", options, "-1e1");
    test("-100", options, "-1e2");
    test("-1000", options, "-1e3");
    test("-10000", options, "-1e4");
    test("-9", options, "-9");
    test("-99", options, "-1e2");
    test("-999", options, "-1e3");
    test("-9999", options, "-1e4");
    test("-99999", options, "-1e5");

    options.set_include_trailing_zeros(false);
    test("-1", options, "-1");
    test("-10", options, "-1e1");
    test("-100", options, "-1e2");
    test("-1000", options, "-1e3");
    test("-10000", options, "-1e4");
    test("-9", options, "-9");
    test("-99", options, "-1e2");
    test("-999", options, "-1e3");
    test("-9999", options, "-1e4");
    test("-99999", options, "-1e5");

    options = ToSciOptions::default();
    options.set_scale(2);
    options.set_include_trailing_zeros(true);
    test("-1", options, "-1.00");
    test("-10", options, "-10.00");
    test("-100", options, "-100.00");
    test("-1000", options, "-1000.00");
    test("-10000", options, "-10000.00");
    test("-9", options, "-9.00");
    test("-99", options, "-99.00");
    test("-999", options, "-999.00");
    test("-9999", options, "-9999.00");
    test("-99999", options, "-99999.00");

    options.set_include_trailing_zeros(false);
    test("-1", options, "-1");
    test("-10", options, "-10");
    test("-100", options, "-100");
    test("-1000", options, "-1000");
    test("-10000", options, "-10000");
    test("-9", options, "-9");
    test("-99", options, "-99");
    test("-999", options, "-999");
    test("-9999", options, "-9999");
    test("-99999", options, "-99999");

    options = ToSciOptions::default();
    options.set_scale(0);
    options.set_include_trailing_zeros(true); // doesn't matter when scale is 0
    test("-1", options, "-1");
    test("-10", options, "-10");
    test("-100", options, "-100");
    test("-1000", options, "-1000");
    test("-10000", options, "-10000");
    test("-9", options, "-9");
    test("-99", options, "-99");
    test("-999", options, "-999");
    test("-9999", options, "-9999");
    test("-99999", options, "-99999");

    options.set_include_trailing_zeros(false);
    test("-1", options, "-1");
    test("-10", options, "-10");
    test("-100", options, "-100");
    test("-1000", options, "-1000");
    test("-10000", options, "-10000");
    test("-9", options, "-9");
    test("-99", options, "-99");
    test("-999", options, "-999");
    test("-9999", options, "-9999");
    test("-99999", options, "-99999");

    options = ToSciOptions::default();
    options.set_precision(2);
    options.set_rounding_mode(Nearest); // This is the default
    test("-123", options, "-1.2e2");
    options.set_rounding_mode(Down);
    test("-123", options, "-1.2e2");
    options.set_rounding_mode(Floor);
    test("-123", options, "-1.3e2");
    options.set_rounding_mode(Up);
    test("-123", options, "-1.3e2");
    options.set_rounding_mode(Ceiling);
    test("-123", options, "-1.2e2");

    options.set_rounding_mode(Nearest);
    test("-135", options, "-1.4e2");
    options.set_rounding_mode(Down);
    test("-135", options, "-1.3e2");
    options.set_rounding_mode(Floor);
    test("-135", options, "-1.4e2");
    options.set_rounding_mode(Up);
    test("-135", options, "-1.4e2");
    options.set_rounding_mode(Ceiling);
    test("-135", options, "-1.3e2");

    options.set_rounding_mode(Exact);
    test("-140", options, "-1.4e2");

    options.set_rounding_mode(Nearest);
    test("-999", options, "-1e3");
    options.set_rounding_mode(Down);
    test("-999", options, "-9.9e2");
    options.set_rounding_mode(Floor);
    test("-999", options, "-1e3");
    options.set_rounding_mode(Up);
    test("-999", options, "-1e3");
    options.set_rounding_mode(Ceiling);
    test("-999", options, "-9.9e2");

    options = ToSciOptions::default();
    options.set_scale(2);
    test("1/3", options, "0.33");
    options.set_scale(1);
    test("1/3", options, "0.3");
    options.set_scale(0);
    test("1/3", options, "0");
    options.set_scale(4);
    test("1/300", options, "0.0033");
    options.set_scale(3);
    test("1/300", options, "0.003");
    options.set_scale(2);
    test("1/300", options, "0");
    options.set_scale(1);
    test("1/300", options, "0");
    options.set_rounding_mode(Ceiling);
    options.set_scale(2);
    test("1/3", options, "0.34");
    options.set_scale(1);
    test("1/3", options, "0.4");
    options.set_scale(0);
    test("1/3", options, "1");
    options.set_scale(4);
    test("1/300", options, "0.0034");
    options.set_scale(3);
    test("1/300", options, "0.004");
    options.set_scale(2);
    test("1/300", options, "0.01");
    options.set_scale(1);
    test("1/300", options, "0.1");

    options = ToSciOptions::default();
    options.set_base(2);
    test("245850922/78256779", options, "11.0010010001");
    options.set_base(3);
    test("245850922/78256779", options, "10.01021101222201");
    options.set_base(4);
    test("245850922/78256779", options, "3.021003331222202");
    options.set_base(5);
    test("245850922/78256779", options, "3.032322143033433");
    options.set_base(8);
    test("245850922/78256779", options, "3.110375524210264");
    options.set_base(16);
    test("245850922/78256779", options, "3.243f6a8885a3033");
    options.set_base(32);
    test("245850922/78256779", options, "3.4gvml245kc1j1qs");
    options.set_base(36);
    test("245850922/78256779", options, "3.53i5ab8p5fhzpkj");

    options = ToSciOptions::default();
    options.set_size_complete();
    test("1/2", options, "0.5");
    test("1/4", options, "0.25");
    test("1/5", options, "0.2");
    test("1/8", options, "0.125");
    test("1/10", options, "0.1");
    options.set_base(2);
    test("1/2", options, "0.1");
    test("1/4", options, "0.01");
    test("1/8", options, "0.001");
    options.set_base(3);
    test("1/3", options, "0.1");
    test("1/9", options, "0.01");
    options.set_base(4);
    test("1/2", options, "0.2");
    test("1/4", options, "0.1");
    test("1/8", options, "0.02");
    options.set_base(5);
    test("1/5", options, "0.1");
    options.set_base(6);
    test("1/2", options, "0.3");
    test("1/3", options, "0.2");
    test("1/6", options, "0.1");
    test("1/9", options, "0.04");
    options.set_base(7);
    test("1/7", options, "0.1");
    options.set_base(8);
    test("1/2", options, "0.4");
    test("1/4", options, "0.2");
    test("1/8", options, "0.1");
    options.set_base(9);
    test("1/3", options, "0.3");
    test("1/9", options, "0.1");

    options = ToSciOptions::default();
    options.set_size_complete();
    test_i(
        &Rational::exact_from(f32::MIN_POSITIVE_SUBNORMAL),
        options,
        "1.401298464324817070923729583289916131280261941876515771757068283889791082685860601486638\
        18836212158203125e-45",
    );
    options.set_base(2);
    test_i(
        &Rational::exact_from(f32::MIN_POSITIVE_SUBNORMAL),
        options,
        "1e-149",
    );
    options.set_base(32);
    test_i(
        &Rational::exact_from(f32::MIN_POSITIVE_SUBNORMAL),
        options,
        "2e-30",
    );
    options.set_base(36);
    test_i(
        &Rational::exact_from(f32::MIN_POSITIVE_SUBNORMAL),
        options,
        "1.whin9rvdkphvrmxkdwtoq8t963n428tj1p07aaum2yy14ie-29",
    );
    options.set_uppercase();
    test_i(
        &Rational::exact_from(f32::MIN_POSITIVE_SUBNORMAL),
        options,
        "1.WHIN9RVDKPHVRMXKDWTOQ8T963N428TJ1P07AAUM2YY14Ie-29",
    );
    options.set_base(10);
    options.set_neg_exp_threshold(-200);
    test_i(
        &Rational::exact_from(f32::MIN_POSITIVE_SUBNORMAL),
        options,
        "0.000000000000000000000000000000000000000000001401298464324817070923729583289916131280261\
        94187651577175706828388979108268586060148663818836212158203125",
    );
    options = ToSciOptions::default();
    test("1/1000", options, "0.001");
    options.set_neg_exp_threshold(-3);
    test("1/1000", options, "1e-3");
    test("1/100", options, "0.01");
    options.set_neg_exp_threshold(-2);
    test("1/100", options, "1e-2");
    test("1/10", options, "0.1");
    options.set_neg_exp_threshold(-1);
    test("1/10", options, "1e-1");

    options = ToSciOptions::default();
    options.set_base(3);
    options.set_scale(1);
    // Nearest uses bankers' rounding: 1/2 is equidistant to 0.1 and 0.2 in base 3, so we choose the
    // even option, 0.2.
    test("1/2", options, "0.2");
}

#[should_panic]
#[test]
pub fn to_sci_with_options_fail() {
    let mut options = ToSciOptions::default();
    options.set_rounding_mode(Exact);
    options.set_precision(2);
    Rational::from(123).to_sci_with_options(options).to_string();
}

#[test]
fn to_sci_properties() {
    let mut powers_of_10 = HashMap::new();
    let ten = Rational::from(10);
    let default_p = 16;
    rational_gen().test_properties(|x| {
        assert!(x.fmt_sci_valid(ToSciOptions::default()));
        let s = x.to_sci().to_string();
        assert_eq!(
            x.to_sci_with_options(ToSciOptions::default()).to_string(),
            s
        );
        assert!(string_is_subset(&s, DECIMAL_SCI_STRING_CHARS));
        assert!(!s.starts_with('+'));
        assert!(!s.starts_with('.'));
        assert!(!s.ends_with('+'));
        assert!(!s.ends_with('-'));
        assert!(!s.ends_with('.'));
        assert!(!s.contains('E'));
        assert!(!s.contains("++"));
        assert!(!s.contains("+-"));
        assert!(!s.contains("-+"));
        assert!(!s.contains("--"));
        assert!(!s.contains("-+"));
        assert!(!s.contains("+."));
        assert!(!s.contains("-."));
        let x_from = Rational::from_sci_string(&s).unwrap();
        if x == 0u32 {
            assert_eq!(x_from, 0u32);
        } else {
            let log = (&x).abs().floor_log_base(&ten);
            let pow = powers_of_10
                .entry(log - default_p + 1)
                .or_insert_with_key(|&p| (&ten).pow(p));
            assert_eq!(x.round_to_multiple(&*pow, Nearest).0, x_from);
        }
    });

    integer_gen().test_properties(|x| {
        assert_eq!(
            x.to_sci().to_string(),
            Rational::from(x).to_sci().to_string()
        );
    });
}

#[test]
fn to_sci_with_options_properties() {
    let mut powers = HashMap::new();
    let mut chars = HashMap::new();
    rational_to_sci_options_pair_gen_var_1().test_properties(|(x, options)| {
        assert!(x.fmt_sci_valid(options));
        let s = x.to_sci_with_options(options).to_string();
        let cs: &mut String = chars.entry(options.get_base()).or_insert_with_key(|&base| {
            let mut cs = "+-.0123456789".to_string();
            if base > 10 {
                let limit = usize::from(base - 10);
                for c in ('a'..='z').take(limit) {
                    cs.push(c);
                }
                for c in ('A'..='Z').take(limit) {
                    cs.push(c);
                }
            }
            if base < 15 {
                cs.push('e');
                cs.push('E');
            }
            cs
        });
        assert!(string_is_subset(&s, cs));
        assert!(!s.starts_with('+'));
        assert!(!s.starts_with('.'));
        assert!(!s.ends_with('+'));
        assert!(!s.ends_with('-'));
        assert!(!s.ends_with('.'));
        assert!(!s.contains("++"));
        assert!(!s.contains("+-"));
        assert!(!s.contains("-+"));
        assert!(!s.contains("--"));
        assert!(!s.contains("-+"));
        assert!(!s.contains("+."));
        assert!(!s.contains("-."));
        assert!(s.chars().filter(|&c| c == '.').count() <= 1);
        assert!(s.chars().filter(|&c| c == '-').count() <= 2);
        assert!(s.chars().filter(|&c| c == '+').count() <= 1);
        let mut from_options = FromSciStringOptions::default();
        from_options.set_base(options.get_base());
        let x_from = Rational::from_sci_string_with_options(&s, from_options).unwrap();
        if x == 0u32 {
            assert_eq!(x_from, 0u32);
        } else {
            let base = options.get_base();
            let q_base = Rational::from(base);
            let log = floor_log_base_of_abs(&x, &q_base);
            let (scale, might_round_to_zero) = match options.get_size_options() {
                SciSizeOptions::Complete => {
                    let scale = x.length_after_point_in_small_base(base).unwrap();
                    assert!(i64::exact_from(scale) + log + 1 > 0);
                    (None, false)
                }
                SciSizeOptions::Scale(scale) => {
                    let scale = i64::exact_from(scale);
                    (Some(scale), scale + log < 0)
                }
                SciSizeOptions::Precision(precision) => {
                    (Some(i64::exact_from(precision - 1) - log), false)
                }
            };
            if might_round_to_zero && x_from == 0u32 {
                // Do nothing
            } else if let Some(neg_scale) = scale.map(|s| -s) {
                let pow = powers
                    .entry((base, neg_scale))
                    .or_insert_with(|| (&q_base).pow(neg_scale));
                let rounded = (&x).round_to_multiple(&*pow, options.get_rounding_mode()).0;
                assert_eq!(rounded, x_from);
            } else {
                assert_eq!(x_from, x);
            }
        }
    });

    integer_to_sci_options_pair_gen_var_1().test_properties(|(x, options)| {
        assert_eq!(
            x.to_sci_with_options(options).to_string(),
            Rational::from(x).to_sci_with_options(options).to_string()
        );
    });
}
