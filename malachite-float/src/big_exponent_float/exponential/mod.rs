// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Functions for computing the compound function $(1+x)^n$ of
/// [`BigExponentFloat`](super::BigExponentFloat)s.
pub mod compound;
/// Functions for computing $e^x$ for [`BigExponentFloat`](super::BigExponentFloat)s and
/// [`Rational`](malachite_q::Rational)s.
pub mod exp;
/// Functions for computing $e^x-1$ for [`BigExponentFloat`](super::BigExponentFloat)s and
/// [`Rational`](malachite_q::Rational)s.
pub mod exp_x_minus_1;
/// Functions for computing $W_0(x)$, the principal branch of the Lambert $W$ function, of
/// [`BigExponentFloat`](super::BigExponentFloat)s and [`Rational`](malachite_q::Rational)s.
pub mod lambert_w0;
/// Functions for computing $W_{-1}(x)$, the lower real branch of the Lambert $W$ function, of
/// [`BigExponentFloat`](super::BigExponentFloat)s and [`Rational`](malachite_q::Rational)s.
pub mod lambert_w_minus_1;
/// Functions for computing the natural logarithm of [`BigExponentFloat`](super::BigExponentFloat)s,
/// [`Rational`](malachite_q::Rational)s, and [`u64`]s.
pub mod ln;
/// Functions for computing $\ln(1+x)$ of [`BigExponentFloat`](super::BigExponentFloat)s.
pub mod ln_1_plus_x;
/// Functions for computing the base-$b$ logarithm, for a [`u64`] $b$, of
/// [`BigExponentFloat`](super::BigExponentFloat)s and [`Rational`](malachite_q::Rational)s.
pub mod log_base;
/// Functions for computing the base-10 logarithm of [`BigExponentFloat`](super::BigExponentFloat)s
/// and [`Rational`](malachite_q::Rational)s.
pub mod log_base_10;
/// Functions for computing $\log_{10}(1+x)$ of [`BigExponentFloat`](super::BigExponentFloat)s.
pub mod log_base_10_1_plus_x;
/// Functions for computing $\log_b(1+x)$, for a [`u64`] $b$, of
/// [`BigExponentFloat`](super::BigExponentFloat)s.
pub mod log_base_1_plus_x;
/// Functions for computing the base-2 logarithm of [`BigExponentFloat`](super::BigExponentFloat)s
/// and [`Rational`](malachite_q::Rational)s.
pub mod log_base_2;
/// Functions for computing $\log_2(1+x)$ of [`BigExponentFloat`](super::BigExponentFloat)s.
pub mod log_base_2_1_plus_x;
/// Functions for computing $\log_b x$, for a [`BigExponentFloat`](super::BigExponentFloat) $b$, of
/// [`BigExponentFloat`](super::BigExponentFloat)s.
pub mod log_base_big_exponent_float_base;
/// Functions for computing $\log_b(1+x)$, for a [`BigExponentFloat`](super::BigExponentFloat) $b$,
/// of [`BigExponentFloat`](super::BigExponentFloat)s.
pub mod log_base_big_exponent_float_base_1_plus_x;
/// Functions for computing the base-$2^k$ logarithm of
/// [`BigExponentFloat`](super::BigExponentFloat)s and [`Rational`](malachite_q::Rational)s.
pub mod log_base_power_of_2;
/// Functions for computing $\log_{2^k}(1+x)$ of [`BigExponentFloat`](super::BigExponentFloat)s.
pub mod log_base_power_of_2_1_plus_x;
/// Functions for computing $\log_b x$, for a [`Rational`](malachite_q::Rational) $b$, of
/// [`BigExponentFloat`](super::BigExponentFloat)s.
pub mod log_base_rational_base;
/// Functions for computing $\log_b(1+x)$, for a [`Rational`](malachite_q::Rational) $b$, of
/// [`BigExponentFloat`](super::BigExponentFloat)s.
pub mod log_base_rational_base_1_plus_x;
/// Functions for computing $\log_b x$ of a [`Rational`](malachite_q::Rational) $x$, for a
/// [`BigExponentFloat`](super::BigExponentFloat) $b$, returning a
/// [`BigExponentFloat`](super::BigExponentFloat).
pub mod log_base_rational_big_exponent_float_base;
/// Functions for computing $\log_b x$ of a [`Rational`](malachite_q::Rational) $x$, for a
/// [`Rational`](malachite_q::Rational) $b$, returning a
/// [`BigExponentFloat`](super::BigExponentFloat).
pub mod log_base_rational_rational_base;
/// Functions for computing $10^x$ for [`BigExponentFloat`](super::BigExponentFloat)s and
/// [`Rational`](malachite_q::Rational)s.
pub mod power_of_10;
/// Functions for computing $10^x-1$ for [`BigExponentFloat`](super::BigExponentFloat)s and
/// [`Rational`](malachite_q::Rational)s.
pub mod power_of_10_x_minus_1;
/// Functions for computing $2^x$ for [`BigExponentFloat`](super::BigExponentFloat)s and
/// [`Rational`](malachite_q::Rational)s.
pub mod power_of_2_of_big_exponent_float;
/// Functions for computing $2^x-1$ for [`BigExponentFloat`](super::BigExponentFloat)s and
/// [`Rational`](malachite_q::Rational)s.
pub mod power_of_2_x_minus_1;
/// Functions for computing $T_0(x)$, the principal branch of the tree function $T$, of
/// [`BigExponentFloat`](super::BigExponentFloat)s and [`Rational`](malachite_q::Rational)s.
pub mod tree_t0;
/// Functions for computing $T_{-1}(x)$, the other real branch of the tree function $T$, of
/// [`BigExponentFloat`](super::BigExponentFloat)s and [`Rational`](malachite_q::Rational)s.
pub mod tree_t_minus_1;
