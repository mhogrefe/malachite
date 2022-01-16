use malachite_base::rational_sequences::RationalSequence;
use malachite_base_test_util::generators::{
    unsigned_rational_sequence_gen, unsigned_rational_sequence_pair_gen,
    unsigned_rational_sequence_triple_gen,
};
use std::cmp::Ordering;

#[test]
fn test_cmp() {
    let xs = &[
        RationalSequence::from_vec(vec![]),
        RationalSequence::from_vec(vec![1, 2, 3]),
        RationalSequence::from_vecs(vec![], vec![1, 2, 3]),
        RationalSequence::from_vecs(vec![1, 2, 3], vec![4, 5, 6]),
        RationalSequence::from_vec(vec![1, 2, 4]),
    ];
    for (i, x) in xs.iter().enumerate() {
        for (j, y) in xs.iter().enumerate() {
            assert_eq!(i.cmp(&j), x.clone().cmp(&y.clone()));
        }
    }
}

#[test]
fn cmp_properties() {
    unsigned_rational_sequence_pair_gen::<u8>().test_properties(|(xs, ys)| {
        let ord = xs.cmp(&ys);
        assert_eq!(ys.cmp(&xs).reverse(), ord);
        assert_eq!(xs == ys, xs.cmp(&ys) == Ordering::Equal);
    });

    let empty = RationalSequence::from_vec(vec![]);
    unsigned_rational_sequence_gen::<u8>().test_properties(|xs| {
        assert_eq!(xs.cmp(&xs), Ordering::Equal);
        assert!(xs >= empty);
    });

    unsigned_rational_sequence_triple_gen::<u8>().test_properties(|(xs, ys, zs)| {
        if xs < ys && ys < zs {
            assert!(xs < zs);
        } else if xs > ys && ys > zs {
            assert!(xs > zs);
        }
    });
}
