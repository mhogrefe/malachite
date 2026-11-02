// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Computes $\cosh(x)$, the hyperbolic cosine of a number.
pub trait Cosh {
    type Output;

    fn cosh(self) -> Self::Output;
}

/// Replaces a number with its hyperbolic cosine, $\cosh(x)$.
pub trait CoshAssign {
    fn cosh_assign(&mut self);
}

/// Computes $\sinh(x)$, the hyperbolic sine of a number.
pub trait Sinh {
    type Output;

    fn sinh(self) -> Self::Output;
}

/// Replaces a number with its hyperbolic sine, $\sinh(x)$.
pub trait SinhAssign {
    fn sinh_assign(&mut self);
}

/// Computes $\sinh(x)$ and $\cosh(x)$, the hyperbolic sine and cosine of a number, together.
pub trait SinhCosh {
    type Output;

    fn sinh_cosh(self) -> (Self::Output, Self::Output);
}

/// Replaces a number with its hyperbolic sine, $\sinh(x)$, and writes its hyperbolic cosine,
/// $\cosh(x)$, to a second number.
pub trait SinhCoshAssign {
    fn sinh_cosh_assign(&mut self, cosh: &mut Self);
}

/// Computes $\tanh(x)$, the hyperbolic tangent of a number.
pub trait Tanh {
    type Output;

    fn tanh(self) -> Self::Output;
}

/// Replaces a number with its hyperbolic tangent, $\tanh(x)$.
pub trait TanhAssign {
    fn tanh_assign(&mut self);
}

/// Computes $\operatorname{sech}(x)$, the hyperbolic secant of a number.
pub trait Sech {
    type Output;

    fn sech(self) -> Self::Output;
}

/// Replaces a number with its hyperbolic secant, $\operatorname{sech}(x)$.
pub trait SechAssign {
    fn sech_assign(&mut self);
}

/// Computes $\operatorname{csch}(x)$, the hyperbolic cosecant of a number.
pub trait Csch {
    type Output;

    fn csch(self) -> Self::Output;
}

/// Replaces a number with its hyperbolic cosecant, $\operatorname{csch}(x)$.
pub trait CschAssign {
    fn csch_assign(&mut self);
}

/// Computes $\coth(x)$, the hyperbolic cotangent of a number.
pub trait Coth {
    type Output;

    fn coth(self) -> Self::Output;
}

/// Replaces a number with its hyperbolic cotangent, $\coth(x)$.
pub trait CothAssign {
    fn coth_assign(&mut self);
}

/// Computes $\operatorname{asinh}(x)$, the inverse hyperbolic sine of a number.
pub trait Asinh {
    type Output;

    fn asinh(self) -> Self::Output;
}

/// Replaces a number with its inverse hyperbolic sine, $\operatorname{asinh}(x)$.
pub trait AsinhAssign {
    fn asinh_assign(&mut self);
}

/// Computes $\operatorname{acosh}(x)$, the inverse hyperbolic cosine of a number.
pub trait Acosh {
    type Output;

    fn acosh(self) -> Self::Output;
}

/// Replaces a number with its inverse hyperbolic cosine, $\operatorname{acosh}(x)$.
pub trait AcoshAssign {
    fn acosh_assign(&mut self);
}

/// Computes $\operatorname{atanh}(x)$, the inverse hyperbolic tangent of a number.
pub trait Atanh {
    type Output;

    fn atanh(self) -> Self::Output;
}

/// Replaces a number with its inverse hyperbolic tangent, $\operatorname{atanh}(x)$.
pub trait AtanhAssign {
    fn atanh_assign(&mut self);
}

/// Computes $\operatorname{asech}(x)$, the inverse hyperbolic secant of a number.
pub trait Asech {
    type Output;

    fn asech(self) -> Self::Output;
}

/// Replaces a number with its inverse hyperbolic secant, $\operatorname{asech}(x)$.
pub trait AsechAssign {
    fn asech_assign(&mut self);
}

/// Computes $\operatorname{acsch}(x)$, the inverse hyperbolic cosecant of a number.
pub trait Acsch {
    type Output;

    fn acsch(self) -> Self::Output;
}

/// Replaces a number with its inverse hyperbolic cosecant, $\operatorname{acsch}(x)$.
pub trait AcschAssign {
    fn acsch_assign(&mut self);
}

/// Computes $\operatorname{acoth}(x)$, the inverse hyperbolic cotangent of a number.
pub trait Acoth {
    type Output;

    fn acoth(self) -> Self::Output;
}

/// Replaces a number with its inverse hyperbolic cotangent, $\operatorname{acoth}(x)$.
pub trait AcothAssign {
    fn acoth_assign(&mut self);
}
