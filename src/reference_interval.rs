use serde::Serialize;
use std::fmt::Display;

/// Reference interval with a units label, lower reference limit, upper reference limite.
#[derive(Debug, Serialize)]
pub struct ReferenceInterval<T: PartialOrd + Display> {

    /// Lower Reference Limit (LRL)
    #[allow(dead_code)]
    pub lower: T,

    /// Upper Reference Limit (URL)
    #[allow(dead_code)]
    pub upper: T,

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let x = ReferenceInterval{
            lower: 11,
            upper: 97,
        };
        assert_eq!(x.lower, 11);
        assert_eq!(x.upper, 97);
    }
}
