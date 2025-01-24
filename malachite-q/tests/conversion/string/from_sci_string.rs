// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Abs, FloorLogBase};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::string::from_sci_string::preprocess_sci_string;
use malachite_base::num::conversion::string::options::{FromSciStringOptions, ToSciOptions};
use malachite_base::num::conversion::traits::{ExactFrom, FromSciString, ToSci};
use malachite_base::test_util::generators::{
    string_from_sci_string_options_pair_gen_var_2, string_from_sci_string_options_pair_gen_var_3,
    string_gen_var_14, string_gen_var_15,
};
use malachite_q::test_util::generators::{
    rational_gen, rational_gen_var_7, rational_unsigned_pair_gen_var_6,
};
use malachite_q::Rational;
use std::str::FromStr;

#[test]
pub fn test_from_sci_string() {
    fn test(s: &str, out: Option<&'static str>) {
        let out = out.map(|s| Rational::from_str(s).unwrap());
        assert_eq!(Rational::from_sci_string(s), out);
        assert_eq!(
            Rational::from_sci_string_with_options(s, FromSciStringOptions::default()),
            out
        );
    }
    test("0", Some("0"));
    test("00", Some("0"));
    test("+0", Some("0"));
    test("-0", Some("0"));
    test("0.00", Some("0"));
    test("0e1", Some("0"));
    test("0e+1", Some("0"));
    test("0e-1", Some("0"));
    test("+0e+1", Some("0"));
    test("-0e+1", Some("0"));
    test("+0.0e+1", Some("0"));
    test("-0.0e+1", Some("0"));
    test(".0", Some("0"));
    test(".00", Some("0"));
    test(".00e0", Some("0"));
    test(".00e1", Some("0"));
    test(".00e-1", Some("0"));
    test("-.0", Some("0"));
    test("-.00", Some("0"));
    test("-.00e0", Some("0"));
    test("-.00e1", Some("0"));
    test("-.00e-1", Some("0"));
    test("+.0", Some("0"));
    test("+.00", Some("0"));
    test("+.00e0", Some("0"));
    test("+.00e1", Some("0"));
    test("+.00e-1", Some("0"));

    test("123", Some("123"));
    test("00123", Some("123"));
    test("+123", Some("123"));
    test("123.00", Some("123"));
    test("123e0", Some("123"));
    test("12.3e1", Some("123"));
    test("1.23e2", Some("123"));
    test("1.23E2", Some("123"));
    test("1.23e+2", Some("123"));
    test("1.23E+2", Some("123"));
    test(".123e3", Some("123"));
    test("0.123e3", Some("123"));
    test("+0.123e3", Some("123"));
    test("0.0123e4", Some("123"));
    test("1230e-1", Some("123"));
    test("12300e-2", Some("123"));
    test("12300E-2", Some("123"));

    test("-123", Some("-123"));
    test("-00123", Some("-123"));
    test("-123.00", Some("-123"));
    test("-123e0", Some("-123"));
    test("-12.3e1", Some("-123"));
    test("-1.23e2", Some("-123"));
    test("-1.23E2", Some("-123"));
    test("-1.23e+2", Some("-123"));
    test("-1.23E+2", Some("-123"));
    test("-.123e3", Some("-123"));
    test("-0.123e3", Some("-123"));
    test("-0.0123e4", Some("-123"));
    test("-1230e-1", Some("-123"));
    test("-12300e-2", Some("-123"));
    test("-12300E-2", Some("-123"));

    test("123.4", Some("617/5"));
    test("123.8", Some("619/5"));
    test("123.5", Some("247/2"));
    test("124.5", Some("249/2"));
    test("127.49", Some("12749/100"));

    test("-123.4", Some("-617/5"));
    test("-123.8", Some("-619/5"));
    test("-123.5", Some("-247/2"));
    test("-124.5", Some("-249/2"));
    test("-127.49", Some("-12749/100"));
    test("-127.5", Some("-255/2"));

    test("0.5", Some("1/2"));
    test(
        "0.3333333333333333",
        Some("3333333333333333/10000000000000000"),
    );
    test("0.25", Some("1/4"));
    test("0.2", Some("1/5"));
    test(
        "0.1666666666666667",
        Some("1666666666666667/10000000000000000"),
    );
    test(
        "0.1428571428571429",
        Some("1428571428571429/10000000000000000"),
    );
    test("0.125", Some("1/8"));
    test(
        "0.1111111111111111",
        Some("1111111111111111/10000000000000000"),
    );
    test("0.1", Some("1/10"));
    test(
        "0.09090909090909091",
        Some("9090909090909091/100000000000000000"),
    );

    test("0.0", Some("0"));
    test("0.1", Some("1/10"));
    test("0.2", Some("1/5"));
    test("0.3", Some("3/10"));
    test("0.4", Some("2/5"));
    test("0.5", Some("1/2"));
    test("0.6", Some("3/5"));
    test("0.7", Some("7/10"));
    test("0.8", Some("4/5"));
    test("0.9", Some("9/10"));

    test("0.00", Some("0"));
    test("0.10", Some("1/10"));
    test("0.20", Some("1/5"));
    test("0.30", Some("3/10"));
    test("0.40", Some("2/5"));
    test("0.50", Some("1/2"));
    test("0.60", Some("3/5"));
    test("0.70", Some("7/10"));
    test("0.80", Some("4/5"));
    test("0.90", Some("9/10"));

    test("123.456456456456", Some("15432057057057/125000000000"));

    test(
        "1.4142135623730951",
        Some("14142135623730951/10000000000000000"),
    );
    test(
        "3.141592653589793",
        Some("3141592653589793/1000000000000000"),
    );
    test("2.718281828459045", Some("543656365691809/200000000000000"));

    test("", None);
    test("+", None);
    test("-", None);
    test("10e", None);
    test("++1", None);
    test("1.0.0", None);
    test("1e++1", None);
    test("1e0.1", None);
    test("--.0", None);
    test("++.0", None);
    test(".+2", None);
    test(".-2", None);
    test("0.000a", None);
    test("0.00ae-10", None);
    test("0e10000000000000000000000000000", None);
    test("0e-10000000000000000000000000000", None);
}

#[test]
pub fn test_from_sci_string_with_options() {
    fn test(s: &str, options: FromSciStringOptions, out: Option<&str>) {
        let out = out.map(|s| Rational::from_str(s).unwrap());
        assert_eq!(Rational::from_sci_string_with_options(s, options), out);
    }
    fn test_i<T: PrimitiveInt>(s: &str, options: FromSciStringOptions, out: Option<T>)
    where
        Rational: From<T>,
    {
        let out = out.map(Rational::from);
        assert_eq!(Rational::from_sci_string_with_options(s, options), out);
    }
    // For tests with the default options, see `test_from_sci_string`

    let mut options = FromSciStringOptions::default();
    options.set_base(2);
    test_i(
        "11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111\
        111111111111111111111111111111111111111",
        options,
        Some(u128::MAX),
    );
    options.set_base(3);
    test_i(
        "202201102121002021012000211012011021221022212021111001022110211020010021100121010",
        options,
        Some(u128::MAX),
    );
    options.set_base(4);
    test_i(
        "3333333333333333333333333333333333333333333333333333333333333333",
        options,
        Some(u128::MAX),
    );
    options.set_base(5);
    test_i(
        "11031110441201303134210404233413032443021130230130231310",
        options,
        Some(u128::MAX),
    );
    options.set_base(8);
    test_i(
        "3777777777777777777777777777777777777777777",
        options,
        Some(u128::MAX),
    );
    options.set_base(16);
    test_i("ffffffffffffffffffffffffffffffff", options, Some(u128::MAX));
    options.set_base(32);
    test_i("7vvvvvvvvvvvvvvvvvvvvvvvvv", options, Some(u128::MAX));
    options.set_base(36);
    test_i("f5lxx1zz5pnorynqglhzmsp33", options, Some(u128::MAX));

    options.set_base(2);
    test_i(
        "11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111\
        11111111111111111111111111111111111111",
        options,
        Some(i128::MAX),
    );
    test_i(
        "-1000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        0000000000000000000000000000000000000000",
        options,
        Some(i128::MIN),
    );
    options.set_base(3);
    test_i(
        "101100201022001010121000102002120122110122221010202000122201220121120010200022001",
        options,
        Some(i128::MAX),
    );
    test_i(
        "-101100201022001010121000102002120122110122221010202000122201220121120010200022002",
        options,
        Some(i128::MIN),
    );
    options.set_base(4);
    test_i(
        "1333333333333333333333333333333333333333333333333333333333333333",
        options,
        Some(i128::MAX),
    );
    test_i(
        "-2000000000000000000000000000000000000000000000000000000000000000",
        options,
        Some(i128::MIN),
    );
    options.set_base(5);
    test_i(
        "3013030220323124042102424341431241221233040112312340402",
        options,
        Some(i128::MAX),
    );
    test_i(
        "-3013030220323124042102424341431241221233040112312340403",
        options,
        Some(i128::MIN),
    );
    options.set_base(8);
    test_i(
        "1777777777777777777777777777777777777777777",
        options,
        Some(i128::MAX),
    );
    test_i(
        "-2000000000000000000000000000000000000000000",
        options,
        Some(i128::MIN),
    );
    options.set_base(16);
    test_i("7fffffffffffffffffffffffffffffff", options, Some(i128::MAX));
    test_i(
        "-80000000000000000000000000000000",
        options,
        Some(i128::MIN),
    );
    options.set_base(32);
    test_i("3vvvvvvvvvvvvvvvvvvvvvvvvv", options, Some(i128::MAX));
    test_i("-40000000000000000000000000", options, Some(i128::MIN));
    options.set_base(36);
    test_i("7ksyyizzkutudzbv8aqztecjj", options, Some(i128::MAX));
    test_i("-7ksyyizzkutudzbv8aqztecjk", options, Some(i128::MIN));

    options.set_base(2);
    test("1e+5", options, Some("32"));
    test("1e5", options, Some("32"));
    options.set_base(3);
    test("1e+5", options, Some("243"));
    test("1e5", options, Some("243"));
    options.set_base(4);
    test("1e+5", options, Some("1024"));
    test("1e5", options, Some("1024"));
    options.set_base(5);
    test("1e+5", options, Some("3125"));
    test("1e5", options, Some("3125"));
    options.set_base(8);
    test("1e+5", options, Some("32768"));
    test("1e5", options, Some("32768"));
    options.set_base(16);
    test("1e+5", options, Some("1048576"));
    test("1e5", options, Some("485"));
    options.set_base(32);
    test("1e+5", options, Some("33554432"));
    test("1e5", options, Some("1477"));
    options.set_base(36);
    test("1e+5", options, Some("60466176"));
    test("1E+5", options, Some("60466176"));
    test("1e5", options, Some("1805"));

    options.set_base(16);
    test("ff", options, Some("255"));
    test("fF", options, Some("255"));
    test("Ff", options, Some("255"));
    test("FF", options, Some("255"));

    options = FromSciStringOptions::default();
    options.set_base(2);
    // 1/2 is 0.1
    test("1.01", options, Some("5/4"));
    test("1.1", options, Some("3/2"));
    test("1.11", options, Some("7/4"));
    test("0.01", options, Some("1/4"));
    test("0.1", options, Some("1/2"));
    test("0.11", options, Some("3/4"));
    options.set_base(3);
    // 1/2 is 0.111...
    test("1.1", options, Some("4/3"));
    test("1.11", options, Some("13/9"));
    test("1.111", options, Some("40/27"));
    test("1.112", options, Some("41/27"));
    test("0.1", options, Some("1/3"));
    test("0.11", options, Some("4/9"));
    test("0.111", options, Some("13/27"));
    test("0.112", options, Some("14/27"));

    options = FromSciStringOptions::default();
    options.set_base(2);
    test("2", options, None);
    test("102", options, None);
    test("12e4", options, None);
    test("12e-4", options, None);
    test("1.2", options, None);
    test("0.2", options, None);
    test("0.002", options, None);

    options.set_base(2);
    test("-1e+5", options, Some("-32"));
    test("-1e5", options, Some("-32"));
    options.set_base(3);
    test("-1e+5", options, Some("-243"));
    test("-1e5", options, Some("-243"));
    options.set_base(4);
    test("-1e+5", options, Some("-1024"));
    test("-1e5", options, Some("-1024"));
    options.set_base(5);
    test("-1e+5", options, Some("-3125"));
    test("-1e5", options, Some("-3125"));
    options.set_base(8);
    test("-1e+5", options, Some("-32768"));
    test("-1e5", options, Some("-32768"));
    options.set_base(16);
    test("-1e+5", options, Some("-1048576"));
    test("-1e5", options, Some("-485"));
    options.set_base(32);
    test("-1e+5", options, Some("-33554432"));
    test("-1e5", options, Some("-1477"));
    options.set_base(36);
    test("-1e+5", options, Some("-60466176"));
    test("-1E+5", options, Some("-60466176"));
    test("-1e5", options, Some("-1805"));

    options.set_base(16);
    test("-ff", options, Some("-255"));
    test("-fF", options, Some("-255"));
    test("-Ff", options, Some("-255"));
    test("-FF", options, Some("-255"));

    options = FromSciStringOptions::default();
    options.set_base(2);
    // 1/2 is 0.1
    test("-1.01", options, Some("-5/4"));
    test("-1.1", options, Some("-3/2"));
    test("-1.11", options, Some("-7/4"));
    test("-0.01", options, Some("-1/4"));
    test("-0.1", options, Some("-1/2"));
    test("-0.11", options, Some("-3/4"));
    options.set_base(3);
    // 1/2 is 0.111...
    test("-1.1", options, Some("-4/3"));
    test("-1.11", options, Some("-13/9"));
    test("-1.111", options, Some("-40/27"));
    test("-1.112", options, Some("-41/27"));
    test("-0.1", options, Some("-1/3"));
    test("-0.11", options, Some("-4/9"));
    test("-0.111", options, Some("-13/27"));
    test("-0.112", options, Some("-14/27"));

    options = FromSciStringOptions::default();
    options.set_base(2);
    test("-2", options, None);
    test("-102", options, None);
    test("-12e4", options, None);
    test("-12e-4", options, None);
    test("-1.2", options, None);
    test("-0.2", options, None);
    test("-0.002", options, None);

    test("0.1111111111", options, Some("1023/1024"));
    options.set_base(3);
    test("0.1111111111", options, Some("29524/59049"));
    options.set_base(4);
    test("0.1111111111", options, Some("349525/1048576"));
    options.set_base(16);
    test("0.1111111111", options, Some("73300775185/1099511627776"));
    options.set_base(32);
    test(
        "0.1111111111",
        options,
        Some("36319351833633/1125899906842624"),
    );
    options.set_base(36);
    test(
        "0.1111111111",
        options,
        Some("104461669716085/3656158440062976"),
    );
}

#[test]
pub fn test_from_sci_string_simplest() {
    fn test(s: &str, out: Option<&'static str>) {
        let out = out.map(|s| Rational::from_str(s).unwrap());
        assert_eq!(Rational::from_sci_string_simplest(s), out);
        assert_eq!(
            Rational::from_sci_string_simplest_with_options(s, FromSciStringOptions::default()),
            out
        );
    }
    test("0", Some("0"));
    test("00", Some("0"));
    test("+0", Some("0"));
    test("-0", Some("0"));
    test("0.00", Some("0"));
    test("0e1", Some("0"));
    test("0e+1", Some("0"));
    test("0e-1", Some("0"));
    test("+0e+1", Some("0"));
    test("-0e+1", Some("0"));
    test("+0.0e+1", Some("0"));
    test("-0.0e+1", Some("0"));
    test(".0", Some("0"));
    test(".00", Some("0"));
    test(".00e0", Some("0"));
    test(".00e1", Some("0"));
    test(".00e-1", Some("0"));
    test("-.0", Some("0"));
    test("-.00", Some("0"));
    test("-.00e0", Some("0"));
    test("-.00e1", Some("0"));
    test("-.00e-1", Some("0"));
    test("+.0", Some("0"));
    test("+.00", Some("0"));
    test("+.00e0", Some("0"));
    test("+.00e1", Some("0"));
    test("+.00e-1", Some("0"));

    test("123", Some("123"));
    test("00123", Some("123"));
    test("+123", Some("123"));
    test("123.00", Some("123"));
    test("123e0", Some("123"));
    test("12.3e1", Some("123"));
    test("1.23e2", Some("123"));
    test("1.23E2", Some("123"));
    test("1.23e+2", Some("123"));
    test("1.23E+2", Some("123"));
    test(".123e3", Some("123"));
    test("0.123e3", Some("123"));
    test("+0.123e3", Some("123"));
    test("0.0123e4", Some("123"));
    test("1230e-1", Some("123"));
    test("12300e-2", Some("123"));
    test("12300E-2", Some("123"));

    test("-123", Some("-123"));
    test("-00123", Some("-123"));
    test("-123.00", Some("-123"));
    test("-123e0", Some("-123"));
    test("-12.3e1", Some("-123"));
    test("-1.23e2", Some("-123"));
    test("-1.23E2", Some("-123"));
    test("-1.23e+2", Some("-123"));
    test("-1.23E+2", Some("-123"));
    test("-.123e3", Some("-123"));
    test("-0.123e3", Some("-123"));
    test("-0.0123e4", Some("-123"));
    test("-1230e-1", Some("-123"));
    test("-12300e-2", Some("-123"));
    test("-12300E-2", Some("-123"));

    test("123.4", Some("617/5"));
    test("123.8", Some("495/4"));
    test("123.5", Some("247/2"));
    test("124.5", Some("249/2"));
    test("127.49", Some("4462/35"));

    test("-123.4", Some("-617/5"));
    test("-123.8", Some("-495/4"));
    test("-123.5", Some("-247/2"));
    test("-124.5", Some("-249/2"));
    test("-127.49", Some("-4462/35"));
    test("-127.5", Some("-255/2"));

    test("0.5", Some("1/2"));
    test("0.3333333333333333", Some("1/3"));
    test("0.25", Some("1/4"));
    test("0.2", Some("1/4"));
    test("0.1666666666666667", Some("1/6"));
    test("0.1428571428571429", Some("1/7"));
    test("0.125", Some("1/8"));
    test("0.1111111111111111", Some("1/9"));
    test("0.1", Some("1/7"));
    test("0.09090909090909091", Some("1/11"));

    test("0.0", Some("0"));
    test("0.1", Some("1/7"));
    test("0.2", Some("1/4"));
    test("0.3", Some("1/3"));
    test("0.4", Some("2/5"));
    test("0.5", Some("1/2"));
    test("0.6", Some("3/5"));
    test("0.7", Some("2/3"));
    test("0.8", Some("3/4"));
    test("0.9", Some("6/7"));

    test("0.00", Some("0"));
    test("0.10", Some("1/10"));
    test("0.20", Some("1/5"));
    test("0.30", Some("3/10"));
    test("0.40", Some("2/5"));
    test("0.50", Some("1/2"));
    test("0.60", Some("3/5"));
    test("0.70", Some("7/10"));
    test("0.80", Some("4/5"));
    test("0.90", Some("9/10"));

    test("123.456456456456", Some("41111/333"));

    test("1.4142135623730951", Some("131836323/93222358"));
    test("3.141592653589793", Some("80143857/25510582"));
    test("2.718281828459045", Some("212385209/78132152"));

    test("", None);
    test("+", None);
    test("-", None);
    test("10e", None);
    test("++1", None);
    test("1.0.0", None);
    test("1e++1", None);
    test("1e0.1", None);
    test("--.0", None);
    test("++.0", None);
    test(".+2", None);
    test(".-2", None);
    test("0.000a", None);
    test("0.00ae-10", None);
    test("0e10000000000000000000000000000", None);
    test("0e-10000000000000000000000000000", None);
}

#[test]
pub fn test_from_sci_string_simplest_with_options() {
    fn test(s: &str, options: FromSciStringOptions, out: Option<&str>) {
        let out = out.map(|s| Rational::from_str(s).unwrap());
        assert_eq!(
            Rational::from_sci_string_simplest_with_options(s, options),
            out
        );
    }
    fn test_i<T: PrimitiveInt>(s: &str, options: FromSciStringOptions, out: Option<T>)
    where
        Rational: From<T>,
    {
        let out = out.map(Rational::from);
        assert_eq!(
            Rational::from_sci_string_simplest_with_options(s, options),
            out
        );
    }
    // For tests with the default options, see `test_from_sci_string`

    let mut options = FromSciStringOptions::default();
    options.set_base(2);
    test_i(
        "11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111\
        111111111111111111111111111111111111111",
        options,
        Some(u128::MAX),
    );
    options.set_base(3);
    test_i(
        "202201102121002021012000211012011021221022212021111001022110211020010021100121010",
        options,
        Some(u128::MAX),
    );
    options.set_base(4);
    test_i(
        "3333333333333333333333333333333333333333333333333333333333333333",
        options,
        Some(u128::MAX),
    );
    options.set_base(5);
    test_i(
        "11031110441201303134210404233413032443021130230130231310",
        options,
        Some(u128::MAX),
    );
    options.set_base(8);
    test_i(
        "3777777777777777777777777777777777777777777",
        options,
        Some(u128::MAX),
    );
    options.set_base(16);
    test_i("ffffffffffffffffffffffffffffffff", options, Some(u128::MAX));
    options.set_base(32);
    test_i("7vvvvvvvvvvvvvvvvvvvvvvvvv", options, Some(u128::MAX));
    options.set_base(36);
    test_i("f5lxx1zz5pnorynqglhzmsp33", options, Some(u128::MAX));

    options.set_base(2);
    test_i(
        "11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111\
        11111111111111111111111111111111111111",
        options,
        Some(i128::MAX),
    );
    test_i(
        "-1000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        0000000000000000000000000000000000000000",
        options,
        Some(i128::MIN),
    );
    options.set_base(3);
    test_i(
        "101100201022001010121000102002120122110122221010202000122201220121120010200022001",
        options,
        Some(i128::MAX),
    );
    test_i(
        "-101100201022001010121000102002120122110122221010202000122201220121120010200022002",
        options,
        Some(i128::MIN),
    );
    options.set_base(4);
    test_i(
        "1333333333333333333333333333333333333333333333333333333333333333",
        options,
        Some(i128::MAX),
    );
    test_i(
        "-2000000000000000000000000000000000000000000000000000000000000000",
        options,
        Some(i128::MIN),
    );
    options.set_base(5);
    test_i(
        "3013030220323124042102424341431241221233040112312340402",
        options,
        Some(i128::MAX),
    );
    test_i(
        "-3013030220323124042102424341431241221233040112312340403",
        options,
        Some(i128::MIN),
    );
    options.set_base(8);
    test_i(
        "1777777777777777777777777777777777777777777",
        options,
        Some(i128::MAX),
    );
    test_i(
        "-2000000000000000000000000000000000000000000",
        options,
        Some(i128::MIN),
    );
    options.set_base(16);
    test_i("7fffffffffffffffffffffffffffffff", options, Some(i128::MAX));
    test_i(
        "-80000000000000000000000000000000",
        options,
        Some(i128::MIN),
    );
    options.set_base(32);
    test_i("3vvvvvvvvvvvvvvvvvvvvvvvvv", options, Some(i128::MAX));
    test_i("-40000000000000000000000000", options, Some(i128::MIN));
    options.set_base(36);
    test_i("7ksyyizzkutudzbv8aqztecjj", options, Some(i128::MAX));
    test_i("-7ksyyizzkutudzbv8aqztecjk", options, Some(i128::MIN));

    options.set_base(2);
    test("1e+5", options, Some("32"));
    test("1e5", options, Some("32"));
    options.set_base(3);
    test("1e+5", options, Some("243"));
    test("1e5", options, Some("243"));
    options.set_base(4);
    test("1e+5", options, Some("1024"));
    test("1e5", options, Some("1024"));
    options.set_base(5);
    test("1e+5", options, Some("3125"));
    test("1e5", options, Some("3125"));
    options.set_base(8);
    test("1e+5", options, Some("32768"));
    test("1e5", options, Some("32768"));
    options.set_base(16);
    test("1e+5", options, Some("1048576"));
    test("1e5", options, Some("485"));
    options.set_base(32);
    test("1e+5", options, Some("33554432"));
    test("1e5", options, Some("1477"));
    options.set_base(36);
    test("1e+5", options, Some("60466176"));
    test("1E+5", options, Some("60466176"));
    test("1e5", options, Some("1805"));

    options.set_base(16);
    test("ff", options, Some("255"));
    test("fF", options, Some("255"));
    test("Ff", options, Some("255"));
    test("FF", options, Some("255"));

    options = FromSciStringOptions::default();
    options.set_base(2);
    // 1/2 is 0.1
    test("1.01", options, Some("4/3"));
    test("1.1", options, Some("3/2"));
    test("1.11", options, Some("5/3"));
    test("0.01", options, Some("1/3"));
    test("0.1", options, Some("1/2"));
    test("0.11", options, Some("2/3"));
    options.set_base(3);
    // 1/2 is 0.111...
    test("1.1", options, Some("3/2"));
    test("1.11", options, Some("3/2"));
    test("1.111", options, Some("3/2"));
    test("1.112", options, Some("3/2"));
    test("0.1", options, Some("1/2"));
    test("0.11", options, Some("1/2"));
    test("0.111", options, Some("1/2"));
    test("0.112", options, Some("1/2"));

    options = FromSciStringOptions::default();
    options.set_base(2);
    test("2", options, None);
    test("102", options, None);
    test("12e4", options, None);
    test("12e-4", options, None);
    test("1.2", options, None);
    test("0.2", options, None);
    test("0.002", options, None);

    options.set_base(2);
    test("-1e+5", options, Some("-32"));
    test("-1e5", options, Some("-32"));
    options.set_base(3);
    test("-1e+5", options, Some("-243"));
    test("-1e5", options, Some("-243"));
    options.set_base(4);
    test("-1e+5", options, Some("-1024"));
    test("-1e5", options, Some("-1024"));
    options.set_base(5);
    test("-1e+5", options, Some("-3125"));
    test("-1e5", options, Some("-3125"));
    options.set_base(8);
    test("-1e+5", options, Some("-32768"));
    test("-1e5", options, Some("-32768"));
    options.set_base(16);
    test("-1e+5", options, Some("-1048576"));
    test("-1e5", options, Some("-485"));
    options.set_base(32);
    test("-1e+5", options, Some("-33554432"));
    test("-1e5", options, Some("-1477"));
    options.set_base(36);
    test("-1e+5", options, Some("-60466176"));
    test("-1E+5", options, Some("-60466176"));
    test("-1e5", options, Some("-1805"));

    options.set_base(16);
    test("-ff", options, Some("-255"));
    test("-fF", options, Some("-255"));
    test("-Ff", options, Some("-255"));
    test("-FF", options, Some("-255"));

    options = FromSciStringOptions::default();
    options.set_base(2);
    // 1/2 is 0.1
    test("-1.01", options, Some("-4/3"));
    test("-1.1", options, Some("-3/2"));
    test("-1.11", options, Some("-5/3"));
    test("-0.01", options, Some("-1/3"));
    test("-0.1", options, Some("-1/2"));
    test("-0.11", options, Some("-2/3"));
    options.set_base(3);
    // 1/2 is 0.111...
    test("-1.1", options, Some("-3/2"));
    test("-1.11", options, Some("-3/2"));
    test("-1.111", options, Some("-3/2"));
    test("-1.112", options, Some("-3/2"));
    test("-0.1", options, Some("-1/2"));
    test("-0.11", options, Some("-1/2"));
    test("-0.111", options, Some("-1/2"));
    test("-0.112", options, Some("-1/2"));

    options = FromSciStringOptions::default();
    options.set_base(2);
    test("-2", options, None);
    test("-102", options, None);
    test("-12e4", options, None);
    test("-12e-4", options, None);
    test("-1.2", options, None);
    test("-0.2", options, None);
    test("-0.002", options, None);

    test("0.1111111111", options, Some("682/683"));
    options.set_base(3);
    test("0.1111111111", options, Some("1/2"));
    options.set_base(4);
    test("0.1111111111", options, Some("1/3"));
    options.set_base(16);
    test("0.1111111111", options, Some("1/15"));
    options.set_base(32);
    test("0.1111111111", options, Some("1/31"));
    options.set_base(36);
    test("0.1111111111", options, Some("1/35"));
}

fn from_sci_string_helper(s: &str) {
    if let Some(x) = Rational::from_sci_string(s) {
        assert!(x.is_valid());
        assert!(!s.ends_with('+'));
        assert!(!s.ends_with('-'));
        assert!(!s.contains("++"));
        assert!(!s.contains("+-"));
        assert!(!s.contains("-+"));
        assert!(!s.contains("--"));
        assert!(!s.contains("-+"));
        assert!(s.chars().filter(|&c| c == '.').count() <= 1);
        assert!(s.chars().filter(|&c| c == '-').count() <= 2);
        assert!(s.chars().filter(|&c| c == '+').count() <= 2);
        let mut to_options = ToSciOptions::default();
        to_options.set_size_complete();
        let s_alt = x.to_sci_with_options(to_options).to_string();
        assert_eq!(Rational::from_sci_string(&s_alt).unwrap(), x);
    }
}

#[test]
fn from_sci_string_properties() {
    string_gen_var_14().test_properties(|s| {
        from_sci_string_helper(&s);
    });

    string_gen_var_15().test_properties(|s| {
        from_sci_string_helper(&s);
    });
}

fn from_sci_string_with_options_helper(s: &str, options: FromSciStringOptions) {
    if let Some(x) = Rational::from_sci_string_with_options(s, options) {
        assert!(x.is_valid());
        assert!(!s.ends_with('+'));
        assert!(!s.ends_with('-'));
        assert!(!s.contains("++"));
        assert!(!s.contains("+-"));
        assert!(!s.contains("-+"));
        assert!(!s.contains("--"));
        assert!(!s.contains("-+"));
        assert!(s.chars().filter(|&c| c == '.').count() <= 1);
        assert!(s.chars().filter(|&c| c == '-').count() <= 2);
        assert!(s.chars().filter(|&c| c == '+').count() <= 2);
        let mut to_options = ToSciOptions::default();
        to_options.set_base(options.get_base());
        to_options.set_size_complete();
        let s_alt = x.to_sci_with_options(to_options).to_string();
        assert_eq!(
            Rational::from_sci_string_with_options(&s_alt, options).unwrap(),
            x
        );
    }
}

#[test]
fn from_sci_string_with_options_properties() {
    string_from_sci_string_options_pair_gen_var_2().test_properties(|(s, options)| {
        from_sci_string_with_options_helper(&s, options);
    });

    string_from_sci_string_options_pair_gen_var_3().test_properties(|(s, options)| {
        from_sci_string_with_options_helper(&s, options);
    });

    let mut options = ToSciOptions::default();
    options.set_size_complete();
    rational_gen().test_properties(|x| {
        if x.fmt_sci_valid(options) {
            assert_eq!(
                Rational::from_sci_string(&x.to_sci_with_options(options).to_string()).unwrap(),
                x
            );
        }
    });
}

fn string_precision(s: &str, base: u8) -> u64 {
    let mut options = FromSciStringOptions::default();
    options.set_base(base);
    let mut s = preprocess_sci_string(s, options).unwrap().0;
    if s[0] == b'+' || s[0] == b'-' {
        s.remove(0);
    }
    let mut leading_zeros = 0;
    for &c in &s {
        if c != b'0' {
            break;
        }
        leading_zeros += 1;
    }
    u64::exact_from(s.len()) - leading_zeros
}

fn from_sci_string_simplest_helper(s: &str) {
    let mut from_options = FromSciStringOptions::default();
    from_options.set_base(10);
    if let Some(x) = Rational::from_sci_string_simplest(s) {
        assert!(x.is_valid());
        if x != 0u32 {
            let mut options = ToSciOptions::default();
            let precision = string_precision(s, 10);
            options.set_precision(precision);
            let s_alt = x.to_sci_with_options(options).to_string();
            let x_1 = Rational::from_sci_string_with_options(s, from_options).unwrap();
            let x_2 = Rational::from_sci_string_with_options(&s_alt, from_options).unwrap();
            // Usually x_1 == x_2. However...
            if x_1 != x_2 {
                let diff_1 = x_1 - &x;
                let diff_2 = x_2 - &x;
                assert_eq!(diff_1, -diff_2);
                let scale = u64::exact_from(
                    i64::exact_from(precision) - x.abs().floor_log_base(&Rational::from(10)) - 1,
                );
                let mut options_2 = ToSciOptions::default();
                options_2.set_scale(scale);
                assert_eq!(diff_1.abs().to_sci_with_options(options_2).to_string(), "0");
            }
        }
    }
}

#[test]
fn from_sci_string_simplest_properties() {
    string_gen_var_14().test_properties(|s| {
        from_sci_string_simplest_helper(&s);
    });

    string_gen_var_15().test_properties(|s| {
        from_sci_string_simplest_helper(&s);
    });

    let mut options = ToSciOptions::default();
    options.set_include_trailing_zeros(true);
    rational_gen_var_7().test_properties(|q| {
        assert_eq!(
            Rational::from_sci_string_simplest(&q.to_sci_with_options(options).to_string())
                .unwrap(),
            q
        );
    });
}

fn from_sci_string_simplest_with_options_helper(s: &str, options: FromSciStringOptions) {
    if let Some(x) = Rational::from_sci_string_simplest_with_options(s, options) {
        assert!(x.is_valid());
        assert!(!s.ends_with('+'));
        assert!(!s.ends_with('-'));
        assert!(!s.contains("++"));
        assert!(!s.contains("+-"));
        assert!(!s.contains("-+"));
        assert!(!s.contains("--"));
        assert!(!s.contains("-+"));
        assert!(s.chars().filter(|&c| c == '.').count() <= 1);
        assert!(s.chars().filter(|&c| c == '-').count() <= 2);
        assert!(s.chars().filter(|&c| c == '+').count() <= 2);
        if x != 0u32 {
            let base = options.get_base();
            let mut to_options = ToSciOptions::default();
            to_options.set_base(base);
            let precision = string_precision(s, base);
            to_options.set_precision(precision);
            let mut from_options = FromSciStringOptions::default();
            from_options.set_base(base);
            let s_alt = x.to_sci_with_options(to_options).to_string();
            let x_1 = Rational::from_sci_string_with_options(s, from_options).unwrap();
            let x_2 = Rational::from_sci_string_with_options(&s_alt, from_options).unwrap();
            // Usually x_1 == x_2. However...
            if x_1 != x_2 {
                let diff_1 = x_1 - &x;
                let diff_2 = x_2 - &x;
                assert_eq!(diff_1, -diff_2);
                let scale = u64::exact_from(
                    i64::exact_from(precision) - x.abs().floor_log_base(&Rational::from(base)) - 1,
                );
                let mut options_2 = ToSciOptions::default();
                options_2.set_base(base);
                options_2.set_scale(scale);
                assert_eq!(diff_1.abs().to_sci_with_options(options_2).to_string(), "0");
            }
        }
    }
}

#[test]
fn from_sci_string_simplest_with_options_properties() {
    string_from_sci_string_options_pair_gen_var_2().test_properties(|(s, options)| {
        from_sci_string_simplest_with_options_helper(&s, options);
    });

    string_from_sci_string_options_pair_gen_var_3().test_properties(|(s, options)| {
        from_sci_string_simplest_with_options_helper(&s, options);
    });

    rational_unsigned_pair_gen_var_6().test_properties(|(q, base)| {
        let mut to_options = ToSciOptions::default();
        to_options.set_include_trailing_zeros(true);
        to_options.set_base(base);
        let mut from_options = FromSciStringOptions::default();
        from_options.set_base(base);
        assert_eq!(
            Rational::from_sci_string_simplest_with_options(
                &q.to_sci_with_options(to_options).to_string(),
                from_options
            )
            .unwrap(),
            q
        );
    });
}
