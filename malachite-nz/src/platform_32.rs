// Copyright © 2024 Mikhail Hogrefe
//
// Uses code adopted from the GNU MP Library.
//
//      `bin2kk`, `bin2kkinv`, and `fac2bin` contributed to the GNU project by Torbjörn Granlund
//      and Marco Bodrato.
//
//      Copyright © 2002, 2010-2018 Free Software Foundation, Inc.
//
// Uses code adopted from the GNU MPFR Library.
//
//      Copyright © 2005-2022 Free Software Foundation, Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

pub type Limb = u32;
pub type HalfLimb = u16;
pub type DoubleLimb = u64;
pub type SignedLimb = i32;
pub type SignedHalfLimb = i16;
pub type SignedDoubleLimb = i64;
pub type FloatWithLimbWidth = f32;

pub const MAX_DIGITS_PER_LIMB: usize = 10;

// TODO tune
pub const AORSMUL_FASTER_2AORSLSH: bool = true;
// TODO tune
pub const AORSMUL_FASTER_3AORSLSH: bool = true;
// TODO tune
pub const AORSMUL_FASTER_AORS_AORSLSH: bool = true;
// TODO tune
pub const AORSMUL_FASTER_AORS_2AORSLSH: bool = true;

// TODO tune
pub const MUL_TOOM22_THRESHOLD: usize = 118;
// TODO tune
pub const MUL_TOOM33_THRESHOLD: usize = 101;
// TODO tune
pub const MUL_TOOM44_THRESHOLD: usize = 530;
// TODO tune
pub const MUL_TOOM6H_THRESHOLD: usize = 738;
// TODO tune
pub const MUL_TOOM8H_THRESHOLD: usize = 984;

// TODO tune
pub const MUL_TOOM32_TO_TOOM43_THRESHOLD: usize = 315;
// TODO tune
pub const MUL_TOOM32_TO_TOOM53_THRESHOLD: usize = 307;
// TODO tune
pub const MUL_TOOM42_TO_TOOM53_THRESHOLD: usize = 328;
// TODO tune
pub const MUL_TOOM42_TO_TOOM63_THRESHOLD: usize = 295;

// TODO tune
pub const MUL_FFT_THRESHOLD: usize = 5608;

// TODO tune
pub const DC_DIV_QR_THRESHOLD: usize = 7;
// TODO tune
pub const DC_DIVAPPR_Q_THRESHOLD: usize = 151;
// TODO tune
pub const MAYBE_DCP1_DIVAPPR: bool = true;
// TODO tune
pub const INV_NEWTON_THRESHOLD: usize = 618;
// TODO tune
pub const MU_DIV_QR_THRESHOLD: usize = 2243;
// TODO tune
pub const INV_MULMOD_BNM1_THRESHOLD: usize = 68;
// TODO tune
pub const MU_DIV_QR_SKEW_THRESHOLD: usize = 233;

// TODO tune
pub const MU_DIVAPPR_Q_THRESHOLD: usize = 2297;
// TODO tune
pub const FUDGE: usize = 261;

// TODO tune
pub const MULLO_BASECASE_THRESHOLD: usize = 0;
// TODO tune
pub const MULLO_DC_THRESHOLD: usize = 216;
// TODO tune
pub const MULLO_MUL_N_THRESHOLD: usize = 100000;

// TODO tune
pub const BINV_NEWTON_THRESHOLD: usize = 3264;
// TODO tune
pub const DC_BDIV_QR_THRESHOLD: usize = 329;
// TODO tune
pub const MU_BDIV_QR_THRESHOLD: usize = 50000;
// TODO tune
pub const DC_BDIV_Q_THRESHOLD: usize = 373;
// TODO tune
pub const MU_BDIV_Q_THRESHOLD: usize = 2390;

// TODO tune
pub const MOD_1_NORM_THRESHOLD: usize = 0;
// TODO tune
pub const MOD_1_UNNORM_THRESHOLD: usize = 0;
// TODO tune
pub const MOD_1_1P_METHOD: bool = true;
// TODO tune
pub const MOD_1N_TO_MOD_1_1_THRESHOLD: usize = 3;
// TODO tune
pub const MOD_1U_TO_MOD_1_1_THRESHOLD: usize = 3;
// TODO tune
pub const MOD_1_1_TO_MOD_1_2_THRESHOLD: usize = 15;
// TODO tune
pub const MOD_1_2_TO_MOD_1_4_THRESHOLD: usize = 43;

// TODO tune
pub const BMOD_1_TO_MOD_1_THRESHOLD: usize = 31;

// TODO tune
pub const SQR_BASECASE_THRESHOLD: usize = 0;
// TODO tune
pub const SQR_TOOM2_THRESHOLD: usize = 222;
// TODO tune
pub const SQR_TOOM3_THRESHOLD: usize = 205;
// TODO tune
pub const SQR_TOOM4_THRESHOLD: usize = 1170;
// TODO tune
pub const SQR_TOOM6_THRESHOLD: usize = 512;
// TODO tune
pub const SQR_TOOM8_THRESHOLD: usize = 644;

// TODO tune
pub const SQRLO_DC_THRESHOLD: usize = 460;

// TODO tune
pub const FROM_DIGITS_DIVIDE_AND_CONQUER_THRESHOLD: u64 = 12000;

// TODO tune
pub const MATRIX22_STRASSEN_THRESHOLD: usize = 50;

// This section is created by digits_data.rs.

// mp_bases[10] data, as literal values
pub const MP_BASES_CHARS_PER_LIMB_10: usize = 9;
pub const MP_BASES_BIG_BASE_CTZ_10: usize = 9;
pub const MP_BASES_BIG_BASE_10: Limb = 0x3b9aca00;
pub const MP_BASES_BIG_BASE_INVERTED_10: Limb = 0x12e0be82;
pub const MP_BASES_BIG_BASE_BINVERTED_10: Limb = 0x3a2e9c6d;
pub const MP_BASES_NORMALIZATION_STEPS_10: u64 = 2;

// Format is (chars_per_limb, logb2, log2b, big_base, big_base_inverted)
pub const BASES: [(usize, Limb, Limb, Limb, Limb); 257] = [
    (0, 0, 0, 0, 0),                                      // 0
    (0, 0, 0, 0, 0),                                      // 1
    (32, 0xffffffff, 0x1fffffff, 0x1, 0x0),               // 2
    (20, 0xa1849cc1, 0x32b80347, 0xcfd41b91, 0x3b563c24), // 3
    (16, 0x7fffffff, 0x3fffffff, 0x2, 0x0),               // 4
    (13, 0x6e40d1a4, 0x4a4d3c25, 0x48c27395, 0xc25c2684), // 5
    (12, 0x6308c91b, 0x52b80347, 0x81bf1000, 0xf91bd1b6), // 6
    (11, 0x5b3064eb, 0x59d5d9fd, 0x75db9c97, 0x1607a2cb), // 7
    (10, 0x55555555, 0x5fffffff, 0x3, 0x0),               // 8
    (10, 0x50c24e60, 0x6570068e, 0xcfd41b91, 0x3b563c24), // 9
    (9, 0x4d104d42, 0x6a4d3c25, 0x3b9aca00, 0x12e0be82),  // 10
    (9, 0x4a002707, 0x6eb3a9f0, 0x8c8b6d2b, 0xd24cde04),  // 11
    (8, 0x4768ce0d, 0x72b80347, 0x19a10000, 0x3fa39ab5),  // 12
    (8, 0x452e53e3, 0x766a008e, 0x309f1021, 0x50f8ac5f),  // 13
    (8, 0x433cfffb, 0x79d5d9fd, 0x57f6c100, 0x74843b1e),  // 14
    (8, 0x41867711, 0x7d053f6d, 0x98c29b81, 0xad0326c2),  // 15
    (8, 0x3fffffff, 0x7fffffff, 0x4, 0x0),                // 16
    (7, 0x3ea16afd, 0x82cc7edf, 0x18754571, 0x4ef0b6bd),  // 17
    (7, 0x3d64598d, 0x8570068e, 0x247dbc80, 0xc0fc48a1),  // 18
    (7, 0x3c43c230, 0x87ef05ae, 0x3547667b, 0x33838942),  // 19
    (7, 0x3b3b9a42, 0x8a4d3c25, 0x4c4b4000, 0xad7f29ab),  // 20
    (7, 0x3a4898f0, 0x8c8ddd44, 0x6b5a6e1d, 0x313c3d15),  // 21
    (7, 0x39680b13, 0x8eb3a9f0, 0x94ace180, 0xb8cca9e0),  // 22
    (7, 0x3897b2b7, 0x90c10500, 0xcaf18367, 0x42ed6de9),  // 23
    (6, 0x37d5aed1, 0x92b80347, 0xb640000, 0x67980e0b),   // 24
    (6, 0x372068d2, 0x949a784b, 0xe8d4a51, 0x19799812),   // 25
    (6, 0x3676867e, 0x966a008e, 0x1269ae40, 0xbce85396),  // 26
    (6, 0x35d6deeb, 0x982809d5, 0x17179149, 0x62c103a9),  // 27
    (6, 0x354071d6, 0x99d5d9fd, 0x1cb91000, 0x1d353d43),  // 28
    (6, 0x34b260c5, 0x9b74948f, 0x23744899, 0xce1decea),  // 29
    (6, 0x342be986, 0x9d053f6d, 0x2b73a840, 0x790fc511),  // 30
    (6, 0x33ac61b9, 0x9e88c6b3, 0x34e63b41, 0x35b865a0),  // 31
    (6, 0x33333333, 0x9fffffff, 0x5, 0x0),                // 32
    (6, 0x32bfd901, 0xa16bad37, 0x4cfa3cc1, 0xa9aed1b3),  // 33
    (6, 0x3251dcf6, 0xa2cc7edf, 0x5c13d840, 0x63dfc229),  // 34
    (6, 0x31e8d59f, 0xa4231623, 0x6d91b519, 0x2b0fee30),  // 35
    (6, 0x3184648d, 0xa570068e, 0x81bf1000, 0xf91bd1b6),  // 36
    (6, 0x312434e8, 0xa6b3d78b, 0x98ede0c9, 0xac89c3a9),  // 37
    (6, 0x30c7fa34, 0xa7ef05ae, 0xb3773e40, 0x6d2c32fe),  // 38
    (6, 0x306f6f4c, 0xa92203d5, 0xd1bbc4d1, 0x387907c9),  // 39
    (6, 0x301a557f, 0xaa4d3c25, 0xf4240000, 0xc6f7a0b),   // 40
    (5, 0x2fc873d1, 0xab7110e6, 0x6e7d349, 0x28928154),   // 41
    (5, 0x2f799652, 0xac8ddd44, 0x7ca30a0, 0x6e8629d),    // 42
    (5, 0x2f2d8d8f, 0xada3f5fb, 0x8c32bbb, 0xd373dca0),   // 43
    (5, 0x2ee42e16, 0xaeb3a9f0, 0x9d46c00, 0xa0b17895),   // 44
    (5, 0x2e9d5009, 0xafbd42b4, 0xaffacfd, 0x746811a5),   // 45
    (5, 0x2e58cec0, 0xb0c10500, 0xc46bee0, 0x4da6500f),   // 46
    (5, 0x2e168874, 0xb1bf311e, 0xdab86ef, 0x2ba23582),   // 47
    (5, 0x2dd65df7, 0xb2b80347, 0xf300000, 0xdb20a88),    // 48
    (5, 0x2d983275, 0xb3abb3fa, 0x10d63af1, 0xe68d5ce4),  // 49
    (5, 0x2d5beb38, 0xb49a784b, 0x12a05f20, 0xb7cdfd9d),  // 50
    (5, 0x2d216f79, 0xb5848226, 0x1490aae3, 0x8e583933),  // 51
    (5, 0x2ce8a82e, 0xb66a008e, 0x16a97400, 0x697cc3ea),  // 52
    (5, 0x2cb17fea, 0xb74b1fd6, 0x18ed2825, 0x48a5ca6c),  // 53
    (5, 0x2c7be2b0, 0xb82809d5, 0x1b5e4d60, 0x2b52db16),  // 54
    (5, 0x2c47bddb, 0xb900e615, 0x1dff8297, 0x111586a6),  // 55
    (5, 0x2c14fffc, 0xb9d5d9fd, 0x20d38000, 0xf31d2b36),  // 56
    (5, 0x2be398c3, 0xbaa708f5, 0x23dd1799, 0xc8d76d19),  // 57
    (5, 0x2bb378e7, 0xbb74948f, 0x271f35a0, 0xa2cb1eb4),  // 58
    (5, 0x2b849210, 0xbc3e9ca2, 0x2a9ce10b, 0x807c3ec3),  // 59
    (5, 0x2b56d6c7, 0xbd053f6d, 0x2e593c00, 0x617ec8bf),  // 60
    (5, 0x2b2a3a60, 0xbdc899ab, 0x3257844d, 0x45746cbe),  // 61
    (5, 0x2afeb0f1, 0xbe88c6b3, 0x369b13e0, 0x2c0aa273),  // 62
    (5, 0x2ad42f3c, 0xbf45e08b, 0x3b27613f, 0x14f90805),  // 63
    (5, 0x2aaaaaaa, 0xbfffffff, 0x6, 0x0),                // 64
    (5, 0x2a82193a, 0xc0b73cb4, 0x4528a141, 0xd9cf0829),  // 65
    (5, 0x2a5a7176, 0xc16bad37, 0x4aa51420, 0xb6fc4841),  // 66
    (5, 0x2a33aa6e, 0xc21d6713, 0x50794633, 0x973054cb),  // 67
    (5, 0x2a0dbbaa, 0xc2cc7edf, 0x56a94400, 0x7a1dbe4b),  // 68
    (5, 0x29e89d24, 0xc3790848, 0x5d393975, 0x5f7fcd7f),  // 69
    (5, 0x29c44740, 0xc4231623, 0x642d7260, 0x47196c84),  // 70
    (5, 0x29a0b2c7, 0xc4caba78, 0x6b8a5ae7, 0x30b43635),  // 71
    (5, 0x297dd8db, 0xc570068e, 0x73548000, 0x1c1fa5f6),  // 72
    (5, 0x295bb2f9, 0xc6130af4, 0x7b908fe9, 0x930634a),   // 73
    (5, 0x293a3aeb, 0xc6b3d78b, 0x84435aa0, 0xef7f4a3c),  // 74
    (5, 0x29196acc, 0xc7527b93, 0x8d71d25b, 0xcf5552d2),  // 75
    (5, 0x28f93cfb, 0xc7ef05ae, 0x97210c00, 0xb1a47c8e),  // 76
    (5, 0x28d9ac1b, 0xc88983ed, 0xa1563f9d, 0x9634b43e),  // 77
    (5, 0x28bab310, 0xc92203d5, 0xac16c8e0, 0x7cd3817d),  // 78
    (5, 0x289c4cf8, 0xc9b89267, 0xb768278f, 0x65536761),  // 79
    (5, 0x287e7529, 0xca4d3c25, 0xc3500000, 0x4f8b588e),  // 80
    (5, 0x28612730, 0xcae00d1c, 0xcfd41b91, 0x3b563c24),  // 81
    (5, 0x28445ec9, 0xcb7110e6, 0xdcfa6920, 0x28928154),  // 82
    (5, 0x282817e1, 0xcc0052b1, 0xeac8fd83, 0x1721bfb0),  // 83
    (5, 0x280c4e90, 0xcc8ddd44, 0xf9461400, 0x6e8629d),   // 84
    (4, 0x27f0ff1b, 0xcd19bb05, 0x31c84b1, 0x491cc17c),   // 85
    (4, 0x27d625ec, 0xcda3f5fb, 0x342ab10, 0x3a11d83b),   // 86
    (4, 0x27bbbf95, 0xce2c97d6, 0x36a2c21, 0x2be074cd),   // 87
    (4, 0x27a1c8c8, 0xceb3a9f0, 0x3931000, 0x1e7a02e7),   // 88
    (4, 0x27883e5e, 0xcf393550, 0x3bd5ee1, 0x11d10edd),   // 89
    (4, 0x276f1d4c, 0xcfbd42b4, 0x3e92110, 0x5d92c68),    // 90
    (4, 0x275662a8, 0xd03fda8b, 0x4165ef1, 0xf50dbfb2),   // 91
    (4, 0x273e0ba3, 0xd0c10500, 0x4452100, 0xdf9f1316),   // 92
    (4, 0x2726158c, 0xd140c9fa, 0x4756fd1, 0xcb52a684),   // 93
    (4, 0x270e7dc9, 0xd1bf311e, 0x4a75410, 0xb8163e97),   // 94
    (4, 0x26f741dd, 0xd23c41d4, 0x4dad681, 0xa5d8f269),   // 95
    (4, 0x26e05f5f, 0xd2b80347, 0x5100000, 0x948b0fcd),   // 96
    (4, 0x26c9d3fe, 0xd3327c6a, 0x546d981, 0x841e0215),   // 97
    (4, 0x26b39d7f, 0xd3abb3fa, 0x57f6c10, 0x74843b1e),   // 98
    (4, 0x269db9bc, 0xd423b07e, 0x5b9c0d1, 0x65b11e6e),   // 99
    (4, 0x268826a1, 0xd49a784b, 0x5f5e100, 0x5798ee23),   // 100
    (4, 0x2672e22d, 0xd5101187, 0x633d5f1, 0x4a30b99b),   // 101
    (4, 0x265dea72, 0xd5848226, 0x673a910, 0x3d6e4d94),   // 102
    (4, 0x26493d93, 0xd5f7cff4, 0x6b563e1, 0x314825b0),   // 103
    (4, 0x2634d9c2, 0xd66a008e, 0x6f91000, 0x25b55f2e),   // 104
    (4, 0x2620bd41, 0xd6db196a, 0x73eb721, 0x1aadaccb),   // 105
    (4, 0x260ce662, 0xd74b1fd6, 0x7866310, 0x10294ba2),   // 106
    (4, 0x25f95385, 0xd7ba18f9, 0x7d01db1, 0x620f8f6),    // 107
    (4, 0x25e60316, 0xd82809d5, 0x81bf100, 0xf91bd1b6),   // 108
    (4, 0x25d2f390, 0xd894f74b, 0x869e711, 0xe6d37b2a),   // 109
    (4, 0x25c02379, 0xd900e615, 0x8ba0a10, 0xd55cff6e),   // 110
    (4, 0x25ad9165, 0xd96bdad2, 0x90c6441, 0xc4ad2db2),   // 111
    (4, 0x259b3bf3, 0xd9d5d9fd, 0x9610000, 0xb4b985cf),   // 112
    (4, 0x258921cb, 0xda3ee7f3, 0x9b7e7c1, 0xa5782bef),   // 113
    (4, 0x257741a2, 0xdaa708f5, 0xa112610, 0x96dfdd2a),   // 114
    (4, 0x25659a37, 0xdb0e4126, 0xa6cc591, 0x88e7e509),   // 115
    (4, 0x25542a50, 0xdb74948f, 0xacad100, 0x7b8813d3),   // 116
    (4, 0x2542f0c2, 0xdbda071c, 0xb2b5331, 0x6eb8b595),   // 117
    (4, 0x2531ec64, 0xdc3e9ca2, 0xb8e5710, 0x627289db),   // 118
    (4, 0x25211c1c, 0xdca258dc, 0xbf3e7a1, 0x56aebc07),   // 119
    (4, 0x25107ed5, 0xdd053f6d, 0xc5c1000, 0x4b66dc33),   // 120
    (4, 0x25001383, 0xdd6753e0, 0xcc6db61, 0x4094d8a3),   // 121
    (4, 0x24efd921, 0xddc899ab, 0xd345510, 0x3632f7a5),   // 122
    (4, 0x24dfceb3, 0xde29142e, 0xda48871, 0x2c3bd1f0),   // 123
    (4, 0x24cff343, 0xde88c6b3, 0xe178100, 0x22aa4d5f),   // 124
    (4, 0x24c045e1, 0xdee7b471, 0xe8d4a51, 0x19799812),   // 125
    (4, 0x24b0c5a6, 0xdf45e08b, 0xf05f010, 0x10a523e5),   // 126
    (4, 0x24a171b0, 0xdfa34e11, 0xf817e01, 0x828a237),    // 127
    (4, 0x24924924, 0xdfffffff, 0x7, 0x0),                // 128
    (4, 0x24834b2c, 0xe05bf942, 0x10818201, 0xf04ec452),  // 129
    (4, 0x247476f9, 0xe0b73cb4, 0x11061010, 0xe136444a),  // 130
    (4, 0x2465cbc0, 0xe111cd1d, 0x118db651, 0xd2af9589),  // 131
    (4, 0x245748bc, 0xe16bad37, 0x12188100, 0xc4b42a83),  // 132
    (4, 0x2448ed2f, 0xe1c4dfab, 0x12a67c71, 0xb73dccf5),  // 133
    (4, 0x243ab85d, 0xe21d6713, 0x1337b510, 0xaa4698c5),  // 134
    (4, 0x242ca992, 0xe27545fb, 0x13cc3761, 0x9dc8f729),  // 135
    (4, 0x241ec01b, 0xe2cc7edf, 0x14641000, 0x91bf9a30),  // 136
    (4, 0x2410fb4d, 0xe323142d, 0x14ff4ba1, 0x86257887),  // 137
    (4, 0x24035a80, 0xe3790848, 0x159df710, 0x7af5c98c),  // 138
    (4, 0x23f5dd10, 0xe3ce5d82, 0x16401f31, 0x702c01a0),  // 139
    (4, 0x23e8825d, 0xe4231623, 0x16e5d100, 0x65c3ceb1),  // 140
    (4, 0x23db49cc, 0xe4773465, 0x178f1991, 0x5bb91502),  // 141
    (4, 0x23ce32c4, 0xe4caba78, 0x183c0610, 0x5207ec23),  // 142
    (4, 0x23c13cb3, 0xe51daa7e, 0x18eca3c1, 0x48ac9c19),  // 143
    (4, 0x23b46706, 0xe570068e, 0x19a10000, 0x3fa39ab5),  // 144
    (4, 0x23a7b132, 0xe5c1d0b5, 0x1a592841, 0x36e98912),  // 145
    (4, 0x239b1aac, 0xe6130af4, 0x1b152a10, 0x2e7b3140),  // 146
    (4, 0x238ea2ef, 0xe663b741, 0x1bd51311, 0x2655840b),  // 147
    (4, 0x23824976, 0xe6b3d78b, 0x1c98f100, 0x1e7596ea),  // 148
    (4, 0x23760dc3, 0xe7036db3, 0x1d60d1b1, 0x16d8a20d),  // 149
    (4, 0x2369ef58, 0xe7527b93, 0x1e2cc310, 0xf7bfe87),   // 150
    (4, 0x235dedbb, 0xe7a102f9, 0x1efcd321, 0x85d2492),   // 151
    (4, 0x23520874, 0xe7ef05ae, 0x1fd11000, 0x179a9f4),   // 152
    (4, 0x23463f10, 0xe83c856d, 0x20a987e1, 0xf59e80eb),  // 153
    (4, 0x233a911b, 0xe88983ed, 0x21864910, 0xe8b768db),  // 154
    (4, 0x232efe26, 0xe8d602d9, 0x226761f1, 0xdc39d6d5),  // 155
    (4, 0x232385c6, 0xe92203d5, 0x234ce100, 0xd021c5d1),  // 156
    (4, 0x2318278e, 0xe96d887e, 0x2436d4d1, 0xc46b5e37),  // 157
    (4, 0x230ce318, 0xe9b89267, 0x25254c10, 0xb912f39c),  // 158
    (4, 0x2301b7fd, 0xea03231d, 0x26185581, 0xae150294),  // 159
    (4, 0x22f6a5d9, 0xea4d3c25, 0x27100000, 0xa36e2eb1),  // 160
    (4, 0x22ebac4c, 0xea96defe, 0x280c5a81, 0x991b4094),  // 161
    (4, 0x22e0caf6, 0xeae00d1c, 0x290d7410, 0x8f19241e),  // 162
    (4, 0x22d60179, 0xeb28c7f2, 0x2a135bd1, 0x8564e6b7),  // 163
    (4, 0x22cb4f7a, 0xeb7110e6, 0x2b1e2100, 0x7bfbb5b4),  // 164
    (4, 0x22c0b4a1, 0xebb8e95d, 0x2c2dd2f1, 0x72dadcc8),  // 165
    (4, 0x22b63095, 0xec0052b1, 0x2d428110, 0x69ffc498),  // 166
    (4, 0x22abc300, 0xec474e39, 0x2e5c3ae1, 0x6167f154),  // 167
    (4, 0x22a16b90, 0xec8ddd44, 0x2f7b1000, 0x5911016e),  // 168
    (4, 0x229729f1, 0xecd4011c, 0x309f1021, 0x50f8ac5f),  // 169
    (4, 0x228cfdd4, 0xed19bb05, 0x31c84b10, 0x491cc17c),  // 170
    (4, 0x2282e6e9, 0xed5f0c3c, 0x32f6d0b1, 0x417b26d8),  // 171
    (4, 0x2278e4e3, 0xeda3f5fb, 0x342ab100, 0x3a11d83b),  // 172
    (4, 0x226ef777, 0xede87974, 0x3563fc11, 0x32dee622),  // 173
    (4, 0x22651e5a, 0xee2c97d6, 0x36a2c210, 0x2be074cd),  // 174
    (4, 0x225b5944, 0xee705249, 0x37e71341, 0x2514bb58),  // 175
    (4, 0x2251a7ee, 0xeeb3a9f0, 0x39310000, 0x1e7a02e7),  // 176
    (4, 0x22480a11, 0xeef69fea, 0x3a8098c1, 0x180ea5d0),  // 177
    (4, 0x223e7f69, 0xef393550, 0x3bd5ee10, 0x11d10edd),  // 178
    (4, 0x223507b4, 0xef7b6b39, 0x3d311091, 0xbbfb88e),   // 179
    (4, 0x222ba2af, 0xefbd42b4, 0x3e921100, 0x5d92c68),   // 180
    (4, 0x22225019, 0xeffebccd, 0x3ff90031, 0x1c024c),    // 181
    (4, 0x22190fb4, 0xf03fda8b, 0x4165ef10, 0xf50dbfb2),  // 182
    (4, 0x220fe141, 0xf0809cf2, 0x42d8eea1, 0xea30efa3),  // 183
    (4, 0x2206c483, 0xf0c10500, 0x44521000, 0xdf9f1316),  // 184
    (4, 0x21fdb93f, 0xf10113b1, 0x45d16461, 0xd555c0c9),  // 185
    (4, 0x21f4bf3a, 0xf140c9fa, 0x4756fd10, 0xcb52a684),  // 186
    (4, 0x21ebd639, 0xf18028cf, 0x48e2eb71, 0xc193881f),  // 187
    (4, 0x21e2fe06, 0xf1bf311e, 0x4a754100, 0xb8163e97),  // 188
    (4, 0x21da3667, 0xf1fde3d3, 0x4c0e0f51, 0xaed8b724),  // 189
    (4, 0x21d17f28, 0xf23c41d4, 0x4dad6810, 0xa5d8f269),  // 190
    (4, 0x21c8d811, 0xf27a4c05, 0x4f535d01, 0x9d15039d),  // 191
    (4, 0x21c040ef, 0xf2b80347, 0x51000000, 0x948b0fcd),  // 192
    (4, 0x21b7b98f, 0xf2f56875, 0x52b36301, 0x8c394d1d),  // 193
    (4, 0x21af41bc, 0xf3327c6a, 0x546d9810, 0x841e0215),  // 194
    (4, 0x21a6d947, 0xf36f3ffb, 0x562eb151, 0x7c3784f8),  // 195
    (4, 0x219e7ffd, 0xf3abb3fa, 0x57f6c100, 0x74843b1e),  // 196
    (4, 0x219635af, 0xf3e7d937, 0x59c5d971, 0x6d02985d),  // 197
    (4, 0x218dfa2e, 0xf423b07e, 0x5b9c0d10, 0x65b11e6e),  // 198
    (4, 0x2185cd4c, 0xf45f3a98, 0x5d796e61, 0x5e8e5c64),  // 199
    (4, 0x217daeda, 0xf49a784b, 0x5f5e1000, 0x5798ee23),  // 200
    (4, 0x21759eac, 0xf4d56a5b, 0x614a04a1, 0x50cf7bde),  // 201
    (4, 0x216d9c96, 0xf5101187, 0x633d5f10, 0x4a30b99b),  // 202
    (4, 0x2165a86e, 0xf54a6e8c, 0x65383231, 0x43bb66bd),  // 203
    (4, 0x215dc207, 0xf5848226, 0x673a9100, 0x3d6e4d94),  // 204
    (4, 0x2155e939, 0xf5be4d0c, 0x69448e91, 0x374842ee),  // 205
    (4, 0x214e1ddb, 0xf5f7cff4, 0x6b563e10, 0x314825b0),  // 206
    (4, 0x21465fc4, 0xf6310b8f, 0x6d6fb2c1, 0x2b6cde75),  // 207
    (4, 0x213eaecd, 0xf66a008e, 0x6f910000, 0x25b55f2e),  // 208
    (4, 0x21370ace, 0xf6a2af9e, 0x71ba3941, 0x2020a2c5),  // 209
    (4, 0x212f73a0, 0xf6db196a, 0x73eb7210, 0x1aadaccb),  // 210
    (4, 0x2127e920, 0xf7133e9b, 0x7624be11, 0x155b891f),  // 211
    (4, 0x21206b26, 0xf74b1fd6, 0x78663100, 0x10294ba2),  // 212
    (4, 0x2118f98f, 0xf782bdbf, 0x7aafdeb1, 0xb160fe9),   // 213
    (4, 0x21119436, 0xf7ba18f9, 0x7d01db10, 0x620f8f6),   // 214
    (4, 0x210a3af8, 0xf7f13221, 0x7f5c3a21, 0x14930ef),   // 215
    (4, 0x2102edb3, 0xf82809d5, 0x81bf1000, 0xf91bd1b6),  // 216
    (4, 0x20fbac44, 0xf85ea0b0, 0x842a70e1, 0xefdcb0c7),  // 217
    (4, 0x20f4768a, 0xf894f74b, 0x869e7110, 0xe6d37b2a),  // 218
    (4, 0x20ed4c62, 0xf8cb0e3b, 0x891b24f1, 0xddfeb94a),  // 219
    (4, 0x20e62dae, 0xf900e615, 0x8ba0a100, 0xd55cff6e),  // 220
    (4, 0x20df1a4b, 0xf9367f6d, 0x8e2ef9d1, 0xcceced50),  // 221
    (4, 0x20d8121c, 0xf96bdad2, 0x90c64410, 0xc4ad2db2),  // 222
    (4, 0x20d11500, 0xf9a0f8d3, 0x93669481, 0xbc9c75f9),  // 223
    (4, 0x20ca22d9, 0xf9d5d9fd, 0x96100000, 0xb4b985cf),  // 224
    (4, 0x20c33b88, 0xfa0a7eda, 0x98c29b81, 0xad0326c2),  // 225
    (4, 0x20bc5ef1, 0xfa3ee7f3, 0x9b7e7c10, 0xa5782bef),  // 226
    (4, 0x20b58cf5, 0xfa7315d0, 0x9e43b6d1, 0x9e1771a9),  // 227
    (4, 0x20aec579, 0xfaa708f5, 0xa1126100, 0x96dfdd2a),  // 228
    (4, 0x20a8085e, 0xfadac1e7, 0xa3ea8ff1, 0x8fd05c41),  // 229
    (4, 0x20a1558b, 0xfb0e4126, 0xa6cc5910, 0x88e7e509),  // 230
    (4, 0x209aace2, 0xfb418734, 0xa9b7d1e1, 0x8225759d),  // 231
    (4, 0x20940e49, 0xfb74948f, 0xacad1000, 0x7b8813d3),  // 232
    (4, 0x208d79a5, 0xfba769b3, 0xafac2921, 0x750eccf9),  // 233
    (4, 0x2086eedb, 0xfbda071c, 0xb2b53310, 0x6eb8b595),  // 234
    (4, 0x20806dd2, 0xfc0c6d44, 0xb5c843b1, 0x6884e923),  // 235
    (4, 0x2079f671, 0xfc3e9ca2, 0xb8e57100, 0x627289db),  // 236
    (4, 0x2073889d, 0xfc7095ae, 0xbc0cd111, 0x5c80c07b),  // 237
    (4, 0x206d243e, 0xfca258dc, 0xbf3e7a10, 0x56aebc07),  // 238
    (4, 0x2066c93c, 0xfcd3e6a0, 0xc27a8241, 0x50fbb19b),  // 239
    (4, 0x2060777e, 0xfd053f6d, 0xc5c10000, 0x4b66dc33),  // 240
    (4, 0x205a2eed, 0xfd3663b2, 0xc91209c1, 0x45ef7c7c),  // 241
    (4, 0x2053ef71, 0xfd6753e0, 0xcc6db610, 0x4094d8a3),  // 242
    (4, 0x204db8f3, 0xfd981064, 0xcfd41b91, 0x3b563c24),  // 243
    (4, 0x20478b5c, 0xfdc899ab, 0xd3455100, 0x3632f7a5),  // 244
    (4, 0x20416696, 0xfdf8f020, 0xd6c16d31, 0x312a60c3),  // 245
    (4, 0x203b4a8b, 0xfe29142e, 0xda488710, 0x2c3bd1f0),  // 246
    (4, 0x20353725, 0xfe59063c, 0xdddab5a1, 0x2766aa45),  // 247
    (4, 0x202f2c4e, 0xfe88c6b3, 0xe1781000, 0x22aa4d5f),  // 248
    (4, 0x202929f0, 0xfeb855f8, 0xe520ad61, 0x1e06233c),  // 249
    (4, 0x20232ff8, 0xfee7b471, 0xe8d4a510, 0x19799812),  // 250
    (4, 0x201d3e50, 0xff16e281, 0xec940e71, 0x15041c33),  // 251
    (4, 0x201754e5, 0xff45e08b, 0xf05f0100, 0x10a523e5),  // 252
    (4, 0x201173a1, 0xff74aef0, 0xf4359451, 0xc5c2749),   // 253
    (4, 0x200b9a71, 0xffa34e11, 0xf817e010, 0x828a237),   // 254
    (4, 0x2005c942, 0xffd1be4c, 0xfc05fc01, 0x40a1423),   // 255
    (4, 0x1fffffff, 0xffffffff, 0x8, 0x0),                // 256
];

// This section is created by factorial_data.rs.

// This is equivalent to `__gmp_oddfac_table` in `mpn/comb_tables.c`, GMP 6.2.1, which is the
// combination of `ONE_LIMB_ODD_FACTORIAL_TABLE` and `ONE_LIMB_ODD_FACTORIAL_EXTTABLE` in
// `fac_table.h`, GMP 6.2.1.
pub const ONE_LIMB_ODD_FACTORIAL_TABLE: [Limb; 35] = [
    0x1, 0x1, 0x1, 0x3, 0x3, 0xf, 0x2d, 0x13b, 0x13b, 0xb13, 0x375f, 0x26115, 0x7233f, 0x5cca33,
    0x2898765, 0x260eeeeb, 0x260eeeeb, 0x86fddd9b, 0xbeecca73, 0x2b930689, 0xd9df20ad, 0xdf4dae31,
    0x98567c1b, 0xafc5266d, 0xf4f7347, 0x7ec241ef, 0x6fdd5923, 0xcc5866b1, 0x966aced7, 0xa196e5b,
    0x977d7755, 0x5831734b, 0x5831734b, 0x5e5fdcab, 0x445da75b,
];
// This is equivalent to `ODD_FACTORIAL_TABLE_LIMIT` in `fac_table.h`, GMP 6.2.1.
pub const ODD_FACTORIAL_TABLE_LIMIT: usize = 16;
// This is equivalent to `ODD_FACTORIAL_EXTTABLE_LIMIT` in `fac_table.h`, GMP 6.2.1.
pub const ODD_FACTORIAL_EXTTABLE_LIMIT: usize = 34;
// This is equivalent to `ODD_FACTORIAL_TABLE_MAX` in `fac_table.h`, GMP 6.2.1.
pub const ODD_FACTORIAL_TABLE_MAX: Limb = 0x260eeeeb;

// This is equivalent to `__gmp_odd2fac_table` in `mpn/comb_tables.c`, GMP 6.2.1, and
// `ONE_LIMB_ODD_DOUBLEFACTORIAL_TABLE` from `fac_table.h`, GMP 6.2.1.
pub const ONE_LIMB_ODD_DOUBLEFACTORIAL_TABLE: [Limb; 10] =
    [0x1, 0x3, 0xf, 0x69, 0x3b1, 0x289b, 0x20fdf, 0x1eee11, 0x20dcf21, 0x27065f73];
// This is equivalent to `ODD_DOUBLEFACTORIAL_TABLE_LIMIT` in `fac_table.h`, GMP 6.2.1.
pub const ODD_DOUBLEFACTORIAL_TABLE_LIMIT: usize = 19;
// This is equivalent to `ODD_DOUBLEFACTORIAL_TABLE_MAX` in `fac_table.h`, GMP 6.2.1.
pub const ODD_DOUBLEFACTORIAL_TABLE_MAX: Limb = 0x27065f73;

// This is equivalent to `__gmp_limbroots_table` in `mpn/comb_tables.c`, GMP 6.2.1, and
// `NTH_ROOT_NUMB_MASK_TABLE` from `fac_table.h`, GMP 6.2.1.
pub const NTH_ROOT_NUMB_MASK_TABLE: [Limb; 8] =
    [Limb::MAX, 0xffff, 0x659, 0xff, 0x54, 0x28, 0x17, 0xf];

// This is equivalent to `ONE_LIMB_ODD_FACTORIAL_INVERSES_TABLE` from `fac_table.h`, GMP 6.2.1.
pub const ONE_LIMB_ODD_FACTORIAL_INVERSES_TABLE: [Limb; 31] = [
    0x1, 0xaaaaaaab, 0xaaaaaaab, 0xeeeeeeef, 0xa4fa4fa5, 0xf2ff2ff3, 0xf2ff2ff3, 0x53e3771b,
    0xdd93e49f, 0xfcdee63d, 0x544a4cbf, 0x7ca340fb, 0xa417526d, 0xd7bd49c3, 0xd7bd49c3, 0x85294093,
    0xf259eabb, 0xd6dc4fb9, 0x915f4325, 0x131cead1, 0xea76fe13, 0x633cd365, 0x21144677, 0x200b0d0f,
    0x8c4f9e8b, 0x21a42251, 0xe03c04e7, 0x600211d3, 0x4aaacdfd, 0x33f4fe63, 0x33f4fe63,
];

pub const ODD_CENTRAL_BINOMIAL_OFFSET: usize = 8;

// This table contains binomial(2k, k) / 2 ^ t.
//
// This is equivalent to `bin2kk` in `mpz/bin_uiui.c`, GMP 6.2.1, and
// `ONE_LIMB_ODD_CENTRAL_BINOMIAL_TABLE` from `fac_table.h`, GMP 6.2.1.
pub const ONE_LIMB_ODD_CENTRAL_BINOMIAL_TABLE: [Limb; 11] = [
    0x1923, 0x2f7b, 0xb46d, 0x15873, 0xa50c7, 0x13d66b, 0x4c842f, 0x93ee7d, 0x11e9e123, 0x22c60053,
    0x873ae4d1,
];

pub const ODD_CENTRAL_BINOMIAL_TABLE_LIMIT: usize = 18;

// This table contains the inverses of elements in the previous table.
//
// This is equivalent to `bin2kkinv` in `mpz/bin_uiui.c`, GMP 6.2.1, and
// `ONE_LIMB_ODD_CENTRAL_BINOMIAL_INVERSE_TABLE` from `fac_table.h`, GMP 6.2.1.
pub const ONE_LIMB_ODD_CENTRAL_BINOMIAL_INVERSE_TABLE: [Limb; 11] = [
    0x16a2de8b, 0x847457b3, 0xfa6f7565, 0xf0e50cbb, 0xdca370f7, 0x9bb12643, 0xdc8342cf, 0x4ebf7ad5,
    0x86ab568b, 0x265843db, 0x8633f431,
];

// This table contains the values t in the formula binomial(2k, k) / 2 ^ t.
//
// This is equivalent to `fac2bin` in `mpz/bin_uiui.c`, GMP 6.2.1, and `CENTRAL_BINOMIAL_2FAC_TABLE`
// from `fac_table.h`, GMP 6.2.1.
pub const CENTRAL_BINOMIAL_2FAC_TABLE: [u64; 11] = [1, 2, 2, 3, 2, 3, 3, 4, 1, 2, 2];

// https://oeis.org/A005187, skipping the initial 0
//
// This is equivalent to `__gmp_fac2cnt_table` in `mpn/comb_tables.c`, GMP 6.2.1, and
// `TABLE_2N_MINUS_POPC_2N` from `fac_table.h`, GMP 6.2.1.
pub const TABLE_2N_MINUS_POPC_2N: [u8; 24] =
    [1, 3, 4, 7, 8, 10, 11, 15, 16, 18, 19, 22, 23, 25, 26, 31, 32, 34, 35, 38, 39, 41, 42, 46];

pub const TABLE_LIMIT_2N_MINUS_POPC_2N: u64 = 49;

// end of auto-generated code

pub const FFT_TAB: [[u8; 2]; 5] = [[3, 3], [3, 2], [2, 1], [2, 1], [0, 0]];

pub const MULMOD_TAB: [u8; 15] = [4, 3, 3, 3, 3, 2, 2, 2, 3, 2, 2, 2, 2, 1, 1];
