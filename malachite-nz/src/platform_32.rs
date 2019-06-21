pub type Limb = u32;
pub type HalfLimb = u16;
pub type DoubleLimb = u64;
pub type SignedLimb = i32;
pub type SignedHalfLimb = i16;
pub type SignedDoubleLimb = i64;
pub type FloatWithLimbWidth = f32;

pub const AORSMUL_FASTER_2AORSLSH: bool = true;
pub const AORSMUL_FASTER_3AORSLSH: bool = true;
pub const AORSMUL_FASTER_AORS_AORSLSH: bool = true;
pub const AORSMUL_FASTER_AORS_2AORSLSH: bool = true;

pub const MUL_TOOM22_THRESHOLD: usize = 106;
pub const MUL_TOOM33_THRESHOLD: usize = 78;
pub const MUL_TOOM44_THRESHOLD: usize = 541;
pub const MUL_TOOM6H_THRESHOLD: usize = 728;
pub const MUL_TOOM8H_THRESHOLD: usize = 1026;

pub const MUL_TOOM32_TO_TOOM43_THRESHOLD: usize = 91;
pub const MUL_TOOM32_TO_TOOM53_THRESHOLD: usize = 307;
pub const MUL_TOOM42_TO_TOOM53_THRESHOLD: usize = 358;
pub const MUL_TOOM42_TO_TOOM63_THRESHOLD: usize = 112;

pub const MUL_FFT_THRESHOLD: usize = 5608;
