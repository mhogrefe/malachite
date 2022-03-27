use malachite_base::num::arithmetic::traits::FloorLogBase2;
use malachite_base::num::comparison::traits::{OrdAbs, PartialOrdAbs};
use std::cmp::Ordering;
use Rational;

macro_rules! impl_float {
    ($t: ident) => {
        impl PartialOrdAbs<$t> for Rational {
            /// Compares the absolute value of a `Rational` to the absolute value of a value of
            /// primitive float type.
            ///
            /// # Worst-case complexity
            /// TODO
            ///
            /// # Examples
            /// See the documentation of the `comparison::partial_cmp_abs_primitive_float` module.
            fn partial_cmp_abs(&self, other: &$t) -> Option<Ordering> {
                if other.is_nan() {
                    None
                } else if *other == 0.0 {
                    self.partial_cmp_abs(&0u32)
                } else if !other.is_finite() || *self == 0u32 {
                    Some(Ordering::Less)
                } else {
                    let ord_cmp = self
                        .floor_log_base_2_of_abs()
                        .cmp(&other.abs().floor_log_base_2());
                    Some(if ord_cmp != Ordering::Equal {
                        ord_cmp
                    } else {
                        self.cmp_abs(&Rational::from(*other))
                    })
                }
            }
        }

        impl PartialOrdAbs<Rational> for $t {
            /// Compares the absolute value of a value of primitive float type to the absolute
            /// value of a `Rational`.
            ///
            /// # Worst-case complexity
            /// TODO
            ///
            /// # Examples
            /// See the documentation of the `comparison::partial_cmp_abs_primitive_float` module.
            #[inline]
            fn partial_cmp_abs(&self, other: &Rational) -> Option<Ordering> {
                other.partial_cmp_abs(self).map(Ordering::reverse)
            }
        }
    };
}
apply_to_primitive_floats!(impl_float);
