// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::iterators::bit_distributor::{BitDistributor, BitDistributorOutputType};
use crate::iterators::iterator_cache::IteratorCache;
use crate::num::arithmetic::traits::CheckedPow;
use crate::num::conversion::traits::{ExactFrom, WrappingFrom};
use crate::num::logic::traits::SignificantBits;
use crate::num::random::{random_unsigned_range, RandomUnsignedRange};
use crate::random::Seed;
use crate::tuples::exhaustive::clone_helper;
use crate::tuples::random::next_helper;
use crate::unions::UnionFromStrError;
use crate::{
    custom_tuples, exhaustive_tuples_1_input, exhaustive_unions, lex_custom_tuples, lex_tuples,
    random_custom_tuples, random_tuples, random_unions, union_struct,
};
use std::cmp::max;
use std::fmt::{self, Display, Formatter};
use std::marker::PhantomData;
use std::str::FromStr;

fn unwrap_triple<X, Y, Z>((a, b, c): (Option<X>, Option<Y>, Option<Z>)) -> (X, Y, Z) {
    (a.unwrap(), b.unwrap(), c.unwrap())
}

#[allow(clippy::missing_const_for_fn)]
fn unwrap_quadruple<X, Y, Z, W>(
    (a, b, c, d): (Option<X>, Option<Y>, Option<Z>, Option<W>),
) -> (X, Y, Z, W) {
    (a.unwrap(), b.unwrap(), c.unwrap(), d.unwrap())
}

lex_tuples!(
    (pub(crate)),
    3,
    LexTriples,
    LexTriplesFromSingle,
    lex_triples,
    lex_triples_from_single,
    (T, T, T),
    [0, X, I, xs, x],
    [1, Y, J, ys, y],
    [2, Z, K, zs, z]
);

lex_custom_tuples!(
    (pub(crate)),
    LexTriplesXYY,
    (X, Y, Y),
    (None, None, None),
    unwrap_triple,
    lex_triples_xyy,
    [X, I, xs, [0, x_0]],
    [Y, J, ys, [1, y_1], [2, y_2]]
);

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
exhaustive_tuples_1_input!(
    (pub(crate)),
    ExhaustiveSextuples1Input,
    exhaustive_sextuples_1_input,
    exhaustive_sextuples_from_single,
    (I::Item, I::Item, I::Item, I::Item, I::Item, I::Item),
    [0, output_type_x],
    [1, output_type_y],
    [2, output_type_z],
    [3, output_type_w],
    [4, output_type_v],
    [5, output_type_u]
);
exhaustive_tuples_1_input!(
    (pub(crate)),
    ExhaustiveOctuples1Input,
    exhaustive_octuples_1_input,
    exhaustive_octuples_from_single,
    (
        I::Item,
        I::Item,
        I::Item,
        I::Item,
        I::Item,
        I::Item,
        I::Item,
        I::Item
    ),
    [0, output_type_x],
    [1, output_type_y],
    [2, output_type_z],
    [3, output_type_w],
    [4, output_type_v],
    [5, output_type_u],
    [6, output_type_t],
    [7, output_type_s]
);
exhaustive_tuples_1_input!(
    (pub(crate)),
    ExhaustiveDuodecuples1Input,
    exhaustive_duodecuples_1_input,
    exhaustive_duodecuples_from_single,
    (
        I::Item,
        I::Item,
        I::Item,
        I::Item,
        I::Item,
        I::Item,
        I::Item,
        I::Item,
        I::Item,
        I::Item,
        I::Item,
        I::Item
    ),
    [0, output_type_x],
    [1, output_type_y],
    [2, output_type_z],
    [3, output_type_w],
    [4, output_type_v],
    [5, output_type_u],
    [6, output_type_t],
    [7, output_type_s],
    [8, output_type_r],
    [9, output_type_q],
    [10, output_type_p],
    [11, output_type_o]
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
     ExhaustiveTriplesXYX,
     (X, Y, X),
    (None, None, None),
    unwrap_triple,
    exhaustive_triples_xyx,
    exhaustive_triples_xyx_custom_output,
    [X, I, xs, xs_done, [0, output_type_xs_0], [2, output_type_ys_1]],
    [Y, J, ys, ys_done, [1, output_type_xs_2]]
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
custom_tuples!(
    (pub(crate)),
    ExhaustiveQuadruplesXXYX,
    (X, X, Y, X),
    (None, None, None, None),
    unwrap_quadruple,
    exhaustive_quadruples_xxyx,
    exhaustive_quadruples_xxyx_custom_output,
    [X, I, xs, xs_done, [0, output_type_xs_0], [1, output_type_xs_1], [3, output_type_xs_3]],
    [Y, J, ys, ys_done, [2, output_type_ys_2]]
);
custom_tuples!(
    (pub(crate)),
    ExhaustiveQuadruplesXYYX,
    (X, Y, Y, X),
    (None, None, None, None),
    unwrap_quadruple,
    exhaustive_quadruples_xyyx,
    exhaustive_quadruples_xyyx_custom_output,
    [X, I, xs, xs_done, [0, output_type_xs_0], [3, output_type_xs_3]],
    [Y, J, ys, ys_done, [1, output_type_ys_1], [2, output_type_ys_2]]
);
custom_tuples!(
    (pub(crate)),
    ExhaustiveQuadruplesXYYZ,
    (X, Y, Y, Z),
    (None, None, None, None),
    unwrap_quadruple,
    exhaustive_quadruples_xyyz,
    exhaustive_quadruples_xyyz_custom_output,
    [X, I, xs, xs_done, [0, output_type_xs_0]],
    [Y, J, ys, ys_done, [1, output_type_ys_1], [2, output_type_ys_2]],
    [Z, K, zs, zs_done, [3, output_type_zs_3]]
);
custom_tuples!(
    (pub(crate)),
    ExhaustiveQuadruplesXYZZ,
    (X, Y, Z, Z),
    (None, None, None, None),
    unwrap_quadruple,
    exhaustive_quadruples_xyzz,
    exhaustive_quadruples_xyzz_custom_output,
    [X, I, xs, xs_done, [0, output_type_xs_0]],
    [Y, J, ys, ys_done, [1, output_type_ys_1]],
    [Z, K, zs, zs_done, [2, output_type_zs_2], [3, output_type_zs_3]]
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
random_tuples!(
    (pub(crate)),
    RandomSextuples,
    RandomSextuplesFromSingle,
    random_sextuples,
    random_sextuples_from_single,
    (I::Item, I::Item, I::Item, I::Item, I::Item, I::Item),
    [0, X, I, xs, xs_gen],
    [1, Y, J, ys, ys_gen],
    [2, Z, K, zs, zs_gen],
    [3, W, L, ws, ws_gen],
    [4, V, M, vs, vs_gen],
    [5, U, N, us, us_gen]
);
random_tuples!(
    (pub(crate)),
    RandomOctuples,
    RandomOctuplesFromSingle,
    random_octuples,
    random_octuples_from_single,
    (
        I::Item,
        I::Item,
        I::Item,
        I::Item,
        I::Item,
        I::Item,
        I::Item,
        I::Item
    ),
    [0, X, I, xs, xs_gen],
    [1, Y, J, ys, ys_gen],
    [2, Z, K, zs, zs_gen],
    [3, W, L, ws, ws_gen],
    [4, V, M, vs, vs_gen],
    [5, U, N, us, us_gen],
    [6, T, O, ts, ts_gen],
    [7, S, P, ss, ss_gen]
);
random_tuples!(
    (pub(crate)),
    RandomDuodecuples,
    RandomDuodecuplesFromSingle,
    random_duodecuples,
    random_duodecuples_from_single,
    (
        I::Item,
        I::Item,
        I::Item,
        I::Item,
        I::Item,
        I::Item,
        I::Item,
        I::Item,
        I::Item,
        I::Item,
        I::Item,
        I::Item
    ),
    [0, X, I, xs, xs_gen],
    [1, Y, J, ys, ys_gen],
    [2, Z, K, zs, zs_gen],
    [3, W, L, ws, ws_gen],
    [4, V, M, vs, vs_gen],
    [5, U, N, us, us_gen],
    [6, T, OI, ts, ts_gen],
    [7, S, PI, ss, ss_gen],
    [8, R, QI, rs, rs_gen],
    [9, Q, RI, qs, qs_gen],
    [10, P, SI, ps, ps_gen],
    [11, O, TI, os, os_gen]
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
    RandomTriplesXYX,
    (X, Y, X),
    random_triples_xyx,
    [X, I, xs, xs_gen, [x_0, x_0], [x_2, y_1]],
    [Y, J, ys, ys_gen, [y_1, x_2]]
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
    RandomQuadruplesXXXY,
    (X, X, X, Y),
    random_quadruples_xxxy,
    [X, I, xs, xs_gen, [x_0, x_0], [x_1, x_1], [x_2, x_2]],
    [Y, J, ys, ys_gen, [y_3, y_3]]
);
random_custom_tuples!(
    (pub(crate)),
    RandomQuadruplesXXYX,
    (X, X, Y, X),
    random_quadruples_xxyx,
    [X, I, xs, xs_gen, [x_0, x_0], [x_1, x_1], [x_3, y_2]],
    [Y, J, ys, ys_gen, [y_2, x_3]]
);
random_custom_tuples!(
    (pub(crate)),
    RandomQuadruplesXYYX,
    (X, Y, Y, X),
    random_quadruples_xyyx,
    [X, I, xs, xs_gen, [x_0, x_0], [x_3, y_1]],
    [Y, J, ys, ys_gen, [y_1, y_2], [y_2, x_3]]
);
random_custom_tuples!(
    (pub(crate)),
    RandomQuadruplesXYXY,
    (X, Y, X, Y),
    random_quadruples_xyxy,
    [X, I, xs, xs_gen, [x_0, x_0], [x_2, y_1]],
    [Y, J, ys, ys_gen, [y_1, x_2], [y_3, y_3]]
);
random_custom_tuples!(
    (pub(crate)),
    RandomQuadruplesXYYZ,
    (X, Y, Y, Z),
    random_quadruples_xyyz,
    [X, I, xs, xs_gen, [x_0, x_0]],
    [Y, J, ys, ys_gen, [y_1, y_1], [y_2, y_2]],
    [Z, K, zs, zs_gen, [z_3, z_3]]
);
random_custom_tuples!(
    (pub(crate)),
    RandomQuadruplesXYZZ,
    (X, Y, Z, Z),
    random_quadruples_xyzz,
    [X, I, xs, xs_gen, [x_0, x_0]],
    [Y, J, ys, ys_gen, [y_1, y_1]],
    [Z, K, zs, zs_gen, [z_2, z_2], [z_3, z_3]]
);

exhaustive_unions!(
    (pub(crate)),
    Union3,
    LexUnion3s,
    ExhaustiveUnion3s,
    lex_union3s,
    exhaustive_union3s,
    3,
    [0, X, I, A, xs, xs_done],
    [1, Y, J, B, ys, ys_done],
    [2, Z, K, C, zs, zs_done]
);

union_struct!(
    (pub(crate)),
    Union3,
    Union3<T, T, T>,
    [A, A, 'A', a],
    [B, B, 'B', b],
    [C, C, 'C', c]
);

random_unions!(
    (pub(crate)),
    Union3,
    RandomUnion3s,
    random_union3s,
    3,
    [0, X, I, A, xs, xs_gen],
    [1, Y, J, B, ys, ys_gen],
    [2, Z, K, C, zs, zs_gen]
);
