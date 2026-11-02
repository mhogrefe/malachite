// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Computes $\cos(x)$, the cosine of a number.
pub trait Cos {
    type Output;

    fn cos(self) -> Self::Output;
}

/// Replaces a number with its cosine, $\cos(x)$.
pub trait CosAssign {
    fn cos_assign(&mut self);
}

/// Computes $\sin(x)$, the sine of a number.
pub trait Sin {
    type Output;

    fn sin(self) -> Self::Output;
}

/// Replaces a number with its sine, $\sin(x)$.
pub trait SinAssign {
    fn sin_assign(&mut self);
}

/// Computes $\sin(x)$ and $\cos(x)$, the sine and cosine of a number, together.
pub trait SinCos {
    type Output;

    fn sin_cos(self) -> (Self::Output, Self::Output);
}

/// Computes $\tan(x)$, the tangent of a number.
pub trait Tan {
    type Output;

    fn tan(self) -> Self::Output;
}

/// Replaces a number with its tangent, $\tan(x)$.
pub trait TanAssign {
    fn tan_assign(&mut self);
}

/// Computes $\sec(x)$, the secant of a number.
pub trait Sec {
    type Output;

    fn sec(self) -> Self::Output;
}

/// Replaces a number with its secant, $\sec(x)$.
pub trait SecAssign {
    fn sec_assign(&mut self);
}

/// Computes $\csc(x)$, the cosecant of a number.
pub trait Csc {
    type Output;

    fn csc(self) -> Self::Output;
}

/// Replaces a number with its cosecant, $\csc(x)$.
pub trait CscAssign {
    fn csc_assign(&mut self);
}

/// Computes $\cot(x)$, the cotangent of a number.
pub trait Cot {
    type Output;

    fn cot(self) -> Self::Output;
}

/// Replaces a number with its cotangent, $\cot(x)$.
pub trait CotAssign {
    fn cot_assign(&mut self);
}

/// Computes $\arctan(x)$, the arctangent of a number.
pub trait Atan {
    type Output;

    fn atan(self) -> Self::Output;
}

/// Replaces a number with its arctangent, $\arctan(x)$.
pub trait AtanAssign {
    fn atan_assign(&mut self);
}

/// Computes $\operatorname{atan2}(y,x)$, the angle of the point $(x,y)$ measured from the positive
/// $x$-axis.
pub trait Atan2<RHS = Self> {
    type Output;

    fn atan2(self, other: RHS) -> Self::Output;
}

/// Replaces a number $y$ with $\operatorname{atan2}(y,x)$, the angle of the point $(x,y)$ measured
/// from the positive $x$-axis.
pub trait Atan2Assign<RHS = Self> {
    fn atan2_assign(&mut self, other: RHS);
}

/// Computes $\arcsin(x)$, the arcsine of a number.
pub trait Asin {
    type Output;

    fn asin(self) -> Self::Output;
}

/// Replaces a number with its arcsine, $\arcsin(x)$.
pub trait AsinAssign {
    fn asin_assign(&mut self);
}

/// Computes $\arccos(x)$, the arccosine of a number.
pub trait Acos {
    type Output;

    fn acos(self) -> Self::Output;
}

/// Replaces a number with its arccosine, $\arccos(x)$.
pub trait AcosAssign {
    fn acos_assign(&mut self);
}

/// Computes $\operatorname{asec}(x)$, the arcsecant of a number.
pub trait Asec {
    type Output;

    fn asec(self) -> Self::Output;
}

/// Replaces a number with its arcsecant, $\operatorname{asec}(x)$.
pub trait AsecAssign {
    fn asec_assign(&mut self);
}

/// Computes $\operatorname{acsc}(x)$, the arccosecant of a number.
pub trait Acsc {
    type Output;

    fn acsc(self) -> Self::Output;
}

/// Replaces a number with its arccosecant, $\operatorname{acsc}(x)$.
pub trait AcscAssign {
    fn acsc_assign(&mut self);
}

/// Computes $\operatorname{acot}(x)$, the arccotangent of a number.
pub trait Acot {
    type Output;

    fn acot(self) -> Self::Output;
}

/// Replaces a number with its arccotangent, $\operatorname{acot}(x)$.
pub trait AcotAssign {
    fn acot_assign(&mut self);
}

/// Replaces a number with its sine, $\sin(x)$, and writes its cosine, $\cos(x)$, to a second
/// number.
pub trait SinCosAssign {
    fn sin_cos_assign(&mut self, cos: &mut Self);
}
