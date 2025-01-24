// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::iterators::bit_distributor::{BitDistributor, BitDistributorOutputType};

fn bit_distributor_helper(bit_distributor: &BitDistributor, bit_map: &[usize]) {
    assert_eq!(bit_distributor.bit_map_as_slice(), bit_map);
}

#[test]
fn test_bit_map_as_slice() {
    bit_distributor_helper(
        &BitDistributor::new(&[BitDistributorOutputType::normal(1)]),
        &[0; 64],
    );
    bit_distributor_helper(
        &BitDistributor::new(&[BitDistributorOutputType::normal(1); 2]),
        &[
            1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1,
            0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0,
            1, 0, 1, 0, 1, 0,
        ],
    );
    bit_distributor_helper(
        &BitDistributor::new(&[BitDistributorOutputType::normal(1); 3]),
        &[
            2, 1, 0, 2, 1, 0, 2, 1, 0, 2, 1, 0, 2, 1, 0, 2, 1, 0, 2, 1, 0, 2, 1, 0, 2, 1, 0, 2, 1,
            0, 2, 1, 0, 2, 1, 0, 2, 1, 0, 2, 1, 0, 2, 1, 0, 2, 1, 0, 2, 1, 0, 2, 1, 0, 2, 1, 0, 2,
            1, 0, 2, 1, 0, 2,
        ],
    );
    bit_distributor_helper(
        &BitDistributor::new(&[BitDistributorOutputType::normal(1); 5]),
        &[
            4, 3, 2, 1, 0, 4, 3, 2, 1, 0, 4, 3, 2, 1, 0, 4, 3, 2, 1, 0, 4, 3, 2, 1, 0, 4, 3, 2, 1,
            0, 4, 3, 2, 1, 0, 4, 3, 2, 1, 0, 4, 3, 2, 1, 0, 4, 3, 2, 1, 0, 4, 3, 2, 1, 0, 4, 3, 2,
            1, 0, 4, 3, 2, 1,
        ],
    );
    bit_distributor_helper(
        &BitDistributor::new(&[BitDistributorOutputType::normal(2); 2]),
        &[
            1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1,
            1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1,
            0, 0, 1, 1, 0, 0,
        ],
    );
    bit_distributor_helper(
        &BitDistributor::new(&[
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(2),
        ]),
        &[
            1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1,
            0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1,
            1, 0, 1, 1, 0, 1,
        ],
    );
    bit_distributor_helper(
        &BitDistributor::new(&[
            BitDistributorOutputType::normal(2),
            BitDistributorOutputType::normal(1),
        ]),
        &[
            1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0,
            0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1,
            0, 0, 1, 0, 0, 1,
        ],
    );
    bit_distributor_helper(
        &BitDistributor::new(&[
            BitDistributorOutputType::normal(2),
            BitDistributorOutputType::normal(3),
        ]),
        &[
            1, 1, 1, 0, 0, 1, 1, 1, 0, 0, 1, 1, 1, 0, 0, 1, 1, 1, 0, 0, 1, 1, 1, 0, 0, 1, 1, 1, 0,
            0, 1, 1, 1, 0, 0, 1, 1, 1, 0, 0, 1, 1, 1, 0, 0, 1, 1, 1, 0, 0, 1, 1, 1, 0, 0, 1, 1, 1,
            0, 0, 1, 1, 1, 0,
        ],
    );
    bit_distributor_helper(
        &BitDistributor::new(&[
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::tiny(),
        ]),
        &[
            1, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 1,
        ],
    );
    bit_distributor_helper(
        &BitDistributor::new(&[
            BitDistributorOutputType::tiny(),
            BitDistributorOutputType::normal(1),
        ]),
        &[
            0, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 0,
        ],
    );
    bit_distributor_helper(
        &BitDistributor::new(&[
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::tiny(),
            BitDistributorOutputType::tiny(),
        ]),
        &[
            2, 1, 0, 2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 2,
        ],
    );
    bit_distributor_helper(
        &BitDistributor::new(&[
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::normal(1),
            BitDistributorOutputType::tiny(),
            BitDistributorOutputType::tiny(),
        ]),
        &[
            3, 2, 1, 3, 0, 1, 0, 2, 1, 0, 1, 0, 1, 0, 1, 3, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0,
            1, 0, 2, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0,
            1, 0, 1, 0, 1, 3,
        ],
    );
    let mut bit_distributor = BitDistributor::new(&[BitDistributorOutputType::normal(1)]);
    bit_distributor.set_max_bits(&[0], 5);
    bit_distributor_helper(
        &bit_distributor,
        &[
            0,
            0,
            0,
            0,
            0,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
        ],
    );
    let mut bit_distributor = BitDistributor::new(&[BitDistributorOutputType::normal(1); 2]);
    bit_distributor.set_max_bits(&[1], 2);
    bit_distributor_helper(
        &bit_distributor,
        &[
            1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0,
        ],
    );
    let mut bit_distributor = BitDistributor::new(&[BitDistributorOutputType::normal(1); 2]);
    bit_distributor.set_max_bits(&[0], 2);
    bit_distributor_helper(
        &bit_distributor,
        &[
            1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1,
        ],
    );
    let mut bit_distributor = BitDistributor::new(&[
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::tiny(),
    ]);
    bit_distributor.set_max_bits(&[0], 2);
    bit_distributor_helper(
        &bit_distributor,
        &[
            1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1,
        ],
    );
    let mut bit_distributor = BitDistributor::new(&[
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::tiny(),
    ]);
    bit_distributor.set_max_bits(&[1], 2);
    bit_distributor_helper(
        &bit_distributor,
        &[
            1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0,
        ],
    );
    let mut bit_distributor = BitDistributor::new(&[BitDistributorOutputType::normal(1); 5]);
    bit_distributor.set_max_bits(&[0], 2);
    bit_distributor.set_max_bits(&[1], 1);
    bit_distributor.set_max_bits(&[2], 5);
    bit_distributor.set_max_bits(&[3], 3);
    bit_distributor.set_max_bits(&[4], 4);
    bit_distributor_helper(
        &bit_distributor,
        &[
            4,
            3,
            2,
            1,
            0,
            4,
            3,
            2,
            0,
            4,
            3,
            2,
            4,
            2,
            2,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
            usize::MAX,
        ],
    );
    let mut bit_distributor = BitDistributor::new(&[BitDistributorOutputType::normal(2); 2]);
    bit_distributor.set_max_bits(&[0], 5);
    bit_distributor_helper(
        &bit_distributor,
        &[
            1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1,
        ],
    );
    let mut bit_distributor = BitDistributor::new(&[
        BitDistributorOutputType::normal(1),
        BitDistributorOutputType::tiny(),
        BitDistributorOutputType::tiny(),
    ]);
    bit_distributor.set_max_bits(&[0], 5);
    bit_distributor_helper(
        &bit_distributor,
        &[
            2, 1, 0, 2, 0, 0, 0, 1, 0, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1,
            2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2,
            1, 2, 1, 2, 1, 2,
        ],
    );
}
