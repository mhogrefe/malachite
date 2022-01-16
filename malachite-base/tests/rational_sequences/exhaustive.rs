use itertools::Itertools;
use malachite_base::bools::exhaustive::exhaustive_bools;
use malachite_base::nevers::nevers;
use malachite_base::num::exhaustive::exhaustive_unsigneds;
use malachite_base::rational_sequences::exhaustive::exhaustive_rational_sequences;
use malachite_base::strings::ExtraToString;
use malachite_base::tuples::exhaustive::exhaustive_units;
use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Eq, PartialEq)]
struct Unit(());

impl Display for Unit {
    fn fmt(&self, f: &mut Formatter) -> Result {
        f.write_str("()")
    }
}

#[test]
fn test_exhaustive_rational_sequences() {
    assert_eq!(
        exhaustive_rational_sequences(nevers())
            .collect_vec()
            .to_string(),
        "[[]]"
    );
    assert_eq!(
        exhaustive_rational_sequences(exhaustive_units().map(Unit))
            .take(20)
            .collect_vec()
            .to_string(),
        "[[], [[()]], [()], [(), ()], [(), (), (), ()], [(), (), ()], [(), (), (), (), ()], \
        [(), (), (), (), (), ()], [(), (), (), (), (), (), (), (), ()], \
        [(), (), (), (), (), (), ()], [(), (), (), (), (), (), (), ()], \
        [(), (), (), (), (), (), (), (), (), ()], \
        [(), (), (), (), (), (), (), (), (), (), (), ()], \
        [(), (), (), (), (), (), (), (), (), (), ()], \
        [(), (), (), (), (), (), (), (), (), (), (), (), ()], \
        [(), (), (), (), (), (), (), (), (), (), (), (), (), ()], \
        [(), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), ()], \
        [(), (), (), (), (), (), (), (), (), (), (), (), (), (), ()], \
        [(), (), (), (), (), (), (), (), (), (), (), (), (), (), (), ()], \
        [(), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), ()]]"
    );
    assert_eq!(
        exhaustive_rational_sequences(exhaustive_bools())
            .take(20)
            .collect_vec()
            .to_string(),
        "[[], [[false]], [false], [[true]], [false, [true]], [true], [true, [false]], \
        [false, false, false], [false, false, false, [true]], [[false, false, true]], \
        [false, [false, false, true]], [[false, true]], [false, [false, true]], \
        [false, false, false, [false, false, true]], [false, false, false, [false, true]], \
        [false, false], [false, false, true], [false, false, true, [false]], \
        [false, false, [true]], [false, true]]"
    );
    assert_eq!(
        exhaustive_rational_sequences(exhaustive_unsigneds::<u8>())
            .take(20)
            .collect_vec()
            .to_string(),
        "[[], [[0]], [0], [[1]], [0, [1]], [1], [1, [0]], [0, 0, 0], [0, 0, 0, [1]], [[2]], \
        [0, [2]], [[3]], [0, [3]], [1, [2]], [0, 0, 0, [2]], [1, [3]], [0, 0, 0, [3]], [2], \
        [2, [0]], [0, 0]]"
    );
}
