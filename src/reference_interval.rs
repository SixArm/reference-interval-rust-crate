/// Reference interval with a units label, lower reference limit, upper reference limite.
#[derive(serde::Serialize)]
pub struct ReferenceInterval<T> {

    /// Units label
    #[allow(dead_code)]
    pub units: String,

    /// Lower Reference Limit (LRL)
    #[allow(dead_code)]
    pub lower: T,

    /// Upper Reference Limit URL)
    #[allow(dead_code)]
    pub upper: T,

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let x = ReferenceInterval{
            units: "units".into(),
            lower: 11,
            upper: 97,
        };
        assert_eq!(x.units, "units");
        assert_eq!(x.lower, 11);
        assert_eq!(x.upper, 97);
    }
}
