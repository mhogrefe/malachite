// Copyright © 2026 Mikhail Hogrefe
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
use malachite_base::random_tuples;
use malachite_base::tuples::random::next_helper;
use std::cmp::max;
use std::marker::PhantomData;

#[allow(clippy::missing_const_for_fn)]
fn unwrap_triple<X, Y, Z>((a, b, c): (Option<X>, Option<Y>, Option<Z>)) -> (X, Y, Z) {
    (a.unwrap(), b.unwrap(), c.unwrap())
}

#[allow(clippy::missing_const_for_fn)]
fn unwrap_quintuple<X, Y, Z, W, V>(
    (a, b, c, d, e): (Option<X>, Option<Y>, Option<Z>, Option<W>, Option<V>),
) -> (X, Y, Z, W, V) {
    (a.unwrap(), b.unwrap(), c.unwrap(), d.unwrap(), e.unwrap())
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
exhaustive_tuples_1_input!(
    (pub(crate)),
    ExhaustiveQuadruples1Input,
    exhaustive_quadruples_1_input,
    exhaustive_quadruples_from_single,
    (I::Item, I::Item, I::Item, I::Item),
    [0, output_type_x],
    [1, output_type_y],
    [2, output_type_z],
    [3, output_type_w]
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
custom_tuples!(
    (pub(crate)),
    ExhaustiveQuintuplesXXXYZ,
    (X, X, X, Y, Z),
    (None, None, None, None, None),
    unwrap_quintuple,
    exhaustive_quintuples_xxxyz,
    exhaustive_quintuples_xxxyz_custom_output,
    [
        X,
        I,
        xs,
        xs_done,
        [0, output_type_xs_0],
        [1, output_type_xs_1],
        [2, output_type_xs_2]
    ],
    [Y, J, ys, ys_done, [3, output_type_ys_3]],
    [Z, K, zs, zs_done, [4, output_type_zs_4]]
);
custom_tuples!(
    (pub(crate)),
    ExhaustiveQuintuplesXXXXY,
    (X, X, X, X, Y),
    (None, None, None, None, None),
    unwrap_quintuple,
    exhaustive_quintuples_xxxxy,
    exhaustive_quintuples_xxxxy_custom_output,
    [
        X,
        I,
        xs,
        xs_done,
        [0, output_type_xs_0],
        [1, output_type_xs_1],
        [2, output_type_xs_2],
        [3, output_type_xs_3]
    ],
    [Y, J, ys, ys_done, [4, output_type_ys_4]]
);
custom_tuples!(
    (pub(crate)),
    ExhaustiveQuadruplesXXXY,
    (X, X, X, Y),
    (None, None, None, None),
    unwrap_quadruple,
    exhaustive_quadruples_xxxy,
    exhaustive_quadruples_xxxy_custom_output,
    [X, I, xs, xs_done, [0, output_type_xs_0], [1, output_type_xs_1], [2, output_type_xs_2]],
    [Y, J, ys, ys_done, [3, output_type_ys_3]]
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
random_tuples!(
    (pub(crate)),
    RandomQuadruples,
    RandomQuadruplesFromSingle,
    random_quadruples,
    random_quadruples_from_single,
    (I::Item, I::Item, I::Item, I::Item),
    [0, X, I, xs, xs_gen],
    [1, Y, J, ys, ys_gen],
    [2, Z, K, zs, zs_gen],
    [3, W, L, ws, ws_gen]
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
random_custom_tuples!(
    (pub(crate)),
    RandomQuintuplesXXXXY,
    (X, X, X, X, Y),
    random_quintuples_xxxxy,
    [X, I, xs, xs_gen, [x_0, x_0], [x_1, x_1], [x_2, x_2], [x_3, x_3]],
    [Y, J, ys, ys_gen, [y_4, y_4]]
);
random_custom_tuples!(
    (pub(crate)),
    RandomSextuplesXXXYZW,
    (X, X, X, Y, Z, W),
    random_sextuples_xxxyzw,
    [X, I, xs, xs_gen, [x_0, x_0], [x_1, x_1], [x_2, x_2]],
    [Y, J, ys, ys_gen, [y_3, y_3]],
    [Z, K, zs, zs_gen, [z_4, z_4]],
    [W, L, ws, ws_gen, [w_5, w_5]]
);
random_custom_tuples!(
    (pub(crate)),
    RandomSextuplesXXXXYZ,
    (X, X, X, X, Y, Z),
    random_sextuples_xxxxyz,
    [X, I, xs, xs_gen, [x_0, x_0], [x_1, x_1], [x_2, x_2], [x_3, x_3]],
    [Y, J, ys, ys_gen, [y_4, y_4]],
    [Z, K, zs, zs_gen, [z_5, z_5]]
);
random_custom_tuples!(
    (pub(crate)),
    RandomQuadruplesXXXY,
    (X, X, X, Y),
    random_quadruples_xxxy,
    [X, I, xs, xs_gen, [x_0, x_0], [x_1, x_1], [x_2, x_2]],
    [Y, J, ys, ys_gen, [y_3, y_3]]
);
random_custom_tuples!(
    (pub(crate)),
    RandomQuintuplesXXYZW,
    (X, X, Y, Z, W),
    random_quintuples_xxyzw,
    [X, I, xs, xs_gen, [x_0, x_0], [x_1, x_1]],
    [Y, J, ys, ys_gen, [y_2, y_2]],
    [Z, K, zs, zs_gen, [z_3, z_3]],
    [W, L, ws, ws_gen, [w_4, w_4]]
);
random_custom_tuples!(
    (pub(crate)),
    RandomQuintuplesXXXYZ,
    (X, X, X, Y, Z),
    random_quintuples_xxxyz,
    [X, I, xs, xs_gen, [x_0, x_0], [x_1, x_1], [x_2, x_2]],
    [Y, J, ys, ys_gen, [y_3, y_3]],
    [Z, K, zs, zs_gen, [z_4, z_4]]
);
