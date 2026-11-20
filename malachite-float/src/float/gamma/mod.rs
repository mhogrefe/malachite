// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Implementations of [`Gamma`](malachite_base::num::gamma::traits::Gamma) and
/// [`GammaAssign`](malachite_base::num::gamma::traits::GammaAssign), traits for computing the gamma
/// function of [`Float`](super::Float)s.
#[allow(clippy::module_inception)]
pub mod gamma;
/// Implementations of [`LnAbsGamma`](malachite_base::num::gamma::traits::LnAbsGamma) and
/// [`LnAbsGammaAssign`](malachite_base::num::gamma::traits::LnAbsGammaAssign), traits for computing
/// the logarithm of the absolute value of the gamma function of [`Float`](super::Float)s.
pub mod ln_abs_gamma;
/// Implementations of [`LnGamma`](malachite_base::num::gamma::traits::LnGamma) and
/// [`LnGammaAssign`](malachite_base::num::gamma::traits::LnGammaAssign), traits for computing the
/// logarithm of the gamma function of [`Float`](super::Float)s.
pub mod ln_gamma;
/// Implementations of [`ReciprocalGamma`](malachite_base::num::gamma::traits::ReciprocalGamma) and
/// [`ReciprocalGammaAssign`](malachite_base::num::gamma::traits::ReciprocalGammaAssign), traits for
/// computing the reciprocal gamma function of [`Float`](super::Float)s.
pub mod reciprocal_gamma;
