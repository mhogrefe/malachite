use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use crate::{ComparableFloat, ComparableFloatRef, Float};
use std::hash::{Hash, Hasher};

impl Hash for ComparableFloat {
    /// Computes a hash of a `ComparableFloat`.
    ///
    /// The hash is compatible with `ComparableFloat` equality: all `NaN`s hash to the same value.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_ref().hash(state)
    }
}

impl<'a> Hash for ComparableFloatRef<'a> {
    /// Computes a hash of a `ComparableFloatRef`.
    ///
    /// The hash is compatible with `ComparableFloatRef` equality: all `NaN`s hash to the same
    /// value.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self.0 {
            float_nan!() => "NaN".hash(state),
            Float(Infinity { sign }) => {
                if *sign {
                    "Infinity".hash(state)
                } else {
                    "-Infinity".hash(state)
                }
            }
            Float(Zero { sign }) => {
                if *sign {
                    "0.0".hash(state)
                } else {
                    "-0.0".hash(state)
                }
            }
            Float(Finite {
                sign,
                exponent,
                precision,
                significand,
            }) => {
                sign.hash(state);
                exponent.hash(state);
                precision.hash(state);
                significand.hash(state);
            }
        }
    }
}
