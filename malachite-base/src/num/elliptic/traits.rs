// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

/// Calculates the AGM (arithmetic-geometric mean) of two numbers.
pub trait Agm<RHS = Self> {
    type Output;

    fn agm(self, other: RHS) -> Self::Output;
}

/// Replaces a number with the AGM (arithmetic-geometric mean) of it and another number.
pub trait AgmAssign<RHS = Self> {
    fn agm_assign(&mut self, other: RHS);
}
