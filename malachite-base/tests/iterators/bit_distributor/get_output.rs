// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::iterators::bit_distributor::{BitDistributor, BitDistributorOutputType};

fn bit_distributor_helper(mut bit_distributor: BitDistributor, expected_outputs: &[&[usize]]) {
    let mut outputs = Vec::new();
    for _ in 0..20 {
        outputs.push(
            (0..bit_distributor.output_types.len())
                .map(|i| bit_distributor.get_output(i))
                .collect::<Vec<usize>>(),
        );
        bit_distributor.increment_counter();
    }
    assert_eq!(outputs, expected_outputs);
}

#[test]
fn test_get_output() {
    bit_distributor_helper(
        BitDistributor::new(&[BitDistributorOutputType::normal(1)]),
        &[
            &[0],
            &[1],
            &[2],
            &[3],
            &[4],
            &[5],
            &[6],
            &[7],
            &[8],
            &[9],
            &[10],
            &[11],
            &[12],
            &[13],
            &[14],
            &[15],
            &[16],
            &[17],
            &[18],
            &[19],
        ],
    );
    bit_distributor_helper(
        BitDistributor::new(&[BitDistributorOutputType::normal(1); 2]),
        &[
            &[0, 0],
            &[0, 1],
            &[1, 0],
            &[1, 1],
            &[0, 2],
            &[0, 3],
            &[1, 2],
            &[1, 3],
            &[2, 0],
            &[2, 1],
            &[3, 0],
            &[3, 1],
            &[2, 2],
            &[2, 3],
            &[3, 2],
            &[3, 3],
            &[0, 4],
            &[0, 5],
            &[1, 4],
            &[1, 5],
        ],
    );
    bit_distributor_helper(
        BitDistributor::new(&[BitDistributorOutputType::normal(1); 3]),
        &[
            &[0, 0, 0],
            &[0, 0, 1],
            &[0, 1, 0],
            &[0, 1, 1],
            &[1, 0, 0],
            &[1, 0, 1],
            &[1, 1, 0],
            &[1, 1, 1],
            &[0, 0, 2],
            &[0, 0, 3],
            &[0, 1, 2],
            &[0, 1, 3],
            &[1, 0, 2],
            &[1, 0, 3],
            &[1, 1, 2],
            &[1, 1, 3],
            &[0, 2, 0],
            &[0, 2, 1],
            &[0, 3, 0],
            &[0, 3, 1],
        ],
    );
    bit_distributor_helper(
        BitDistributor::new(&[BitDistributorOutputType::normal(1); 5]),
        &[
            &[0, 0, 0, 0, 0],
            &[0, 0, 0, 0, 1],
            &[0, 0, 0, 1, 0],
            &[0, 0, 0, 1, 1],
            &[0, 0, 1, 0, 0],
            &[0, 0, 1, 0, 1],
            &[0, 0, 1, 1, 0],
            &[0, 0, 1, 1, 1],
            &[0, 1, 0, 0, 0],
            &[0, 1, 0, 0, 1],
            &[0, 1, 0, 1, 0],
            &[0, 1, 0, 1, 1],
            &[0, 1, 1, 0, 0],
            &[0, 1, 1, 0, 1],
            &[0, 1, 1, 1, 0],
            &[0, 1, 1, 1, 1],
            &[1, 0, 0, 0, 0],
            &[1, 0, 0, 0, 1],
            &[1, 0, 0, 1, 0],
            &[1, 0, 0, 1, 1],
        ],
    );
    bit_distributor_helper(
        BitDistributor::new(&[BitDistributorOutputType::normal(2); 2]),
        &[
            &[0, 0],
            &[0, 1],
            &[0, 2],
            &[0, 3],
            &[1, 0],
            &[1, 1],
            &[1, 2],
            &[1, 3],
            &[2, 0],
            &[2, 1],
            &[2, 2],
            &[2, 3],
            &[3, 0],
            &[3, 1],
            &[3, 2],
            &[3, 3],
            &[0, 4],
            &[0, 5],
            &[0, 6],
            &[0, 7],
        ],
    );
    bit_distributor_helper(
        BitDistributor::new(&[
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(2),
        ]),
        &[
            &[0, 0],
            &[0, 1],
            &[0, 2],
            &[0, 3],
            &[1, 0],
            &[1, 1],
            &[1, 2],
            &[1, 3],
            &[0, 4],
            &[0, 5],
            &[0, 6],
            &[0, 7],
            &[1, 4],
            &[1, 5],
            &[1, 6],
            &[1, 7],
            &[0, 8],
            &[0, 9],
            &[0, 10],
            &[0, 11],
        ],
    );
    bit_distributor_helper(
        BitDistributor::new(&[
            BitDistributorOutputType::normal(2),
            BitDistributorOutputType::normal(1),
        ]),
        &[
            &[0, 0],
            &[0, 1],
            &[1, 0],
            &[1, 1],
            &[2, 0],
            &[2, 1],
            &[3, 0],
            &[3, 1],
            &[0, 2],
            &[0, 3],
            &[1, 2],
            &[1, 3],
            &[2, 2],
            &[2, 3],
            &[3, 2],
            &[3, 3],
            &[4, 0],
            &[4, 1],
            &[5, 0],
            &[5, 1],
        ],
    );
    bit_distributor_helper(
        BitDistributor::new(&[
            BitDistributorOutputType::normal(2),
            BitDistributorOutputType::normal(3),
        ]),
        &[
            &[0, 0],
            &[0, 1],
            &[0, 2],
            &[0, 3],
            &[0, 4],
            &[0, 5],
            &[0, 6],
            &[0, 7],
            &[1, 0],
            &[1, 1],
            &[1, 2],
            &[1, 3],
            &[1, 4],
            &[1, 5],
            &[1, 6],
            &[1, 7],
            &[2, 0],
            &[2, 1],
            &[2, 2],
            &[2, 3],
        ],
    );
    bit_distributor_helper(
        BitDistributor::new(&[
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::tiny(),
        ]),
        &[
            &[0, 0],
            &[0, 1],
            &[0, 2],
            &[0, 3],
            &[1, 0],
            &[1, 1],
            &[1, 2],
            &[1, 3],
            &[0, 4],
            &[0, 5],
            &[0, 6],
            &[0, 7],
            &[1, 4],
            &[1, 5],
            &[1, 6],
            &[1, 7],
            &[2, 0],
            &[2, 1],
            &[2, 2],
            &[2, 3],
        ],
    );
    bit_distributor_helper(
        BitDistributor::new(&[
            BitDistributorOutputType::tiny(),
            BitDistributorOutputType::normal(1),
        ]),
        &[
            &[0, 0],
            &[1, 0],
            &[2, 0],
            &[3, 0],
            &[0, 1],
            &[1, 1],
            &[2, 1],
            &[3, 1],
            &[4, 0],
            &[5, 0],
            &[6, 0],
            &[7, 0],
            &[4, 1],
            &[5, 1],
            &[6, 1],
            &[7, 1],
            &[0, 2],
            &[1, 2],
            &[2, 2],
            &[3, 2],
        ],
    );
    bit_distributor_helper(
        BitDistributor::new(&[
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::tiny(),
            BitDistributorOutputType::tiny(),
        ]),
        &[
            &[0, 0, 0],
            &[0, 0, 1],
            &[0, 1, 0],
            &[0, 1, 1],
            &[1, 0, 0],
            &[1, 0, 1],
            &[1, 1, 0],
            &[1, 1, 1],
            &[0, 0, 2],
            &[0, 0, 3],
            &[0, 1, 2],
            &[0, 1, 3],
            &[1, 0, 2],
            &[1, 0, 3],
            &[1, 1, 2],
            &[1, 1, 3],
            &[2, 0, 0],
            &[2, 0, 1],
            &[2, 1, 0],
            &[2, 1, 1],
        ],
    );
    bit_distributor_helper(
        BitDistributor::new(&[
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::tiny(),
            BitDistributorOutputType::tiny(),
        ]),
        &[
            &[0, 0, 0, 0],
            &[0, 0, 0, 1],
            &[0, 0, 1, 0],
            &[0, 0, 1, 1],
            &[0, 1, 0, 0],
            &[0, 1, 0, 1],
            &[0, 1, 1, 0],
            &[0, 1, 1, 1],
            &[0, 0, 0, 2],
            &[0, 0, 0, 3],
            &[0, 0, 1, 2],
            &[0, 0, 1, 3],
            &[0, 1, 0, 2],
            &[0, 1, 0, 3],
            &[0, 1, 1, 2],
            &[0, 1, 1, 3],
            &[1, 0, 0, 0],
            &[1, 0, 0, 1],
            &[1, 0, 1, 0],
            &[1, 0, 1, 1],
        ],
    );
    let mut bit_distributor = BitDistributor::new(&[BitDistributorOutputType::normal(1)]);
    bit_distributor.set_max_bits(&[0], 5);
    bit_distributor_helper(
        bit_distributor,
        &[
            &[0],
            &[1],
            &[2],
            &[3],
            &[4],
            &[5],
            &[6],
            &[7],
            &[8],
            &[9],
            &[10],
            &[11],
            &[12],
            &[13],
            &[14],
            &[15],
            &[16],
            &[17],
            &[18],
            &[19],
        ],
    );
    let mut bit_distributor = BitDistributor::new(&[BitDistributorOutputType::normal(1); 2]);
    bit_distributor.set_max_bits(&[1], 2);
    bit_distributor_helper(
        bit_distributor,
        &[
            &[0, 0],
            &[0, 1],
            &[1, 0],
            &[1, 1],
            &[0, 2],
            &[0, 3],
            &[1, 2],
            &[1, 3],
            &[2, 0],
            &[2, 1],
            &[3, 0],
            &[3, 1],
            &[2, 2],
            &[2, 3],
            &[3, 2],
            &[3, 3],
            &[4, 0],
            &[4, 1],
            &[5, 0],
            &[5, 1],
        ],
    );
    let mut bit_distributor = BitDistributor::new(&[BitDistributorOutputType::normal(1); 2]);
    bit_distributor.set_max_bits(&[0], 2);
    bit_distributor_helper(
        bit_distributor,
        &[
            &[0, 0],
            &[0, 1],
            &[1, 0],
            &[1, 1],
            &[0, 2],
            &[0, 3],
            &[1, 2],
            &[1, 3],
            &[2, 0],
            &[2, 1],
            &[3, 0],
            &[3, 1],
            &[2, 2],
            &[2, 3],
            &[3, 2],
            &[3, 3],
            &[0, 4],
            &[0, 5],
            &[1, 4],
            &[1, 5],
        ],
    );
    let mut bit_distributor = BitDistributor::new(&[
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::tiny(),
    ]);
    bit_distributor.set_max_bits(&[0], 2);
    bit_distributor_helper(
        bit_distributor,
        &[
            &[0, 0],
            &[0, 1],
            &[0, 2],
            &[0, 3],
            &[1, 0],
            &[1, 1],
            &[1, 2],
            &[1, 3],
            &[0, 4],
            &[0, 5],
            &[0, 6],
            &[0, 7],
            &[1, 4],
            &[1, 5],
            &[1, 6],
            &[1, 7],
            &[2, 0],
            &[2, 1],
            &[2, 2],
            &[2, 3],
        ],
    );
    let mut bit_distributor = BitDistributor::new(&[
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::tiny(),
    ]);
    bit_distributor.set_max_bits(&[1], 2);
    bit_distributor_helper(
        bit_distributor,
        &[
            &[0, 0],
            &[0, 1],
            &[0, 2],
            &[0, 3],
            &[1, 0],
            &[1, 1],
            &[1, 2],
            &[1, 3],
            &[2, 0],
            &[2, 1],
            &[2, 2],
            &[2, 3],
            &[3, 0],
            &[3, 1],
            &[3, 2],
            &[3, 3],
            &[4, 0],
            &[4, 1],
            &[4, 2],
            &[4, 3],
        ],
    );
    let mut bit_distributor = BitDistributor::new(&[BitDistributorOutputType::normal(1); 5]);
    bit_distributor.set_max_bits(&[0], 2);
    bit_distributor.set_max_bits(&[1], 1);
    bit_distributor.set_max_bits(&[2], 5);
    bit_distributor.set_max_bits(&[3], 3);
    bit_distributor.set_max_bits(&[4], 4);
    bit_distributor_helper(
        bit_distributor,
        &[
            &[0, 0, 0, 0, 0],
            &[0, 0, 0, 0, 1],
            &[0, 0, 0, 1, 0],
            &[0, 0, 0, 1, 1],
            &[0, 0, 1, 0, 0],
            &[0, 0, 1, 0, 1],
            &[0, 0, 1, 1, 0],
            &[0, 0, 1, 1, 1],
            &[0, 1, 0, 0, 0],
            &[0, 1, 0, 0, 1],
            &[0, 1, 0, 1, 0],
            &[0, 1, 0, 1, 1],
            &[0, 1, 1, 0, 0],
            &[0, 1, 1, 0, 1],
            &[0, 1, 1, 1, 0],
            &[0, 1, 1, 1, 1],
            &[1, 0, 0, 0, 0],
            &[1, 0, 0, 0, 1],
            &[1, 0, 0, 1, 0],
            &[1, 0, 0, 1, 1],
        ],
    );
    let mut bit_distributor = BitDistributor::new(&[BitDistributorOutputType::normal(2); 2]);
    bit_distributor.set_max_bits(&[0], 5);
    bit_distributor_helper(
        bit_distributor,
        &[
            &[0, 0],
            &[0, 1],
            &[0, 2],
            &[0, 3],
            &[1, 0],
            &[1, 1],
            &[1, 2],
            &[1, 3],
            &[2, 0],
            &[2, 1],
            &[2, 2],
            &[2, 3],
            &[3, 0],
            &[3, 1],
            &[3, 2],
            &[3, 3],
            &[0, 4],
            &[0, 5],
            &[0, 6],
            &[0, 7],
        ],
    );
    let mut bit_distributor = BitDistributor::new(&[
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::tiny(),
        BitDistributorOutputType::tiny(),
    ]);
    bit_distributor.set_max_bits(&[0], 5);
    bit_distributor_helper(
        bit_distributor,
        &[
            &[0, 0, 0],
            &[0, 0, 1],
            &[0, 1, 0],
            &[0, 1, 1],
            &[1, 0, 0],
            &[1, 0, 1],
            &[1, 1, 0],
            &[1, 1, 1],
            &[0, 0, 2],
            &[0, 0, 3],
            &[0, 1, 2],
            &[0, 1, 3],
            &[1, 0, 2],
            &[1, 0, 3],
            &[1, 1, 2],
            &[1, 1, 3],
            &[2, 0, 0],
            &[2, 0, 1],
            &[2, 1, 0],
            &[2, 1, 1],
        ],
    );
}

#[test]
#[should_panic]
fn get_output_fail() {
    let bd = BitDistributor::new(&[BitDistributorOutputType::normal(2); 3]);
    bd.get_output(4);
}
