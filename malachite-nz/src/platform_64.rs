pub type Limb = u64;
pub type HalfLimb = u32;
pub type DoubleLimb = u128;
pub type SignedLimb = i64;
pub type SignedHalfLimb = i32;
pub type SignedDoubleLimb = i128;
pub type FloatWithLimbWidth = f64;

pub const AORSMUL_FASTER_2AORSLSH: bool = true;
pub const AORSMUL_FASTER_3AORSLSH: bool = true;
pub const AORSMUL_FASTER_AORS_AORSLSH: bool = true;
pub const AORSMUL_FASTER_AORS_2AORSLSH: bool = true;

pub const MUL_TOOM22_THRESHOLD: usize = 106;
pub const MUL_TOOM33_THRESHOLD: usize = 74;
pub const MUL_TOOM44_THRESHOLD: usize = 178;
pub const MUL_TOOM6H_THRESHOLD: usize = 624;
pub const MUL_TOOM8H_THRESHOLD: usize = 371;

pub const MUL_TOOM32_TO_TOOM43_THRESHOLD: usize = 310;
pub const MUL_TOOM32_TO_TOOM53_THRESHOLD: usize = 300;
pub const MUL_TOOM42_TO_TOOM53_THRESHOLD: usize = 314;
pub const MUL_TOOM42_TO_TOOM63_THRESHOLD: usize = 103;

pub const MUL_FFT_THRESHOLD: usize = 4037;

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

pub const BMOD_1_TO_MOD_1_THRESHOLD: usize = std::usize::MAX;

pub const SQR_TOOM2_THRESHOLD: usize = 43;
pub const SQR_TOOM3_THRESHOLD: usize = 390;
pub const SQR_TOOM4_THRESHOLD: usize = 1090;
pub const SQR_TOOM6_THRESHOLD: usize = 336;
pub const SQR_TOOM8_THRESHOLD: usize = 837;
