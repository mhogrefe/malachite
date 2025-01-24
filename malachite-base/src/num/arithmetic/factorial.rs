// Copyright © 2025 Mikhail Hogrefe
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

pub_test! {checked_multifactorial_naive<T: PrimitiveUnsigned>(n: u64, m: u64) -> Option<T> {
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
    fn checked_factorial(n: u64) -> Option<usize> {
        FACTORIALS_U64
            .get(usize::try_from(n).ok()?)
            .and_then(|&f| usize::try_from(f).ok())
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
    fn checked_subfactorial(n: u64) -> Option<usize> {
        SUBFACTORIALS_U64
            .get(usize::try_from(n).ok()?)
            .and_then(|&f| usize::try_from(f).ok())
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
    fn checked_double_factorial(n: u64) -> Option<usize> {
        if USIZE_IS_U32 {
            u32::checked_double_factorial(n).map(usize::wrapping_from)
        } else {
            u64::checked_double_factorial(n).map(usize::wrapping_from)
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
