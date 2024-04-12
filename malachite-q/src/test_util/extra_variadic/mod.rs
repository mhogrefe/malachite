// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::iterators::bit_distributor::{BitDistributor, BitDistributorOutputType};
use malachite_base::iterators::iterator_cache::IteratorCache;
use malachite_base::num::arithmetic::traits::CheckedPow;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::random::Seed;
use malachite_base::sets::random::{random_b_tree_sets_fixed_length, RandomBTreeSetsFixedLength};
use malachite_base::tuples::random::next_helper;
use malachite_base::vecs::exhaustive::next_bit_pattern;
use malachite_base::{
    exhaustive_ordered_unique_tuples, exhaustive_tuples_1_input, random_custom_tuples,
    random_tuples,
};
use std::cmp::max;
use std::marker::PhantomData;

#[allow(clippy::missing_const_for_fn)]
fn unwrap_triple<X, Y, Z>((a, b, c): (Option<X>, Option<Y>, Option<Z>)) -> (X, Y, Z) {
    (a.unwrap(), b.unwrap(), c.unwrap())
}

#[allow(clippy::missing_const_for_fn)]
fn unwrap_quadruple<X, Y, Z, W>(
    (a, b, c, d): (Option<X>, Option<Y>, Option<Z>, Option<W>),
) -> (X, Y, Z, W) {
    (a.unwrap(), b.unwrap(), c.unwrap(), d.unwrap())
}

exhaustive_tuples_1_input!(
    (pub(crate)),
    ExhaustiveTriples1Input,
    exhaustive_triples_1_input,
    exhaustive_triples_from_single,
    (I::Item, I::Item, I::Item),
    [0, output_type_x],
    [1, output_type_y],
    [2, output_type_z]
);

custom_tuples!(
    (pub(crate)),
    ExhaustiveTriplesXXY,
    (X, X, Y),
    (None, None, None),
    unwrap_triple,
    exhaustive_triples_xxy,
    exhaustive_triples_xxy_custom_output,
    [X, I, xs, xs_done, [0, output_type_xs_0], [1, output_type_xs_1]],
    [Y, J, ys, ys_done, [2, output_type_ys_2]]
);
custom_tuples!(
    (pub(crate)),
    ExhaustiveQuadruplesXXYZ,
    (X, X, Y, Z),
    (None, None, None, None),
    unwrap_quadruple,
    exhaustive_quadruples_xxyz,
    exhaustive_quadruples_xxyz_custom_output,
    [X, I, xs, xs_done, [0, output_type_xs_0], [1, output_type_xs_1]],
    [Y, J, ys, ys_done, [2, output_type_ys_2]],
    [Z, K, zs, zs_done, [3, output_type_zs_3]]
);

exhaustive_ordered_unique_tuples!(
    (pub(crate)),
    ExhaustiveOrderedUniqueTriples,
    3,
    (I::Item, I::Item, I::Item),
    exhaustive_ordered_unique_triples,
    [0, 1, 2]
);

random_tuples!(
    (pub(crate)),
    RandomTriples,
    RandomTriplesFromSingle,
    random_triples,
    random_triples_from_single,
    (I::Item, I::Item, I::Item),
    [0, X, I, xs, xs_gen],
    [1, Y, J, ys, ys_gen],
    [2, Z, K, zs, zs_gen]
);

random_custom_tuples!(
    (pub(crate)),
    RandomTriplesXXY,
    (X, X, Y),
    random_triples_xxy,
    [X, I, xs, xs_gen, [x_0, x_0], [x_1, x_1]],
    [Y, J, ys, ys_gen, [y_2, y_2]]
);
random_custom_tuples!(
    (pub(crate)),
    RandomTriplesXYY,
    (X, Y, Y),
    random_triples_xyy,
    [X, I, xs, xs_gen, [x_0, x_0]],
    [Y, J, ys, ys_gen, [y_1, y_1], [y_2, y_2]]
);
random_custom_tuples!(
    (pub(crate)),
    RandomQuadruplesXXYZ,
    (X, X, Y, Z),
    random_quadruples_xxyz,
    [X, I, xs, xs_gen, [x_0, x_0], [x_1, x_1]],
    [Y, J, ys, ys_gen, [y_2, y_2]],
    [Z, K, zs, zs_gen, [z_3, z_3]]
);

random_ordered_unique_tuples!(
    (pub(crate)),
    RandomOrderedUniqueTriples,
    3,
    (I::Item, I::Item, I::Item),
    random_ordered_unique_triples,
    [0, 1, 2]
);
