// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::string::options::FromSciStringOptions;
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_base::test_util::generators::{
    signed_gen, string_from_sci_string_options_pair_gen,
    string_from_sci_string_options_pair_gen_var_1, string_gen, string_gen_var_13, unsigned_gen,
};

#[test]
pub fn test_from_sci_string() {
    fn test<T: PrimitiveInt>(s: &str, out: Option<T>) {
        assert_eq!(T::from_sci_string(s), out);
        assert_eq!(
            T::from_sci_string_with_options(s, FromSciStringOptions::default()),
            out
        );
    }
    test::<u8>("0", Some(0));
    test::<u8>("00", Some(0));
    test::<u8>("+0", Some(0));
    test::<u8>("-0", Some(0));
    test::<u8>("0.00", Some(0));
    test::<u8>("0e1", Some(0));
    test::<u8>("0e+1", Some(0));
    test::<u8>("0e-1", Some(0));
    test::<u8>("+0e+1", Some(0));
    test::<u8>("-0e+1", Some(0));
    test::<u8>("+0.0e+1", Some(0));
    test::<u8>("-0.0e+1", Some(0));
    test::<u8>(".0", Some(0));
    test::<u8>(".00", Some(0));
    test::<u8>(".00e0", Some(0));
    test::<u8>(".00e1", Some(0));
    test::<u8>(".00e-1", Some(0));
    test::<u8>("-.0", Some(0));
    test::<u8>("-.00", Some(0));
    test::<u8>("-.00e0", Some(0));
    test::<u8>("-.00e1", Some(0));
    test::<u8>("-.00e-1", Some(0));
    test::<u8>("+.0", Some(0));
    test::<u8>("+.00", Some(0));
    test::<u8>("+.00e0", Some(0));
    test::<u8>("+.00e1", Some(0));
    test::<u8>("+.00e-1", Some(0));

    test::<u8>("123", Some(123));
    test::<u8>("00123", Some(123));
    test::<u8>("+123", Some(123));
    test::<u8>("123.00", Some(123));
    test::<u8>("123e0", Some(123));
    test::<u8>("12.3e1", Some(123));
    test::<u8>("1.23e2", Some(123));
    test::<u8>("1.23E2", Some(123));
    test::<u8>("1.23e+2", Some(123));
    test::<u8>("1.23E+2", Some(123));
    test::<u8>(".123e3", Some(123));
    test::<u8>("0.123e3", Some(123));
    test::<u8>("+0.123e3", Some(123));
    test::<u8>("0.0123e4", Some(123));
    test::<u8>("1230e-1", Some(123));
    test::<u8>("12300e-2", Some(123));
    test::<u8>("12300E-2", Some(123));

    test::<u8>("123.4", Some(123));
    test::<u8>("123.8", Some(124));
    test::<u8>("123.5", Some(124));
    test::<u8>("124.5", Some(124));
    test::<u8>("255.49", Some(255));
    test::<u8>("255.5", None);

    test::<u8>("0.5", Some(0));
    test::<u8>("0.51", Some(1));
    test::<u8>("0.001", Some(0));
    test::<u8>("1e-10", Some(0));
    test::<u8>("-0.5", Some(0));
    test::<u8>("-0.51", None);
    test::<u8>("-0.001", Some(0));
    test::<u8>("-1e-10", Some(0));

    test::<u8>("", None);
    test::<u8>("+", None);
    test::<u8>("-", None);
    test::<u8>("10e", None);
    test::<u8>("++1", None);
    test::<u8>("256", None);
    test::<u8>("-1", None);
    test::<u8>("1e10", None);
    test::<u8>("1.0.0", None);
    test::<u8>("1e++1", None);
    test::<u8>("1e0.1", None);
    test::<u8>("--.0", None);
    test::<u8>("++.0", None);
    test::<u8>("0.000a", None);
    test::<u8>("0.00ae-10", None);

    test::<i8>("0", Some(0));
    test::<i8>("00", Some(0));
    test::<i8>("+0", Some(0));
    test::<i8>("-0", Some(0));
    test::<i8>("0.00", Some(0));
    test::<i8>("0e1", Some(0));
    test::<i8>("0e+1", Some(0));
    test::<i8>("0e-1", Some(0));
    test::<i8>("+0e+1", Some(0));
    test::<i8>("-0e+1", Some(0));
    test::<i8>("+0.0e+1", Some(0));
    test::<i8>("-0.0e+1", Some(0));
    test::<i8>(".0", Some(0));
    test::<i8>(".00", Some(0));
    test::<i8>(".00e0", Some(0));
    test::<i8>(".00e1", Some(0));
    test::<i8>(".00e-1", Some(0));
    test::<i8>("-.0", Some(0));
    test::<i8>("-.00", Some(0));
    test::<i8>("-.00e0", Some(0));
    test::<i8>("-.00e1", Some(0));
    test::<i8>("-.00e-1", Some(0));
    test::<i8>("+.0", Some(0));
    test::<i8>("+.00", Some(0));
    test::<i8>("+.00e0", Some(0));
    test::<i8>("+.00e1", Some(0));
    test::<i8>("+.00e-1", Some(0));

    test::<i8>("123", Some(123));
    test::<i8>("00123", Some(123));
    test::<i8>("+123", Some(123));
    test::<i8>("123.00", Some(123));
    test::<i8>("123e0", Some(123));
    test::<i8>("12.3e1", Some(123));
    test::<i8>("1.23e2", Some(123));
    test::<i8>("1.23E2", Some(123));
    test::<i8>("1.23e+2", Some(123));
    test::<i8>("1.23E+2", Some(123));
    test::<i8>(".123e3", Some(123));
    test::<i8>("0.123e3", Some(123));
    test::<i8>("+0.123e3", Some(123));
    test::<i8>("0.0123e4", Some(123));
    test::<i8>("1230e-1", Some(123));
    test::<i8>("12300e-2", Some(123));
    test::<i8>("12300E-2", Some(123));

    test::<i8>("-123", Some(-123));
    test::<i8>("-00123", Some(-123));
    test::<i8>("-123.00", Some(-123));
    test::<i8>("-123e0", Some(-123));
    test::<i8>("-12.3e1", Some(-123));
    test::<i8>("-1.23e2", Some(-123));
    test::<i8>("-1.23E2", Some(-123));
    test::<i8>("-1.23e+2", Some(-123));
    test::<i8>("-1.23E+2", Some(-123));
    test::<i8>("-.123e3", Some(-123));
    test::<i8>("-0.123e3", Some(-123));
    test::<i8>("-0.0123e4", Some(-123));
    test::<i8>("-1230e-1", Some(-123));
    test::<i8>("-12300e-2", Some(-123));
    test::<i8>("-12300E-2", Some(-123));

    test::<i8>("123.4", Some(123));
    test::<i8>("123.8", Some(124));
    test::<i8>("123.5", Some(124));
    test::<i8>("124.5", Some(124));
    test::<i8>("127.49", Some(127));
    test::<i8>("127.5", None);

    test::<i8>("-123.4", Some(-123));
    test::<i8>("-123.8", Some(-124));
    test::<i8>("-123.5", Some(-124));
    test::<i8>("-124.5", Some(-124));
    test::<i8>("-127.49", Some(-127));
    test::<i8>("-127.5", Some(-128));
    test::<i8>("-128.51", None);

    test::<i8>("", None);
    test::<i8>("+", None);
    test::<i8>("-", None);
    test::<i8>("10e", None);
    test::<i8>("++1", None);
    test::<i8>("128", None);
    test::<i8>("-129", None);
    test::<i8>("1e10", None);
    test::<i8>("1.0.0", None);
    test::<i8>("1e++1", None);
    test::<i8>("1e0.1", None);
    test::<i8>("--.0", None);
    test::<i8>("++.0", None);
    test::<i8>(".+2", None);
    test::<i8>(".-2", None);
    test::<i8>("0.000a", None);
    test::<i8>("0.00ae-10", None);
    test::<i8>("0e10000000000000000000000000000", None);
    test::<i8>("0e-10000000000000000000000000000", None);
}

#[test]
pub fn test_from_sci_string_with_options() {
    fn test<T: PrimitiveInt>(s: &str, options: FromSciStringOptions, out: Option<T>) {
        assert_eq!(T::from_sci_string_with_options(s, options), out);
    }
    // For tests with the default options, see `test_from_sci_string`

    let mut options = FromSciStringOptions::default();
    options.set_base(2);
    test::<u128>(
        "11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111\
        111111111111111111111111111111111111111",
        options,
        Some(u128::MAX),
    );
    options.set_base(3);
    test::<u128>(
        "202201102121002021012000211012011021221022212021111001022110211020010021100121010",
        options,
        Some(u128::MAX),
    );
    options.set_base(4);
    test::<u128>(
        "3333333333333333333333333333333333333333333333333333333333333333",
        options,
        Some(u128::MAX),
    );
    options.set_base(5);
    test::<u128>(
        "11031110441201303134210404233413032443021130230130231310",
        options,
        Some(u128::MAX),
    );
    options.set_base(8);
    test::<u128>(
        "3777777777777777777777777777777777777777777",
        options,
        Some(u128::MAX),
    );
    options.set_base(16);
    test::<u128>("ffffffffffffffffffffffffffffffff", options, Some(u128::MAX));
    options.set_base(32);
    test::<u128>("7vvvvvvvvvvvvvvvvvvvvvvvvv", options, Some(u128::MAX));
    options.set_base(36);
    test::<u128>("f5lxx1zz5pnorynqglhzmsp33", options, Some(u128::MAX));

    options.set_base(2);
    test::<i128>(
        "11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111\
        11111111111111111111111111111111111111",
        options,
        Some(i128::MAX),
    );
    test::<i128>(
        "-1000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        0000000000000000000000000000000000000000",
        options,
        Some(i128::MIN),
    );
    options.set_base(3);
    test::<i128>(
        "101100201022001010121000102002120122110122221010202000122201220121120010200022001",
        options,
        Some(i128::MAX),
    );
    test::<i128>(
        "-101100201022001010121000102002120122110122221010202000122201220121120010200022002",
        options,
        Some(i128::MIN),
    );
    options.set_base(4);
    test::<i128>(
        "1333333333333333333333333333333333333333333333333333333333333333",
        options,
        Some(i128::MAX),
    );
    test::<i128>(
        "-2000000000000000000000000000000000000000000000000000000000000000",
        options,
        Some(i128::MIN),
    );
    options.set_base(5);
    test::<i128>(
        "3013030220323124042102424341431241221233040112312340402",
        options,
        Some(i128::MAX),
    );
    test::<i128>(
        "-3013030220323124042102424341431241221233040112312340403",
        options,
        Some(i128::MIN),
    );
    options.set_base(8);
    test::<i128>(
        "1777777777777777777777777777777777777777777",
        options,
        Some(i128::MAX),
    );
    test::<i128>(
        "-2000000000000000000000000000000000000000000",
        options,
        Some(i128::MIN),
    );
    options.set_base(16);
    test::<i128>("7fffffffffffffffffffffffffffffff", options, Some(i128::MAX));
    test::<i128>(
        "-80000000000000000000000000000000",
        options,
        Some(i128::MIN),
    );
    options.set_base(32);
    test::<i128>("3vvvvvvvvvvvvvvvvvvvvvvvvv", options, Some(i128::MAX));
    test::<i128>("-40000000000000000000000000", options, Some(i128::MIN));
    options.set_base(36);
    test::<i128>("7ksyyizzkutudzbv8aqztecjj", options, Some(i128::MAX));
    test::<i128>("-7ksyyizzkutudzbv8aqztecjk", options, Some(i128::MIN));

    options.set_base(2);
    test::<u32>("1e+5", options, Some(32));
    test::<u32>("1e5", options, Some(32));
    options.set_base(3);
    test::<u32>("1e+5", options, Some(243));
    test::<u32>("1e5", options, Some(243));
    options.set_base(4);
    test::<u32>("1e+5", options, Some(1024));
    test::<u32>("1e5", options, Some(1024));
    options.set_base(5);
    test::<u32>("1e+5", options, Some(3125));
    test::<u32>("1e5", options, Some(3125));
    options.set_base(8);
    test::<u32>("1e+5", options, Some(32768));
    test::<u32>("1e5", options, Some(32768));
    options.set_base(16);
    test::<u32>("1e+5", options, Some(1048576));
    test::<u32>("1e5", options, Some(485));
    options.set_base(32);
    test::<u32>("1e+5", options, Some(33554432));
    test::<u32>("1e5", options, Some(1477));
    options.set_base(36);
    test::<u32>("1e+5", options, Some(60466176));
    test::<u32>("1E+5", options, Some(60466176));
    test::<u32>("1e5", options, Some(1805));

    options.set_base(16);
    test::<u8>("ff", options, Some(255));
    test::<u8>("fF", options, Some(255));
    test::<u8>("Ff", options, Some(255));
    test::<u8>("FF", options, Some(255));

    options = FromSciStringOptions::default();
    options.set_rounding_mode(Down);
    test::<u8>("123.4", options, Some(123));
    options.set_rounding_mode(Floor);
    test::<u8>("123.4", options, Some(123));
    options.set_rounding_mode(Up);
    test::<u8>("123.4", options, Some(124));
    options.set_rounding_mode(Ceiling);
    test::<u8>("123.4", options, Some(124));
    options.set_rounding_mode(Nearest);
    test::<u8>("123.4", options, Some(123));
    options.set_rounding_mode(Exact);
    test::<u8>("123.4", options, None);

    options.set_rounding_mode(Down);
    test::<u8>("123.5", options, Some(123));
    options.set_rounding_mode(Floor);
    test::<u8>("123.5", options, Some(123));
    options.set_rounding_mode(Up);
    test::<u8>("123.5", options, Some(124));
    options.set_rounding_mode(Ceiling);
    test::<u8>("123.5", options, Some(124));
    options.set_rounding_mode(Nearest);
    test::<u8>("123.5", options, Some(124));
    options.set_rounding_mode(Exact);
    test::<u8>("123.5", options, None);

    options.set_rounding_mode(Down);
    test::<u8>("0.4", options, Some(0));
    options.set_rounding_mode(Floor);
    test::<u8>("0.4", options, Some(0));
    options.set_rounding_mode(Up);
    test::<u8>("0.4", options, Some(1));
    options.set_rounding_mode(Ceiling);
    test::<u8>("0.4", options, Some(1));
    options.set_rounding_mode(Nearest);
    test::<u8>("0.4", options, Some(0));
    options.set_rounding_mode(Exact);
    test::<u8>("0.4", options, None);

    options.set_rounding_mode(Down);
    test::<u8>("0.04", options, Some(0));
    options.set_rounding_mode(Floor);
    test::<u8>("0.04", options, Some(0));
    options.set_rounding_mode(Up);
    test::<u8>("0.04", options, Some(1));
    options.set_rounding_mode(Ceiling);
    test::<u8>("0.04", options, Some(1));
    options.set_rounding_mode(Nearest);
    test::<u8>("0.04", options, Some(0));
    options.set_rounding_mode(Exact);
    test::<u8>("0.04", options, None);

    options = FromSciStringOptions::default();
    options.set_base(2);
    // 1/2 is 0.1
    test::<u8>("1.01", options, Some(1));
    test::<u8>("1.1", options, Some(2));
    test::<u8>("1.11", options, Some(2));
    test::<u8>("0.01", options, Some(0));
    test::<u8>("0.1", options, Some(0));
    test::<u8>("0.11", options, Some(1));
    options.set_base(3);
    // 1/2 is 0.111...
    test::<u8>("1.1", options, Some(1));
    test::<u8>("1.11", options, Some(1));
    test::<u8>("1.111", options, Some(1));
    test::<u8>("1.112", options, Some(2));
    test::<u8>("0.1", options, Some(0));
    test::<u8>("0.11", options, Some(0));
    test::<u8>("0.111", options, Some(0));
    test::<u8>("0.112", options, Some(1));

    options = FromSciStringOptions::default();
    options.set_base(2);
    test::<u8>("2", options, None);
    test::<u8>("102", options, None);
    test::<u8>("12e4", options, None);
    test::<u8>("12e-4", options, None);
    test::<u8>("1.2", options, None);
    test::<u8>("0.2", options, None);
    test::<u8>("0.002", options, None);

    options = FromSciStringOptions::default();
    options.set_rounding_mode(Exact);
    test::<u8>("1.5", options, None);
    test::<u8>("1.9999999999999999999999999999", options, None);

    options.set_base(2);
    test::<i128>(
        "11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111\
        11111111111111111111111111111111111111",
        options,
        Some(i128::MAX),
    );
    test::<i128>(
        "-1000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        0000000000000000000000000000000000000000",
        options,
        Some(i128::MIN),
    );
    options.set_base(3);
    test::<i128>(
        "101100201022001010121000102002120122110122221010202000122201220121120010200022001",
        options,
        Some(i128::MAX),
    );
    test::<i128>(
        "-101100201022001010121000102002120122110122221010202000122201220121120010200022002",
        options,
        Some(i128::MIN),
    );
    options.set_base(4);
    test::<i128>(
        "1333333333333333333333333333333333333333333333333333333333333333",
        options,
        Some(i128::MAX),
    );
    test::<i128>(
        "-2000000000000000000000000000000000000000000000000000000000000000",
        options,
        Some(i128::MIN),
    );
    options.set_base(5);
    test::<i128>(
        "3013030220323124042102424341431241221233040112312340402",
        options,
        Some(i128::MAX),
    );
    test::<i128>(
        "-3013030220323124042102424341431241221233040112312340403",
        options,
        Some(i128::MIN),
    );
    options.set_base(8);
    test::<i128>(
        "1777777777777777777777777777777777777777777",
        options,
        Some(i128::MAX),
    );
    test::<i128>(
        "-2000000000000000000000000000000000000000000",
        options,
        Some(i128::MIN),
    );
    options.set_base(16);
    test::<i128>("7fffffffffffffffffffffffffffffff", options, Some(i128::MAX));
    test::<i128>(
        "-80000000000000000000000000000000",
        options,
        Some(i128::MIN),
    );
    options.set_base(32);
    test::<i128>("3vvvvvvvvvvvvvvvvvvvvvvvvv", options, Some(i128::MAX));
    test::<i128>("-40000000000000000000000000", options, Some(i128::MIN));
    options.set_base(36);
    test::<i128>("7ksyyizzkutudzbv8aqztecjj", options, Some(i128::MAX));
    test::<i128>("-7ksyyizzkutudzbv8aqztecjk", options, Some(i128::MIN));

    options.set_base(2);
    test::<i32>("1e+5", options, Some(32));
    test::<i32>("1e5", options, Some(32));
    options.set_base(3);
    test::<i32>("1e+5", options, Some(243));
    test::<i32>("1e5", options, Some(243));
    options.set_base(4);
    test::<i32>("1e+5", options, Some(1024));
    test::<i32>("1e5", options, Some(1024));
    options.set_base(5);
    test::<i32>("1e+5", options, Some(3125));
    test::<i32>("1e5", options, Some(3125));
    options.set_base(8);
    test::<i32>("1e+5", options, Some(32768));
    test::<i32>("1e5", options, Some(32768));
    options.set_base(16);
    test::<i32>("1e+5", options, Some(1048576));
    test::<i32>("1e5", options, Some(485));
    options.set_base(32);
    test::<i32>("1e+5", options, Some(33554432));
    test::<i32>("1e5", options, Some(1477));
    options.set_base(36);
    test::<i32>("1e+5", options, Some(60466176));
    test::<i32>("1E+5", options, Some(60466176));
    test::<i32>("1e5", options, Some(1805));

    options.set_base(2);
    test::<i32>("-1e+5", options, Some(-32));
    test::<i32>("-1e5", options, Some(-32));
    options.set_base(3);
    test::<i32>("-1e+5", options, Some(-243));
    test::<i32>("-1e5", options, Some(-243));
    options.set_base(4);
    test::<i32>("-1e+5", options, Some(-1024));
    test::<i32>("-1e5", options, Some(-1024));
    options.set_base(5);
    test::<i32>("-1e+5", options, Some(-3125));
    test::<i32>("-1e5", options, Some(-3125));
    options.set_base(8);
    test::<i32>("-1e+5", options, Some(-32768));
    test::<i32>("-1e5", options, Some(-32768));
    options.set_base(16);
    test::<i32>("-1e+5", options, Some(-1048576));
    test::<i32>("-1e5", options, Some(-485));
    options.set_base(32);
    test::<i32>("-1e+5", options, Some(-33554432));
    test::<i32>("-1e5", options, Some(-1477));
    options.set_base(36);
    test::<i32>("-1e+5", options, Some(-60466176));
    test::<i32>("-1E+5", options, Some(-60466176));
    test::<i32>("-1e5", options, Some(-1805));

    options.set_base(16);
    test::<i16>("ff", options, Some(255));
    test::<i16>("fF", options, Some(255));
    test::<i16>("Ff", options, Some(255));
    test::<i16>("FF", options, Some(255));
    test::<i16>("-ff", options, Some(-255));
    test::<i16>("-fF", options, Some(-255));
    test::<i16>("-Ff", options, Some(-255));
    test::<i16>("-FF", options, Some(-255));

    options = FromSciStringOptions::default();
    options.set_rounding_mode(Down);
    test::<i8>("123.4", options, Some(123));
    options.set_rounding_mode(Floor);
    test::<i8>("123.4", options, Some(123));
    options.set_rounding_mode(Up);
    test::<i8>("123.4", options, Some(124));
    options.set_rounding_mode(Ceiling);
    test::<i8>("123.4", options, Some(124));
    options.set_rounding_mode(Nearest);
    test::<i8>("123.4", options, Some(123));
    options.set_rounding_mode(Exact);
    test::<i8>("123.4", options, None);

    options.set_rounding_mode(Down);
    test::<i8>("-123.4", options, Some(-123));
    options.set_rounding_mode(Floor);
    test::<i8>("-123.4", options, Some(-124));
    options.set_rounding_mode(Up);
    test::<i8>("-123.4", options, Some(-124));
    options.set_rounding_mode(Ceiling);
    test::<i8>("-123.4", options, Some(-123));
    options.set_rounding_mode(Nearest);
    test::<i8>("-123.4", options, Some(-123));
    options.set_rounding_mode(Exact);
    test::<i8>("-123.4", options, None);

    options.set_rounding_mode(Down);
    test::<i8>("123.5", options, Some(123));
    options.set_rounding_mode(Floor);
    test::<i8>("123.5", options, Some(123));
    options.set_rounding_mode(Up);
    test::<i8>("123.5", options, Some(124));
    options.set_rounding_mode(Ceiling);
    test::<i8>("123.5", options, Some(124));
    options.set_rounding_mode(Nearest);
    test::<i8>("123.5", options, Some(124));
    options.set_rounding_mode(Exact);
    test::<i8>("123.5", options, None);

    options.set_rounding_mode(Down);
    test::<i8>("-123.5", options, Some(-123));
    options.set_rounding_mode(Floor);
    test::<i8>("-123.5", options, Some(-124));
    options.set_rounding_mode(Up);
    test::<i8>("-123.5", options, Some(-124));
    options.set_rounding_mode(Ceiling);
    test::<i8>("-123.5", options, Some(-123));
    options.set_rounding_mode(Nearest);
    test::<i8>("-123.5", options, Some(-124));
    options.set_rounding_mode(Exact);
    test::<i8>("-123.5", options, None);

    options.set_rounding_mode(Down);
    test::<i8>("0.4", options, Some(0));
    options.set_rounding_mode(Floor);
    test::<i8>("0.4", options, Some(0));
    options.set_rounding_mode(Up);
    test::<i8>("0.4", options, Some(1));
    options.set_rounding_mode(Ceiling);
    test::<i8>("0.4", options, Some(1));
    options.set_rounding_mode(Nearest);
    test::<i8>("0.4", options, Some(0));
    options.set_rounding_mode(Exact);
    test::<i8>("0.4", options, None);

    options.set_rounding_mode(Down);
    test::<i8>("-0.4", options, Some(0));
    options.set_rounding_mode(Floor);
    test::<i8>("-0.4", options, Some(-1));
    options.set_rounding_mode(Up);
    test::<i8>("-0.4", options, Some(-1));
    options.set_rounding_mode(Ceiling);
    test::<i8>("-0.4", options, Some(0));
    options.set_rounding_mode(Nearest);
    test::<i8>("-0.4", options, Some(0));
    options.set_rounding_mode(Exact);
    test::<i8>("-0.4", options, None);

    options.set_rounding_mode(Down);
    test::<i8>("0.04", options, Some(0));
    options.set_rounding_mode(Floor);
    test::<i8>("0.04", options, Some(0));
    options.set_rounding_mode(Up);
    test::<i8>("0.04", options, Some(1));
    options.set_rounding_mode(Ceiling);
    test::<i8>("0.04", options, Some(1));
    options.set_rounding_mode(Nearest);
    test::<i8>("0.04", options, Some(0));
    options.set_rounding_mode(Exact);
    test::<i8>("0.04", options, None);

    options.set_rounding_mode(Down);
    test::<i8>("-0.04", options, Some(0));
    options.set_rounding_mode(Floor);
    test::<i8>("-0.04", options, Some(-1));
    options.set_rounding_mode(Up);
    test::<i8>("-0.04", options, Some(-1));
    options.set_rounding_mode(Ceiling);
    test::<i8>("-0.04", options, Some(0));
    options.set_rounding_mode(Nearest);
    test::<i8>("-0.04", options, Some(0));
    options.set_rounding_mode(Exact);
    test::<i8>("-0.04", options, None);

    options = FromSciStringOptions::default();
    options.set_base(2);
    // 1/2 is 0.1
    test::<i8>("1.01", options, Some(1));
    test::<i8>("1.1", options, Some(2));
    test::<i8>("1.11", options, Some(2));
    test::<i8>("0.01", options, Some(0));
    test::<i8>("0.1", options, Some(0));
    test::<i8>("0.11", options, Some(1));
    test::<i8>("-1.01", options, Some(-1));
    test::<i8>("-1.1", options, Some(-2));
    test::<i8>("-1.11", options, Some(-2));
    test::<i8>("-0.01", options, Some(0));
    test::<i8>("-0.1", options, Some(0));
    test::<i8>("-0.11", options, Some(-1));
    options.set_base(3);
    // 1/2 is 0.111...
    test::<i8>("1.1", options, Some(1));
    test::<i8>("1.11", options, Some(1));
    test::<i8>("1.111", options, Some(1));
    test::<i8>("1.112", options, Some(2));
    test::<i8>("0.1", options, Some(0));
    test::<i8>("0.11", options, Some(0));
    test::<i8>("0.111", options, Some(0));
    test::<i8>("0.112", options, Some(1));
    test::<i8>("-1.1", options, Some(-1));
    test::<i8>("-1.11", options, Some(-1));
    test::<i8>("-1.111", options, Some(-1));
    test::<i8>("-1.112", options, Some(-2));
    test::<i8>("-0.1", options, Some(0));
    test::<i8>("-0.11", options, Some(0));
    test::<i8>("-0.111", options, Some(0));
    test::<i8>("-0.112", options, Some(-1));

    options = FromSciStringOptions::default();
    options.set_base(2);
    test::<i8>("2", options, None);
    test::<i8>("102", options, None);
    test::<i8>("12e4", options, None);
    test::<i8>("12e-4", options, None);
    test::<i8>("1.2", options, None);
    test::<i8>("0.2", options, None);
    test::<i8>("0.002", options, None);
    test::<i8>("-2", options, None);
    test::<i8>("-102", options, None);
    test::<i8>("-12e4", options, None);
    test::<i8>("-12e-4", options, None);
    test::<i8>("-1.2", options, None);
    test::<i8>("-0.2", options, None);
    test::<i8>("-0.002", options, None);
    options.set_base(25);
    options.set_rounding_mode(Ceiling);
    test::<i8>(".be.2", options, None);

    options = FromSciStringOptions::default();
    options.set_rounding_mode(Exact);
    test::<i8>("1.5", options, None);
    test::<i8>("1.9999999999999999999999999999", options, None);
    test::<i8>("-1.5", options, None);
    test::<i8>("-1.9999999999999999999999999999", options, None);
}

fn from_sci_string_helper_helper<T: PrimitiveInt>(s: &str) {
    if let Some(x) = T::from_sci_string(s) {
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
        for c in ['.', 'e', 'E', '+'] {
            if s.contains(c) {
                return;
            }
        }
        if s.starts_with('0') || s.starts_with("-0") {
            return;
        }
        assert_eq!(x.to_string(), s);
    }
}

fn from_sci_string_helper<T: PrimitiveInt>() {
    string_gen().test_properties(|s| {
        from_sci_string_helper_helper::<T>(&s);
    });

    string_gen_var_13().test_properties(|s| {
        from_sci_string_helper_helper::<T>(&s);
    });
}

fn from_sci_string_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_gen::<T>().test_properties(|x| {
        assert_eq!(T::from_sci_string(&x.to_string()).unwrap(), x);
    });
}

fn from_sci_string_helper_signed<T: PrimitiveSigned>() {
    signed_gen::<T>().test_properties(|x| {
        assert_eq!(T::from_sci_string(&x.to_string()).unwrap(), x);
    });
}

#[test]
fn from_sci_string_properties() {
    apply_fn_to_primitive_ints!(from_sci_string_helper);
    apply_fn_to_unsigneds!(from_sci_string_helper_unsigned);
    apply_fn_to_signeds!(from_sci_string_helper_signed);
}

fn from_sci_string_with_options_helper_helper<T: PrimitiveInt>(
    s: &str,
    options: FromSciStringOptions,
) {
    if let Some(x) = T::from_sci_string_with_options(s, options) {
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
        for c in ['.', 'e', 'E', '+'] {
            if s.contains(c) {
                return;
            }
        }
        if s.starts_with('0') || s.starts_with("-0") {
            return;
        }
        assert_eq!(x.to_string_base(options.get_base()), s.to_lowercase());
    }
}

fn from_sci_string_with_options_helper<T: PrimitiveInt>() {
    string_from_sci_string_options_pair_gen().test_properties(|(s, options)| {
        from_sci_string_with_options_helper_helper::<T>(&s, options);
    });

    string_from_sci_string_options_pair_gen_var_1().test_properties(|(s, options)| {
        from_sci_string_with_options_helper_helper::<T>(&s, options);
    });
}

#[test]
fn from_sci_string_with_options_properties() {
    apply_fn_to_primitive_ints!(from_sci_string_with_options_helper);
}
