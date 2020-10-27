use std::fmt::Debug;

use malachite_base::bools::exhaustive::exhaustive_bools;
use malachite_base::chars::exhaustive::exhaustive_ascii_chars;
use malachite_base::nevers::nevers;
use malachite_base::num::exhaustive::exhaustive_unsigneds;
use malachite_base::tuples::exhaustive::{
    exhaustive_octuples_from_single, exhaustive_pairs_from_single,
    exhaustive_quadruples_from_single, exhaustive_quintuples_from_single,
    exhaustive_triples_from_single, exhaustive_units,
};

macro_rules! helpers {
    ($t: ty, $ts: ident, $ts_helper: ident, $ts_small_helper: ident) => {
        fn $ts_helper<I: Iterator>(xs: I, out: &[$t])
        where
            I::Item: Clone + Debug + Eq,
        {
            let ts = $ts(xs).take(20).collect::<Vec<_>>();
            assert_eq!(ts.as_slice(), out);
        }

        fn $ts_small_helper<I: Clone + Iterator>(xs: I, out_len: usize, out: &[$t])
        where
            I::Item: Clone + Debug + Eq,
        {
            let ts = $ts(xs);
            let ts_prefix = ts.clone().take(20).collect::<Vec<_>>();
            assert_eq!(ts_prefix.as_slice(), out);
            assert_eq!(ts.count(), out_len);
        }
    };
}
helpers!(
    (I::Item, I::Item),
    exhaustive_pairs_from_single,
    exhaustive_pairs_from_single_helper,
    exhaustive_pairs_from_single_small_helper
);
helpers!(
    (I::Item, I::Item, I::Item),
    exhaustive_triples_from_single,
    exhaustive_triples_from_single_helper,
    _exhaustive_triples_from_single_small_helper
);
helpers!(
    (I::Item, I::Item, I::Item, I::Item),
    exhaustive_quadruples_from_single,
    _exhaustive_quadruples_from_single_helper,
    exhaustive_quadruples_from_single_small_helper
);
helpers!(
    (I::Item, I::Item, I::Item, I::Item, I::Item),
    exhaustive_quintuples_from_single,
    _exhaustive_quintuples_from_single_helper,
    exhaustive_quintuples_from_single_small_helper
);
helpers!(
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
    exhaustive_octuples_from_single,
    _exhaustive_octuples_from_single_helper,
    exhaustive_octuples_from_single_small_helper
);

#[test]
fn test_exhaustive_tuples_from_single() {
    exhaustive_pairs_from_single_small_helper(nevers(), 0, &[]);
    exhaustive_quintuples_from_single_small_helper(nevers(), 0, &[]);
    exhaustive_pairs_from_single_small_helper(exhaustive_units(), 1, &[((), ())]);
    exhaustive_quintuples_from_single_small_helper(exhaustive_units(), 1, &[((), (), (), (), ())]);
    exhaustive_pairs_from_single_helper(
        exhaustive_unsigneds::<u8>(),
        &[
            (0, 0),
            (0, 1),
            (1, 0),
            (1, 1),
            (0, 2),
            (0, 3),
            (1, 2),
            (1, 3),
            (2, 0),
            (2, 1),
            (3, 0),
            (3, 1),
            (2, 2),
            (2, 3),
            (3, 2),
            (3, 3),
            (0, 4),
            (0, 5),
            (1, 4),
            (1, 5),
        ],
    );
    exhaustive_triples_from_single_helper(
        exhaustive_unsigneds::<u8>(),
        &[
            (0, 0, 0),
            (0, 0, 1),
            (0, 1, 0),
            (0, 1, 1),
            (1, 0, 0),
            (1, 0, 1),
            (1, 1, 0),
            (1, 1, 1),
            (0, 0, 2),
            (0, 0, 3),
            (0, 1, 2),
            (0, 1, 3),
            (1, 0, 2),
            (1, 0, 3),
            (1, 1, 2),
            (1, 1, 3),
            (0, 2, 0),
            (0, 2, 1),
            (0, 3, 0),
            (0, 3, 1),
        ],
    );
    exhaustive_pairs_from_single_small_helper(
        exhaustive_ascii_chars(),
        0x4000,
        &[
            ('a', 'a'),
            ('a', 'b'),
            ('b', 'a'),
            ('b', 'b'),
            ('a', 'c'),
            ('a', 'd'),
            ('b', 'c'),
            ('b', 'd'),
            ('c', 'a'),
            ('c', 'b'),
            ('d', 'a'),
            ('d', 'b'),
            ('c', 'c'),
            ('c', 'd'),
            ('d', 'c'),
            ('d', 'd'),
            ('a', 'e'),
            ('a', 'f'),
            ('b', 'e'),
            ('b', 'f'),
        ],
    );
    exhaustive_pairs_from_single_small_helper(
        exhaustive_bools(),
        4,
        &[(false, false), (false, true), (true, false), (true, true)],
    );
    exhaustive_quadruples_from_single_small_helper(
        exhaustive_bools(),
        16,
        &[
            (false, false, false, false),
            (false, false, false, true),
            (false, false, true, false),
            (false, false, true, true),
            (false, true, false, false),
            (false, true, false, true),
            (false, true, true, false),
            (false, true, true, true),
            (true, false, false, false),
            (true, false, false, true),
            (true, false, true, false),
            (true, false, true, true),
            (true, true, false, false),
            (true, true, false, true),
            (true, true, true, false),
            (true, true, true, true),
        ],
    );
    exhaustive_octuples_from_single_small_helper(
        exhaustive_bools(),
        256,
        &[
            (false, false, false, false, false, false, false, false),
            (false, false, false, false, false, false, false, true),
            (false, false, false, false, false, false, true, false),
            (false, false, false, false, false, false, true, true),
            (false, false, false, false, false, true, false, false),
            (false, false, false, false, false, true, false, true),
            (false, false, false, false, false, true, true, false),
            (false, false, false, false, false, true, true, true),
            (false, false, false, false, true, false, false, false),
            (false, false, false, false, true, false, false, true),
            (false, false, false, false, true, false, true, false),
            (false, false, false, false, true, false, true, true),
            (false, false, false, false, true, true, false, false),
            (false, false, false, false, true, true, false, true),
            (false, false, false, false, true, true, true, false),
            (false, false, false, false, true, true, true, true),
            (false, false, false, true, false, false, false, false),
            (false, false, false, true, false, false, false, true),
            (false, false, false, true, false, false, true, false),
            (false, false, false, true, false, false, true, true),
        ],
    );
    exhaustive_octuples_from_single_small_helper(
        0..3,
        6561,
        &[
            (0, 0, 0, 0, 0, 0, 0, 0),
            (0, 0, 0, 0, 0, 0, 0, 1),
            (0, 0, 0, 0, 0, 0, 1, 0),
            (0, 0, 0, 0, 0, 0, 1, 1),
            (0, 0, 0, 0, 0, 1, 0, 0),
            (0, 0, 0, 0, 0, 1, 0, 1),
            (0, 0, 0, 0, 0, 1, 1, 0),
            (0, 0, 0, 0, 0, 1, 1, 1),
            (0, 0, 0, 0, 1, 0, 0, 0),
            (0, 0, 0, 0, 1, 0, 0, 1),
            (0, 0, 0, 0, 1, 0, 1, 0),
            (0, 0, 0, 0, 1, 0, 1, 1),
            (0, 0, 0, 0, 1, 1, 0, 0),
            (0, 0, 0, 0, 1, 1, 0, 1),
            (0, 0, 0, 0, 1, 1, 1, 0),
            (0, 0, 0, 0, 1, 1, 1, 1),
            (0, 0, 0, 1, 0, 0, 0, 0),
            (0, 0, 0, 1, 0, 0, 0, 1),
            (0, 0, 0, 1, 0, 0, 1, 0),
            (0, 0, 0, 1, 0, 0, 1, 1),
        ],
    );
    exhaustive_pairs_from_single_helper(
        exhaustive_pairs_from_single(exhaustive_unsigneds::<u8>()),
        &[
            ((0, 0), (0, 0)),
            ((0, 0), (0, 1)),
            ((0, 1), (0, 0)),
            ((0, 1), (0, 1)),
            ((0, 0), (1, 0)),
            ((0, 0), (1, 1)),
            ((0, 1), (1, 0)),
            ((0, 1), (1, 1)),
            ((1, 0), (0, 0)),
            ((1, 0), (0, 1)),
            ((1, 1), (0, 0)),
            ((1, 1), (0, 1)),
            ((1, 0), (1, 0)),
            ((1, 0), (1, 1)),
            ((1, 1), (1, 0)),
            ((1, 1), (1, 1)),
            ((0, 0), (0, 2)),
            ((0, 0), (0, 3)),
            ((0, 1), (0, 2)),
            ((0, 1), (0, 3)),
        ],
    );
}
