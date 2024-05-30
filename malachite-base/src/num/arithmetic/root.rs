// Copyright © 2024 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2015 William Hart
//
//      Copyright © 2015 Fredrik Johansson
//
//      Copyright © 2015 Kushagra Singh
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

#[cfg(feature = "test_build")]
use crate::num::arithmetic::sqrt::floor_inverse_checked_binary;
#[cfg(feature = "test_build")]
use crate::num::arithmetic::traits::DivRound;
use crate::num::arithmetic::traits::{
    CeilingRoot, CeilingRootAssign, CeilingSqrt, CheckedRoot, CheckedSqrt, DivMod, FloorRoot,
    FloorRootAssign, FloorSqrt, Parity, Pow, PowerOf2, RootAssignRem, RootRem, SqrtRem, XMulYToZZ,
};
use crate::num::basic::floats::PrimitiveFloat;
use crate::num::basic::integers::PrimitiveInt;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::{
    RawMantissaAndExponent, RoundingFrom, SaturatingFrom, WrappingFrom,
};
use crate::num::logic::traits::{LowMask, SignificantBits};
use crate::rounding_modes::RoundingMode::*;
use core::cmp::Ordering::*;

const U8_CUBES: [u8; 7] = [0, 1, 8, 27, 64, 125, 216];

// This section is created by max_base.rs.
const MAX_BASE_8: [u8; 8] = [0, 255, 15, 6, 3, 3, 2, 2];

const MAX_POWER_8: [u8; 8] = [0, 255, 225, 216, 81, 243, 64, 128];

const MAX_BASE_16: [u16; 16] = [0, 65535, 255, 40, 15, 9, 6, 4, 3, 3, 3, 2, 2, 2, 2, 2];

const MAX_POWER_16: [u16; 16] = [
    0, 65535, 65025, 64000, 50625, 59049, 46656, 16384, 6561, 19683, 59049, 2048, 4096, 8192,
    16384, 32768,
];

const MAX_BASE_32: [u32; 32] = [
    0, 4294967295, 65535, 1625, 255, 84, 40, 23, 15, 11, 9, 7, 6, 5, 4, 4, 3, 3, 3, 3, 3, 2, 2, 2,
    2, 2, 2, 2, 2, 2, 2, 2,
];

const MAX_POWER_32: [u32; 32] = [
    0, 4294967295, 4294836225, 4291015625, 4228250625, 4182119424, 4096000000, 3404825447,
    2562890625, 2357947691, 3486784401, 1977326743, 2176782336, 1220703125, 268435456, 1073741824,
    43046721, 129140163, 387420489, 1162261467, 3486784401, 2097152, 4194304, 8388608, 16777216,
    33554432, 67108864, 134217728, 268435456, 536870912, 1073741824, 2147483648,
];

const MAX_BASE_64: [u64; 64] = [
    0,
    18446744073709551615,
    4294967295,
    2642245,
    65535,
    7131,
    1625,
    565,
    255,
    138,
    84,
    56,
    40,
    30,
    23,
    19,
    15,
    13,
    11,
    10,
    9,
    8,
    7,
    6,
    6,
    5,
    5,
    5,
    4,
    4,
    4,
    4,
    3,
    3,
    3,
    3,
    3,
    3,
    3,
    3,
    3,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
];

const MAX_POWER_64: [u64; 64] = [
    0,
    18446744073709551615,
    18446744065119617025,
    18446724184312856125,
    18445618199572250625,
    18439629140666724651,
    18412815093994140625,
    18379730316001328125,
    17878103347812890625,
    18151468971815029248,
    17490122876598091776,
    16985107389382393856,
    16777216000000000000,
    15943230000000000000,
    11592836324538749809,
    15181127029874798299,
    6568408355712890625,
    8650415919381337933,
    5559917313492231481,
    10000000000000000000,
    12157665459056928801,
    9223372036854775808,
    3909821048582988049,
    789730223053602816,
    4738381338321616896,
    298023223876953125,
    1490116119384765625,
    7450580596923828125,
    72057594037927936,
    288230376151711744,
    1152921504606846976,
    4611686018427387904,
    1853020188851841,
    5559060566555523,
    16677181699666569,
    50031545098999707,
    150094635296999121,
    450283905890997363,
    1350851717672992089,
    4052555153018976267,
    12157665459056928801,
    2199023255552,
    4398046511104,
    8796093022208,
    17592186044416,
    35184372088832,
    70368744177664,
    140737488355328,
    281474976710656,
    562949953421312,
    1125899906842624,
    2251799813685248,
    4503599627370496,
    9007199254740992,
    18014398509481984,
    36028797018963968,
    72057594037927936,
    144115188075855872,
    288230376151711744,
    576460752303423488,
    1152921504606846976,
    2305843009213693952,
    4611686018427387904,
    9223372036854775808,
];

const MAX_BASE_128: [u128; 128] = [
    0,
    340282366920938463463374607431768211455,
    18446744073709551615,
    6981463658331,
    4294967295,
    50859008,
    2642245,
    319557,
    65535,
    19112,
    7131,
    3183,
    1625,
    920,
    565,
    370,
    255,
    184,
    138,
    106,
    84,
    68,
    56,
    47,
    40,
    34,
    30,
    26,
    23,
    21,
    19,
    17,
    15,
    14,
    13,
    12,
    11,
    11,
    10,
    9,
    9,
    8,
    8,
    7,
    7,
    7,
    6,
    6,
    6,
    6,
    5,
    5,
    5,
    5,
    5,
    5,
    4,
    4,
    4,
    4,
    4,
    4,
    4,
    4,
    3,
    3,
    3,
    3,
    3,
    3,
    3,
    3,
    3,
    3,
    3,
    3,
    3,
    3,
    3,
    3,
    3,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
];

const MAX_POWER_128: [u128; 128] = [
    0,
    340282366920938463463374607431768211455,
    340282366920938463426481119284349108225,
    340282366920856711588743492508790678691,
    340282366604025813516997721482669850625,
    340282351457171161640582485552312352768,
    340281633132112807150397932954950015625,
    340281506971235808117106851925354131693,
    340240830764391036687105719527812890625,
    340216388952569572744243119867142602752,
    340019922845325450206316382040251071801,
    339784078391451014674643196649809097167,
    339031759685618453659117221832275390625,
    338253076642491662461829120000000000000,
    337814486488938281014651876763916015625,
    333446267951815307088493000000000000000,
    319626579315078487616775634918212890625,
    317616452802997733092688724349413228544,
    329475825834763755052723200291095445504,
    302559950208758936970093677790372560896,
    305904398238499908683087849324518834176,
    303869538891536196286006028740295917568,
    288493873028852398739253829029106548736,
    287243845682065590744605010781602099023,
    281474976710656000000000000000000000000,
    193630125104980427932766033374162714624,
    254186582832900000000000000000000000000,
    160059109085386090080713531498405298176,
    134393854047545109686936775588697536481,
    220983347100817338120002444455525554981,
    230466617897195215045509519405933293401,
    139288917338851014461418017489467720433,
    43143988327398919500410556793212890625,
    66408730383449729837806206197059026944,
    74829695578286078013428929473144712489,
    59066822915424320448445358917464096768,
    30912680532870672635673352936887453361,
    340039485861577398992406882305761986971,
    100000000000000000000000000000000000000,
    16423203268260658146231467800709255289,
    147808829414345923316083210206383297601,
    10633823966279326983230456482242756608,
    85070591730234615865843651857942052864,
    2183814375991796599109312252753832343,
    15286700631942576193765185769276826401,
    107006904423598033356356300384937784807,
    623673825204293256669089197883129856,
    3742042951225759540014535187298779136,
    22452257707354557240087211123792674816,
    134713546244127343440523266742756048896,
    88817841970012523233890533447265625,
    444089209850062616169452667236328125,
    2220446049250313080847263336181640625,
    11102230246251565404236316680908203125,
    55511151231257827021181583404541015625,
    277555756156289135105907917022705078125,
    5192296858534827628530496329220096,
    20769187434139310514121985316880384,
    83076749736557242056487941267521536,
    332306998946228968225951765070086144,
    1329227995784915872903807060280344576,
    5316911983139663491615228241121378304,
    21267647932558653966460912964485513216,
    85070591730234615865843651857942052864,
    3433683820292512484657849089281,
    10301051460877537453973547267843,
    30903154382632612361920641803529,
    92709463147897837085761925410587,
    278128389443693511257285776231761,
    834385168331080533771857328695283,
    2503155504993241601315571986085849,
    7509466514979724803946715958257547,
    22528399544939174411840147874772641,
    67585198634817523235520443624317923,
    202755595904452569706561330872953769,
    608266787713357709119683992618861307,
    1824800363140073127359051977856583921,
    5474401089420219382077155933569751763,
    16423203268260658146231467800709255289,
    49269609804781974438694403402127765867,
    147808829414345923316083210206383297601,
    2417851639229258349412352,
    4835703278458516698824704,
    9671406556917033397649408,
    19342813113834066795298816,
    38685626227668133590597632,
    77371252455336267181195264,
    154742504910672534362390528,
    309485009821345068724781056,
    618970019642690137449562112,
    1237940039285380274899124224,
    2475880078570760549798248448,
    4951760157141521099596496896,
    9903520314283042199192993792,
    19807040628566084398385987584,
    39614081257132168796771975168,
    79228162514264337593543950336,
    158456325028528675187087900672,
    316912650057057350374175801344,
    633825300114114700748351602688,
    1267650600228229401496703205376,
    2535301200456458802993406410752,
    5070602400912917605986812821504,
    10141204801825835211973625643008,
    20282409603651670423947251286016,
    40564819207303340847894502572032,
    81129638414606681695789005144064,
    162259276829213363391578010288128,
    324518553658426726783156020576256,
    649037107316853453566312041152512,
    1298074214633706907132624082305024,
    2596148429267413814265248164610048,
    5192296858534827628530496329220096,
    10384593717069655257060992658440192,
    20769187434139310514121985316880384,
    41538374868278621028243970633760768,
    83076749736557242056487941267521536,
    166153499473114484112975882535043072,
    332306998946228968225951765070086144,
    664613997892457936451903530140172288,
    1329227995784915872903807060280344576,
    2658455991569831745807614120560689152,
    5316911983139663491615228241121378304,
    10633823966279326983230456482242756608,
    21267647932558653966460912964485513216,
    42535295865117307932921825928971026432,
    85070591730234615865843651857942052864,
    170141183460469231731687303715884105728,
];

pub_test! {floor_root_approx_and_refine<T: PrimitiveUnsigned, F: Fn(T) -> f64, G: Fn(f64) -> T>(
    f: F,
    g: G,
    x: T,
    exp: u64,
) -> T {
    assert_ne!(exp, 0);
    if x == T::ZERO || exp == 1 {
        return x;
    }
    if exp >= T::WIDTH {
        return T::ONE;
    }
    let exp_usize = usize::wrapping_from(exp);
    let max_root = match T::WIDTH {
        u8::WIDTH => T::wrapping_from(MAX_BASE_8[exp_usize]),
        u16::WIDTH => T::wrapping_from(MAX_BASE_16[exp_usize]),
        u32::WIDTH => T::wrapping_from(MAX_BASE_32[exp_usize]),
        u64::WIDTH => T::wrapping_from(MAX_BASE_64[exp_usize]),
        u128::WIDTH => T::wrapping_from(MAX_BASE_128[exp_usize]),
        _ => unreachable!(),
    };
    let max_pow = match T::WIDTH {
        u8::WIDTH => T::wrapping_from(MAX_POWER_8[exp_usize]),
        u16::WIDTH => T::wrapping_from(MAX_POWER_16[exp_usize]),
        u32::WIDTH => T::wrapping_from(MAX_POWER_32[exp_usize]),
        u64::WIDTH => T::wrapping_from(MAX_POWER_64[exp_usize]),
        u128::WIDTH => T::wrapping_from(MAX_POWER_128[exp_usize]),
        _ => unreachable!(),
    };
    if x >= max_pow {
        return max_root;
    }
    let mut root = g(f(x).pow(1.0 / (exp as f64)));
    let mut pow = if let Some(pow) = root.checked_pow(exp) {
        pow
    } else {
        // set to max possible pow
        root = max_root;
        max_pow
    };
    match pow.cmp(&x) {
        Equal => root,
        Less => loop {
            root += T::ONE;
            pow = root.pow(exp);
            match pow.cmp(&x) {
                Equal => return root,
                Less => {}
                Greater => return root - T::ONE,
            }
        },
        Greater => loop {
            root -= T::ONE;
            pow = root.pow(exp);
            if pow <= x {
                return root;
            }
        },
    }
}}

// Coefficients of Chebyshev's approximation polynomial (deg 2) {c0, c1, c2} splitting 0.5 to 1 into
// 8 equal intervals
//
// Values of these coefficients of Chebyshev's approximation polynomial have been calculated from
// the python module, "mpmath" - http://mpmath.org/ function call: mpmath.chebyfit(lambda x:
// mpmath.root(x,3), [i, j], 3, error=True) where (i, j) is the  range.
//
// ```
//          c0          c1           c2        range
// 0.445434042 0.864136635 -0.335205926 [0.50000, 0.53125]
// 0.454263239 0.830878907 -0.303884962 [0.53125, 0.56250]
// 0.462761624 0.800647514 -0.276997626 [0.56250, 0.59375]
// 0.470958569 0.773024522 -0.253724515 [0.59375, 0.62500]
// 0.478879482 0.747667468 -0.233429710 [0.62500, 0.65625]
// 0.486546506 0.724292830 -0.215613166 [0.65625, 0.68750]
// 0.493979069 0.702663686 -0.199877008 [0.68750, 0.71875]
// 0.501194325 0.682580388 -0.185901247 [0.71875, 0.75000]
// 0.508207500 0.663873398 -0.173426009 [0.75000, 0.78125]
// 0.515032183 0.646397742 -0.162238357 [0.78125, 0.81250]
// 0.521680556 0.630028647 -0.152162376 [0.81250, 0.84375]
// 0.528163588 0.614658092 -0.143051642 [0.84375, 0.87500]
// 0.534491194 0.600192044 -0.134783425 [0.87500, 0.90625]
// 0.540672371 0.586548233 -0.127254189 [0.90625, 0.93750]
// 0.546715310 0.573654340 -0.120376066 [0.93750, 0.96875]
// 0.552627494 0.561446514 -0.114074068 [0.96875, 1.00000]
// ```
//
// 1^(1/3), 2^(1/3), 4^(1/3)
const FACTOR_TABLE: [f32; 3] = [1.000000, 1.259921, 1.587401];

#[allow(clippy::excessive_precision)]
const COEFF: [[f32; 3]; 16] = [
    [0.445434042, 0.864136635, -0.335205926],
    [0.454263239, 0.830878907, -0.303884962],
    [0.462761624, 0.800647514, -0.276997626],
    [0.470958569, 0.773024522, -0.253724515],
    [0.478879482, 0.747667468, -0.233429710],
    [0.486546506, 0.724292830, -0.215613166],
    [0.493979069, 0.702663686, -0.199877008],
    [0.501194325, 0.682580388, -0.185901247],
    [0.508207500, 0.663873398, -0.173426009],
    [0.515032183, 0.646397742, -0.162238357],
    [0.521680556, 0.630028647, -0.152162376],
    [0.528163588, 0.614658092, -0.143051642],
    [0.534491194, 0.600192044, -0.134783425],
    [0.540672371, 0.586548233, -0.127254189],
    [0.546715310, 0.573654340, -0.120376066],
    [0.552627494, 0.561446514, -0.114074068],
];

// n cannot be 0
//
// This is equivalent to `n_cbrt_chebyshev_approx` from
// `ulong_extras/cbrt_chebyshev_approximation.c`, FLINT 2.7.1, where `FLINT64` is `false`.
pub_test! {cbrt_chebyshev_approx_u32(n: u32) -> u32 {
    // UPPER_LIMIT is the max cube root possible for one word
    const UPPER_LIMIT: u32 = 1625; // 1625 < (2^32)^(1/3)
    const BIAS_HEX: u32 = 0x3f000000;
    const BIAS: u32 = 126;
    let (mantissa, exponent) = (n as f32).raw_mantissa_and_exponent();
    let mut mantissa = u32::wrapping_from(mantissa);
    let table_index = usize::wrapping_from(mantissa >> (f32::MANTISSA_WIDTH - 4));
    mantissa |= BIAS_HEX;
    let (exponent_over_3, exponent_rem) = (u32::wrapping_from(exponent) - BIAS).div_mod(3);

    // Calculating cube root of dec using Chebyshev approximation polynomial
    //
    // Evaluating approx polynomial at (dec) by Estrin's scheme
    let x = f32::from_bits(mantissa);
    let row = COEFF[table_index];
    let mut cbrt = ((row[0] + row[1] * x + row[2] * (x * x))
        * f32::power_of_2(i64::wrapping_from(exponent_over_3))
        * FACTOR_TABLE[usize::wrapping_from(exponent_rem)]) as u32;
    const MAX_CUBE: u32 = UPPER_LIMIT * UPPER_LIMIT * UPPER_LIMIT;
    if cbrt >= UPPER_LIMIT {
        if n >= MAX_CUBE {
            return UPPER_LIMIT;
        }
        cbrt = UPPER_LIMIT - 1;
    }
    while cbrt * cbrt * cbrt <= n {
        cbrt += 1;
        if cbrt == UPPER_LIMIT {
            break;
        }
    }
    while cbrt * cbrt * cbrt > n {
        cbrt -= 1;
    }
    cbrt
}}

// n cannot be 0
//
// This is equivalent to `n_cbrt_chebyshev_approx` from
// `ulong_extras/cbrt_chebyshev_approximation.c`, FLINT 2.7.1, where `FLINT64` is `true`.
pub_test! {cbrt_chebyshev_approx_u64(n: u64) -> u64 {
    // UPPER_LIMIT is the max cube root possible for one word
    const UPPER_LIMIT: u64 = 2642245; // 2642245 < (2^64)^(1/3)
    const BIAS_HEX: u64 = 0x3fe0000000000000;
    const BIAS: u64 = 1022;
    let (mut mantissa, exponent) = (n as f64).raw_mantissa_and_exponent();
    let table_index = usize::wrapping_from(mantissa >> (f64::MANTISSA_WIDTH - 4));
    mantissa |= BIAS_HEX;
    let (exponent_over_3, exponent_rem) = (exponent - BIAS).div_mod(3);

    // Calculating cube root of dec using Chebyshev approximation polynomial
    //
    // Evaluating approx polynomial at x by Estrin's scheme
    let x = f64::from_bits(mantissa);
    let row = COEFF[table_index];
    let mut cbrt = ((f64::from(row[0]) + f64::from(row[1]) * x + f64::from(row[2]) * (x * x))
        * f64::power_of_2(i64::wrapping_from(exponent_over_3))
        * f64::from(FACTOR_TABLE[usize::wrapping_from(exponent_rem)])) as u64;
    const MAX_CUBE: u64 = UPPER_LIMIT * UPPER_LIMIT * UPPER_LIMIT;
    if cbrt >= UPPER_LIMIT {
        if n >= MAX_CUBE {
            return UPPER_LIMIT;
        }
        cbrt = UPPER_LIMIT - 1;
    }
    while cbrt * cbrt * cbrt <= n {
        cbrt += 1;
        if cbrt == UPPER_LIMIT {
            break;
        }
    }
    while cbrt * cbrt * cbrt > n {
        cbrt -= 1;
    }
    cbrt
}}

// This is equivalent to `n_cbrt_estimate` from `ulong_extras/n_cbrt_estimate.c`, FLINT 2.7.1, where
// `FLINT64` is `true`.
#[cfg(feature = "test_build")]
fn cbrt_estimate_f64(a: f64) -> f64 {
    const S: u64 = 4607182418800017408; // ((1 << 10) - 1) << 52
    f64::from_bits(
        u64::wrapping_from((u128::from(a.to_bits() - S) * 6148914691236517205) >> 64) + S,
    )
}

// This is equivalent to `n_cbrt` from `ulong_extras/cbrt.c`, FLINT 2.7.1, where `FLINT64` is
// `false`.
#[cfg(feature = "test_build")]
pub fn fast_floor_cbrt_u32(n: u32) -> u32 {
    // Taking care of smaller roots
    if n < 125 {
        return if n >= 64 {
            4
        } else if n >= 27 {
            3
        } else if n >= 8 {
            2
        } else {
            u32::from(n >= 1)
        };
    }
    if n < 1331 {
        return if n >= 1000 {
            10
        } else if n >= 729 {
            9
        } else if n >= 512 {
            8
        } else if n >= 343 {
            7
        } else if n >= 216 {
            6
        } else {
            5
        };
    }
    if n < 4913 {
        return if n >= 4096 {
            16
        } else if n >= 3375 {
            15
        } else if n >= 2744 {
            14
        } else if n >= 2197 {
            13
        } else if n >= 1728 {
            12
        } else {
            11
        };
    }
    let val = f64::from(n);
    const UPPER_LIMIT: u32 = 1625; // 1625 < (2^32)^(1/3)
    let mut x = cbrt_estimate_f64(val);
    // Kahan's iterations to get cube root
    let xcub = x * x * x;
    let num = (xcub - val) * x;
    let den = xcub + xcub + val;
    x -= num / den;
    let mut ret = x as u32;
    const UPPER_LIMIT_CUBE: u32 = UPPER_LIMIT * UPPER_LIMIT * UPPER_LIMIT;
    // In case ret ^ 3 or (ret + 1) ^ 3 will cause overflow
    if ret >= UPPER_LIMIT {
        if n >= UPPER_LIMIT_CUBE {
            return UPPER_LIMIT;
        }
        ret = UPPER_LIMIT - 1;
    }
    while ret * ret * ret <= n {
        ret += 1;
        if ret == UPPER_LIMIT {
            break;
        }
    }
    while ret * ret * ret > n {
        ret -= 1;
    }
    ret
}

// TODO tune
#[cfg(feature = "test_build")]
const CBRT_CHEBYSHEV_THRESHOLD: u64 = 10;

// This is equivalent to `n_cbrt` from `ulong_extras/cbrt.c`, FLINT 2.7.1, where `FLINT64` is
// `true`.
#[cfg(feature = "test_build")]
pub fn fast_floor_cbrt_u64(n: u64) -> u64 {
    // Taking care of smaller roots
    if n < 125 {
        return if n >= 64 {
            4
        } else if n >= 27 {
            3
        } else if n >= 8 {
            2
        } else {
            u64::from(n >= 1)
        };
    }
    if n < 1331 {
        return if n >= 1000 {
            10
        } else if n >= 729 {
            9
        } else if n >= 512 {
            8
        } else if n >= 343 {
            7
        } else if n >= 216 {
            6
        } else {
            5
        };
    }
    if n < 4913 {
        return if n >= 4096 {
            16
        } else if n >= 3375 {
            15
        } else if n >= 2744 {
            14
        } else if n >= 2197 {
            13
        } else if n >= 1728 {
            12
        } else {
            11
        };
    }
    if n.significant_bits() > CBRT_CHEBYSHEV_THRESHOLD {
        return cbrt_chebyshev_approx_u64(n);
    }
    let val = n as f64;
    const UPPER_LIMIT: u64 = 2642245; // 2642245 < (2^64)^(1/3)
    let mut x = cbrt_estimate_f64(val);
    // Kahan's iterations to get cube root
    let xcub = x * x * x;
    let num = (xcub - val) * x;
    let den = xcub + xcub + val;
    x -= num / den;
    let mut ret = x as u64;
    const UPPER_LIMIT_CUBE: u64 = UPPER_LIMIT * UPPER_LIMIT * UPPER_LIMIT;
    // In case ret ^ 3 or (ret + 1) ^ 3 will cause overflow
    if ret >= UPPER_LIMIT {
        if n >= UPPER_LIMIT_CUBE {
            return UPPER_LIMIT;
        }
        ret = UPPER_LIMIT - 1;
    }
    while ret * ret * ret <= n {
        ret += 1;
        if ret == UPPER_LIMIT {
            break;
        }
    }
    while ret * ret * ret > n {
        ret -= 1;
    }
    ret
}

// this table contains the value of UWORD_MAX / n, for n in range [1, 32]
const MUL_FACTOR_32: [u32; 33] = [
    0,
    u32::MAX,
    2147483647,
    1431655765,
    1073741823,
    858993459,
    715827882,
    613566756,
    536870911,
    477218588,
    429496729,
    390451572,
    357913941,
    330382099,
    306783378,
    286331153,
    268435455,
    252645135,
    238609294,
    226050910,
    214748364,
    204522252,
    195225786,
    186737708,
    178956970,
    171798691,
    165191049,
    159072862,
    153391689,
    148102320,
    143165576,
    138547332,
    134217727,
];

// this table contains the value of UWORD_MAX / n, for n in range [1, 64]
const MUL_FACTOR_64: [u64; 65] = [
    0,
    u64::MAX,
    9223372036854775807,
    6148914691236517205,
    4611686018427387903,
    3689348814741910323,
    3074457345618258602,
    2635249153387078802,
    2305843009213693951,
    2049638230412172401,
    1844674407370955161,
    1676976733973595601,
    1537228672809129301,
    1418980313362273201,
    1317624576693539401,
    1229782938247303441,
    1152921504606846975,
    1085102592571150095,
    1024819115206086200,
    970881267037344821,
    922337203685477580,
    878416384462359600,
    838488366986797800,
    802032351030850070,
    768614336404564650,
    737869762948382064,
    709490156681136600,
    683212743470724133,
    658812288346769700,
    636094623231363848,
    614891469123651720,
    595056260442243600,
    576460752303423487,
    558992244657865200,
    542551296285575047,
    527049830677415760,
    512409557603043100,
    498560650640798692,
    485440633518672410,
    472993437787424400,
    461168601842738790,
    449920587163647600,
    439208192231179800,
    428994048225803525,
    419244183493398900,
    409927646082434480,
    401016175515425035,
    392483916461905353,
    384307168202282325,
    376464164769582686,
    368934881474191032,
    361700864190383365,
    354745078340568300,
    348051774975651917,
    341606371735362066,
    335395346794719120,
    329406144173384850,
    323627089012448273,
    318047311615681924,
    312656679215416129,
    307445734561825860,
    302405640552615600,
    297528130221121800,
    292805461487453200,
    288230376151711743,
];

// This is equivalent to `n_root_estimate` from `ulong_extras/root_estimate.c`, FLINT 2.7.1, where
// `FLINT64` is `false`.
fn root_estimate_32(a: f64, n: usize) -> u32 {
    let s = u32::low_mask(f32::EXPONENT_WIDTH - 1) << f32::MANTISSA_WIDTH;
    f32::from_bits(u32::x_mul_y_to_zz((a as f32).to_bits() - s, MUL_FACTOR_32[n]).0 + s) as u32
}

// This is equivalent to `n_root_estimate` from `ulong_extras/root_estimate.c`, FLINT 2.7.1, where
// `FLINT64` is `true`.
fn root_estimate_64(a: f64, n: usize) -> u64 {
    let s = u64::low_mask(f64::EXPONENT_WIDTH - 1) << f64::MANTISSA_WIDTH;
    f64::from_bits(u64::x_mul_y_to_zz(a.to_bits() - s, MUL_FACTOR_64[n]).0 + s) as u64
}

const INV_TABLE: [f64; 65] = [
    0.000000000000000,
    1.000000000000000,
    0.500000000000000,
    0.333333333333333,
    0.250000000000000,
    0.200000000000000,
    0.166666666666667,
    0.142857142857143,
    0.125000000000000,
    0.111111111111111,
    0.100000000000000,
    0.090909090909091,
    0.083333333333333,
    0.076923076923077,
    0.071428571428571,
    0.066666666666667,
    0.062500000000000,
    0.058823529411765,
    0.055555555555556,
    0.052631578947368,
    0.050000000000000,
    0.047619047619048,
    0.045454545454545,
    0.043478260869565,
    0.041666666666667,
    0.040000000000000,
    0.038461538461538,
    0.037037037037037,
    0.035714285714286,
    0.034482758620690,
    0.033333333333333,
    0.032258064516129,
    0.031250000000000,
    0.030303030303030,
    0.029411764705882,
    0.028571428571429,
    0.027777777777778,
    0.027027027027027,
    0.026315789473684,
    0.025641025641026,
    0.025000000000000,
    0.024390243902439,
    0.023809523809524,
    0.023255813953488,
    0.022727272727273,
    0.022222222222222,
    0.021739130434783,
    0.021276595744681,
    0.020833333333333,
    0.020408163265306,
    0.020000000000000,
    0.019607843137255,
    0.019230769230769,
    0.018867924528302,
    0.018518518518519,
    0.018181818181818,
    0.017857142857143,
    0.017543859649123,
    0.017241379310345,
    0.016949152542373,
    0.016666666666667,
    0.016393442622951,
    0.016129032258065,
    0.015873015873016,
    0.015625000000000,
];

// This is equivalent to `n_root` from `ulong_extras/root.c`, FLINT 2.7.1, where `FLINT64` is
// `false` and `root` is nonzero.
pub_test! {fast_floor_root_u32(n: u32, exp: u64) -> u32 {
    assert_ne!(exp, 0);
    if n < 2 || exp == 1 {
        return n;
    }
    if exp >= u32::WIDTH || n.significant_bits() <= exp {
        return 1;
    }
    if exp == 2 {
        return n.floor_sqrt();
    }
    if exp == 3 {
        return cbrt_chebyshev_approx_u32(n);
    }
    let exp = u32::wrapping_from(exp);
    let exp_usize = usize::wrapping_from(exp);
    let upper_limit = MAX_BASE_32[exp_usize]; // n <= upper_limit ^ exp
    let x = root_estimate_32(f64::from(n), exp_usize);
    // one round of Newton iteration
    let mut root = u32::rounding_from(
        (f64::from(n / x.pow(exp - 1)) - f64::from(x)) * INV_TABLE[exp_usize],
        Down,
    ).0;
    if root >= upper_limit {
        root = upper_limit - 1;
    }
    let mut pow = root.pow(exp);
    if pow == n {
        return root;
    }
    while pow <= n {
        root += 1;
        pow = root.pow(exp);
        if root == upper_limit {
            break;
        }
    }
    while pow > n {
        root -= 1;
        pow = root.pow(exp);
    }
    root
}}

// This is equivalent to `n_root` from `ulong_extras/root.c`, FLINT 2.7.1, where `FLINT64` is `true`
// and `root` is nonzero.
pub_test! {fast_floor_root_u64(n: u64, exp: u64) -> u64 {
    assert_ne!(exp, 0);
    if n < 2 || exp == 1 {
        return n;
    }
    if exp == 2 {
        return n.floor_sqrt();
    }
    if exp == 3 {
        return cbrt_chebyshev_approx_u64(n);
    }
    if exp >= u64::WIDTH || (1 << exp) > n {
        return 1;
    }
    let exp = u32::wrapping_from(exp);
    let exp_usize = usize::wrapping_from(exp);
    let upper_limit = MAX_BASE_64[exp_usize]; // n <= upper_limit ^ exp
    let x = root_estimate_64(n as f64, exp_usize);
    // one round of Newton iteration
    let mut root = u64::rounding_from(
        (((n / x.saturating_pow(exp - 1)) as f64) - x as f64) * INV_TABLE[exp_usize],
        Down,
    ).0;
    if root >= upper_limit {
        root = upper_limit - 1;
    }
    let mut pow = root.pow(exp);
    if pow == n {
        return root;
    }
    while pow <= n {
        root += 1;
        pow = root.pow(exp);
        if root == upper_limit {
            break;
        }
    }
    while pow > n {
        root -= 1;
        pow = root.pow(exp);
    }
    root
}}

pub_test! {fast_ceiling_root_u32(n: u32, exp: u64) -> u32 {
    assert_ne!(exp, 0);
    if n < 2 || exp == 1 {
        return n;
    }
    if exp >= u32::WIDTH || n.significant_bits() <= exp {
        return 2;
    }
    if exp == 2 {
        return n.ceiling_sqrt();
    }
    if exp == 3 {
        let root = cbrt_chebyshev_approx_u32(n);
        return if root.pow(3) == n { root } else { root + 1 };
    }
    let exp = u32::wrapping_from(exp);
    let exp_usize = usize::wrapping_from(exp);
    let upper_limit = MAX_BASE_32[exp_usize]; // n <= upper_limit ^ exp
    let x = root_estimate_32(f64::from(n), exp_usize);
    // one round of Newton iteration
    let mut root = u32::rounding_from(
        (f64::from(n / x.pow(exp - 1)) - f64::from(x)) * INV_TABLE[exp_usize],
        Down,
    ).0;
    if root >= upper_limit {
        root = upper_limit - 1;
    }
    let mut pow = root.pow(exp);
    if pow == n {
        return root;
    }
    while pow <= n {
        root += 1;
        pow = root.pow(exp);
        if root == upper_limit {
            break;
        }
    }
    while pow > n {
        root -= 1;
        pow = root.pow(exp);
    }
    if pow == n {
        root
    } else {
        root + 1
    }
}}

pub_test! {fast_ceiling_root_u64(n: u64, exp: u64) -> u64 {
    assert_ne!(exp, 0);
    if n < 2 || exp == 1 {
        return n;
    }
    if exp >= u64::WIDTH || n.significant_bits() <= exp {
        return 2;
    }
    if exp == 2 {
        return n.ceiling_sqrt();
    }
    if exp == 3 {
        let root = cbrt_chebyshev_approx_u64(n);
        return if root.pow(3) == n { root } else { root + 1 };
    }
    let exp = u32::wrapping_from(exp);
    let exp_usize = usize::wrapping_from(exp);
    let upper_limit = MAX_BASE_64[exp_usize]; // n <= upper_limit ^ root
    let x = root_estimate_64(n as f64, exp_usize);
    // one round of Newton iteration
    let mut root = u64::rounding_from(
        (((n / x.pow(exp - 1)) as f64) - x as f64) * INV_TABLE[exp_usize],
        Down,
    ).0;
    if root >= upper_limit {
        root = upper_limit - 1;
    }
    let mut pow = root.pow(exp);
    if pow == n {
        return root;
    }
    while pow <= n {
        root += 1;
        pow = root.pow(exp);
        if root == upper_limit {
            break;
        }
    }
    while pow > n {
        root -= 1;
        pow = root.pow(exp);
    }
    if pow == n {
        root
    } else {
        root + 1
    }
}}

pub_test! {fast_checked_root_u32(n: u32, exp: u64) -> Option<u32> {
    assert_ne!(exp, 0);
    if n < 2 || exp == 1 {
        return Some(n);
    }
    if exp >= u32::WIDTH || n.significant_bits() <= exp {
        return None;
    }
    if exp == 2 {
        return n.checked_sqrt();
    }
    if exp == 3 {
        let root = cbrt_chebyshev_approx_u32(n);
        return if root.pow(3) == n { Some(root) } else { None };
    }
    let exp = u32::wrapping_from(exp);
    let exp_usize = usize::wrapping_from(exp);
    let upper_limit = MAX_BASE_32[exp_usize]; // n <= upper_limit ^ exp
    let x = root_estimate_32(f64::from(n), exp_usize);
    // one round of Newton iteration
    let mut root = u32::rounding_from(
        (f64::from(n / x.pow(exp - 1)) - f64::from(x)) * INV_TABLE[exp_usize],
        Down,
    ).0;
    if root >= upper_limit {
        root = upper_limit - 1;
    }
    let mut pow = root.pow(exp);
    if pow == n {
        return Some(root);
    }
    while pow <= n {
        root += 1;
        pow = root.pow(exp);
        if root == upper_limit {
            break;
        }
    }
    while pow > n {
        root -= 1;
        pow = root.pow(exp);
    }
    if pow == n {
        Some(root)
    } else {
        None
    }
}}

pub_test! {fast_checked_root_u64(n: u64, exp: u64) -> Option<u64> {
    assert_ne!(exp, 0);
    if n < 2 || exp == 1 {
        return Some(n);
    }
    if exp >= u64::WIDTH || n.significant_bits() <= exp {
        return None;
    }
    if exp == 2 {
        return n.checked_sqrt();
    }
    if exp == 3 {
        let root = cbrt_chebyshev_approx_u64(n);
        return if root.pow(3) == n { Some(root) } else { None };
    }
    let exp = u32::wrapping_from(exp);
    let exp_usize = usize::wrapping_from(exp);
    let upper_limit = MAX_BASE_64[exp_usize]; // n <= upper_limit ^ root
    let x = root_estimate_64(n as f64, exp_usize);
    // one round of Newton iteration
    let mut root = u64::rounding_from(
        (((n / x.pow(exp - 1)) as f64) - x as f64) * INV_TABLE[exp_usize],
        Down,
    ).0;
    if root >= upper_limit {
        root = upper_limit - 1;
    }
    let mut pow = root.pow(exp);
    if pow == n {
        return Some(root);
    }
    while pow <= n {
        root += 1;
        pow = root.pow(exp);
        if root == upper_limit {
            break;
        }
    }
    while pow > n {
        root -= 1;
        pow = root.pow(exp);
    }
    if pow == n {
        Some(root)
    } else {
        None
    }
}}

pub_test! {fast_root_rem_u32(n: u32, exp: u64) -> (u32, u32) {
    assert_ne!(exp, 0);
    if n < 2 || exp == 1 {
        return (n, 0);
    }
    if exp >= u32::WIDTH || n.significant_bits() <= exp {
        return (1, n - 1);
    }
    if exp == 2 {
        return n.sqrt_rem();
    }
    if exp == 3 {
        let root = cbrt_chebyshev_approx_u32(n);
        let pow = root.pow(3);
        return (root, n - pow);
    }
    let exp = u32::wrapping_from(exp);
    let exp_usize = usize::wrapping_from(exp);
    let upper_limit = MAX_BASE_32[exp_usize]; // n <= upper_limit ^ exp
    let x = root_estimate_32(f64::from(n), exp_usize);
    // one round of Newton iteration
    let mut root = u32::rounding_from(
        (f64::from(n / x.pow(exp - 1)) - f64::from(x)) * INV_TABLE[exp_usize],
        Down,
    ).0;
    if root >= upper_limit {
        root = upper_limit - 1;
    }
    let mut pow = root.pow(exp);
    if pow == n {
        return (root, 0);
    }
    while pow <= n {
        root += 1;
        pow = root.pow(exp);
        if root == upper_limit {
            break;
        }
    }
    while pow > n {
        root -= 1;
        pow = root.pow(exp);
    }
    (root, n - pow)
}}

pub_test! {fast_root_rem_u64(n: u64, exp: u64) -> (u64, u64) {
    assert_ne!(exp, 0);
    if n < 2 || exp == 1 {
        return (n, 0);
    }
    if exp >= u64::WIDTH || n.significant_bits() <= exp {
        return (1, n - 1);
    }
    if exp == 2 {
        return n.sqrt_rem();
    }
    if exp == 3 {
        let root = cbrt_chebyshev_approx_u64(n);
        let pow = root.pow(3);
        return (root, n - pow);
    }
    let exp = u32::wrapping_from(exp);
    let exp_usize = usize::wrapping_from(exp);
    let upper_limit = MAX_BASE_64[exp_usize]; // n <= upper_limit ^ root
    let x = root_estimate_64(n as f64, exp_usize);
    // one round of Newton iteration
    let mut root = u64::rounding_from(
        (((n / x.pow(exp - 1)) as f64) - x as f64) * INV_TABLE[exp_usize],
        Down,
    ).0;
    if root >= upper_limit {
        root = upper_limit - 1;
    }
    let mut pow = root.pow(exp);
    if pow == n {
        return (root, 0);
    }
    while pow <= n {
        root += 1;
        pow = root.pow(exp);
        if root == upper_limit {
            break;
        }
    }
    while pow > n {
        root -= 1;
        pow = root.pow(exp);
    }
    (root, n - pow)
}}

#[cfg(feature = "test_build")]
pub fn floor_root_binary<T: PrimitiveUnsigned>(x: T, exp: u64) -> T {
    if exp == 0 {
        panic!("Cannot take 0th root");
    } else if exp == 1 || x < T::TWO {
        x
    } else {
        let bits = x.significant_bits();
        if bits <= exp {
            T::ONE
        } else {
            let p = T::power_of_2(bits.div_round(exp, Ceiling).0);
            floor_inverse_checked_binary(|i| i.checked_pow(exp), x, p >> 1, p)
        }
    }
}

#[cfg(feature = "test_build")]
pub fn ceiling_root_binary<T: PrimitiveUnsigned>(x: T, exp: u64) -> T {
    let floor_root = floor_root_binary(x, exp);
    if floor_root.pow(exp) == x {
        floor_root
    } else {
        floor_root + T::ONE
    }
}

#[cfg(feature = "test_build")]
pub fn checked_root_binary<T: PrimitiveUnsigned>(x: T, exp: u64) -> Option<T> {
    let floor_root = floor_root_binary(x, exp);
    if floor_root.pow(exp) == x {
        Some(floor_root)
    } else {
        None
    }
}

#[cfg(feature = "test_build")]
pub fn root_rem_binary<T: PrimitiveUnsigned>(x: T, exp: u64) -> (T, T) {
    let floor_root = floor_root_binary(x, exp);
    (floor_root, x - floor_root.pow(exp))
}

impl FloorRoot<u64> for u8 {
    type Output = u8;

    /// Returns the floor of the $n$th root of a [`u8`].
    ///
    /// $f(x, n) = \lfloor\sqrt\[n\]{x}\rfloor$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `exp` is zero.
    ///
    /// # Examples
    /// See [here](super::root#floor_root).
    ///
    /// # Notes
    /// The [`u8`] implementation uses lookup tables.
    #[inline]
    fn floor_root(self, exp: u64) -> u8 {
        match (self, exp) {
            (_, 0) => panic!(),
            (0 | 1, _) | (_, 1) => self,
            (_, 8..=u64::MAX) => 1,
            (x, 2) => x.floor_sqrt(),
            (x, 3) => u8::wrapping_from(match U8_CUBES.binary_search(&x) {
                Ok(i) => i,
                Err(i) => i - 1,
            }),
            (x, 4) if x < 16 => 1,
            (x, 4) if x < 81 => 2,
            (x, 5) if x < 32 => 1,
            (x, 5) if x < 243 => 2,
            (_, 4 | 5) => 3,
            (x, 6) if x < 64 => 1,
            (x, 7) if x < 128 => 1,
            (_, 6 | 7) => 2,
        }
    }
}

impl CeilingRoot<u64> for u8 {
    type Output = u8;

    /// Returns the ceiling of the $n$th root of a [`u8`].
    ///
    /// $f(x, n) = \lceil\sqrt\[n\]{x}\rceil$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `exp` is zero.
    ///
    /// # Examples
    /// See [here](super::root#ceiling_root).
    ///
    /// # Notes
    /// The [`u8`] implementation uses lookup tables.
    fn ceiling_root(self, exp: u64) -> u8 {
        match (self, exp) {
            (_, 0) => panic!(),
            (0 | 1, _) | (_, 1) => self,
            (_, 8..=u64::MAX) => 2,
            (x, 2) => x.ceiling_sqrt(),
            (x, 3) => u8::wrapping_from(match U8_CUBES.binary_search(&x) {
                Ok(i) | Err(i) => i,
            }),
            (x, 4) if x <= 16 => 2,
            (x, 4) if x <= 81 => 3,
            (x, 5) if x <= 32 => 2,
            (x, 5) if x <= 243 => 3,
            (_, 4 | 5) => 4,
            (x, 6) if x <= 64 => 2,
            (x, 7) if x <= 128 => 2,
            (_, 6 | 7) => 3,
        }
    }
}

impl CheckedRoot<u64> for u8 {
    type Output = u8;

    /// Returns the the $n$th root of a [`u8`], or `None` if the [`u8`] is not a perfect $n$th
    /// power.
    ///
    /// $$
    /// f(x, n) = \\begin{cases}
    ///     \operatorname{Some}(sqrt\[n\]{x}) & \text{if} \\quad \sqrt\[n\]{x} \in \Z, \\\\
    ///     \operatorname{None} & \textrm{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `exp` is zero.
    ///
    /// # Examples
    /// See [here](super::root#checked_root).
    ///
    /// # Notes
    /// The [`u8`] implementation uses lookup tables.
    fn checked_root(self, exp: u64) -> Option<u8> {
        match (self, exp) {
            (_, 0) => panic!(),
            (0 | 1, _) | (_, 1) => Some(self),
            (x, 2) => x.checked_sqrt(),
            (x, 3) => U8_CUBES.binary_search(&x).ok().map(u8::wrapping_from),
            (16, 4) | (32, 5) | (64, 6) | (128, 7) => Some(2),
            (81, 4) | (243, 5) => Some(3),
            _ => None,
        }
    }
}

impl RootRem<u64> for u8 {
    type RootOutput = u8;
    type RemOutput = u8;

    /// Returns the floor of the $n$th root of a [`u8`], and the remainder (the difference between
    /// the [`u8`] and the $n$th power of the floor).
    ///
    /// $f(x, n) = (\lfloor\sqrt\[n\]{x}\rfloor, x - \lfloor\sqrt\[n\]{x}\rfloor^2)$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `exp` is zero.
    ///
    /// # Examples
    /// See [here](super::root#root_rem).
    ///
    /// # Notes
    /// The [`u8`] implementation uses lookup tables.
    fn root_rem(self, exp: u64) -> (u8, u8) {
        match (self, exp) {
            (_, 0) => panic!(),
            (0 | 1, _) | (_, 1) => (self, 0),
            (x, 8..=u64::MAX) => (1, x - 1),
            (x, 2) => x.sqrt_rem(),
            (x, 3) => match U8_CUBES.binary_search(&x) {
                Ok(i) => (u8::wrapping_from(i), 0),
                Err(i) => (u8::wrapping_from(i - 1), x - U8_CUBES[i - 1]),
            },
            (x, 4) if x < 16 => (1, x - 1),
            (x, 4) if x < 81 => (2, x - 16),
            (x, 4) => (3, x - 81),
            (x, 5) if x < 32 => (1, x - 1),
            (x, 5) if x < 243 => (2, x - 32),
            (x, 5) => (3, x - 243),
            (x, 6) if x < 64 => (1, x - 1),
            (x, 6) => (2, x - 64),
            (x, 7) if x < 128 => (1, x - 1),
            (x, 7) => (2, x - 128),
        }
    }
}

impl FloorRoot<u64> for u16 {
    type Output = u16;

    /// Returns the floor of the $n$th root of a [`u16`].
    ///
    /// $f(x, n) = \lfloor\sqrt\[n\]{x}\rfloor$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `exp` is zero.
    ///
    /// # Examples
    /// See [here](super::root#checked_root).
    ///
    /// # Notes
    /// The [`u16`] implementation calls the implementation for [`u32`]s.
    #[inline]
    fn floor_root(self, exp: u64) -> u16 {
        u16::wrapping_from(u32::from(self).floor_root(exp))
    }
}

impl CeilingRoot<u64> for u16 {
    type Output = u16;

    /// Returns the ceiling of the $n$th root of a [`u16`].
    ///
    /// $f(x, n) = \lceil\sqrt\[n\]{x}\rceil$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `exp` is zero.
    ///
    /// # Examples
    /// See [here](super::root#ceiling_root).
    ///
    /// # Notes
    /// The [`u16`] implementation calls the implementation for [`u32`]s.
    #[inline]
    fn ceiling_root(self, exp: u64) -> u16 {
        u16::wrapping_from(u32::from(self).ceiling_root(exp))
    }
}

impl CheckedRoot<u64> for u16 {
    type Output = u16;

    /// Returns the the $n$th root of a [`u16`], or `None` if the [`u16`] is not a perfect $n$th
    /// power.
    ///
    /// $$
    /// f(x, n) = \\begin{cases}
    ///     \operatorname{Some}(sqrt\[n\]{x}) & \text{if} \\quad \sqrt\[n\]{x} \in \Z, \\\\
    ///     \operatorname{None} & \textrm{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `exp` is zero.
    ///
    /// # Examples
    /// See [here](super::root#checked_root).
    ///
    /// # Notes
    /// The [`u16`] implementation calls the implementation for [`u32`]s.
    #[inline]
    fn checked_root(self, exp: u64) -> Option<u16> {
        u32::from(self).checked_root(exp).map(u16::wrapping_from)
    }
}

impl RootRem<u64> for u16 {
    type RootOutput = u16;
    type RemOutput = u16;

    /// Returns the floor of the $n$th root of a [`u16`], and the remainder (the difference between
    /// the [`u16`] and the $n$th power of the floor).
    ///
    /// $f(x, n) = (\lfloor\sqrt\[n\]{x}\rfloor, x - \lfloor\sqrt\[n\]{x}\rfloor^2)$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `exp` is zero.
    ///
    /// # Examples
    /// See [here](super::root#root_rem).
    ///
    /// # Notes
    /// The [`u16`] implementation calls the implementation for [`u32`]s.
    #[inline]
    fn root_rem(self, exp: u64) -> (u16, u16) {
        let (sqrt, rem) = u32::from(self).root_rem(exp);
        (u16::wrapping_from(sqrt), u16::wrapping_from(rem))
    }
}

impl FloorRoot<u64> for u32 {
    type Output = u32;

    /// Returns the floor of the $n$th root of a [`u32`].
    ///
    /// $f(x, n) = \lfloor\sqrt\[n\]{x}\rfloor$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `exp` is zero.
    ///
    /// # Examples
    /// See [here](super::root#floor_root).
    ///
    /// # Notes
    /// For cube roots, the [`u32`] implementation uses a piecewise Chebyshev approximation. For
    /// other roots, it uses Newton's method. In both implementations, the result of these
    /// approximations is adjusted afterwards to account for error.
    #[inline]
    fn floor_root(self, exp: u64) -> u32 {
        fast_floor_root_u32(self, exp)
    }
}

impl CeilingRoot<u64> for u32 {
    type Output = u32;

    /// Returns the ceiling of the $n$th root of a [`u32`].
    ///
    /// $f(x, n) = \lceil\sqrt\[n\]{x}\rceil$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `exp` is zero.
    ///
    /// # Examples
    /// See [here](super::root#ceiling_root).
    ///
    /// # Notes
    /// For cube roots, the [`u32`] implementation uses a piecewise Chebyshev approximation. For
    /// other roots, it uses Newton's method. In both implementations, the result of these
    /// approximations is adjusted afterwards to account for error.
    #[inline]
    fn ceiling_root(self, exp: u64) -> u32 {
        fast_ceiling_root_u32(self, exp)
    }
}

impl CheckedRoot<u64> for u32 {
    type Output = u32;

    /// Returns the the $n$th root of a [`u32`], or `None` if the [`u32`] is not a perfect $n$th
    /// power.
    ///
    /// $$
    /// f(x, n) = \\begin{cases}
    ///     \operatorname{Some}(sqrt\[n\]{x}) & \text{if} \\quad \sqrt\[n\]{x} \in \Z, \\\\
    ///     \operatorname{None} & \textrm{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `exp` is zero.
    ///
    /// # Examples
    /// See [here](super::root#checked_root).
    ///
    /// # Notes
    /// For cube roots, the [`u32`] implementation uses a piecewise Chebyshev approximation. For
    /// other roots, it uses Newton's method. In both implementations, the result of these
    /// approximations is adjusted afterwards to account for error.
    #[inline]
    fn checked_root(self, exp: u64) -> Option<u32> {
        fast_checked_root_u32(self, exp)
    }
}

impl RootRem<u64> for u32 {
    type RootOutput = u32;
    type RemOutput = u32;

    /// Returns the floor of the $n$th root of a [`u32`], and the remainder (the difference between
    /// the [`u32`] and the $n$th power of the floor).
    ///
    /// $f(x, n) = (\lfloor\sqrt\[n\]{x}\rfloor, x - \lfloor\sqrt\[n\]{x}\rfloor^2)$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `exp` is zero.
    ///
    /// # Examples
    /// See [here](super::root#root_rem).
    ///
    /// # Notes
    /// For cube roots, the [`u32`] implementation uses a piecewise Chebyshev approximation. For
    /// other roots, it uses Newton's method. In both implementations, the result of these
    /// approximations is adjusted afterwards to account for error.
    #[inline]
    fn root_rem(self, exp: u64) -> (u32, u32) {
        fast_root_rem_u32(self, exp)
    }
}

impl FloorRoot<u64> for u64 {
    type Output = u64;

    /// Returns the floor of the $n$th root of a [`u64`].
    ///
    /// $f(x, n) = \lfloor\sqrt\[n\]{x}\rfloor$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `exp` is zero.
    ///
    /// # Examples
    /// See [here](super::root#floor_root).
    ///
    /// # Notes
    /// For cube roots, the [`u64`] implementation uses a piecewise Chebyshev approximation. For
    /// other roots, it uses Newton's method. In both implementations, the result of these
    /// approximations is adjusted afterwards to account for error.
    #[inline]
    fn floor_root(self, exp: u64) -> u64 {
        fast_floor_root_u64(self, exp)
    }
}

impl CeilingRoot<u64> for u64 {
    type Output = u64;

    /// Returns the ceiling of the $n$th root of a [`u64`].
    ///
    /// $f(x, n) = \lceil\sqrt\[n\]{x}\rceil$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `exp` is zero.
    ///
    /// # Examples
    /// See [here](super::root#ceiling_root).
    ///
    /// # Notes
    /// For cube roots, the [`u64`] implementation uses a piecewise Chebyshev approximation. For
    /// other roots, it uses Newton's method. In both implementations, the result of these
    /// approximations is adjusted afterwards to account for error.
    #[inline]
    fn ceiling_root(self, exp: u64) -> u64 {
        fast_ceiling_root_u64(self, exp)
    }
}

impl CheckedRoot<u64> for u64 {
    type Output = u64;

    /// Returns the the $n$th root of a [`u64`], or `None` if the [`u64`] is not a perfect $n$th
    /// power.
    ///
    /// $$
    /// f(x, n) = \\begin{cases}
    ///     \operatorname{Some}(sqrt\[n\]{x}) & \text{if} \\quad \sqrt\[n\]{x} \in \Z, \\\\
    ///     \operatorname{None} & \textrm{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `exp` is zero.
    ///
    /// # Examples
    /// See [here](super::root#checked_root).
    ///
    /// # Notes
    /// For cube roots, the [`u64`] implementation uses a piecewise Chebyshev approximation. For
    /// other roots, it uses Newton's method. In both implementations, the result of these
    /// approximations is adjusted afterwards to account for error.
    #[inline]
    fn checked_root(self, exp: u64) -> Option<u64> {
        fast_checked_root_u64(self, exp)
    }
}

impl RootRem<u64> for u64 {
    type RootOutput = u64;
    type RemOutput = u64;

    /// Returns the floor of the $n$th root of a [`u64`], and the remainder (the difference between
    /// the [`u64`] and the $n$th power of the floor).
    ///
    /// $f(x, n) = (\lfloor\sqrt\[n\]{x}\rfloor, x - \lfloor\sqrt\[n\]{x}\rfloor^2)$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `exp` is zero.
    ///
    /// # Examples
    /// See [here](super::root#root_rem).
    ///
    /// # Notes
    /// For cube roots, the [`u64`] implementation uses a piecewise Chebyshev approximation. For
    /// other roots, it uses Newton's method. In both implementations, the result of these
    /// approximations is adjusted afterwards to account for error.
    #[inline]
    fn root_rem(self, exp: u64) -> (u64, u64) {
        fast_root_rem_u64(self, exp)
    }
}

impl FloorRoot<u64> for usize {
    type Output = usize;

    /// Returns the floor of the $n$th root of a [`usize`].
    ///
    /// $f(x, n) = \lfloor\sqrt\[n\]{x}\rfloor$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `exp` is zero.
    ///
    /// # Examples
    /// See [here](super::root#floor_root).
    ///
    /// # Notes
    /// The [`usize`] implementation calls the [`u32`] or [`u64`] implementations.
    #[inline]
    fn floor_root(self, exp: u64) -> usize {
        match usize::WIDTH {
            u32::WIDTH => usize::wrapping_from(u32::wrapping_from(self).floor_root(exp)),
            u64::WIDTH => usize::wrapping_from(u64::wrapping_from(self).floor_root(exp)),
            _ => panic!("Unsupported usize size"),
        }
    }
}

impl CeilingRoot<u64> for usize {
    type Output = usize;

    /// Returns the ceiling of the $n$th root of a [`usize`].
    ///
    /// $f(x, n) = \lceil\sqrt\[n\]{x}\rceil$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `exp` is zero.
    ///
    /// # Examples
    /// See [here](super::root#ceiling_root).
    ///
    /// # Notes
    /// The [`usize`] implementation calls the [`u32`] or [`u64`] implementations.
    #[inline]
    fn ceiling_root(self, exp: u64) -> usize {
        match usize::WIDTH {
            u32::WIDTH => usize::wrapping_from(u32::wrapping_from(self).ceiling_root(exp)),
            u64::WIDTH => usize::wrapping_from(u64::wrapping_from(self).ceiling_root(exp)),
            _ => panic!("Unsupported usize size"),
        }
    }
}

impl CheckedRoot<u64> for usize {
    type Output = usize;

    /// Returns the the $n$th root of a [`usize`], or `None` if the [`usize`] is not a perfect $n$th
    /// power.
    ///
    /// $$
    /// f(x, n) = \\begin{cases}
    ///     \operatorname{Some}(sqrt\[n\]{x}) & \text{if} \\quad \sqrt\[n\]{x} \in \Z, \\\\
    ///     \operatorname{None} & \textrm{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `exp` is zero.
    ///
    /// # Examples
    /// See [here](super::root#checked_root).
    ///
    /// # Notes
    /// The [`usize`] implementation calls the [`u32`] or [`u64`] implementations.
    #[inline]
    fn checked_root(self, exp: u64) -> Option<usize> {
        match usize::WIDTH {
            u32::WIDTH => u32::wrapping_from(self)
                .checked_root(exp)
                .map(usize::wrapping_from),
            u64::WIDTH => u64::wrapping_from(self)
                .checked_root(exp)
                .map(usize::wrapping_from),
            _ => panic!("Unsupported usize size"),
        }
    }
}

impl RootRem<u64> for usize {
    type RootOutput = usize;
    type RemOutput = usize;

    /// Returns the floor of the $n$th root of a [`usize`], and the remainder (the difference
    /// between the [`usize`] and the $n$th power of the floor).
    ///
    /// $f(x, n) = (\lfloor\sqrt\[n\]{x}\rfloor, x - \lfloor\sqrt\[n\]{x}\rfloor^2)$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `exp` is zero.
    ///
    /// # Examples
    /// See [here](super::root#root_rem).
    ///
    /// # Notes
    /// The [`usize`] implementation calls the [`u32`] or [`u64`] implementations.
    #[inline]
    fn root_rem(self, exp: u64) -> (usize, usize) {
        match usize::WIDTH {
            u32::WIDTH => {
                let (sqrt, rem) = u32::wrapping_from(self).root_rem(exp);
                (usize::wrapping_from(sqrt), usize::wrapping_from(rem))
            }
            u64::WIDTH => {
                let (sqrt, rem) = u64::wrapping_from(self).root_rem(exp);
                (usize::wrapping_from(sqrt), usize::wrapping_from(rem))
            }
            _ => panic!("Unsupported usize size"),
        }
    }
}

impl FloorRoot<u64> for u128 {
    type Output = u128;

    /// Returns the floor of the $n$th root of a [`u128`].
    ///
    /// $f(x, n) = \lfloor\sqrt\[n\]{x}\rfloor$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `exp` is zero.
    ///
    /// # Examples
    /// See [here](super::root#floor_root).
    ///
    /// # Notes
    /// The [`u128`] implementation computes the root using floating-point arithmetic. The
    /// approximate result is adjusted afterwards to account for error.
    fn floor_root(self, exp: u64) -> u128 {
        if exp == 2 {
            return self.floor_sqrt();
        }
        floor_root_approx_and_refine(|x| x as f64, |x| x as u128, self, exp)
    }
}

impl CeilingRoot<u64> for u128 {
    type Output = u128;

    /// Returns the ceiling of the $n$th root of a [`u128`].
    ///
    /// $f(x, n) = \lceil\sqrt\[n\]{x}\rceil$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `exp` is zero.
    ///
    /// # Examples
    /// See [here](super::root#ceiling_root).
    ///
    /// # Notes
    /// The [`u128`] implementation computes the root using floating-point arithmetic. The
    /// approximate result is adjusted afterwards to account for error.
    fn ceiling_root(self, exp: u64) -> u128 {
        if exp == 2 {
            return self.ceiling_sqrt();
        }
        let root = floor_root_approx_and_refine(|x| x as f64, |x| x as u128, self, exp);
        if root.pow(u32::saturating_from(exp)) == self {
            root
        } else {
            root + 1
        }
    }
}

impl CheckedRoot<u64> for u128 {
    type Output = u128;

    /// Returns the the $n$th root of a [`u128`], or `None` if the [`u128`] is not a perfect $n$th
    /// power.
    ///
    /// $$
    /// f(x, n) = \\begin{cases}
    ///     \operatorname{Some}(sqrt\[n\]{x}) & \text{if} \\quad \sqrt\[n\]{x} \in \Z, \\\\
    ///     \operatorname{None} & \textrm{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `exp` is zero.
    ///
    /// # Examples
    /// See [here](super::root#checked_root).
    ///
    /// # Notes
    /// The [`u128`] implementation computes the root using floating-point arithmetic. The
    /// approximate result is adjusted afterwards to account for error.
    fn checked_root(self, exp: u64) -> Option<u128> {
        if exp == 2 {
            return self.checked_sqrt();
        }
        let root = floor_root_approx_and_refine(|x| x as f64, |x| x as u128, self, exp);
        if root.pow(u32::saturating_from(exp)) == self {
            Some(root)
        } else {
            None
        }
    }
}

impl RootRem<u64> for u128 {
    type RootOutput = u128;
    type RemOutput = u128;

    /// Returns the floor of the $n$th root of a [`u128`], and the remainder (the difference between
    /// the [`u128`] and the $n$th power of the floor).
    ///
    /// $f(x, n) = (\lfloor\sqrt\[n\]{x}\rfloor, x - \lfloor\sqrt\[n\]{x}\rfloor^n)$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `exp` is zero.
    ///
    /// # Examples
    /// See [here](super::root#root_rem).
    ///
    /// # Notes
    /// The [`u128`] implementation computes the root using floating-point arithmetic. The
    /// approximate result is adjusted afterwards to account for error.
    fn root_rem(self, exp: u64) -> (u128, u128) {
        if exp == 2 {
            return self.sqrt_rem();
        }
        let root = floor_root_approx_and_refine(|x| x as f64, |x| x as u128, self, exp);
        (root, self - root.pow(u32::saturating_from(exp)))
    }
}

macro_rules! impl_root_assign_rem {
    ($t: ident) => {
        impl RootAssignRem<u64> for $t {
            type RemOutput = $t;

            /// Replaces an integer with the floor of its $n$th root, and returns the remainder (the
            /// difference between the original integer and the $n$th power of the floor).
            ///
            /// $f(x, n) = x - \lfloor\sqrt\[n\]{x}\rfloor^n$,
            ///
            /// $x \gets \lfloor\sqrt\[n\]{x}\rfloor$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `exp` is zero.
            ///
            /// # Examples
            /// See [here](super::root#root_assign_rem).
            #[inline]
            fn root_assign_rem(&mut self, exp: u64) -> $t {
                let rem;
                (*self, rem) = self.root_rem(exp);
                rem
            }
        }
    };
}
apply_to_unsigneds!(impl_root_assign_rem);

macro_rules! impl_root_signed {
    ($t: ident) => {
        impl FloorRoot<u64> for $t {
            type Output = $t;

            /// Returns the floor of the $n$th root of an integer.
            ///
            /// $f(x, n) = \lfloor\sqrt\[n\]{x}\rfloor$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `exp` is zero, or if `self` is negative and `exp` is even.
            ///
            /// # Examples
            /// See [here](super::root#floor_root).
            #[inline]
            fn floor_root(self, exp: u64) -> $t {
                if self >= 0 {
                    $t::wrapping_from(self.unsigned_abs().floor_root(exp))
                } else if exp.odd() {
                    $t::wrapping_from(self.unsigned_abs().ceiling_root(exp)).wrapping_neg()
                } else {
                    panic!("Cannot take even root of a negative number");
                }
            }
        }

        impl CeilingRoot<u64> for $t {
            type Output = $t;

            /// Returns the ceiling of the $n$th root of an integer.
            ///
            /// $f(x, n) = \lceil\sqrt\[n\]{x}\rceil$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `exp` is zero, or if `self` is negative and `exp` is even.
            ///
            /// # Examples
            /// See [here](super::root#ceiling_root).
            #[inline]
            fn ceiling_root(self, exp: u64) -> $t {
                if self >= 0 {
                    $t::wrapping_from(self.unsigned_abs().ceiling_root(exp))
                } else if exp.odd() {
                    $t::wrapping_from(self.unsigned_abs().floor_root(exp)).wrapping_neg()
                } else {
                    panic!("Cannot take even root of a negative number");
                }
            }
        }

        impl CheckedRoot<u64> for $t {
            type Output = $t;

            /// Returns the the $n$th root of an integer, or `None` if the integer is not a perfect
            /// $n$th power.
            ///
            /// $$
            /// f(x, n) = \\begin{cases}
            ///     \operatorname{Some}(sqrt\[n\]{x}) & \text{if} \\quad \sqrt\[n\]{x} \in \Z, \\\\
            ///     \operatorname{None} & \textrm{otherwise}.
            /// \\end{cases}
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `exp` is zero, or if `self` is negative and `exp` is even.
            ///
            /// # Examples
            /// See [here](super::root#checked_root).
            #[inline]
            fn checked_root(self, exp: u64) -> Option<$t> {
                if self >= 0 {
                    self.unsigned_abs().checked_root(exp).map($t::wrapping_from)
                } else if exp.odd() {
                    self.unsigned_abs()
                        .checked_root(exp)
                        .map(|x| $t::wrapping_from(x).wrapping_neg())
                } else {
                    panic!("Cannot take even root of a negative number");
                }
            }
        }
    };
}
apply_to_signeds!(impl_root_signed);

macro_rules! impl_root_primitive_int {
    ($t: ident) => {
        impl FloorRootAssign<u64> for $t {
            /// Replaces an integer with the floor of its $n$th root.
            ///
            /// $x \gets \lfloor\sqrt\[n\]{x}\rfloor$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `exp` is zero, or if `self` is negative and `exp` is even.
            ///
            /// # Examples
            /// See [here](super::root#floor_root_assign).
            #[inline]
            fn floor_root_assign(&mut self, exp: u64) {
                *self = self.floor_root(exp);
            }
        }

        impl CeilingRootAssign<u64> for $t {
            /// Replaces an integer with the ceiling of its $n$th root.
            ///
            /// $x \gets \lceil\sqrt\[n\]{x}\rceil$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `exp` is zero, or if `self` is negative and `exp` is even.
            ///
            /// # Examples
            /// See [here](super::root#ceiling_root_assign).
            #[inline]
            fn ceiling_root_assign(&mut self, exp: u64) {
                *self = self.ceiling_root(exp);
            }
        }
    };
}
apply_to_primitive_ints!(impl_root_primitive_int);
