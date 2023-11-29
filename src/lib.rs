// Feature 'avx2' currently requires unstable 'stdsimd'
#![cfg_attr(all(target_arch = "x86_64"), feature(stdsimd))]

#[rustfmt::skip]
mod gxhash;
mod hasher;

pub use crate::gxhash::*;
pub use crate::hasher::*;