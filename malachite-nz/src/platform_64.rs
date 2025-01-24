// Copyright © 2025 Mikhail Hogrefe
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

pub type Limb = u64;
pub type HalfLimb = u32;
pub type DoubleLimb = u128;
pub type SignedLimb = i64;
pub type SignedHalfLimb = i32;
pub type SignedDoubleLimb = i128;
pub type FloatWithLimbWidth = f64;

pub const MAX_DIGITS_PER_LIMB: usize = 20;

pub const AORSMUL_FASTER_2AORSLSH: bool = true;
pub const AORSMUL_FASTER_3AORSLSH: bool = true;
pub const AORSMUL_FASTER_AORS_AORSLSH: bool = true;
pub const AORSMUL_FASTER_AORS_2AORSLSH: bool = true;

pub const MUL_TOOM22_THRESHOLD: usize = 20;
pub const MUL_TOOM33_THRESHOLD: usize = 39;
pub const MUL_TOOM44_THRESHOLD: usize = 340; // unclear when 44 is better than 33
pub const MUL_TOOM6H_THRESHOLD: usize = 345;
pub const MUL_TOOM8H_THRESHOLD: usize = 640;

pub const MUL_TOOM32_TO_TOOM43_THRESHOLD: usize = 60;
pub const MUL_TOOM32_TO_TOOM53_THRESHOLD: usize = 300;
pub const MUL_TOOM42_TO_TOOM53_THRESHOLD: usize = 600;
pub const MUL_TOOM42_TO_TOOM63_THRESHOLD: usize = 103;

pub const MUL_FFT_THRESHOLD: usize = 1500;

pub const DC_DIV_QR_THRESHOLD: usize = 85;
pub const DC_DIVAPPR_Q_THRESHOLD: usize = 211;
pub const MAYBE_DCP1_DIVAPPR: bool = true;
pub const INV_NEWTON_THRESHOLD: usize = 789;
pub const MU_DIV_QR_THRESHOLD: usize = 2094;
pub const INV_MULMOD_BNM1_THRESHOLD: usize = 62;
pub const MU_DIV_QR_SKEW_THRESHOLD: usize = 231;

pub const MU_DIVAPPR_Q_THRESHOLD: usize = 2965;
pub const FUDGE: usize = 311;

pub const MULLO_BASECASE_THRESHOLD: usize = 1;
pub const MULLO_DC_THRESHOLD: usize = 56;
pub const MULLO_MUL_N_THRESHOLD: usize = 10806;

pub const BINV_NEWTON_THRESHOLD: usize = 2211;
pub const DC_BDIV_QR_THRESHOLD: usize = 211;
pub const MU_BDIV_QR_THRESHOLD: usize = 7547;
pub const DC_BDIV_Q_THRESHOLD: usize = 211;
pub const MU_BDIV_Q_THRESHOLD: usize = 1998;

pub const MOD_1_NORM_THRESHOLD: usize = 0;
pub const MOD_1_UNNORM_THRESHOLD: usize = 0;
pub const MOD_1_1P_METHOD: bool = false;
pub const MOD_1N_TO_MOD_1_1_THRESHOLD: usize = 3;
pub const MOD_1U_TO_MOD_1_1_THRESHOLD: usize = 0;
pub const MOD_1_1_TO_MOD_1_2_THRESHOLD: usize = 6;
pub const MOD_1_2_TO_MOD_1_4_THRESHOLD: usize = 26;

pub const BMOD_1_TO_MOD_1_THRESHOLD: usize = 100000000;

pub const SQR_BASECASE_THRESHOLD: usize = 0;
pub const SQR_TOOM2_THRESHOLD: usize = 43;
pub const SQR_TOOM3_THRESHOLD: usize = 390;
pub const SQR_TOOM4_THRESHOLD: usize = 1090;
pub const SQR_TOOM6_THRESHOLD: usize = 336;
pub const SQR_TOOM8_THRESHOLD: usize = 837;

pub const SQRLO_DC_THRESHOLD: usize = 389;

pub const FROM_DIGITS_DIVIDE_AND_CONQUER_THRESHOLD: u64 = 6500;

pub const MATRIX22_STRASSEN_THRESHOLD: usize = 30;

// This section is created by digits_data.rs.

// mp_bases[10] data, as literal values
pub const MP_BASES_CHARS_PER_LIMB_10: usize = 19;
pub const MP_BASES_BIG_BASE_CTZ_10: usize = 19;
pub const MP_BASES_BIG_BASE_10: Limb = 0x8ac7230489e80000;
pub const MP_BASES_BIG_BASE_INVERTED_10: Limb = 0xd83c94fb6d2ac34a;
pub const MP_BASES_BIG_BASE_BINVERTED_10: Limb = 0x26b172506559ce15;
pub const MP_BASES_NORMALIZATION_STEPS_10: u64 = 0;

// Format is (chars_per_limb, logb2, log2b, big_base, big_base_inverted)
pub const BASES: [(usize, Limb, Limb, Limb, Limb); 257] = [
    (0, 0, 0, 0, 0),                                        // 0
    (0, 0, 0, 0, 0),                                        // 1
    (64, 0xffffffffffffffff, 0x1fffffffffffffff, 0x1, 0x0), // 2
    (
        40,
        0xa1849cc1a9a9e94e,
        0x32b803473f7ad0f3,
        0xa8b8b452291fe821,
        0x846d550e37b5063d,
    ), // 3
    (32, 0x7fffffffffffffff, 0x3fffffffffffffff, 0x2, 0x0), // 4
    (
        27,
        0x6e40d1a4143dcb94,
        0x4a4d3c25e68dc57f,
        0x6765c793fa10079d,
        0x3ce9a36f23c0fc90,
    ), // 5
    (
        24,
        0x6308c91b702a7cf4,
        0x52b803473f7ad0f3,
        0x41c21cb8e1000000,
        0xf24f62335024a295,
    ), // 6
    (
        22,
        0x5b3064eb3aa6d388,
        0x59d5d9fd5010b366,
        0x3642798750226111,
        0x2df495ccaa57147b,
    ), // 7
    (21, 0x5555555555555555, 0x5fffffffffffffff, 0x3, 0x0), // 8
    (
        20,
        0x50c24e60d4d4f4a7,
        0x6570068e7ef5a1e7,
        0xa8b8b452291fe821,
        0x846d550e37b5063d,
    ), // 9
    (
        19,
        0x4d104d427de7fbcc,
        0x6a4d3c25e68dc57f,
        0x8ac7230489e80000,
        0xd83c94fb6d2ac34a,
    ), // 10
    (
        18,
        0x4a00270775914e88,
        0x6eb3a9f01975077f,
        0x4d28cb56c33fa539,
        0xa8adf7ae45e7577b,
    ), // 11
    (
        17,
        0x4768ce0d05818e12,
        0x72b803473f7ad0f3,
        0x1eca170c00000000,
        0xa10c2bec5da8f8f,
    ), // 12
    (
        17,
        0x452e53e365907bda,
        0x766a008e4788cbcd,
        0x780c7372621bd74d,
        0x10f4becafe412ec3,
    ), // 13
    (
        16,
        0x433cfffb4b5aae55,
        0x79d5d9fd5010b366,
        0x1e39a5057d810000,
        0xf08480f672b4e86,
    ), // 14
    (
        16,
        0x41867711b4f85355,
        0x7d053f6d26089673,
        0x5b27ac993df97701,
        0x6779c7f90dc42f48,
    ), // 15
    (16, 0x3fffffffffffffff, 0x7fffffffffffffff, 0x4, 0x0), // 16
    (
        15,
        0x3ea16afd58b10966,
        0x82cc7edf592262cf,
        0x27b95e997e21d9f1,
        0x9c71e11bab279323,
    ), // 17
    (
        15,
        0x3d64598d154dc4de,
        0x8570068e7ef5a1e7,
        0x5da0e1e53c5c8000,
        0x5dfaa697ec6f6a1c,
    ), // 18
    (
        15,
        0x3c43c23018bb5563,
        0x87ef05ae409a0288,
        0xd2ae3299c1c4aedb,
        0x3711783f6be7e9ec,
    ), // 19
    (
        14,
        0x3b3b9a42873069c7,
        0x8a4d3c25e68dc57f,
        0x16bcc41e90000000,
        0x6849b86a12b9b01e,
    ), // 20
    (
        14,
        0x3a4898f06cf41ac9,
        0x8c8ddd448f8b845a,
        0x2d04b7fdd9c0ef49,
        0x6bf097ba5ca5e239,
    ), // 21
    (
        14,
        0x39680b13582e7c18,
        0x8eb3a9f01975077f,
        0x5658597bcaa24000,
        0x7b8015c8d7af8f08,
    ), // 22
    (
        14,
        0x3897b2b751ae561a,
        0x90c10500d63aa658,
        0xa0e2073737609371,
        0x975a24b3a3151b38,
    ), // 23
    (
        13,
        0x37d5aed131f19c98,
        0x92b803473f7ad0f3,
        0xc29e98000000000,
        0x50bd367972689db1,
    ), // 24
    (
        13,
        0x372068d20a1ee5ca,
        0x949a784bcd1b8afe,
        0x14adf4b7320334b9,
        0x8c240c4aecb13bb5,
    ), // 25
    (
        13,
        0x3676867e5d60de29,
        0x966a008e4788cbcd,
        0x226ed36478bfa000,
        0xdbd2e56854e118c9,
    ), // 26
    (
        13,
        0x35d6deeb388df86f,
        0x982809d5be7072db,
        0x383d9170b85ff80b,
        0x2351ffcaa9c7c4ae,
    ), // 27
    (
        13,
        0x354071d61c77fa2e,
        0x99d5d9fd5010b366,
        0x5a3c23e39c000000,
        0x6b24188ca33b0636,
    ), // 28
    (
        13,
        0x34b260c5671b18ac,
        0x9b74948f5532da4b,
        0x8e65137388122bcd,
        0xcc3dceaf2b8ba99d,
    ), // 29
    (
        13,
        0x342be986572b45cc,
        0x9d053f6d26089673,
        0xdd41bb36d259e000,
        0x2832e835c6c7d6b6,
    ), // 30
    (
        12,
        0x33ac61b998fbbdf2,
        0x9e88c6b3626a72aa,
        0xaee5720ee830681,
        0x76b6aa272e1873c5,
    ), // 31
    (12, 0x3333333333333333, 0x9fffffffffffffff, 0x5, 0x0), // 32
    (
        12,
        0x32bfd90114c12861,
        0xa16bad3758efd873,
        0x172588ad4f5f0981,
        0x61eaf5d402c7bf4f,
    ), // 33
    (
        12,
        0x3251dcf6169e45f2,
        0xa2cc7edf592262cf,
        0x211e44f7d02c1000,
        0xeeb658123ffb27ec,
    ), // 34
    (
        12,
        0x31e8d59f180dc630,
        0xa4231623369e78e5,
        0x2ee56725f06e5c71,
        0x5d5e3762e6fdf509,
    ), // 35
    (
        12,
        0x3184648db8153e7a,
        0xa570068e7ef5a1e7,
        0x41c21cb8e1000000,
        0xf24f62335024a295,
    ), // 36
    (
        12,
        0x312434e89c35dacd,
        0xa6b3d78b6d3b24fb,
        0x5b5b57f8a98a5dd1,
        0x66ae7831762efb6f,
    ), // 37
    (
        12,
        0x30c7fa349460a541,
        0xa7ef05ae409a0288,
        0x7dcff8986ea31000,
        0x47388865a00f544,
    ), // 38
    (
        12,
        0x306f6f4c8432bc6d,
        0xa92203d587039cc1,
        0xabd4211662a6b2a1,
        0x7d673c33a123b54c,
    ), // 39
    (
        12,
        0x301a557ffbfdd252,
        0xaa4d3c25e68dc57f,
        0xe8d4a51000000000,
        0x19799812dea11197,
    ), // 40
    (
        11,
        0x2fc873d1fda55f3b,
        0xab7110e6ce866f2b,
        0x7a32956ad081b79,
        0xc27e62e0686feae,
    ), // 41
    (
        11,
        0x2f799652a4e6dc49,
        0xac8ddd448f8b845a,
        0x9f49aaff0e86800,
        0x9b6e7507064ce7c7,
    ), // 42
    (
        11,
        0x2f2d8d8f64460aad,
        0xada3f5fb9c415052,
        0xce583bb812d37b3,
        0x3d9ac2bf66cfed94,
    ), // 43
    (
        11,
        0x2ee42e164e8f53a4,
        0xaeb3a9f01975077f,
        0x109b79a654c00000,
        0xed46bc50ce59712a,
    ), // 44
    (
        11,
        0x2e9d500984041dbd,
        0xafbd42b465836767,
        0x1543beff214c8b95,
        0x813d97e2c89b8d46,
    ), // 45
    (
        11,
        0x2e58cec05a6a8144,
        0xb0c10500d63aa658,
        0x1b149a79459a3800,
        0x2e81751956af8083,
    ), // 46
    (
        11,
        0x2e1688743ef9104c,
        0xb1bf311e95d00de3,
        0x224edfb5434a830f,
        0xdd8e0a95e30c0988,
    ), // 47
    (
        11,
        0x2dd65df7a583598f,
        0xb2b803473f7ad0f3,
        0x2b3fb00000000000,
        0x7ad4dd48a0b5b167,
    ), // 48
    (
        11,
        0x2d9832759d5369c4,
        0xb3abb3faa02166cc,
        0x3642798750226111,
        0x2df495ccaa57147b,
    ), // 49
    (
        11,
        0x2d5beb38dcd1394c,
        0xb49a784bcd1b8afe,
        0x43c33c1937564800,
        0xe392010175ee5962,
    ), // 50
    (
        11,
        0x2d216f7943e2ba6a,
        0xb5848226989d33c3,
        0x54411b2441c3cd8b,
        0x84eaf11b2fe7738e,
    ), // 51
    (
        11,
        0x2ce8a82efbb3ff2c,
        0xb66a008e4788cbcd,
        0x6851455acd400000,
        0x3a1e3971e008995d,
    ), // 52
    (
        11,
        0x2cb17fea7ad7e332,
        0xb74b1fd64e0753c6,
        0x80a23b117c8feb6d,
        0xfd7a462344ffce25,
    ), // 53
    (
        11,
        0x2c7be2b0cfa1ba50,
        0xb82809d5be7072db,
        0x9dff7d32d5dc1800,
        0x9eca40b40ebcef8a,
    ), // 54
    (
        11,
        0x2c47bddba92d7463,
        0xb900e6160002ccfe,
        0xc155af6faeffe6a7,
        0x52fa161a4a48e43d,
    ), // 55
    (
        11,
        0x2c14fffcaa8b131e,
        0xb9d5d9fd5010b366,
        0xebb7392e00000000,
        0x1607a2cbacf930c1,
    ), // 56
    (
        10,
        0x2be398c3a38be053,
        0xbaa708f58014d37c,
        0x50633659656d971,
        0x97a014f8e3be55f1,
    ), // 57
    (
        10,
        0x2bb378e758451068,
        0xbb74948f5532da4b,
        0x5fa8624c7fba400,
        0x568df8b76cbf212c,
    ), // 58
    (
        10,
        0x2b8492108be5e5f7,
        0xbc3e9ca2e1a05533,
        0x717d9faa73c5679,
        0x20ba7c4b4e6ef492,
    ), // 59
    (
        10,
        0x2b56d6c70d55481b,
        0xbd053f6d26089673,
        0x86430aac6100000,
        0xe81ee46b9ef492f5,
    ), // 60
    (
        10,
        0x2b2a3a608c72ddd5,
        0xbdc899ab3ff56c5e,
        0x9e64d9944b57f29,
        0x9dc0d10d51940416,
    ), // 61
    (
        10,
        0x2afeb0f1060c7e41,
        0xbe88c6b3626a72aa,
        0xba5ca5392cb0400,
        0x5fa8ed2f450272a5,
    ), // 62
    (
        10,
        0x2ad42f3c9aca595c,
        0xbf45e08bcf06554e,
        0xdab2ce1d022cd81,
        0x2ba9eb8c5e04e641,
    ), // 63
    (10, 0x2aaaaaaaaaaaaaaa, 0xbfffffffffffffff, 0x6, 0x0), // 64
    (
        10,
        0x2a82193a13425883,
        0xc0b73cb42e16914c,
        0x12aeed5fd3e2d281,
        0xb67759cc00287bf1,
    ), // 65
    (
        10,
        0x2a5a717672f66450,
        0xc16bad3758efd873,
        0x15c3da1572d50400,
        0x78621feeb7f4ed33,
    ), // 66
    (
        10,
        0x2a33aa6e56d9c71c,
        0xc21d6713f453f356,
        0x194c05534f75ee29,
        0x43d55b5f72943bc0,
    ), // 67
    (
        10,
        0x2a0dbbaa3bdfcea4,
        0xc2cc7edf592262cf,
        0x1d56299ada100000,
        0x173decb64d1d4409,
    ), // 68
    (
        10,
        0x29e89d244eb4bfaf,
        0xc379084815b5774c,
        0x21f2a089a4ff4f79,
        0xe29fb54fd6b6074f,
    ), // 69
    (
        10,
        0x29c44740d7db51e6,
        0xc4231623369e78e5,
        0x2733896c68d9a400,
        0xa1f1f5c210d54e62,
    ), // 70
    (
        10,
        0x29a0b2c743b14d74,
        0xc4caba789e2b8687,
        0x2d2cf2c33b533c71,
        0x6aac7f9bfafd57b2,
    ), // 71
    (
        10,
        0x297dd8dbb7c22a2d,
        0xc570068e7ef5a1e7,
        0x33f506e440000000,
        0x3b563c2478b72ee2,
    ), // 72
    (
        10,
        0x295bb2f9285c8c1b,
        0xc6130af40bc0ecbf,
        0x3ba43bec1d062211,
        0x12b536b574e92d1b,
    ), // 73
    (
        10,
        0x293a3aebe2be1c92,
        0xc6b3d78b6d3b24fb,
        0x4455872d8fd4e400,
        0xdf86c03020404fa5,
    ), // 74
    (
        10,
        0x29196acc815ebd9f,
        0xc7527b930c965bf2,
        0x4e2694539f2f6c59,
        0xa34adf02234eea8e,
    ), // 75
    (
        10,
        0x28f93cfb40f5c22a,
        0xc7ef05ae409a0288,
        0x5938006c18900000,
        0x6f46eb8574eb59dd,
    ), // 76
    (
        10,
        0x28d9ac1badc64117,
        0xc88983ed6985bae5,
        0x65ad9912474aa649,
        0x42459b481df47cec,
    ), // 77
    (
        10,
        0x28bab310a196b478,
        0xc92203d587039cc1,
        0x73ae9ff4241ec400,
        0x1b424b95d80ca505,
    ), // 78
    (
        10,
        0x289c4cf88b774469,
        0xc9b892675266f66c,
        0x836612ee9c4ce1e1,
        0xf2c1b982203a0dac,
    ), // 79
    (
        10,
        0x287e7529fb244e91,
        0xca4d3c25e68dc57f,
        0x9502f90000000000,
        0xb7cdfd9d7bdbab7d,
    ), // 80
    (
        10,
        0x286127306a6a7a53,
        0xcae00d1cfdeb43cf,
        0xa8b8b452291fe821,
        0x846d550e37b5063d,
    ), // 81
    (
        10,
        0x28445ec93f792b1e,
        0xcb7110e6ce866f2b,
        0xbebf59a07dab4400,
        0x57931eeaf85cf64f,
    ), // 82
    (
        10,
        0x282817e1038950fa,
        0xcc0052b18b0e2a19,
        0xd7540d4093bc3109,
        0x305a944507c82f47,
    ), // 83
    (
        10,
        0x280c4e90c9ab1f45,
        0xcc8ddd448f8b845a,
        0xf2b96616f1900000,
        0xe007ccc9c22781a,
    ), // 84
    (
        9,
        0x27f0ff1bc1ee87cd,
        0xcd19bb053fb0284e,
        0x336de62af2bca35,
        0x3e92c42e000eeed4,
    ), // 85
    (
        9,
        0x27d625ecf571c340,
        0xcda3f5fb9c415052,
        0x39235ec33d49600,
        0x1ebe59130db2795e,
    ), // 86
    (
        9,
        0x27bbbf95282fcd45,
        0xce2c97d694adab3f,
        0x3f674e539585a17,
        0x268859e90f51b89,
    ), // 87
    (
        9,
        0x27a1c8c8ddaf84da,
        0xceb3a9f01975077f,
        0x4645b6958000000,
        0xd24cde0463108cfa,
    ), // 88
    (
        9,
        0x27883e5e7df3f518,
        0xcf393550f3aa6906,
        0x4dcb74afbc49c19,
        0xa536009f37adc383,
    ), // 89
    (
        9,
        0x276f1d4c9847e90e,
        0xcfbd42b465836767,
        0x56064e1d18d9a00,
        0x7cea06ce1c9ace10,
    ), // 90
    (
        9,
        0x275662a841b30191,
        0xd03fda8b97997f33,
        0x5f04fe2cd8a39fb,
        0x58db032e72e8ba43,
    ), // 91
    (
        9,
        0x273e0ba38d15a47b,
        0xd0c10500d63aa658,
        0x68d74421f5c0000,
        0x388cc17cae105447,
    ), // 92
    (
        9,
        0x2726158c1b13cf03,
        0xd140c9faa1e5439e,
        0x738df1f6ab4827d,
        0x1b92672857620ce0,
    ), // 93
    (
        9,
        0x270e7dc9c01d8e9b,
        0xd1bf311e95d00de3,
        0x7f3afbc9cfb5e00,
        0x18c6a9575c2ade4,
    ), // 94
    (
        9,
        0x26f741dd3f070d61,
        0xd23c41d42727c808,
        0x8bf187fba88f35f,
        0xd44da7da8e44b24f,
    ), // 95
    (
        9,
        0x26e05f5f16c2159e,
        0xd2b803473f7ad0f3,
        0x99c600000000000,
        0xaa2f78f1b4cc6794,
    ), // 96
    (
        9,
        0x26c9d3fe61e80598,
        0xd3327c6ab49ca6c8,
        0xa8ce21eb6531361,
        0x843c067d091ee4cc,
    ), // 97
    (
        9,
        0x26b39d7fc6ddab08,
        0xd3abb3faa02166cc,
        0xb92112c1a0b6200,
        0x62005e1e913356e3,
    ), // 98
    (
        9,
        0x269db9bc7772a5cc,
        0xd423b07e986aa967,
        0xcad7718b8747c43,
        0x4316eed01dedd518,
    ), // 99
    (
        9,
        0x268826a13ef3fde6,
        0xd49a784bcd1b8afe,
        0xde0b6b3a7640000,
        0x2725dd1d243aba0e,
    ), // 100
    (
        9,
        0x2672e22d9dbdbd9f,
        0xd510118708a8f8dd,
        0xf2d8cf5fe6d74c5,
        0xddd9057c24cb54f,
    ), // 101
    (
        9,
        0x265dea72f169cc99,
        0xd5848226989d33c3,
        0x1095d25bfa712600,
        0xedeee175a736d2a1,
    ), // 102
    (
        9,
        0x26493d93a8cb2514,
        0xd5f7cff41e09aeb8,
        0x121b7c4c3698faa7,
        0xc4699f3df8b6b328,
    ), // 103
    (
        9,
        0x2634d9c282f3ef82,
        0xd66a008e4788cbcd,
        0x13c09e8d68000000,
        0x9ebbe7d859cb5a7c,
    ), // 104
    (
        9,
        0x2620bd41d8933adc,
        0xd6db196a761949d9,
        0x15876ccb0b709ca9,
        0x7c828b9887eb2179,
    ), // 105
    (
        9,
        0x260ce662ef04088a,
        0xd74b1fd64e0753c6,
        0x17723c2976da2a00,
        0x5d652ab99001adcf,
    ), // 106
    (
        9,
        0x25f95385547353fd,
        0xd7ba18f93502e409,
        0x198384e9c259048b,
        0x4114f1754e5d7b32,
    ), // 107
    (
        9,
        0x25e60316448db8e1,
        0xd82809d5be7072db,
        0x1bbde41dfeec0000,
        0x274b7c902f7e0188,
    ), // 108
    (
        9,
        0x25d2f390152f74f5,
        0xd894f74b06ef8b40,
        0x1e241d6e3337910d,
        0xfc9e0fbb32e210c,
    ), // 109
    (
        9,
        0x25c02379aa9ad043,
        0xd900e6160002ccfe,
        0x20b91cee9901ee00,
        0xf4afa3e594f8ea1f,
    ), // 110
    (
        9,
        0x25ad9165f2c18907,
        0xd96bdad2acb5f5ef,
        0x237ff9079863dfef,
        0xcd85c32e9e4437b0,
    ), // 111
    (
        9,
        0x259b3bf36735c90c,
        0xd9d5d9fd5010b366,
        0x267bf47000000000,
        0xa9bbb147e0dd92a8,
    ), // 112
    (
        9,
        0x258921cb955e7693,
        0xda3ee7f38e181ed0,
        0x29b08039fbeda7f1,
        0x8900447b70e8eb82,
    ), // 113
    (
        9,
        0x257741a2ac9170af,
        0xdaa708f58014d37c,
        0x2d213df34f65f200,
        0x6b0a92adaad5848a,
    ), // 114
    (
        9,
        0x25659a3711bc827d,
        0xdb0e4126bcc86bd7,
        0x30d201d957a7c2d3,
        0x4f990ad8740f0ee5,
    ), // 115
    (
        9,
        0x25542a50f84b9c39,
        0xdb74948f5532da4b,
        0x34c6d52160f40000,
        0x3670a9663a8d3610,
    ), // 116
    (
        9,
        0x2542f0c20000377d,
        0xdbda071cc67e6db5,
        0x3903f855d8f4c755,
        0x1f5c44188057be3c,
    ), // 117
    (
        9,
        0x2531ec64d772bd64,
        0xdc3e9ca2e1a05533,
        0x3d8de5c8ec59b600,
        0xa2bea956c4e4977,
    ), // 118
    (
        9,
        0x25211c1ce2fb5a6e,
        0xdca258dca9331635,
        0x4269541d1ff01337,
        0xed68b23033c3637e,
    ), // 119
    (
        9,
        0x25107ed5e7c3ec3b,
        0xdd053f6d26089673,
        0x479b38e478000000,
        0xc99cf624e50549c5,
    ), // 120
    (
        9,
        0x25001383bac8a744,
        0xdd6753e032ea0efe,
        0x4d28cb56c33fa539,
        0xa8adf7ae45e7577b,
    ), // 121
    (
        9,
        0x24efd921f390bce3,
        0xddc899ab3ff56c5e,
        0x5317871fa13aba00,
        0x8a5bc740b1c113e5,
    ), // 122
    (
        9,
        0x24dfceb3a26bb203,
        0xde29142e0e01401f,
        0x596d2f44de9fa71b,
        0x6e6c7efb81cfbb9b,
    ), // 123
    (
        9,
        0x24cff3430a0341a7,
        0xde88c6b3626a72aa,
        0x602fd125c47c0000,
        0x54aba5c5cada5f10,
    ), // 124
    (
        9,
        0x24c045e15c149931,
        0xdee7b471b3a9507d,
        0x6765c793fa10079d,
        0x3ce9a36f23c0fc90,
    ), // 125
    (
        9,
        0x24b0c5a679267ae2,
        0xdf45e08bcf06554e,
        0x6f15be069b847e00,
        0x26fb43de2c8cd2a8,
    ), // 126
    (
        9,
        0x24a171b0b31461c8,
        0xdfa34e1177c23362,
        0x7746b3e82a77047f,
        0x12b94793db8486a1,
    ), // 127
    (9, 0x2492492492492492, 0xdfffffffffffffff, 0x7, 0x0),  // 128
    (
        9,
        0x24834b2c9d85cdfe,
        0xe05bf942dbbc2145,
        0x894953f7ea890481,
        0xdd5deca404c0156d,
    ), // 129
    (
        9,
        0x247476f924137501,
        0xe0b73cb42e16914c,
        0x932abffea4848200,
        0xbd51373330291de0,
    ), // 130
    (
        9,
        0x2465cbc00a40cec0,
        0xe111cd1d5133412e,
        0x9dacb687d3d6a163,
        0x9fa4025d66f23085,
    ), // 131
    (
        9,
        0x245748bc980e0427,
        0xe16bad3758efd873,
        0xa8d8102a44840000,
        0x842530ee2db4949d,
    ), // 132
    (
        9,
        0x2448ed2f49eb0633,
        0xe1c4dfab90aab5ef,
        0xb4b60f9d140541e5,
        0x6aa7f2766b03dc25,
    ), // 133
    (
        9,
        0x243ab85da36e3167,
        0xe21d6713f453f356,
        0xc15065d4856e4600,
        0x53035ba7ebf32e8d,
    ), // 134
    (
        9,
        0x242ca99203ea8c18,
        0xe27545fba4fe385a,
        0xceb1363f396d23c7,
        0x3d12091fc9fb4914,
    ), // 135
    (
        9,
        0x241ec01b7cce4ea0,
        0xe2cc7edf592262cf,
        0xdce31b2488000000,
        0x28b1cb81b1ef1849,
    ), // 136
    (
        9,
        0x2410fb4da9b3b0fc,
        0xe323142dc8c66b55,
        0xebf12a24bca135c9,
        0x15c35be67ae3e2c9,
    ), // 137
    (
        9,
        0x24035a808a0f315e,
        0xe379084815b5774c,
        0xfbe6f8dbf88f4a00,
        0x42a17bd09be1ff0,
    ), // 138
    (
        8,
        0x23f5dd105c67ab9d,
        0xe3ce5d822ff4b643,
        0x1ef156c084ce761,
        0x8bf461f03cf0bbf,
    ), // 139
    (
        8,
        0x23e8825d7b05abb1,
        0xe4231623369e78e5,
        0x20c4e3b94a10000,
        0xf3fbb43f68a32d05,
    ), // 140
    (
        8,
        0x23db49cc3a0866fe,
        0xe4773465d54aded7,
        0x22b0695a08ba421,
        0xd84f44c48564dc19,
    ), // 141
    (
        8,
        0x23ce32c4c6cfb9f5,
        0xe4caba789e2b8687,
        0x24b4f35d7a4c100,
        0xbe58ebcce7956abe,
    ), // 142
    (
        8,
        0x23c13cb308ab6ab7,
        0xe51daa7e60fdd34c,
        0x26d397284975781,
        0xa5fac463c7c134b7,
    ), // 143
    (
        8,
        0x23b4670682c0c709,
        0xe570068e7ef5a1e7,
        0x290d74100000000,
        0x8f19241e28c7d757,
    ), // 144
    (
        8,
        0x23a7b13237187c8b,
        0xe5c1d0b53bc09fca,
        0x2b63b3a37866081,
        0x799a6d046c0ae1ae,
    ), // 145
    (
        8,
        0x239b1aac8ac74728,
        0xe6130af40bc0ecbf,
        0x2dd789f4d894100,
        0x6566e37d746a9e40,
    ), // 146
    (
        8,
        0x238ea2ef2b24c379,
        0xe663b741df9c37c0,
        0x306a35e51b58721,
        0x526887dbfb5f788f,
    ), // 147
    (
        8,
        0x23824976f4045a26,
        0xe6b3d78b6d3b24fb,
        0x331d01712e10000,
        0x408af3382b8efd3d,
    ), // 148
    (
        8,
        0x23760dc3d6e4d729,
        0xe7036db376537b90,
        0x35f14200a827c61,
        0x2fbb374806ec05f1,
    ), // 149
    (
        8,
        0x2369ef58c30bd43e,
        0xe7527b930c965bf2,
        0x38e858b62216100,
        0x1fe7c0f0afce87fe,
    ), // 150
    (
        8,
        0x235dedbb8e82aa1c,
        0xe7a102f9d39a9331,
        0x3c03b2c13176a41,
        0x11003d517540d32e,
    ), // 151
    (
        8,
        0x23520874dfeb1ffd,
        0xe7ef05ae409a0288,
        0x3f44c9b21000000,
        0x2f5810f98eff0dc,
    ), // 152
    (
        8,
        0x23463f1019228dd7,
        0xe83c856dd81804b7,
        0x42ad23cef3113c1,
        0xeb72e35e7840d910,
    ), // 153
    (
        8,
        0x233a911b42aa9b3c,
        0xe88983ed6985bae5,
        0x463e546b19a2100,
        0xd27de19593dc3614,
    ), // 154
    (
        8,
        0x232efe26f7cf33f9,
        0xe8d602d948f83829,
        0x49f9fc3f96684e1,
        0xbaf391fd3e5e6fc2,
    ), // 155
    (
        8,
        0x232385c65381b485,
        0xe92203d587039cc1,
        0x4de1c9c5dc10000,
        0xa4bd38c55228c81d,
    ), // 156
    (
        8,
        0x2318278edde1b39b,
        0xe96d887e26cd57b7,
        0x51f77994116d2a1,
        0x8fc5a8de8e1de782,
    ), // 157
    (
        8,
        0x230ce3187a6c2be9,
        0xe9b892675266f66c,
        0x563cd6bb3398100,
        0x7bf9265bea9d3a3b,
    ), // 158
    (
        8,
        0x2301b7fd56ca21bb,
        0xea03231d8d8224ba,
        0x5ab3bb270beeb01,
        0x69454b325983dccd,
    ), // 159
    (
        8,
        0x22f6a5d9da38341c,
        0xea4d3c25e68dc57f,
        0x5f5e10000000000,
        0x5798ee2308c39df9,
    ), // 160
    (
        8,
        0x22ebac4c9580d89f,
        0xea96defe264b59be,
        0x643dce0ec16f501,
        0x46e40ba0fa66a753,
    ), // 161
    (
        8,
        0x22e0caf633834beb,
        0xeae00d1cfdeb43cf,
        0x6954fe21e3e8100,
        0x3717b0870b0db3a7,
    ), // 162
    (
        8,
        0x22d601796a418886,
        0xeb28c7f233bdd372,
        0x6ea5b9755f440a1,
        0x2825e6775d11cdeb,
    ), // 163
    (
        8,
        0x22cb4f7aec6fd8b4,
        0xeb7110e6ce866f2b,
        0x74322a1c0410000,
        0x1a01a1c09d1b4dac,
    ), // 164
    (
        8,
        0x22c0b4a15b80d83e,
        0xebb8e95d3f7d9df2,
        0x79fc8b6ae8a46e1,
        0xc9eb0a8bebc8f3e,
    ), // 165
    (
        8,
        0x22b630953a28f77a,
        0xec0052b18b0e2a19,
        0x80072a66d512100,
        0xffe357ff59e6a004,
    ), // 166
    (
        8,
        0x22abc300df54ca7c,
        0xec474e39705912d2,
        0x86546633b42b9c1,
        0xe7dfd1be05fa61a8,
    ), // 167
    (
        8,
        0x22a16b90698da5d2,
        0xec8ddd448f8b845a,
        0x8ce6b0861000000,
        0xd11ed6fc78f760e5,
    ), // 168
    (
        8,
        0x229729f1b2c83ded,
        0xecd4011c8f11979a,
        0x93c08e16a022441,
        0xbb8db609dd29ebfe,
    ), // 169
    (
        8,
        0x228cfdd444992f78,
        0xed19bb053fb0284e,
        0x9ae49717f026100,
        0xa71aec8d1813d532,
    ), // 170
    (
        8,
        0x2282e6e94ccb8588,
        0xed5f0c3cbf8fa470,
        0xa25577ae24c1a61,
        0x93b612a9f20fbc02,
    ), // 171
    (
        8,
        0x2278e4e392557ecf,
        0xeda3f5fb9c415052,
        0xaa15f068e610000,
        0x814fc7b19a67d317,
    ), // 172
    (
        8,
        0x226ef7776aa7fd29,
        0xede87974f3c81855,
        0xb228d6bf7577921,
        0x6fd9a03f2e0a4b7c,
    ), // 173
    (
        8,
        0x22651e5aaf5532d0,
        0xee2c97d694adab3f,
        0xba91158ef5c4100,
        0x5f4615a38d0d316e,
    ), // 174
    (
        8,
        0x225b5944b40b4694,
        0xee7052491d2c3e64,
        0xc351ad9aec0b681,
        0x4f8876863479a286,
    ), // 175
    (
        8,
        0x2251a7ee3cdfcca5,
        0xeeb3a9f01975077f,
        0xcc6db6100000000,
        0x4094d8a3041b60eb,
    ), // 176
    (
        8,
        0x22480a1174e913d9,
        0xeef69fea211b2627,
        0xd5e85d09025c181,
        0x32600b8ed883a09b,
    ), // 177
    (
        8,
        0x223e7f69e522683c,
        0xef393550f3aa6906,
        0xdfc4e816401c100,
        0x24df8c6eb4b6d1f1,
    ), // 178
    (
        8,
        0x223507b46b988abe,
        0xef7b6b399471103e,
        0xea06b4c72947221,
        0x18097a8ee151acef,
    ), // 179
    (
        8,
        0x222ba2af32dbbb9e,
        0xefbd42b465836767,
        0xf4b139365210000,
        0xbd48cc8ec1cd8e3,
    ), // 180
    (
        8,
        0x22225019a9b4d16c,
        0xeffebccd41ffcd5c,
        0xffc80497d520961,
        0x3807a8d67485fb,
    ), // 181
    (
        8,
        0x22190fb47b1af172,
        0xf03fda8b97997f33,
        0x10b4ebfca1dee100,
        0xea5768860b62e8d8,
    ), // 182
    (
        8,
        0x220fe14186679801,
        0xf0809cf27f703d52,
        0x117492de921fc141,
        0xd54faf5b635c5005,
    ), // 183
    (
        8,
        0x2206c483d7c6b786,
        0xf0c10500d63aa658,
        0x123bb2ce41000000,
        0xc14a56233a377926,
    ), // 184
    (
        8,
        0x21fdb93fa0e0ccc5,
        0xf10113b153c8ea7b,
        0x130a8b6157bdecc1,
        0xae39a88db7cd329f,
    ), // 185
    (
        8,
        0x21f4bf3a31bcdcaa,
        0xf140c9faa1e5439e,
        0x13e15dede0e8a100,
        0x9c10bde69efa7ab6,
    ), // 186
    (
        8,
        0x21ebd639f1d86584,
        0xf18028cf72976a4e,
        0x14c06d941c0ca7e1,
        0x8ac36c42a2836497,
    ), // 187
    (
        8,
        0x21e2fe06597361a6,
        0xf1bf311e95d00de3,
        0x15a7ff487a810000,
        0x7a463c8b84f5ef67,
    ), // 188
    (
        8,
        0x21da3667eb0e8ccb,
        0xf1fde3d30e812642,
        0x169859ddc5c697a1,
        0x6a8e5f5ad090fd4b,
    ), // 189
    (
        8,
        0x21d17f282d1a300e,
        0xf23c41d42727c808,
        0x1791c60f6fed0100,
        0x5b91a2943596fc56,
    ), // 190
    (
        8,
        0x21c8d811a3d3c9e1,
        0xf27a4c0585cbf805,
        0x18948e8c0e6fba01,
        0x4d4667b1c468e8f0,
    ), // 191
    (
        8,
        0x21c040efcb50f858,
        0xf2b803473f7ad0f3,
        0x19a1000000000000,
        0x3fa39ab547994daf,
    ), // 192
    (
        8,
        0x21b7b98f11b61c1a,
        0xf2f56875eb3f2614,
        0x1ab769203dafc601,
        0x32a0a9b2faee1e2a,
    ), // 193
    (
        8,
        0x21af41bcd19739ba,
        0xf3327c6ab49ca6c8,
        0x1bd81ab557f30100,
        0x26357ceac0e96962,
    ), // 194
    (
        8,
        0x21a6d9474c81adf0,
        0xf36f3ffb6d916240,
        0x1d0367a69fed1ba1,
        0x1a5a6f65caa5859e,
    ), // 195
    (
        8,
        0x219e7ffda5ad572a,
        0xf3abb3faa02166cc,
        0x1e39a5057d810000,
        0xf08480f672b4e86,
    ), // 196
    (
        8,
        0x219635afdcd3e46d,
        0xf3e7d9379f70166a,
        0x1f7b2a18f29ac3e1,
        0x4383340615612ca,
    ), // 197
    (
        8,
        0x218dfa2ec92d0643,
        0xf423b07e986aa967,
        0x20c850694c2aa100,
        0xf3c77969ee4be5a2,
    ), // 198
    (
        8,
        0x2185cd4c148e4ae2,
        0xf45f3a98a20738a4,
        0x222173cc014980c1,
        0xe00993cc187c5ec9,
    ), // 199
    (
        8,
        0x217daeda36ad7a5c,
        0xf49a784bcd1b8afe,
        0x2386f26fc1000000,
        0xcd2b297d889bc2b6,
    ), // 200
    (
        8,
        0x21759eac708452fe,
        0xf4d56a5b33cec44a,
        0x24f92ce8af296d41,
        0xbb214d5064862b22,
    ), // 201
    (
        8,
        0x216d9c96c7d490d4,
        0xf510118708a8f8dd,
        0x2678863cd0ece100,
        0xa9e1a7ca7ea10e20,
    ), // 202
    (
        8,
        0x2165a86e02cb358c,
        0xf54a6e8ca5438db1,
        0x280563f0a9472d61,
        0x99626e72b39ea0cf,
    ), // 203
    (
        8,
        0x215dc207a3c20fdf,
        0xf5848226989d33c3,
        0x29a02e1406210000,
        0x899a5ba9c13fafd9,
    ), // 204
    (
        8,
        0x2155e939e51e8b37,
        0xf5be4d0cb51434aa,
        0x2b494f4efe6d2e21,
        0x7a80a705391e96ff,
    ), // 205
    (
        8,
        0x214e1ddbb54cd933,
        0xf5f7cff41e09aeb8,
        0x2d0134ef21cbc100,
        0x6c0cfe23de23042a,
    ), // 206
    (
        8,
        0x21465fc4b2d68f98,
        0xf6310b8f55304840,
        0x2ec84ef4da2ef581,
        0x5e377df359c944dd,
    ), // 207
    (
        8,
        0x213eaecd2893dd60,
        0xf66a008e4788cbcd,
        0x309f102100000000,
        0x50f8ac5fc8f53985,
    ), // 208
    (
        8,
        0x21370ace09f681c6,
        0xf6a2af9e5a0f0a08,
        0x3285ee02a1420281,
        0x44497266278e35b7,
    ), // 209
    (
        8,
        0x212f73a0ef6db7cb,
        0xf6db196a761949d9,
        0x347d6104fc324100,
        0x382316831f7ee175,
    ), // 210
    (
        8,
        0x2127e92012e25004,
        0xf7133e9b156c7be5,
        0x3685e47dade53d21,
        0x2c7f377833b8946e,
    ), // 211
    (
        8,
        0x21206b264c4a39a7,
        0xf74b1fd64e0753c6,
        0x389ff6bb15610000,
        0x2157c761ab4163ef,
    ), // 212
    (
        8,
        0x2118f98f0e52c28f,
        0xf782bdbfdda6577b,
        0x3acc1912ebb57661,
        0x16a7071803cc49a9,
    ), // 213
    (
        8,
        0x211194366320dc66,
        0xf7ba18f93502e409,
        0x3d0acff111946100,
        0xc6781d80f8224fc,
    ), // 214
    (
        8,
        0x210a3af8e926bb78,
        0xf7f1322182cf15d1,
        0x3f5ca2e692eaf841,
        0x294092d370a900b,
    ), // 215
    (
        8,
        0x2102edb3d00e29a6,
        0xf82809d5be7072db,
        0x41c21cb8e1000000,
        0xf24f62335024a295,
    ), // 216
    (
        8,
        0x20fbac44d5b6edc2,
        0xf85ea0b0b27b2610,
        0x443bcb714399a5c1,
        0xe03b98f103fad6d2,
    ), // 217
    (
        8,
        0x20f4768a4348ad08,
        0xf894f74b06ef8b40,
        0x46ca406c81af2100,
        0xcee3d32cad2a9049,
    ), // 218
    (
        8,
        0x20ed4c62ea57b1f0,
        0xf8cb0e3b4b3bbdb3,
        0x496e106ac22aaae1,
        0xbe3f9df9277fdada,
    ), // 219
    (
        8,
        0x20e62dae221c087a,
        0xf900e6160002ccfe,
        0x4c27d39fa5410000,
        0xae46f0d94c05e933,
    ), // 220
    (
        8,
        0x20df1a4bc4ba6525,
        0xf9367f6da0ab2e9c,
        0x4ef825c296e43ca1,
        0x9ef2280fb437a33d,
    ), // 221
    (
        8,
        0x20d8121c2c9e506e,
        0xf96bdad2acb5f5ef,
        0x51dfa61f5ad88100,
        0x9039ff426d3f284b,
    ), // 222
    (
        8,
        0x20d1150031e51549,
        0xf9a0f8d3b0e04fde,
        0x54def7a6d2f16901,
        0x82178c6d6b51f8f4,
    ), // 223
    (
        8,
        0x20ca22d927d8f54d,
        0xf9d5d9fd5010b366,
        0x57f6c10000000000,
        0x74843b1ee4c1e053,
    ), // 224
    (
        8,
        0x20c33b88da7c29aa,
        0xfa0a7eda4c112ce6,
        0x5b27ac993df97701,
        0x6779c7f90dc42f48,
    ), // 225
    (
        8,
        0x20bc5ef18c233bdf,
        0xfa3ee7f38e181ed0,
        0x5e7268b9bbdf8100,
        0x5af23c74f9ad9fe9,
    ), // 226
    (
        8,
        0x20b58cf5f31e4526,
        0xfa7315d02f20c7bd,
        0x61d7a7932ff3d6a1,
        0x4ee7eae2acdc617e,
    ), // 227
    (
        8,
        0x20aec5793770a74d,
        0xfaa708f58014d37c,
        0x65581f53c8c10000,
        0x43556aa2ac262a0b,
    ), // 228
    (
        8,
        0x20a8085ef096d530,
        0xfadac1e711c832d1,
        0x68f48a385b8320e1,
        0x3835949593b8ddd1,
    ), // 229
    (
        8,
        0x20a1558b2359c4b1,
        0xfb0e4126bcc86bd7,
        0x6cada69ed07c2100,
        0x2d837fbe78458762,
    ), // 230
    (
        8,
        0x209aace23fafa72e,
        0xfb418734a9008bd9,
        0x70843718cdbf27c1,
        0x233a7e150a54a555,
    ), // 231
    (
        8,
        0x20940e491ea988d7,
        0xfb74948f5532da4b,
        0x7479027ea1000000,
        0x19561984a50ff8fe,
    ), // 232
    (
        8,
        0x208d79a5006d7a47,
        0xfba769b39e49640e,
        0x788cd40268f39641,
        0xfd211159fe3490f,
    ), // 233
    (
        8,
        0x2086eedb8a3cead3,
        0xfbda071cc67e6db5,
        0x7cc07b437ecf6100,
        0x6aa563e655033e3,
    ), // 234
    (
        8,
        0x20806dd2c486dcc6,
        0xfc0c6d447c5dd362,
        0x8114cc6220762061,
        0xfbb614b3f2d3b14c,
    ), // 235
    (
        8,
        0x2079f67119059fae,
        0xfc3e9ca2e1a05533,
        0x858aa0135be10000,
        0xeac0f8837fb05773,
    ), // 236
    (
        8,
        0x2073889d50e7bf63,
        0xfc7095ae91e1c760,
        0x8a22d3b53c54c321,
        0xda6e4c10e8615ca5,
    ), // 237
    (
        8,
        0x206d243e9303d929,
        0xfca258dca9331635,
        0x8ede496339f34100,
        0xcab755a8d01fa67f,
    ), // 238
    (
        8,
        0x2066c93c62170aa8,
        0xfcd3e6a0ca8906c2,
        0x93bde80aec3a1481,
        0xbb95a9ae71aa3e0c,
    ), // 239
    (
        8,
        0x2060777e9b0db0f6,
        0xfd053f6d26089673,
        0x98c29b8100000000,
        0xad0326c296b4f529,
    ), // 240
    (
        8,
        0x205a2eed73563032,
        0xfd3663b27f31d529,
        0x9ded549671832381,
        0x9ef9f21eed31b7c1,
    ), // 241
    (
        8,
        0x2053ef71773d7e6a,
        0xfd6753e032ea0efe,
        0xa33f092e0b1ac100,
        0x91747422be14b0b2,
    ), // 242
    (
        8,
        0x204db8f388552ea9,
        0xfd9810643d6614c3,
        0xa8b8b452291fe821,
        0x846d550e37b5063d,
    ), // 243
    (
        8,
        0x20478b5cdbe2bb2f,
        0xfdc899ab3ff56c5e,
        0xae5b564ac3a10000,
        0x77df79e9a96c06f6,
    ), // 244
    (
        8,
        0x20416696f957cfbf,
        0xfdf8f02086af2c4b,
        0xb427f4b3be74c361,
        0x6bc6019636c7d0c2,
    ), // 245
    (
        8,
        0x203b4a8bb8d356e7,
        0xfe29142e0e01401f,
        0xba1f9a938041e100,
        0x601c4205aebd9e47,
    ), // 246
    (
        8,
        0x2035372541ab0f0d,
        0xfe59063c8822ce56,
        0xc0435871d1110f41,
        0x54ddc59756f05016,
    ), // 247
    (
        8,
        0x202f2c4e08fd6dcc,
        0xfe88c6b3626a72aa,
        0xc694446f01000000,
        0x4a0648979c838c18,
    ), // 248
    (
        8,
        0x202929f0d04b99e9,
        0xfeb855f8ca88fb0d,
        0xcd137a5b57ac3ec1,
        0x3f91b6e0bb3a053d,
    ), // 249
    (
        8,
        0x20232ff8a41b45eb,
        0xfee7b471b3a9507d,
        0xd3c21bcecceda100,
        0x357c299a88ea76a5,
    ), // 250
    (
        8,
        0x201d3e50daa036db,
        0xff16e281db76303b,
        0xdaa150410b788de1,
        0x2bc1e517aecc56e3,
    ), // 251
    (
        8,
        0x201754e5126d446d,
        0xff45e08bcf06554e,
        0xe1b24521be010000,
        0x225f56ceb3da9f5d,
    ), // 252
    (
        8,
        0x201173a1312ca135,
        0xff74aef0efafadd7,
        0xe8f62df12777c1a1,
        0x1951136d53ad63ac,
    ), // 253
    (
        8,
        0x200b9a71625f3b13,
        0xffa34e1177c23362,
        0xf06e445906fc0100,
        0x1093d504b3cd7d93,
    ), // 254
    (
        8,
        0x2005c94216230568,
        0xffd1be4c7f2af942,
        0xf81bc845c81bf801,
        0x824794d1ec1814f,
    ), // 255
    (8, 0x1fffffffffffffff, 0xffffffffffffffff, 0x8, 0x0),  // 256
];

// This section is created by factorial_data.rs.

// This is equivalent to `__gmp_oddfac_table` in `mpn/comb_tables.c`, GMP 6.2.1, which is the
// combination of `ONE_LIMB_ODD_FACTORIAL_TABLE` and `ONE_LIMB_ODD_FACTORIAL_EXTTABLE` in
// `fac_table.h`, GMP 6.2.1.
pub const ONE_LIMB_ODD_FACTORIAL_TABLE: [Limb; 68] = [
    0x1,
    0x1,
    0x1,
    0x3,
    0x3,
    0xf,
    0x2d,
    0x13b,
    0x13b,
    0xb13,
    0x375f,
    0x26115,
    0x7233f,
    0x5cca33,
    0x2898765,
    0x260eeeeb,
    0x260eeeeb,
    0x286fddd9b,
    0x16beecca73,
    0x1b02b930689,
    0x870d9df20ad,
    0xb141df4dae31,
    0x79dd498567c1b,
    0xaf2e19afc5266d,
    0x20d8a4d0f4f7347,
    0x335281867ec241ef,
    0x9b3093d46fdd5923,
    0x5e1f9767cc5866b1,
    0x92dd23d6966aced7,
    0xa30d0f4f0a196e5b,
    0x8dc3e5a1977d7755,
    0x2ab8ce915831734b,
    0x2ab8ce915831734b,
    0x81d2a0bc5e5fdcab,
    0x9efcac82445da75b,
    0xbc8b95cf58cde171,
    0xa0e8444a1f3cecf9,
    0x4191deb683ce3ffd,
    0xddd3878bc84ebfc7,
    0xcb39a64b83ff3751,
    0xf8203f7993fc1495,
    0xbd2a2a78b35f4bdd,
    0x84757be6b6d13921,
    0x3fbbcfc0b524988b,
    0xbd11ed47c8928df9,
    0x3c26b59e41c2f4c5,
    0x677a5137e883fdb3,
    0xff74e943b03b93dd,
    0xfe5ebbcb10b2bb97,
    0xb021f1de3235e7e7,
    0x33509eb2e743a58f,
    0x390f9da41279fb7d,
    0xe5cb0154f031c559,
    0x93074695ba4ddb6d,
    0x81c471caa636247f,
    0xe1347289b5a1d749,
    0x286f21c3f76ce2ff,
    0xbe84a2173e8ac7,
    0x1595065ca215b88b,
    0xf95877595b018809,
    0x9c2efe3c5516f887,
    0x373294604679382b,
    0xaf1ff7a888adcd35,
    0x18ddf279a2c5800b,
    0x18ddf279a2c5800b,
    0x505a90e2542582cb,
    0x5bacad2cd8d5dc2b,
    0xfe3152bcbff89f41,
];
// This is equivalent to `ODD_FACTORIAL_TABLE_LIMIT` in `fac_table.h`, GMP 6.2.1.
pub const ODD_FACTORIAL_TABLE_LIMIT: usize = 25;
// This is equivalent to `ODD_FACTORIAL_EXTTABLE_LIMIT` in `fac_table.h`, GMP 6.2.1.
pub const ODD_FACTORIAL_EXTTABLE_LIMIT: usize = 67;
// This is equivalent to `ODD_FACTORIAL_TABLE_MAX` in `fac_table.h`, GMP 6.2.1.
pub const ODD_FACTORIAL_TABLE_MAX: Limb = 0x335281867ec241ef;

// This is equivalent to `__gmp_odd2fac_table` in `mpn/comb_tables.c`, GMP 6.2.1, and
// `ONE_LIMB_ODD_DOUBLEFACTORIAL_TABLE` from `fac_table.h`, GMP 6.2.1.
pub const ONE_LIMB_ODD_DOUBLEFACTORIAL_TABLE: [Limb; 17] = [
    0x1,
    0x3,
    0xf,
    0x69,
    0x3b1,
    0x289b,
    0x20fdf,
    0x1eee11,
    0x20dcf21,
    0x27065f73,
    0x33385d46f,
    0x49a10615f9,
    0x730b9982551,
    0xc223930bef8b,
    0x15fe07a85a22bf,
    0x2a9c2ed62ea3521,
    0x57e22099c030d941,
];
// This is equivalent to `ODD_DOUBLEFACTORIAL_TABLE_LIMIT` in `fac_table.h`, GMP 6.2.1.
pub const ODD_DOUBLEFACTORIAL_TABLE_LIMIT: usize = 33;
// This is equivalent to `ODD_DOUBLEFACTORIAL_TABLE_MAX` in `fac_table.h`, GMP 6.2.1.
pub const ODD_DOUBLEFACTORIAL_TABLE_MAX: Limb = 0x57e22099c030d941;

// This is equivalent to `__gmp_limbroots_table` in `mpn/comb_tables.c`, GMP 6.2.1, and
// `NTH_ROOT_NUMB_MASK_TABLE` from `fac_table.h`, GMP 6.2.1.
pub const NTH_ROOT_NUMB_MASK_TABLE: [Limb; 8] =
    [Limb::MAX, 0xffffffff, 0x285145, 0xffff, 0x1bdb, 0x659, 0x235, 0xff];

// This is equivalent to `ONE_LIMB_ODD_FACTORIAL_INVERSES_TABLE` from `fac_table.h`, GMP 6.2.1.
pub const ONE_LIMB_ODD_FACTORIAL_INVERSES_TABLE: [Limb; 64] = [
    0x1,
    0xaaaaaaaaaaaaaaab,
    0xaaaaaaaaaaaaaaab,
    0xeeeeeeeeeeeeeeef,
    0x4fa4fa4fa4fa4fa5,
    0x2ff2ff2ff2ff2ff3,
    0x2ff2ff2ff2ff2ff3,
    0x938cc70553e3771b,
    0xb71c27cddd93e49f,
    0xb38e3229fcdee63d,
    0xe684bb63544a4cbf,
    0xc2f684917ca340fb,
    0xf747c9cba417526d,
    0xbb26eb51d7bd49c3,
    0xbb26eb51d7bd49c3,
    0xb0a7efb985294093,
    0xbe4b8c69f259eabb,
    0x6854d17ed6dc4fb9,
    0xe1aa904c915f4325,
    0x3b8206df131cead1,
    0x79c6009fea76fe13,
    0xd8c5d381633cd365,
    0x4841f12b21144677,
    0x4a91ff68200b0d0f,
    0x8f9513a58c4f9e8b,
    0x2b3e690621a42251,
    0x4f520f00e03c04e7,
    0x2edf84ee600211d3,
    0xadcaa2764aaacdfd,
    0x161f4f9033f4fe63,
    0x161f4f9033f4fe63,
    0xbada2932ea4d3e03,
    0xcec189f3efaa30d3,
    0xf7475bb68330bf91,
    0x37eb7bf7d5b01549,
    0x46b35660a4e91555,
    0xa567c12d81f151f7,
    0x4c724007bb2071b1,
    0xf4a0cce58a016bd,
    0xfa21068e66106475,
    0x244ab72b5a318ae1,
    0x366ce67e080d0f23,
    0xd666fdae5dd2a449,
    0xd740ddd0acc06a0d,
    0xb050bbbb28e6f97b,
    0x70b003fe890a5c75,
    0xd03aabff83037427,
    0x13ec4ca72c783bd7,
    0x90282c06afdbd96f,
    0x4414ddb9db4a95d5,
    0xa2c68735ae6832e9,
    0xbf72d71455676665,
    0xa8469fab6b759b7f,
    0xc1e55b56e606caf9,
    0x40455630fc4a1cff,
    0x120a7b0046d16f7,
    0xa7c3553b08faef23,
    0x9f0bfd1b08d48639,
    0xa433ffce9a304d37,
    0xa22ad1d53915c683,
    0xcb6cbc723ba5dd1d,
    0x547fb1b8ab9d0ba3,
    0x547fb1b8ab9d0ba3,
    0x8f15a826498852e3,
];

pub const ODD_CENTRAL_BINOMIAL_OFFSET: usize = 13;

// This table contains binomial(2k, k) / 2 ^ t.
//
// This is equivalent to `bin2kk` in `mpz/bin_uiui.c`, GMP 6.2.1, and
// `ONE_LIMB_ODD_CENTRAL_BINOMIAL_TABLE` from `fac_table.h`, GMP 6.2.1.
pub const ONE_LIMB_ODD_CENTRAL_BINOMIAL_TABLE: [Limb; 23] = [
    0x13d66b,
    0x4c842f,
    0x93ee7d,
    0x11e9e123,
    0x22c60053,
    0x873ae4d1,
    0x10757bd97,
    0x80612c6cd,
    0xfaa556bc1,
    0x3d3cc24821,
    0x77cfeb6bbb,
    0x7550ebd97c7,
    0xe5f08695caf,
    0x386120ffce11,
    0x6eabb28dd6df,
    0x3658e31c82a8f,
    0x6ad2050312783,
    0x1a42902a5af0bf,
    0x33ac44f881661d,
    0xcb764f927d82123,
    0x190c23fa46b93983,
    0x62b7609e25caf1b9,
    0xc29cb72925ef2cff,
];

pub const ODD_CENTRAL_BINOMIAL_TABLE_LIMIT: usize = 35;

// This table contains the inverses of elements in the previous table.
//
// This is equivalent to `bin2kkinv` in `mpz/bin_uiui.c`, GMP 6.2.1, and
// `ONE_LIMB_ODD_CENTRAL_BINOMIAL_INVERSE_TABLE` from `fac_table.h`, GMP 6.2.1.
pub const ONE_LIMB_ODD_CENTRAL_BINOMIAL_INVERSE_TABLE: [Limb; 23] = [
    0x61e5bd199bb12643,
    0x78321494dc8342cf,
    0x4fd348704ebf7ad5,
    0x7e722ba086ab568b,
    0xa5fcc124265843db,
    0x89c4a6b18633f431,
    0x4daa2c15f8ce9227,
    0x801c618ca9be9605,
    0x32dc192f948a441,
    0xd02b90c2bf3be1,
    0xd897e8c1749aa173,
    0x54a234fc01fef9f7,
    0x83ff2ab4d1ff7a4f,
    0xa427f1c9b304e2f1,
    0x9c14595d1793651f,
    0x883a71c607a7b46f,
    0xd089863c54bc9f2b,
    0x9022f6bce5d07f3f,
    0xbec207e218768c35,
    0x9d70cb4cbb4f168b,
    0x3c3d3403828a9d2b,
    0x7672df58c56bc489,
    0x1e66ca55d727d2ff,
];

// This table contains the values t in the formula binomial(2k, k) / 2 ^ t.
//
// This is equivalent to `fac2bin` in `mpz/bin_uiui.c`, GMP 6.2.1, and `CENTRAL_BINOMIAL_2FAC_TABLE`
// from `fac_table.h`, GMP 6.2.1.
pub const CENTRAL_BINOMIAL_2FAC_TABLE: [u64; 23] =
    [3, 3, 4, 1, 2, 2, 3, 2, 3, 3, 4, 2, 3, 3, 4, 3, 4, 4, 5, 1, 2, 2, 3];

// https://oeis.org/A005187, skipping the initial 0
//
// This is equivalent to `__gmp_fac2cnt_table` in `mpn/comb_tables.c`, GMP 6.2.1, and
// `TABLE_2N_MINUS_POPC_2N` from `fac_table.h`, GMP 6.2.1.
pub const TABLE_2N_MINUS_POPC_2N: [u8; 40] = [
    1, 3, 4, 7, 8, 10, 11, 15, 16, 18, 19, 22, 23, 25, 26, 31, 32, 34, 35, 38, 39, 41, 42, 46, 47,
    49, 50, 53, 54, 56, 57, 63, 64, 66, 67, 70, 71, 73, 74, 78,
];

pub const TABLE_LIMIT_2N_MINUS_POPC_2N: u64 = 81;

pub const FFT_TAB: [[u8; 2]; 5] = [[4, 4], [4, 3], [3, 2], [2, 1], [2, 1]];

pub const MULMOD_TAB: [u8; 19] = [4, 4, 4, 4, 4, 3, 3, 3, 3, 3, 3, 3, 2, 2, 2, 2, 2, 1, 1];

// Use these tables to match FLINT for debugging
// ```
// pub const FFT_TAB: [[u8; 2]; 5] = [[3, 3], [3, 2], [2, 1], [2, 1], [0, 0]];
// pub const MULMOD_TAB: [u8; 15] = [4, 3, 3, 3, 3, 2, 2, 2, 3, 2, 2, 2, 2, 1, 1];
// ```
