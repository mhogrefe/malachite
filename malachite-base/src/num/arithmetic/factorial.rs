// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{
    CheckedDoubleFactorial, CheckedFactorial, CheckedMultifactorial, CheckedSubfactorial,
    DoubleFactorial, Factorial, Multifactorial, Parity, Subfactorial,
};
use crate::num::basic::integers::USIZE_IS_U32;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::WrappingFrom;

private_test_fn! {checked_multifactorial_naive<T: PrimitiveUnsigned>(n: u64, m: u64) -> Option<T> {
    assert_ne!(m, 0);
    let mut f = T::ONE;
    let mut n = T::try_from(n).ok()?;
    let m = T::saturating_from(m);
    while n != T::ZERO {
        f = f.checked_mul(n)?;
        n.saturating_sub_assign(m);
    }
    Some(f)
}}

const FACTORIALS_U8: [u8; 6] = [1, 1, 2, 6, 24, 120];
const FACTORIALS_U16: [u16; 9] = [1, 1, 2, 6, 24, 120, 720, 5040, 40320];
const FACTORIALS_U32: [u32; 13] =
    [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880, 3628800, 39916800, 479001600];
const FACTORIALS_U64: [u64; 21] = [
    1,
    1,
    2,
    6,
    24,
    120,
    720,
    5040,
    40320,
    362880,
    3628800,
    39916800,
    479001600,
    6227020800,
    87178291200,
    1307674368000,
    20922789888000,
    355687428096000,
    6402373705728000,
    121645100408832000,
    2432902008176640000,
];
const FACTORIALS_U128: [u128; 35] = [
    1,
    1,
    2,
    6,
    24,
    120,
    720,
    5040,
    40320,
    362880,
    3628800,
    39916800,
    479001600,
    6227020800,
    87178291200,
    1307674368000,
    20922789888000,
    355687428096000,
    6402373705728000,
    121645100408832000,
    2432902008176640000,
    51090942171709440000,
    1124000727777607680000,
    25852016738884976640000,
    620448401733239439360000,
    15511210043330985984000000,
    403291461126605635584000000,
    10888869450418352160768000000,
    304888344611713860501504000000,
    8841761993739701954543616000000,
    265252859812191058636308480000000,
    8222838654177922817725562880000000,
    263130836933693530167218012160000000,
    8683317618811886495518194401280000000,
    295232799039604140847618609643520000000,
];

const ODD_DOUBLE_FACTORIALS_U8: [u8; 4] = [1, 3, 15, 105];
const ODD_DOUBLE_FACTORIALS_U16: [u16; 6] = [1, 3, 15, 105, 945, 10395];
const ODD_DOUBLE_FACTORIALS_U32: [u32; 10] =
    [1, 3, 15, 105, 945, 10395, 135135, 2027025, 34459425, 654729075];
const ODD_DOUBLE_FACTORIALS_U64: [u64; 17] = [
    1,
    3,
    15,
    105,
    945,
    10395,
    135135,
    2027025,
    34459425,
    654729075,
    13749310575,
    316234143225,
    7905853580625,
    213458046676875,
    6190283353629375,
    191898783962510625,
    6332659870762850625,
];
const ODD_DOUBLE_FACTORIALS_U128: [u128; 28] = [
    1,
    3,
    15,
    105,
    945,
    10395,
    135135,
    2027025,
    34459425,
    654729075,
    13749310575,
    316234143225,
    7905853580625,
    213458046676875,
    6190283353629375,
    191898783962510625,
    6332659870762850625,
    221643095476699771875,
    8200794532637891559375,
    319830986772877770815625,
    13113070457687988603440625,
    563862029680583509947946875,
    25373791335626257947657609375,
    1192568192774434123539907640625,
    58435841445947272053455474390625,
    2980227913743310874726229193921875,
    157952079428395476360490147277859375,
    8687364368561751199826958100282265625,
];

const SUBFACTORIALS_U8: [u8; 6] = [1, 0, 1, 2, 9, 44];
const SUBFACTORIALS_U16: [u16; 9] = [1, 0, 1, 2, 9, 44, 265, 1854, 14833];
const SUBFACTORIALS_U32: [u32; 14] =
    [1, 0, 1, 2, 9, 44, 265, 1854, 14833, 133496, 1334961, 14684570, 176214841, 2290792932];
const SUBFACTORIALS_U64: [u64; 21] = [
    1,
    0,
    1,
    2,
    9,
    44,
    265,
    1854,
    14833,
    133496,
    1334961,
    14684570,
    176214841,
    2290792932,
    32071101049,
    481066515734,
    7697064251745,
    130850092279664,
    2355301661033953,
    44750731559645106,
    895014631192902121,
];
const SUBFACTORIALS_U128: [u128; 35] = [
    1,
    0,
    1,
    2,
    9,
    44,
    265,
    1854,
    14833,
    133496,
    1334961,
    14684570,
    176214841,
    2290792932,
    32071101049,
    481066515734,
    7697064251745,
    130850092279664,
    2355301661033953,
    44750731559645106,
    895014631192902121,
    18795307255050944540,
    413496759611120779881,
    9510425471055777937262,
    228250211305338670494289,
    5706255282633466762357224,
    148362637348470135821287825,
    4005791208408693667174771274,
    112162153835443422680893595673,
    3252702461227859257745914274516,
    97581073836835777732377428235481,
    3025013288941909109703700275299910,
    96800425246141091510518408809597121,
    3194414033122656019847107490716704992,
    108610077126170304674801654684367969729,
];

macro_rules! impl_factorials_a {
    ($t:ident, $fs:ident, $odfs:ident, $sfs:ident, $df_limit:expr) => {
        impl CheckedFactorial for $t {
            /// Computes the factorial of a number.
            ///
            /// If the input is too large, the function returns `None`.
            ///
            /// $$
            /// f(n) = \\begin{cases}
            ///     \operatorname{Some}(n!) & \text{if} \\quad n! < 2^W, \\\\
            ///     \operatorname{None} & \text{if} \\quad n! \geq 2^W,
            /// \\end{cases}
            /// $$
            /// where $W$ is `Self::WIDTH`.
            ///
            /// $n! = O(\sqrt{n}(n/e)^n)$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::factorial#checked_factorial).
            #[inline]
            fn checked_factorial(n: u64) -> Option<$t> {
                $fs.get(usize::try_from(n).ok()?).copied()
            }
        }

        impl CheckedDoubleFactorial for $t {
            /// Computes the double factorial of a number.
            ///
            /// If the input is too large, the function returns `None`.
            ///
            /// $$
            /// f(n) = \\begin{cases}
            ///     \operatorname{Some}(n!!) & \text{if} \\quad n!! < 2^W, \\\\
            ///     \operatorname{None} & \text{if} \\quad n!! \geq 2^W,
            /// \\end{cases}
            /// $$
            /// where $W$ is `Self::WIDTH`.
            ///
            /// $n!! = O(\sqrt{n}(n/e)^{n/2})$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::factorial#checked_double_factorial).
            #[inline]
            fn checked_double_factorial(n: u64) -> Option<$t> {
                if n > $df_limit {
                    None
                } else if n.odd() {
                    $odfs.get(usize::try_from(n >> 1).ok()?).copied()
                } else {
                    let half = n >> 1;
                    $fs.get(usize::try_from(half).ok()?).map(|&f| f << half)
                }
            }
        }

        impl CheckedSubfactorial for $t {
            /// Computes the subfactorial of a number.
            ///
            /// The subfactorial of $n$ counts the number of derangements of a set of size $n$; a
            /// derangement is a permutation with no fixed points.
            ///
            /// If the input is too large, the function returns `None`.
            ///
            /// $$
            /// f(n) = \\begin{cases}
            ///     \operatorname{Some}(!n) & \text{if} \\quad !n < 2^W, \\\\
            ///     \operatorname{None} & \text{if} \\quad !n \geq 2^W,
            /// \\end{cases}
            /// $$
            /// where $W$ is `Self::WIDTH`.
            ///
            /// $!n = O(n!) = O(\sqrt{n}(n/e)^n)$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::factorial#checked_subfactorial).
            #[inline]
            fn checked_subfactorial(n: u64) -> Option<$t> {
                $sfs.get(usize::try_from(n).ok()?).copied()
            }
        }
    };
}
impl_factorials_a!(
    u8,
    FACTORIALS_U8,
    ODD_DOUBLE_FACTORIALS_U8,
    SUBFACTORIALS_U8,
    7
);
impl_factorials_a!(
    u16,
    FACTORIALS_U16,
    ODD_DOUBLE_FACTORIALS_U16,
    SUBFACTORIALS_U16,
    12
);
impl_factorials_a!(
    u32,
    FACTORIALS_U32,
    ODD_DOUBLE_FACTORIALS_U32,
    SUBFACTORIALS_U32,
    20
);
impl_factorials_a!(
    u64,
    FACTORIALS_U64,
    ODD_DOUBLE_FACTORIALS_U64,
    SUBFACTORIALS_U64,
    33
);
impl_factorials_a!(
    u128,
    FACTORIALS_U128,
    ODD_DOUBLE_FACTORIALS_U128,
    SUBFACTORIALS_U128,
    56
);

impl CheckedFactorial for usize {
    /// Computes the factorial of a [`usize`].
    ///
    /// If the input is too large, the function returns `None`.
    ///
    /// $$
    /// f(n) = \\begin{cases}
    ///     \operatorname{Some}(n!) & \text{if} \\quad n! < 2^W, \\\\
    ///     \operatorname{None} & \text{if} \\quad n! \geq 2^W,
    /// \\end{cases}
    /// $$
    /// where $W$ is `usize::WIDTH`.
    ///
    /// $n! = O(\sqrt{n}(n/e)^n)$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// See [here](super::factorial#checked_factorial).
    #[inline]
    fn checked_factorial(n: u64) -> Option<Self> {
        FACTORIALS_U64
            .get(Self::try_from(n).ok()?)
            .and_then(|&f| Self::try_from(f).ok())
    }
}

impl CheckedSubfactorial for usize {
    /// Computes the subfactorial of a [`usize`].
    ///
    /// The subfactorial of $n$ counts the number of derangements of a set of size $n$; a
    /// derangement is a permutation with no fixed points.
    ///
    /// If the input is too large, the function returns `None`.
    ///
    /// $$
    /// f(n) = \\begin{cases}
    ///     \operatorname{Some}(!n) & \text{if} \\quad !n < 2^W, \\\\
    ///     \operatorname{None} & \text{if} \\quad !n \geq 2^W,
    /// \\end{cases}
    /// $$
    /// where $W$ is `usize::WIDTH`.
    ///
    /// $!n = O(n!) = O(\sqrt{n}(n/e)^n)$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// See [here](super::factorial#checked_subfactorial).
    #[inline]
    fn checked_subfactorial(n: u64) -> Option<Self> {
        SUBFACTORIALS_U64
            .get(Self::try_from(n).ok()?)
            .and_then(|&f| Self::try_from(f).ok())
    }
}

impl CheckedDoubleFactorial for usize {
    /// Computes the double factorial of a [`usize`].
    ///
    /// If the input is too large, the function returns `None`.
    ///
    /// $$
    /// f(n) = \\begin{cases}
    ///     \operatorname{Some}(n!!) & \text{if} \\quad n!! < 2^W, \\\\
    ///     \operatorname{None} & \text{if} \\quad n!! \geq 2^W,
    /// \\end{cases}
    /// $$
    /// where $W$ is `usize::WIDTH`.
    ///
    /// $n!! = O(\sqrt{n}(n/e)^{n/2})$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// See [here](super::factorial#checked_double_factorial).
    #[inline]
    fn checked_double_factorial(n: u64) -> Option<Self> {
        if USIZE_IS_U32 {
            u32::checked_double_factorial(n).map(Self::wrapping_from)
        } else {
            u64::checked_double_factorial(n).map(Self::wrapping_from)
        }
    }
}

macro_rules! impl_factorials_b {
    ($t:ident) => {
        impl Factorial for $t {
            /// Computes the factorial of a number.
            ///
            /// If the input is too large, the function panics. For a function that returns `None`
            /// instead, try [`checked_factorial`](CheckedFactorial::checked_factorial).
            ///
            /// $$
            /// f(n) = n! = 1 \times 2 \times 3 \times \cdots \times n.
            /// $$
            ///
            /// $n! = O(\sqrt{n}(n/e)^n)$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if the output is too large to be represented.
            ///
            /// # Examples
            /// See [here](super::factorial#factorial).
            #[inline]
            fn factorial(n: u64) -> $t {
                $t::checked_factorial(n).unwrap()
            }
        }

        impl DoubleFactorial for $t {
            /// Computes the double factorial of a number.
            ///
            /// If the input is too large, the function panics. For a function that returns `None`
            /// instead, try
            /// [`checked_double_factorial`](CheckedDoubleFactorial::checked_double_factorial).
            ///
            /// $$
            /// f(n) = n!! = n \times (n - 2) \times (n - 4) \times \cdots \times i,
            /// $$
            /// where $i$ is 1 if $n$ is odd and $2$ if $n$ is even.
            ///
            /// $n!! = O(\sqrt{n}(n/e)^{n/2})$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if the output is too large to be represented.
            ///
            /// # Examples
            /// See [here](super::factorial#double_factorial).
            #[inline]
            fn double_factorial(n: u64) -> $t {
                $t::checked_double_factorial(n).unwrap()
            }
        }

        impl Multifactorial for $t {
            /// Computes a multifactorial of a number.
            ///
            /// If the input is too large, the function panics. For a function that returns `None`
            /// instead, try
            /// [`checked_multifactorial`](CheckedMultifactorial::checked_multifactorial).
            ///
            /// $$
            /// f(n, m) = n!^{(m)} = n \times (n - m) \times (n - 2m) \times \cdots \times i.
            /// $$
            /// If $n$ is divisible by $m$, then $i$ is $m$; otherwise, $i$ is the remainder when
            /// $n$ is divided by $m$.
            ///
            /// $n!^{(m)} = O(\sqrt{n}(n/e)^{n/m})$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if the output is too large to be represented.
            ///
            /// # Examples
            /// See [here](super::factorial#multifactorial).
            #[inline]
            fn multifactorial(n: u64, m: u64) -> $t {
                $t::checked_multifactorial(n, m).unwrap()
            }
        }

        impl CheckedMultifactorial for $t {
            /// Computes a multifactorial of a number.
            ///
            /// If the input is too large, the function returns `None`.
            ///
            /// $$
            /// f(n, m) = \\begin{cases}
            ///     \operatorname{Some}(n!^{(m)}) & \text{if} \\quad n!^{(m)} < 2^W, \\\\
            ///     \operatorname{None} & \text{if} \\quad n!^{(m)} \geq 2^W,
            /// \\end{cases}
            /// $$
            /// where $W$ is `Self::WIDTH`.
            ///
            /// $n!^{(m)} = O(\sqrt{n}(n/e)^{n/m})$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::factorial#checked_multifactorial).
            #[inline]
            fn checked_multifactorial(n: u64, m: u64) -> Option<$t> {
                assert_ne!(m, 0);
                if m == 1 {
                    $t::checked_factorial(n)
                } else if m == 2 {
                    $t::checked_double_factorial(n)
                } else {
                    checked_multifactorial_naive(n, m)
                }
            }
        }

        impl Subfactorial for $t {
            /// Computes the subfactorial of a number.
            ///
            /// The subfactorial of $n$ counts the number of derangements of a set of size $n$; a
            /// derangement is a permutation with no fixed points.
            ///
            /// If the input is too large, the function panics. For a function that returns `None`
            /// instead, try [`checked_subfactorial`](CheckedSubfactorial::checked_subfactorial).
            ///
            /// $$
            /// f(n) = \\ !n = \lfloor n!/e \rfloor.
            /// $$
            ///
            /// $!n = O(n!) = O(\sqrt{n}(n/e)^n)$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if the output is too large to be represented.
            ///
            /// # Examples
            /// See [here](super::factorial#subfactorial).
            #[inline]
            fn subfactorial(n: u64) -> $t {
                $t::checked_subfactorial(n).unwrap()
            }
        }
    };
}
apply_to_unsigneds!(impl_factorials_b);

// The correctly rounded (to nearest) values of $n!$ as primitive floats, for every $n$ at which
// the result is finite. Beyond each table the factorial overflows to infinity. The entries were
// generated by rounding the exact `Natural` factorials, and are verified against them by a test
// in malachite-float.
const F64_FACTORIALS: [f64; 171] = [
    1.0,
    1.0,
    2.0,
    6.0,
    24.0,
    120.0,
    720.0,
    5040.0,
    40320.0,
    362880.0,
    3628800.0,
    39916800.0,
    479001600.0,
    6227020800.0,
    87178291200.0,
    1307674368000.0,
    20922789888000.0,
    355687428096000.0,
    6402373705728000.0,
    1.21645100408832e17,
    2.43290200817664e18,
    5.109094217170944e19,
    1.1240007277776077e21,
    2.585201673888498e22,
    6.204484017332394e23,
    1.5511210043330986e25,
    4.0329146112660565e26,
    1.0888869450418352e28,
    3.0488834461171387e29,
    8.841761993739702e30,
    2.6525285981219107e32,
    8.222838654177922e33,
    2.631308369336935e35,
    8.683317618811886e36,
    2.9523279903960416e38,
    1.0333147966386145e40,
    3.7199332678990125e41,
    1.3763753091226346e43,
    5.230226174666011e44,
    2.0397882081197444e46,
    8.159152832478977e47,
    3.345252661316381e49,
    1.40500611775288e51,
    6.041526306337383e52,
    2.658271574788449e54,
    1.1962222086548019e56,
    5.502622159812089e57,
    2.5862324151116818e59,
    1.2413915592536073e61,
    6.082818640342675e62,
    3.0414093201713376e64,
    1.5511187532873822e66,
    8.065817517094388e67,
    4.2748832840600255e69,
    2.308436973392414e71,
    1.2696403353658276e73,
    7.109985878048635e74,
    4.0526919504877214e76,
    2.3505613312828785e78,
    1.3868311854568984e80,
    8.32098711274139e81,
    5.075802138772248e83,
    3.146997326038794e85,
    1.98260831540444e87,
    1.2688693218588417e89,
    8.247650592082472e90,
    5.443449390774431e92,
    3.647111091818868e94,
    2.4800355424368305e96,
    1.711224524281413e98,
    1.1978571669969892e100,
    8.504785885678623e101,
    6.1234458376886085e103,
    4.4701154615126844e105,
    3.307885441519386e107,
    2.48091408113954e109,
    1.8854947016660504e111,
    1.4518309202828587e113,
    1.1324281178206297e115,
    8.946182130782976e116,
    7.156945704626381e118,
    5.797126020747368e120,
    4.753643337012842e122,
    3.945523969720659e124,
    3.314240134565353e126,
    2.81710411438055e128,
    2.4227095383672734e130,
    2.107757298379528e132,
    1.8548264225739844e134,
    1.650795516090846e136,
    1.4857159644817615e138,
    1.352001527678403e140,
    1.2438414054641308e142,
    1.1567725070816416e144,
    1.087366156656743e146,
    1.032997848823906e148,
    9.916779348709496e149,
    9.619275968248212e151,
    9.426890448883248e153,
    9.332621544394415e155,
    9.332621544394415e157,
    9.42594775983836e159,
    9.614466715035127e161,
    9.90290071648618e163,
    1.0299016745145628e166,
    1.081396758240291e168,
    1.1462805637347084e170,
    1.226520203196138e172,
    1.324641819451829e174,
    1.4438595832024937e176,
    1.588245541522743e178,
    1.7629525510902446e180,
    1.974506857221074e182,
    2.2311927486598138e184,
    2.5435597334721877e186,
    2.925093693493016e188,
    3.393108684451898e190,
    3.969937160808721e192,
    4.684525849754291e194,
    5.574585761207606e196,
    6.689502913449127e198,
    8.094298525273444e200,
    9.875044200833601e202,
    1.214630436702533e205,
    1.506141741511141e207,
    1.882677176888926e209,
    2.372173242880047e211,
    3.0126600184576594e213,
    3.856204823625804e215,
    4.974504222477287e217,
    6.466855489220474e219,
    8.47158069087882e221,
    1.1182486511960043e224,
    1.4872707060906857e226,
    1.9929427461615188e228,
    2.6904727073180504e230,
    3.659042881952549e232,
    5.012888748274992e234,
    6.917786472619489e236,
    9.615723196941089e238,
    1.3462012475717526e241,
    1.898143759076171e243,
    2.695364137888163e245,
    3.854370717180073e247,
    5.5502938327393044e249,
    8.047926057471992e251,
    1.1749972043909107e254,
    1.727245890454639e256,
    2.5563239178728654e258,
    3.80892263763057e260,
    5.713383956445855e262,
    8.62720977423324e264,
    1.3113358856834524e267,
    2.0063439050956823e269,
    3.0897696138473508e271,
    4.789142901463394e273,
    7.471062926282894e275,
    1.1729568794264145e278,
    1.853271869493735e280,
    2.9467022724950384e282,
    4.7147236359920616e284,
    7.590705053947219e286,
    1.2296942187394494e289,
    2.0044015765453026e291,
    3.287218585534296e293,
    5.423910666131589e295,
    9.003691705778438e297,
    1.503616514864999e300,
    2.5260757449731984e302,
    4.269068009004705e304,
    7.257415615307999e306,
];

const F32_FACTORIALS: [f32; 35] = [
    1.0,
    1.0,
    2.0,
    6.0,
    24.0,
    120.0,
    720.0,
    5040.0,
    40320.0,
    362880.0,
    3628800.0,
    39916800.0,
    479001600.0,
    6227021000.0,
    87178290000.0,
    1307674400000.0,
    2.092279e13,
    3.556874e14,
    6.4023735e15,
    1.21645105e17,
    2.432902e18,
    5.109094e19,
    1.1240007e21,
    2.5852017e22,
    6.204484e23,
    1.551121e25,
    4.0329146e26,
    1.0888869e28,
    3.0488835e29,
    8.841762e30,
    2.6525285e32,
    8.2228384e33,
    2.6313083e35,
    8.683318e36,
    2.952328e38,
];

macro_rules! impl_factorial_primitive_float {
    ($t:ident, $table:ident) => {
        impl Factorial for $t {
            /// Computes the factorial of a `u64`, correctly rounded to the nearest value.
            ///
            /// Only finitely many factorials are finite in a primitive float type, so the values
            /// are read from a hardcoded table. When the factorial exceeds the largest finite
            /// value, positive infinity is returned; unlike the primitive-integer
            /// implementations, this function does not panic on overflow, since infinity is the
            /// correctly rounded result.
            ///
            /// $$
            /// f(n) = n!+\varepsilon.
            /// $$
            /// - If $n!$ is representable, $\varepsilon$ is 0.
            /// - If $n!$ is finite but not representable,
            ///   $|\varepsilon| < 2^{\lfloor\log_2 n!\rfloor-M}$, where $M$ is the mantissa
            ///   width.
            /// - If $n!$ is larger than the largest finite value, $f(n) = \infty$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::Factorial;
            ///
            /// assert_eq!(f64::factorial(5), 120.0);
            /// assert_eq!(f64::factorial(200), f64::INFINITY);
            /// ```
            #[inline]
            fn factorial(n: u64) -> $t {
                usize::try_from(n)
                    .ok()
                    .and_then(|i| $table.get(i))
                    .map_or($t::INFINITY, |&f| f)
            }
        }
    };
}
impl_factorial_primitive_float!(f64, F64_FACTORIALS);
impl_factorial_primitive_float!(f32, F32_FACTORIALS);
