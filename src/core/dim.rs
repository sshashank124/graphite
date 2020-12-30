use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum Dim {
    X,
    Y,
    Z,
}
pub use Dim::*;

pub const XYZ: A3<Dim> = A3(X, Y, Z);


#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn x()
    { assert_eq!(serde_json::from_str::<Dim>("\"X\"").unwrap(), X); }
    #[test] fn y()
    { assert_eq!(serde_json::from_str::<Dim>("\"Y\"").unwrap(), Y); }
    #[test] fn z()
    { assert_eq!(serde_json::from_str::<Dim>("\"Z\"").unwrap(), Z); }
}
