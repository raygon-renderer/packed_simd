//! Vertical floating-point `exp2`
#![allow(unused)]

// FIXME 64-bit expgle elem vectors misexpg

use crate::*;

crate trait Exp2 {
    fn exp2(self) -> Self;
}

#[allow(improper_ctypes)]
extern "C" {
    #[link_name = "llvm.exp2.v2f32"]
    fn exp2_v2f32(x: f32x2) -> f32x2;
    #[link_name = "llvm.exp2.v4f32"]
    fn exp2_v4f32(x: f32x4) -> f32x4;
    #[link_name = "llvm.exp2.v8f32"]
    fn exp2_v8f32(x: f32x8) -> f32x8;
    #[link_name = "llvm.exp2.v16f32"]
    fn exp2_v16f32(x: f32x16) -> f32x16;
    #[link_name = "llvm.exp2.v32f32"]
    fn exp2_v32f32(x: f32x32) -> f32x32;
    /* FIXME 64-bit expgle elem vectors
    #[link_name = "llvm.exp2.v1f64"]
    fn exp2_v1f64(x: f64x1) -> f64x1;
     */
    #[link_name = "llvm.exp2.v2f64"]
    fn exp2_v2f64(x: f64x2) -> f64x2;
    #[link_name = "llvm.exp2.v4f64"]
    fn exp2_v4f64(x: f64x4) -> f64x4;
    #[link_name = "llvm.exp2.v8f64"]
    fn exp2_v8f64(x: f64x8) -> f64x8;
    #[link_name = "llvm.exp2.v16f64"]
    fn exp2_v16f64(x: f64x16) -> f64x16;

    #[link_name = "llvm.exp2.f32"]
    fn exp2_f32(x: f32) -> f32;
    #[link_name = "llvm.exp2.f64"]
    fn exp2_f64(x: f64) -> f64;
}

gen_unary_impl_table!(Exp2, exp2);

cfg_if! {
    if #[cfg(target_arch = "s390x")] {
        // FIXME: https://github.com/rust-lang-nursery/packed_simd/issues/14
        impl_unary!(f32x2[f32; 2]: exp2_f32);
        impl_unary!(f32x4[f32; 4]: exp2_f32);
        impl_unary!(f32x8[f32; 8]: exp2_f32);
        impl_unary!(f32x16[f32; 16]: exp2_f32);

        impl_unary!(f64x2[f64; 2]: exp2_f64);
        impl_unary!(f64x4[f64; 4]: exp2_f64);
        impl_unary!(f64x8[f64; 8]: exp2_f64);
    } else if #[cfg(all(target_arch = "x86_64", feature = "sleef-sys"))] {
        use sleef_sys::*;
        cfg_if! {
            if #[cfg(target_feature = "avx2")] {
                impl_unary!(f32x2[t => f32x4]: Sleef_expf4_u10avx2128);
                impl_unary!(f32x16[h => f32x8]: Sleef_expf8_u10avx2);
                impl_unary!(f64x8[h => f64x4]: Sleef_expd4_u10avx2);

                impl_unary!(f32x4: Sleef_expf4_u10avx2128);
                impl_unary!(f32x8: Sleef_expf8_u10avx2);
                impl_unary!(f64x2: Sleef_expd2_u10avx2128);
                impl_unary!(f64x4: Sleef_expd4_u10avx2);
            } else if #[cfg(target_feature = "avx")] {
                impl_unary!(f32x2[t => f32x4]: Sleef_expf4_u10sse4);
                impl_unary!(f32x16[h => f32x8]: Sleef_expf8_u10avx);
                impl_unary!(f64x8[h => f64x4]: Sleef_expd4_u10avx);

                impl_unary!(f32x4: Sleef_expf4_u10sse4);
                impl_unary!(f32x8: Sleef_expf8_u10avx);
                impl_unary!(f64x2: Sleef_expd2_u10sse4);
                impl_unary!(f64x4: Sleef_expd4_u10avx);
            } else if #[cfg(target_feature = "sse4.2")] {
                impl_unary!(f32x2[t => f32x4]: Sleef_expf4_u10sse4);
                impl_unary!(f32x16[q => f32x4]: Sleef_expf4_u10sse4);
                impl_unary!(f64x8[q => f64x2]: Sleef_expd2_u10sse4);

                impl_unary!(f32x4: Sleef_expf4_u10sse4);
                impl_unary!(f32x8[h => f32x4]: Sleef_expf4_u10sse4);
                impl_unary!(f64x2: Sleef_expd2_u10sse4);
                impl_unary!(f64x4[h => f64x2]: Sleef_expd2_u10sse4);
            } else if #[cfg(target_feature = "sse2")] {
                impl_unary!(f32x2[t => f32x4]: Sleef_expf4_u10sse2);
                impl_unary!(f32x16[q => f32x4]: Sleef_expf4_u10sse2);
                impl_unary!(f64x8[q => f64x2]: Sleef_expd2_u10sse2);

                impl_unary!(f32x4: Sleef_expf4_u10sse2);
                impl_unary!(f32x8[h => f32x4]: Sleef_expf4_u10sse2);
                impl_unary!(f64x2: Sleef_expd2_u10sse2);
                impl_unary!(f64x4[h => f64x2]: Sleef_expd2_u10sse2);
            } else {
                impl_unary!(f32x2[f32; 2]: exp2_f32);
                impl_unary!(f32x16: exp2_v16f32);
                impl_unary!(f64x8: exp2_v8f64);

                impl_unary!(f32x4: exp2_v4f32);
                impl_unary!(f32x8: exp2_v8f32);
                impl_unary!(f64x2: exp2_v2f64);
                impl_unary!(f64x4: exp2_v4f64);
            }
        }
    } else {
        impl_unary!(f32x2[f32; 2]: exp2_f32);
        impl_unary!(f32x4: exp2_v4f32);
        impl_unary!(f32x8: exp2_v8f32);
        impl_unary!(f32x16: exp2_v16f32);
        impl_unary!(f32x32: exp2_v32f32);

        impl_unary!(f64x2: exp2_v2f64);
        impl_unary!(f64x4: exp2_v4f64);
        impl_unary!(f64x8: exp2_v8f64);
        impl_unary!(f64x16: exp2_v16f64);
    }
}
