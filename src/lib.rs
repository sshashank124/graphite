#![feature(array_map)]
#![feature(const_fn)]

#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(clippy::cast_possible_truncation,
         clippy::cast_possible_wrap,
         clippy::cast_precision_loss,
         clippy::cast_sign_loss,
         clippy::inline_always,
         clippy::module_name_repetitions,
         clippy::must_use_candidate,
         clippy::suspicious_arithmetic_impl,
         clippy::use_self,
         clippy::wildcard_imports)]

mod core;
mod geometry;

pub use crate::core::*;
pub use crate::geometry::*;
