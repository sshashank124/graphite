mod array2;
mod array3;
mod convert;
mod dim;
mod interpolate;
mod num;
mod ops;
mod scalar;

#[cfg(feature="serde-derive")]
pub(crate) use serde::{Deserialize, Serialize};

pub use array2::*;
pub use array3::*;
pub use convert::*;
pub use dim::*;
pub use interpolate::*;
pub use num::*;
pub use scalar::*;

pub type I = i32;
pub type U = u32;

#[cfg(not(feature="f64"))]
pub type F = f32;
#[cfg(feature="f64")]
pub type F = f64;

#[cfg(not(feature="f64"))]
use std::f32 as fmod;
#[cfg(feature="f64")]
use std::f64 as fmod;
