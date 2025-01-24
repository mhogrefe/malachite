// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::iterators::bit_distributor::{BitDistributor, BitDistributorOutputType};
use malachite_base::iterators::iterator_cache::IteratorCache;
use malachite_base::num::arithmetic::traits::CheckedPow;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::num::iterators::{ruler_sequence, RulerSequence};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::num::random::{random_unsigned_range, RandomUnsignedRange};
use malachite_base::random::Seed;
use malachite_base::sets::random::{random_b_tree_sets_fixed_length, RandomBTreeSetsFixedLength};
use malachite_base::tuples::exhaustive::{
    clone_helper, exhaustive_dependent_pairs, ExhaustiveDependentPairs,
};
use malachite_base::tuples::random::next_helper;
use malachite_base::unions::UnionFromStrError;
use malachite_base::vecs::exhaustive::{
    exhaustive_ordered_unique_vecs_fixed_length, fixed_length_ordered_unique_indices_helper,
    next_bit_pattern, unique_indices, validate_oi_map, ExhaustiveOrderedUniqueCollections,
    ExhaustiveUniqueVecsGenerator, LexFixedLengthVecsOutput, UniqueIndices,
};
use malachite_base::vecs::ExhaustiveVecPermutations;
use std::cmp::max;
use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};
use std::hash::Hash;
use std::marker::PhantomData;
use std::str::FromStr;

fn unwrap_triple<X, Y, Z>((a, b, c): (Option<X>, Option<Y>, Option<Z>)) -> (X, Y, Z) {
    (a.unwrap(), b.unwrap(), c.unwrap())
}

lex_custom_tuples! {
    (pub(crate)),
    LexTriplesXXY,
    (X, X, Y),
    (None, None, None),
    unwrap_triple,
    lex_triples_xxy,
    [X, I, xs, [0, x_0], [1, x_1]],
    [Y, J, ys, [2, y_2]]
}

lex_custom_tuples!(
    (pub(crate)),
    LexTriplesXYX,
    (X, Y, X),
    (None, None, None),
    unwrap_triple,
    lex_triples_xyx,
    [X, I, xs, [0, x_0], [2, x_2]],
    [Y, J, ys, [1, y_1]]
);

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

lex_tuples!(
    (pub(crate)),
    4,
    LexQuadruples,
    LexQuadruplesFromSingle,
    lex_quadruples,
    lex_quadruples_from_single,
    (T, T, T, T),
    [0, X, I, xs, x],
    [1, Y, J, ys, y],
    [2, Z, K, zs, z],
    [3, W, L, ws, w]
);

lex_tuples!(
    (pub(crate)),
    5,
    LexQuintuples,
    LexQuintuplesFromSingle,
    lex_quintuples,
    lex_quintuples_from_single,
    (T, T, T, T, T),
    [0, X, I, xs, x],
    [1, Y, J, ys, y],
    [2, Z, K, zs, z],
    [3, W, L, ws, w],
    [4, V, M, vs, v]
);

lex_tuples!(
    (pub(crate)),
    8,
    LexOctuples,
    LexOctuplesFromSingle,
    lex_octuples,
    lex_octuples_from_single,
    (T, T, T, T, T, T, T, T),
    [0, X, I, xs, x],
    [1, Y, J, ys, y],
    [2, Z, K, zs, z],
    [3, W, L, ws, w],
    [4, V, M, vs, v],
    [5, U, N, us, u],
    [6, T, O, ts, t],
    [7, S, P, ss, s]
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
    ExhaustiveQuintuples1Input,
    exhaustive_quintuples_1_input,
    exhaustive_quintuples_from_single,
    (I::Item, I::Item, I::Item, I::Item, I::Item),
    [0, output_type_x],
    [1, output_type_y],
    [2, output_type_z],
    [3, output_type_w],
    [4, output_type_v]
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

lex_ordered_unique_tuples!(
     (pub(crate)),
     LexOrderedUniqueTriples,
     3,
     (I::Item, I::Item, I::Item),
    lex_ordered_unique_triples,
    [0, 1, 2]
);
lex_ordered_unique_tuples!(
    (pub(crate)),
    LexOrderedUniqueQuadruples,
    4,
    (I::Item, I::Item, I::Item, I::Item),
    lex_ordered_unique_quadruples,
    [0, 1, 2, 3]
);
lex_ordered_unique_tuples!(
    (pub(crate)),
    LexOrderedUniqueQuintuples,
    5,
    (I::Item, I::Item, I::Item, I::Item, I::Item),
    lex_ordered_unique_quintuples,
    [0, 1, 2, 3, 4]
);

exhaustive_ordered_unique_tuples!(
    (pub(crate)),
    ExhaustiveOrderedUniqueTriples,
    3,
    (I::Item, I::Item, I::Item),
    exhaustive_ordered_unique_triples,
    [0, 1, 2]
);
exhaustive_ordered_unique_tuples!(
    (pub(crate)),
    ExhaustiveOrderedUniqueQuadruples,
    4,
    (I::Item, I::Item, I::Item, I::Item),
    exhaustive_ordered_unique_quadruples,
    [0, 1, 2, 3]
);
exhaustive_ordered_unique_tuples!(
    (pub(crate)),
    ExhaustiveOrderedUniqueQuintuples,
    5,
    (I::Item, I::Item, I::Item, I::Item, I::Item),
    exhaustive_ordered_unique_quintuples,
    [0, 1, 2, 3, 4]
);

lex_unique_tuples!(
     (pub(crate)),
     LexUniqueTriples,
     3,
     (I::Item, I::Item, I::Item),
    lex_unique_triples,
    [0, 1, 2]
);
lex_unique_tuples!(
    (pub(crate)),
    LexUniqueQuadruples,
    4,
    (I::Item, I::Item, I::Item, I::Item),
    lex_unique_quadruples,
    [0, 1, 2, 3]
);
lex_unique_tuples!(
    (pub(crate)),
    LexUniqueQuintuples,
    5,
    (I::Item, I::Item, I::Item, I::Item, I::Item),
    lex_unique_quintuples,
    [0, 1, 2, 3, 4]
);

exhaustive_unique_tuples!(
     (pub(crate)),
     ExhaustiveUniqueTriples,
     3,
     (I::Item, I::Item, I::Item),
     exhaustive_unique_triples,
     [0, 1, 2]
);
exhaustive_unique_tuples!(
    (pub(crate)),
    ExhaustiveUniqueQuadruples,
    4,
    (I::Item, I::Item, I::Item, I::Item),
    exhaustive_unique_quadruples,
    [0, 1, 2, 3]
);
exhaustive_unique_tuples!(
    (pub(crate)),
    ExhaustiveUniqueQuintuples,
    5,
    (I::Item, I::Item, I::Item, I::Item, I::Item),
    exhaustive_unique_quintuples,
    [0, 1, 2, 3, 4]
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
    RandomTriplesXYX,
    (X, Y, X),
    random_triples_xyx,
    [X, I, xs, xs_gen, [x_0, x_0], [x_2, y_1]],
    [Y, J, ys, ys_gen, [y_1, x_2]]
);

random_ordered_unique_tuples!(
    (pub(crate)),
    RandomOrderedUniqueTriples,
    3,
    (I::Item, I::Item, I::Item),
    random_ordered_unique_triples,
    [0, 1, 2]
);
random_unique_tuples!(
    (pub(crate)),
    RandomUniqueTriples,
    3,
    (I::Item, I::Item, I::Item),
    random_unique_triples,
    [0, 1, 2]
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

lex_vecs_fixed_length!(
    (pub(crate)),
    LexFixedLengthVecs3Inputs,
    lex_vecs_fixed_length_3_inputs,
    lex_vecs_length_3,
    [0, I, xs, xs_outputs],
    [1, J, ys, ys_outputs],
    [2, K, zs, zs_outputs]
);

exhaustive_vecs_fixed_length!(
    (pub(crate)),
    ExhaustiveFixedLengthVecs3Inputs,
    exhaustive_vecs_fixed_length_3_inputs,
    exhaustive_vecs_length_3,
    [0, I, xs, xs_done, xs_outputs],
    [1, J, ys, ys_done, ys_outputs],
    [2, K, zs, zs_done, zs_outputs]
);

random_vecs_fixed_length!(
    (pub(crate)),
    RandomFixedLengthVecs3Inputs,
    random_vecs_fixed_length_3_inputs,
    random_vecs_length_3,
    [0, I, xs, xs_gen],
    [1, J, ys, ys_gen],
    [2, K, zs, zs_gen]
);
