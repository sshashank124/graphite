mod array2;
mod array3;
mod color;
mod dim;
mod interpolate;
mod num;
mod ops;
mod scalar;

pub use array2::*;
pub use array3::*;
pub use color::*;
pub use dim::*;
pub use interpolate::*;
pub use num::*;
pub use scalar::*;

pub type I = i32;

#[cfg(not(feature="f64"))]
pub type F = f32;
#[cfg(feature="f64")]
pub type F = f64;

#[cfg(not(feature="f64"))]
use std::f32 as fmod;
#[cfg(feature="f64")]
use std::f64 as fmod;
