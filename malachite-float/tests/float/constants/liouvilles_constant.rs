// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::apply_fn_to_primitive_floats;
use malachite_base::num::arithmetic::traits::IsPowerOf2;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::traits::LiouvillesConstant;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::{
    unsigned_gen_var_11, unsigned_gen_var_31, unsigned_rounding_mode_pair_gen_var_4,
};
use malachite_float::float::constants::liouvilles_constant::*;
use malachite_float::test_util::common::{test_constant, to_hex_string};
use malachite_float::test_util::float::constants::digit_constants::*;
use malachite_float::test_util::generators::{
    unsigned_pair_gen_var_51, unsigned_unsigned_rounding_mode_triple_gen_var_10,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use std::cmp::Ordering::{self, *};

fn test_liouvilles_constant_base_prec_helper(
    base: u64,
    prec: u64,
    out: &str,
    out_hex: &str,
    out_o: Ordering,
) {
    let (x, o) = Float::liouvilles_constant_base_prec(base, prec);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);

    let (x_alt, o_alt) = liouvilles_constant_base_prec_round_naive(base, prec, Nearest);
    assert_eq!(x, x_alt);
    assert_eq!(o, o_alt);
}

// Regression test: in base 3 each digit is worth only one whole bit, so at this precision the
// bracket's numerator and denominator both exceed `MAX_EXPONENT` bits and have no `Float` of their
// own, even though their quotient is an ordinary number below 1. Converting them individually used
// to panic; the quotient now goes through a `Rational` instead. Check that the result is finite and
// that its leading 100 bits match a directly computed low-precision value. (~130 MB intermediates
// and about a minute and a half, so release-only.)
#[test]
fn test_liouvilles_constant_base_prec_high() {
    let (high, _) = Float::liouvilles_constant_base_prec(3, 678_000_000);
    assert!(high.is_valid());
    assert!(high.is_normal());
    assert_eq!(high.get_prec(), Some(678_000_000));
    let (high_rounded, _) = Float::from_float_prec_round(high, 100, Nearest);
    let (low, _) = Float::liouvilles_constant_base_prec(3, 100);
    assert_eq!(ComparableFloatRef(&high_rounded), ComparableFloatRef(&low));
}

#[test]
fn test_liouvilles_constant_base_prec() {
    test_liouvilles_constant_base_prec_helper(
        10,
        100,
        "0.11000100000000000000000099999997",
        "0x0.1c29068986fcdee34fc7466d12#100",
        Less,
    );
    test_liouvilles_constant_base_prec_helper(
        2,
        100,
        "0.76562505960464477539062500000000",
        "0x0.c400010000000000000000000#100",
        Less,
    );
    test_liouvilles_constant_base_prec_helper(
        3,
        100,
        "0.44581618656046800382951055854881",
        "0x0.722102754dfd33a95c713adf58#100",
        Greater,
    );
    test_liouvilles_constant_base_prec_helper(
        4,
        100,
        "0.31274414062500355271367880050093",
        "0x0.50100000000100000000000000#100",
        Less,
    );
    test_liouvilles_constant_base_prec_helper(
        5,
        100,
        "0.24006400000000001677721599999997",
        "0x0.3d74d594f26aed3ca0e109f660#100",
        Less,
    );
    test_liouvilles_constant_base_prec_helper(
        6,
        100,
        "0.19446587791495198923710563314571",
        "0x0.31c88409d52862577876082d08#100",
        Less,
    );
    test_liouvilles_constant_base_prec_helper(
        7,
        100,
        "0.16327380598220129368387402019417",
        "0x0.29cc4fe8fcac4682a41d9c297c#100",
        Greater,
    );
    test_liouvilles_constant_base_prec_helper(
        8,
        100,
        "0.14062881469726562500021175823681",
        "0x0.24004000000000000100000000#100",
        Less,
    );
    test_liouvilles_constant_base_prec_helper(
        9,
        100,
        "0.12345867179987994904421499745653",
        "0x0.1f9afccdc9bfbb383a4cbd8b78#100",
        Less,
    );
    test_liouvilles_constant_base_prec_helper(
        11,
        100,
        "0.099174118192938318240241335104335",
        "0x0.1963799a3f91289c482cd3fbc8#100",
        Less,
    );
    test_liouvilles_constant_base_prec_helper(
        12,
        100,
        "0.090278112675754458161865581852105",
        "0x0.171c77657ca9f6dea3a4b6490e#100",
        Greater,
    );
    test_liouvilles_constant_base_prec_helper(
        13,
        100,
        "0.082840443862601565547756293835658",
        "0x0.153508052d0e2e727103906be8#100",
        Greater,
    );
    test_liouvilles_constant_base_prec_helper(
        14,
        100,
        "0.076530745055206589091279994191191",
        "0x0.139784d7268f6d2ef00e84b66c#100",
        Less,
    );
    test_liouvilles_constant_base_prec_helper(
        15,
        100,
        "0.071111198902606310013717421184271",
        "0x0.123457f1aa81f0079850e120d0#100",
        Greater,
    );
    test_liouvilles_constant_base_prec_helper(
        16,
        100,
        "0.066406309604644775390625000012622",
        "0x0.11000100000000000000000100#100",
        Less,
    );
}

fn test_liouvilles_constant_base_prec_round_helper(
    base: u64,
    prec: u64,
    rm: RoundingMode,
    out: &str,
    out_hex: &str,
    out_o: Ordering,
) {
    let (x, o) = Float::liouvilles_constant_base_prec_round(base, prec, rm);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);

    let (x_alt, o_alt) = liouvilles_constant_base_prec_round_naive(base, prec, rm);
    assert_eq!(x, x_alt);
    assert_eq!(o, o_alt);
}

#[test]
fn test_liouvilles_constant_base_prec_round() {
    test_liouvilles_constant_base_prec_round_helper(
        2,
        100,
        Floor,
        "0.76562505960464477539062500000000",
        "0x0.c400010000000000000000000#100",
        Less,
    );
    test_liouvilles_constant_base_prec_round_helper(
        2,
        100,
        Ceiling,
        "0.76562505960464477539062500000079",
        "0x0.c400010000000000000000001#100",
        Greater,
    );
    test_liouvilles_constant_base_prec_round_helper(
        2,
        100,
        Down,
        "0.76562505960464477539062500000000",
        "0x0.c400010000000000000000000#100",
        Less,
    );
    test_liouvilles_constant_base_prec_round_helper(
        2,
        100,
        Up,
        "0.76562505960464477539062500000079",
        "0x0.c400010000000000000000001#100",
        Greater,
    );
    test_liouvilles_constant_base_prec_round_helper(
        2,
        100,
        Nearest,
        "0.76562505960464477539062500000000",
        "0x0.c400010000000000000000000#100",
        Less,
    );
    test_liouvilles_constant_base_prec_round_helper(
        10,
        100,
        Floor,
        "0.11000100000000000000000099999997",
        "0x0.1c29068986fcdee34fc7466d12#100",
        Less,
    );
    test_liouvilles_constant_base_prec_round_helper(
        10,
        100,
        Ceiling,
        "0.11000100000000000000000100000007",
        "0x0.1c29068986fcdee34fc7466d14#100",
        Greater,
    );
    test_liouvilles_constant_base_prec_round_helper(
        10,
        100,
        Down,
        "0.11000100000000000000000099999997",
        "0x0.1c29068986fcdee34fc7466d12#100",
        Less,
    );
    test_liouvilles_constant_base_prec_round_helper(
        10,
        100,
        Up,
        "0.11000100000000000000000100000007",
        "0x0.1c29068986fcdee34fc7466d14#100",
        Greater,
    );
    test_liouvilles_constant_base_prec_round_helper(
        10,
        100,
        Nearest,
        "0.11000100000000000000000099999997",
        "0x0.1c29068986fcdee34fc7466d12#100",
        Less,
    );
}

// Successive Floor approximations must be bit-prefixes of one another, in every base.
#[test]
fn test_liouvilles_constant_base_prefixes() {
    for base in 2..=16 {
        test_constant(
            |prec, rm| Float::liouvilles_constant_base_prec_round(base, prec, rm),
            100,
        );
    }
}

#[test]
#[should_panic]
fn liouvilles_constant_base_prec_round_fail_1() {
    Float::liouvilles_constant_base_prec_round(10, 0, Floor);
}

#[test]
#[should_panic]
fn liouvilles_constant_base_prec_round_fail_2() {
    Float::liouvilles_constant_base_prec_round(10, 100, Exact);
}

#[test]
#[should_panic]
fn liouvilles_constant_base_prec_round_fail_3() {
    Float::liouvilles_constant_base_prec_round(1, 100, Floor);
}

#[test]
#[should_panic]
fn liouvilles_constant_base_prec_round_fail_4() {
    Float::liouvilles_constant_base_prec_round(0, 100, Floor);
}

#[test]
#[should_panic]
fn liouvilles_constant_base_prec_fail_1() {
    Float::liouvilles_constant_base_prec(10, 0);
}

#[test]
#[should_panic]
fn liouvilles_constant_base_prec_fail_2() {
    Float::liouvilles_constant_base_prec(1, 100);
}

#[test]
fn liouvilles_constant_base_prec_properties() {
    unsigned_pair_gen_var_51().test_properties(|(base, prec)| {
        let (x, o) = Float::liouvilles_constant_base_prec(base, prec);
        assert!(x.is_valid());
        assert_eq!(x.get_prec(), Some(prec));
        // The constant is irrational, so no precision can represent it exactly.
        assert_ne!(o, Equal);
        if o == Less {
            let (x_alt, o_alt) = Float::liouvilles_constant_base_prec_round(base, prec, Ceiling);
            let mut next_upper = x.clone();
            next_upper.increment();
            if !next_upper.is_power_of_2() {
                assert_eq!(ComparableFloat(x_alt), ComparableFloat(next_upper));
                assert_eq!(o_alt, Greater);
            }
        } else if !x.is_power_of_2() {
            let (x_alt, o_alt) = Float::liouvilles_constant_base_prec_round(base, prec, Floor);
            let mut next_lower = x.clone();
            next_lower.decrement();
            assert_eq!(ComparableFloat(x_alt), ComparableFloat(next_lower));
            assert_eq!(o_alt, Less);
        }
        let (x_alt, o_alt) = Float::liouvilles_constant_base_prec_round(base, prec, Nearest);
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&x));
        assert_eq!(o_alt, o);

        let (x_alt, o_alt) = liouvilles_constant_base_prec_round_naive(base, prec, Nearest);
        assert_eq!(x, x_alt);
        assert_eq!(o, o_alt);
    });
}

#[test]
fn liouvilles_constant_base_prec_round_properties() {
    unsigned_unsigned_rounding_mode_triple_gen_var_10().test_properties(|(base, prec, rm)| {
        let (x, o) = Float::liouvilles_constant_base_prec_round(base, prec, rm);
        assert!(x.is_valid());
        assert_eq!(x.get_prec(), Some(prec));
        assert_ne!(o, Equal);
        // The constant is positive, so `Down` is `Floor` and `Up` is `Ceiling`.
        match rm {
            Floor | Down => assert_eq!(o, Less),
            Ceiling | Up => assert_eq!(o, Greater),
            Nearest => {}
            Exact => unreachable!(),
        }
        let (x_alt, o_alt) = Float::liouvilles_constant_base_prec_round(
            base,
            prec,
            if o == Less { Floor } else { Ceiling },
        );
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&x));
        assert_eq!(o_alt, o);

        let (x_alt, o_alt) = liouvilles_constant_base_prec_round_naive(base, prec, rm);
        assert_eq!(x, x_alt);
        assert_eq!(o, o_alt);
    });
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_liouvilles_constant_base() {
    fn test<T: PrimitiveFloat>(base: u64, out: T)
    where
        Float: PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(
            NiceFloat(primitive_float_liouvilles_constant_base::<T>(base)),
            NiceFloat(out)
        );
    }
    test::<f32>(2, 0.76562506);
    test::<f32>(3, 0.4458162);
    test::<f32>(4, 0.31274414);
    test::<f32>(5, 0.240064);
    test::<f32>(6, 0.19446588);
    test::<f32>(7, 0.16327381);
    test::<f32>(8, 0.14062881);
    test::<f32>(9, 0.12345867);
    test::<f32>(10, 0.110001);
    test::<f32>(11, 0.09917412);
    test::<f32>(12, 0.09027811);
    test::<f32>(13, 0.08284044);
    test::<f32>(14, 0.07653075);
    test::<f32>(15, 0.0711112);
    test::<f32>(16, 0.06640631);
    test::<f32>(62, 0.016389178);
    test::<f32>(1000, 0.001001);
    test::<f32>(18446744073709551615, 5.421011e-20);

    test::<f64>(2, 0.7656250596046448);
    test::<f64>(3, 0.445816186560468);
    test::<f64>(4, 0.31274414062500355);
    test::<f64>(5, 0.24006400000000003);
    test::<f64>(6, 0.19446587791495198);
    test::<f64>(7, 0.1632738059822013);
    test::<f64>(8, 0.14062881469726562);
    test::<f64>(9, 0.12345867179987995);
    test::<f64>(10, 0.110001);
    test::<f64>(11, 0.09917411819293832);
    test::<f64>(12, 0.09027811267575446);
    test::<f64>(13, 0.08284044386260156);
    test::<f64>(14, 0.07653074505520659);
    test::<f64>(15, 0.07111119890260631);
    test::<f64>(16, 0.06640630960464478);
    test::<f64>(62, 0.016389177957251762);
    test::<f64>(1000, 0.001001000000000001);
    test::<f64>(18446744073709551615, 5.421010862427522e-20);
}

#[test]
#[should_panic]
fn primitive_float_liouvilles_constant_base_fail_1() {
    primitive_float_liouvilles_constant_base::<f32>(1);
}

#[test]
#[should_panic]
fn primitive_float_liouvilles_constant_base_fail_2() {
    primitive_float_liouvilles_constant_base::<f64>(0);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_liouvilles_constant_base_properties_helper<T: PrimitiveFloat>()
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    unsigned_gen_var_31::<u64>().test_properties(|base| {
        let x = primitive_float_liouvilles_constant_base::<T>(base);
        // The constant lies in [1/base, 1), so it is always finite, positive, and normal.
        assert!(x.is_finite());
        assert!(x > T::ZERO);
        assert!(x < T::ONE);
        // Computing at a much higher precision and rounding once must give the same answer, which
        // is what correct rounding means.
        let (y, _) = Float::liouvilles_constant_base_prec(base, 200);
        assert_eq!(NiceFloat(x), NiceFloat(T::rounding_from(&y, Nearest).0));
    });
}

#[test]
fn primitive_float_liouvilles_constant_base_properties() {
    apply_fn_to_primitive_floats!(primitive_float_liouvilles_constant_base_properties_helper);
}

fn test_liouvilles_constant_prec_helper(prec: u64, out: &str, out_hex: &str, out_o: Ordering) {
    let (x, o) = Float::liouvilles_constant_prec(prec);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);

    // The base-10 specialization must agree with the general function at base 10.
    let (x_alt, o_alt) = Float::liouvilles_constant_base_prec(10, prec);
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&x));
    assert_eq!(o_alt, o);

    let (x_alt, o_alt) = liouvilles_constant_base_prec_round_naive(10, prec, Nearest);
    assert_eq!(x, x_alt);
    assert_eq!(o, o_alt);
}

#[test]
fn test_liouvilles_constant_prec() {
    test_liouvilles_constant_prec_helper(1, "0.12", "0x0.2#1", Greater);
    test_liouvilles_constant_prec_helper(2, "0.12", "0x0.2#2", Greater);
    test_liouvilles_constant_prec_helper(3, "0.11", "0x0.1c#3", Less);
    test_liouvilles_constant_prec_helper(4, "0.109", "0x0.1c#4", Less);
    test_liouvilles_constant_prec_helper(5, "0.109", "0x0.1c#5", Less);
    test_liouvilles_constant_prec_helper(10, "0.10999", "0x0.1c28#10", Less);
    test_liouvilles_constant_prec_helper(
        100,
        "0.11000100000000000000000099999997",
        "0x0.1c29068986fcdee34fc7466d12#100",
        Less,
    );
    test_liouvilles_constant_prec_helper(
        1000,
        "0.110001000000000000000001000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000999999999999999999999999999999999999999999999999999999999\
        999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
        999999999999999999999999999999999994",
        "0x0.1c29068986fcdee34fc7466d12a6509ed5bd46b0bc5654577d2eff086e879946a3af4c449612380413c47\
        6bd0efe751af3a7445e293084b5ee6f221653f43182b7d85f05e85d60fb7763d36fadfa945d6ff9ac7c24fd34b\
        bddeda7643ee4a091d1537ba3dfcc901383254f6c8887a94e5fd8d6b944a4b0037db7ec684da#1000",
        Less,
    );
    test_liouvilles_constant_prec_helper(
        10000,
        "0.110001000000000000000001000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
        999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
        999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
        999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
        999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
        999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
        999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
        999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
        999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
        999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
        999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
        999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
        999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
        999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
        999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
        999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
        999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
        999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
        999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
        999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
        999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
        999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
        999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
        999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
        999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\
        999999999999999999999999999999999999999999998",
        "0x0.1c29068986fcdee34fc7466d12a6509ed5bd46b0bc5654577d2eff086e879946a3af4c449612380413c47\
        6bd0efe751af3a7445e293084b5ee6f221653f43182b7d85f05e85d60fb7763d36fadfa945d6ff9ac7c24fd34b\
        bddeda7643ee4a091d1537ba3dfcc901383254f6c8887a94e5fd8d6b944a4b0037db7ec684dafe3584bd1d9a47\
        4f50daec6380d9cd24023daab7599505c8892e325cd1daf7374772be79a0aeb049b0d7389d38e1af5ac41d485e\
        3f34a21eb663b2959f1b0772da85f971e0f9f6766924945c611b3be85e6f8bf143ce6c987f013da97ea8d587cc\
        939b84f423194a7538d4dde38017970e3edc709d24273139aab0b4583d14123280fe85dd11fc876720709a5579\
        a5c1f0a815f3569db4137711c3cf244492d576da260a8e2544ac8085e721ab6875278afbaf8d1b592c88143a60\
        e82ad62cc024a570a4751a12a4f659de7146c4ceb3d45ff8393f9404c48459e26d941abb1f7d323d5be0812bc1\
        e4f508c7e51f0c19d990b8b5e42bf50471e5d94698e7c258559cb76b7e1494181af59f667b9d846a9a10d9f92a\
        ed10231c03bf52748343762d8bbc2afcc1a2d907d37f1f24f384d020ddf419aa7c7fe010c001698f5432fef035\
        f034cd6415fb4eb511675204b10c4c6a413fc1dadcf70d68fb6182ea4aad4702810d486d21d25ea23dcab61692\
        ed4a05f13dc7730312bc6c0a081631e3704131616c39870fa6053f79dd303ece0d4d9ee65941b90beabd98687a\
        e9f468b641d078e858c9b584ebaad3727ab64dbefec296f47eceba1c155e432cec353966037d8ce3e05c589845\
        70d8676c14876dd00c62dfcdb300b74502382bc7b311e25cf122e18bfdb9334d0bc6339c6d5b1fdb62d980b554\
        c677058d45f20e043339eadc3c9881bf2ffa65bcf1275d53bc7ed5089c33e6efc7dc31ee794eb8d6a8281569dc\
        a7aa8e99dbb3b2f4654c2f87cb84653e03b3dae095bd15be8bedeeff4c564c913800a5ac4d3f68cf057ad5644e\
        da86b08dde862313cb696e4f129233a22927e90afe0253be437040e23bc823fb196581062ba4073039ef0efb10\
        ace6bdb5e11937ed94b9222ff546b21dd67d966c657ecb4e396a28a0624ede6e50a3bc6dd2c7bb2bf2e6147be3\
        bb1d7dfdba727e9c831eb23418b69e74a04ae0378aeef2c5f20513bf60bb18e7a6a757591e831af9ca4383c3a8\
        c7f7dd83f25afe1fa8841c63a14fba7f69944c4dafb113098f6e4a49cdf18c6158d393bd563533fdacb19f22a4\
        70f8dc9a2ed469b9ddc4714b70b8ec2d1daee6cf5aaff09044d2d040f325568a1c4a25949aeb0e79cdfc801dbe\
        fad9175f2e8b30b50d1477baf566c4af3db1c9aa8bc7eae03aaea77f7f4de6a3b8bedf18c3945884a9c0ad0c67\
        ab4646924fddb80d566d4b8a33fcb7e130c7149e7a1a40961fb46a098ab50d892e8edae9c46a91491f9d1ab5a5\
        0f959c5c237b6d0e57449e54d0c03916a59b0f14f7831706c035d5e125c6df589930beb8d3edfde8025403a37b\
        8e0b2e4f5f116808b5a024fe5b643ea2a113651df6729393f55435e6969905eb26732e7bd69e2aa3aae29159b1\
        5f39466bc9af150cf9193158dba8727d9157f7240ab3cd11534a7af16949a474fb7098a8790038b8636476785c\
        7caf564a7da7888acb7a7c3921df2bac6cbe933a7cdd483bfef68c86008dec1bb612ac7dd171cb1eb2d4556593\
        0bcdeedda3baf8b9cf7ce806d2fbe5beb95c8fc042e9fd6ee9e783f2317b15e5f93f882b6106#10000",
        Less,
    );

    let x_f32 = Float::liouvilles_constant_prec(24).0;
    assert_eq!(x_f32.to_string(), "0.110000998");
    assert_eq!(to_hex_string(&x_f32), "0x0.1c29068#24");
    assert_eq!(x_f32, f32::LIOUVILLES_CONSTANT);

    let x_f64 = Float::liouvilles_constant_prec(53).0;
    assert_eq!(x_f64.to_string(), "0.11000100000000000");
    assert_eq!(to_hex_string(&x_f64), "0x0.1c29068986fcdf#53");
    assert_eq!(x_f64, f64::LIOUVILLES_CONSTANT);
}

fn test_liouvilles_constant_prec_round_helper(
    prec: u64,
    rm: RoundingMode,
    out: &str,
    out_hex: &str,
    out_o: Ordering,
) {
    let (x, o) = Float::liouvilles_constant_prec_round(prec, rm);
    assert!(x.is_valid());
    assert_eq!(x.to_string(), out);
    assert_eq!(to_hex_string(&x), out_hex);
    assert_eq!(o, out_o);

    let (x_alt, o_alt) = Float::liouvilles_constant_base_prec_round(10, prec, rm);
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&x));
    assert_eq!(o_alt, o);

    let (x_alt, o_alt) = liouvilles_constant_base_prec_round_naive(10, prec, rm);
    assert_eq!(x, x_alt);
    assert_eq!(o, o_alt);
}

#[test]
fn test_liouvilles_constant_prec_round() {
    test_liouvilles_constant_prec_round_helper(1, Floor, "0.062", "0x0.1#1", Less);
    test_liouvilles_constant_prec_round_helper(1, Ceiling, "0.12", "0x0.2#1", Greater);
    test_liouvilles_constant_prec_round_helper(1, Down, "0.062", "0x0.1#1", Less);
    test_liouvilles_constant_prec_round_helper(1, Up, "0.12", "0x0.2#1", Greater);
    test_liouvilles_constant_prec_round_helper(1, Nearest, "0.12", "0x0.2#1", Greater);
    test_liouvilles_constant_prec_round_helper(2, Floor, "0.094", "0x0.18#2", Less);
    test_liouvilles_constant_prec_round_helper(2, Ceiling, "0.12", "0x0.2#2", Greater);
    test_liouvilles_constant_prec_round_helper(2, Down, "0.094", "0x0.18#2", Less);
    test_liouvilles_constant_prec_round_helper(2, Up, "0.12", "0x0.2#2", Greater);
    test_liouvilles_constant_prec_round_helper(2, Nearest, "0.12", "0x0.2#2", Greater);
    test_liouvilles_constant_prec_round_helper(3, Floor, "0.11", "0x0.1c#3", Less);
    test_liouvilles_constant_prec_round_helper(3, Ceiling, "0.12", "0x0.20#3", Greater);
    test_liouvilles_constant_prec_round_helper(3, Down, "0.11", "0x0.1c#3", Less);
    test_liouvilles_constant_prec_round_helper(3, Up, "0.12", "0x0.20#3", Greater);
    test_liouvilles_constant_prec_round_helper(3, Nearest, "0.11", "0x0.1c#3", Less);
    test_liouvilles_constant_prec_round_helper(4, Floor, "0.109", "0x0.1c#4", Less);
    test_liouvilles_constant_prec_round_helper(4, Ceiling, "0.117", "0x0.1e#4", Greater);
    test_liouvilles_constant_prec_round_helper(4, Down, "0.109", "0x0.1c#4", Less);
    test_liouvilles_constant_prec_round_helper(4, Up, "0.117", "0x0.1e#4", Greater);
    test_liouvilles_constant_prec_round_helper(4, Nearest, "0.109", "0x0.1c#4", Less);
    test_liouvilles_constant_prec_round_helper(5, Floor, "0.109", "0x0.1c#5", Less);
    test_liouvilles_constant_prec_round_helper(5, Ceiling, "0.113", "0x0.1d#5", Greater);
    test_liouvilles_constant_prec_round_helper(5, Down, "0.109", "0x0.1c#5", Less);
    test_liouvilles_constant_prec_round_helper(5, Up, "0.113", "0x0.1d#5", Greater);
    test_liouvilles_constant_prec_round_helper(5, Nearest, "0.109", "0x0.1c#5", Less);
    test_liouvilles_constant_prec_round_helper(
        100,
        Floor,
        "0.11000100000000000000000099999997",
        "0x0.1c29068986fcdee34fc7466d12#100",
        Less,
    );
    test_liouvilles_constant_prec_round_helper(
        100,
        Ceiling,
        "0.11000100000000000000000100000007",
        "0x0.1c29068986fcdee34fc7466d14#100",
        Greater,
    );
    test_liouvilles_constant_prec_round_helper(
        100,
        Down,
        "0.11000100000000000000000099999997",
        "0x0.1c29068986fcdee34fc7466d12#100",
        Less,
    );
    test_liouvilles_constant_prec_round_helper(
        100,
        Up,
        "0.11000100000000000000000100000007",
        "0x0.1c29068986fcdee34fc7466d14#100",
        Greater,
    );
    test_liouvilles_constant_prec_round_helper(
        100,
        Nearest,
        "0.11000100000000000000000099999997",
        "0x0.1c29068986fcdee34fc7466d12#100",
        Less,
    );
}

#[test]
#[should_panic]
fn liouvilles_constant_prec_fail() {
    Float::liouvilles_constant_prec(0);
}

#[test]
#[should_panic]
fn liouvilles_constant_prec_round_fail_1() {
    Float::liouvilles_constant_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn liouvilles_constant_prec_round_fail_2() {
    Float::liouvilles_constant_prec_round(100, Exact);
}

#[test]
fn liouvilles_constant_prec_properties() {
    unsigned_gen_var_11().test_properties(|prec| {
        let (x, o) = Float::liouvilles_constant_prec(prec);
        assert!(x.is_valid());
        assert_eq!(x.get_prec(), Some(prec));
        assert_ne!(o, Equal);
        let (x_alt, o_alt) = Float::liouvilles_constant_base_prec(10, prec);
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&x));
        assert_eq!(o_alt, o);
    });
}

#[test]
fn liouvilles_constant_prec_round_properties() {
    unsigned_rounding_mode_pair_gen_var_4().test_properties(|(prec, rm)| {
        let (x, o) = Float::liouvilles_constant_prec_round(prec, rm);
        assert!(x.is_valid());
        assert_eq!(x.get_prec(), Some(prec));
        assert_ne!(o, Equal);
        let (x_alt, o_alt) = Float::liouvilles_constant_base_prec_round(10, prec, rm);
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&x));
        assert_eq!(o_alt, o);
    });
}
