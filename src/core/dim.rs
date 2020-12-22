use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dim {
    X,
    Y,
    Z,
}
pub use Dim::*;

pub const XYZ: A3<Dim> = A3(X, Y, Z);
