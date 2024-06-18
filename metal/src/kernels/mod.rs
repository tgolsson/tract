mod array_ops;
mod bin_ops;
mod element_wise;
mod mat_vec;
pub mod mfa_gemm;
mod mmm_tile_8x8;

pub use array_ops::MultiBroadcastCast;
pub use bin_ops::BinOps;
pub use element_wise::ElementWiseOps;
pub use mat_vec::{mat_vec, mat_vec_with_slice, metal_mat_vec};
pub use mfa_gemm::{mfa_gemm, GemmPrecision};
pub use mmm_tile_8x8::{metal_mmm_tile_8x8, mmm_tile_8x8};

#[cfg(target_os = "ios")]
pub const METAL_FLASH_ATTENTION_LIB: &[u8] = include_bytes!("libMetalFlashAttention-ios.metallib");
#[cfg(target_os = "macos")]
pub const METAL_FLASH_ATTENTION_LIB: &[u8] =
    include_bytes!("libMetalFlashAttention-macos.metallib");

pub const MMM_TILE_8X8_METAL_SOURCE: &str = include_str!("mmm_tile_8x8.metal");
pub const MUL_MAT_VEC: &str = include_str!("mat_vec.metal");
pub const ARRAY_OPS: &str = include_str!("array_ops.metal");
pub const BIN_OPS: &str = include_str!("bin_ops.metal");
pub const ELEMENT_WISE_OPS: &str = include_str!("element_wise.metal");

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LibraryContent<'a> {
    Data(&'a [u8]),
    Source(&'a str),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LibraryName {
    MfaLib,
    MmmTile8x8,
    MulMatVec,
    BinOps,
    ArrayOps,
    ElementWiseOps,
}

impl LibraryName {
    pub fn content(&self) -> LibraryContent<'static> {
        match self {
            Self::MfaLib => LibraryContent::Data(METAL_FLASH_ATTENTION_LIB),
            Self::MmmTile8x8 => LibraryContent::Source(MMM_TILE_8X8_METAL_SOURCE),
            Self::MulMatVec => LibraryContent::Source(MUL_MAT_VEC),
            Self::ArrayOps => LibraryContent::Source(ARRAY_OPS),
            Self::BinOps => LibraryContent::Source(BIN_OPS),
            Self::ElementWiseOps => LibraryContent::Source(ELEMENT_WISE_OPS),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BroadcastKind {
    Unicast,
    ByScalarLeft,
    ByScalarRight,
    Nd1,
    Nd2,
    Nd3,
    Nd4,
    Nd5,
}