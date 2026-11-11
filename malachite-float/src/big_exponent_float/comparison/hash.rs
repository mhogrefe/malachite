// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::{ComparableBigExponentFloat, ComparableBigExponentFloatRef};
use core::hash::{Hash, Hasher};

impl Hash for ComparableBigExponentFloat {
    /// Computes a hash of a [`ComparableBigExponentFloat`].
    ///
    /// The hash is compatible with [`ComparableBigExponentFloat`] equality: NaN has a single hash
    /// value.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.0.complexity()`.
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.0.hash(state);
    }
}

impl Hash for ComparableBigExponentFloatRef<'_> {
    /// Computes a hash of a [`ComparableBigExponentFloatRef`].
    ///
    /// The hash is compatible with [`ComparableBigExponentFloatRef`] equality: NaN has a single
    /// hash value.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.0.complexity()`.
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.0.hash(state);
    }
}
