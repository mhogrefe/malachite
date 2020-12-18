use malachite_base::iterators::bit_distributor::BitDistributorOutputType;
use malachite_base::num::iterators::bit_distributor_sequence;

fn bit_distributor_sequence_helper(
    x_output_type: BitDistributorOutputType,
    y_output_type: BitDistributorOutputType,
    out: &[usize],
) {
    assert_eq!(
        bit_distributor_sequence(x_output_type, y_output_type)
            .take(50)
            .collect::<Vec<_>>(),
        out
    );
}

#[test]
fn test_bit_distributor_sequence() {
    bit_distributor_sequence_helper(
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::normal(1),
        &[
            0, 1, 0, 1, 2, 3, 2, 3, 0, 1, 0, 1, 2, 3, 2, 3, 4, 5, 4, 5, 6, 7, 6, 7, 4, 5, 4, 5, 6,
            7, 6, 7, 0, 1, 0, 1, 2, 3, 2, 3, 0, 1, 0, 1, 2, 3, 2, 3, 4, 5,
        ],
    );
    bit_distributor_sequence_helper(
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::normal(2),
        &[
            0, 1, 0, 1, 0, 1, 0, 1, 2, 3, 2, 3, 2, 3, 2, 3, 0, 1, 0, 1, 0, 1, 0, 1, 2, 3, 2, 3, 2,
            3, 2, 3, 0, 1, 0, 1, 0, 1, 0, 1, 2, 3, 2, 3, 2, 3, 2, 3, 0, 1,
        ],
    );
    bit_distributor_sequence_helper(
        BitDistributorOutputType::normal(2),
        BitDistributorOutputType::normal(1),
        &[
            0, 1, 2, 3, 0, 1, 2, 3, 4, 5, 6, 7, 4, 5, 6, 7, 8, 9, 10, 11, 8, 9, 10, 11, 12, 13, 14,
            15, 12, 13, 14, 15, 0, 1, 2, 3, 0, 1, 2, 3, 4, 5, 6, 7, 4, 5, 6, 7, 8, 9,
        ],
    );
    bit_distributor_sequence_helper(
        BitDistributorOutputType::tiny(),
        BitDistributorOutputType::normal(1),
        &[
            0, 1, 2, 3, 0, 1, 2, 3, 4, 5, 6, 7, 4, 5, 6, 7, 0, 1, 2, 3, 0, 1, 2, 3, 4, 5, 6, 7, 4,
            5, 6, 7, 0, 1, 2, 3, 0, 1, 2, 3, 4, 5, 6, 7, 4, 5, 6, 7, 0, 1,
        ],
    );
    bit_distributor_sequence_helper(
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::tiny(),
        &[
            0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 2, 2, 2, 2, 3,
            3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 4, 4, 4, 4, 5, 5, 5, 5, 6, 6,
        ],
    );
}

#[test]
#[should_panic]
fn test_bit_distributor_sequence_fail() {
    bit_distributor_sequence(
        BitDistributorOutputType::tiny(),
        BitDistributorOutputType::tiny(),
    );
}