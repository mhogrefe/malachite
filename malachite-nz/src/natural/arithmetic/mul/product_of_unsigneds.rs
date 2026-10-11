// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::InnerNatural::Small;
use crate::natural::Natural;
use crate::natural::arithmetic::mul::product_of_limbs::limbs_product;
use crate::platform::Limb;
use alloc::vec::Vec;
use core::iter::Product;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ConvertibleFrom, WrappingFrom};

// Multiplies unsigned primitive integers together, producing a `Natural`.
//
// Factors that fit in a limb are packed: adjacent ones are multiplied as limbs while their product
// fits, and each packed limb becomes one leaf of the balanced product tree of `limbs_product`, as
// in `limbs_product_of_first_n_primes`. Factors wider than a limb are multiplied separately and the
// two products combined. Returns as soon as a factor is zero, without consuming the rest of `xs`.
pub(crate) fn product_of_unsigneds<T: PrimitiveUnsigned, I: Iterator<Item = T>>(xs: I) -> Natural
where
    Limb: ConvertibleFrom<T> + WrappingFrom<T>,
    Natural: From<T>,
{
    let mut packed = Vec::new();
    let mut wide = Vec::new();
    let mut acc: Limb = 1;
    for x in xs {
        if x == T::ZERO {
            return Natural::ZERO;
        }
        if Limb::convertible_from(x) {
            let x = Limb::wrapping_from(x);
            acc = if let Some(p) = acc.checked_mul(x) {
                p
            } else {
                packed.push(acc);
                x
            };
        } else {
            wide.push(Natural::from(x));
        }
    }
    // `limbs_product` needs at least two factors, so a single packed limb is handled directly.
    let product = if packed.is_empty() {
        Natural(Small(acc))
    } else {
        packed.push(acc);
        let mut out = Vec::new();
        let (out_len, new_out) = limbs_product(&mut out, &mut packed);
        let mut out = new_out.unwrap();
        out.truncate(out_len);
        Natural::from_owned_limbs_asc(out)
    };
    if wide.is_empty() {
        product
    } else {
        product * Natural::product(wide.into_iter())
    }
}

macro_rules! impl_product_of_unsigneds {
    ($t:ident) => {
        impl Product<$t> for Natural {
            /// Multiplies together all the unsigned primitive integers in an iterator, producing a
            /// [`Natural`].
            ///
            /// Adjacent factors are multiplied as machine words while their product fits in one,
            /// and the resulting words are multiplied in a balanced tree. As soon as a factor is
            /// zero, the rest of the iterator is not consumed.
            ///
            /// $$
            /// f((x_i)_ {i=0}^{n-1}) = \prod_ {i=0}^{n-1} x_i.
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n (\log n)^2 \log\log n)$
            ///
            /// $M(n) = O(n \log n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is the sum of the numbers of
            /// significant bits of the factors.
            ///
            /// # Examples
            /// See [here](super::product_of_unsigneds#product).
            ///
            /// This is equivalent to `_fmpz_ui_vec_prod` from `fmpz_vec/prod.c`, FLINT 3.6.0.
            #[inline]
            fn product<I: Iterator<Item = $t>>(xs: I) -> Natural {
                product_of_unsigneds(xs)
            }
        }

        impl<'a> Product<&'a $t> for Natural {
            /// Multiplies together all the unsigned primitive integers in an iterator of
            /// references, producing a [`Natural`].
            ///
            /// Adjacent factors are multiplied as machine words while their product fits in one,
            /// and the resulting words are multiplied in a balanced tree. As soon as a factor is
            /// zero, the rest of the iterator is not consumed.
            ///
            /// $$
            /// f((x_i)_ {i=0}^{n-1}) = \prod_ {i=0}^{n-1} x_i.
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n (\log n)^2 \log\log n)$
            ///
            /// $M(n) = O(n \log n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is the sum of the numbers of
            /// significant bits of the factors.
            ///
            /// # Examples
            /// See [here](super::product_of_unsigneds#product).
            ///
            /// This is equivalent to `_fmpz_ui_vec_prod` from `fmpz_vec/prod.c`, FLINT 3.6.0.
            #[inline]
            fn product<I: Iterator<Item = &'a $t>>(xs: I) -> Natural {
                product_of_unsigneds(xs.copied())
            }
        }
    };
}
apply_to_unsigneds!(impl_product_of_unsigneds);
