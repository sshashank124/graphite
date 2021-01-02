use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature="serde-derive", derive(Deserialize, Serialize))]
pub enum Dim {
    X,
    Y,
    Z,
}
pub use Dim::*;

pub const XYZ: A3<Dim> = A3(X, Y, Z);


#[cfg(feature="serde-derive")]
#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn deser_x()
    { assert_eq!(serde_json::from_str::<Dim>("\"X\"").unwrap(), X); }

    #[test] fn deser_y()
    { assert_eq!(serde_json::from_str::<Dim>("\"Y\"").unwrap(), Y); }

    #[test] fn deser_z()
    { assert_eq!(serde_json::from_str::<Dim>("\"Z\"").unwrap(), Z); }
}
