use serde::Serialize;
use crate::reference_interval::ReferenceInterval;

/// Reference interval with a lower reference limit and upper reference limit.
#[derive(Debug, Serialize)]
pub struct ReferenceIntervalStruct<T: PartialOrd> {

    /// Lower Reference Limit (LRL)
    #[allow(dead_code)]
    pub lower: T,

    /// Upper Reference Limit (URL)
    #[allow(dead_code)]
    pub upper: T,

}

impl<T: PartialOrd> ReferenceInterval<T> for ReferenceIntervalStruct<T> {

    fn lower(&self) -> &T {
        &self.lower
    }

    fn upper(&self) -> &T {
        &self.upper
    }    
   
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_struct() {
        let x = ReferenceIntervalStruct{
            lower: 11,
            upper: 97,
        };
        assert_eq!(x.lower, 11);
        assert_eq!(x.upper, 97);
    }

    #[test]
    fn test_trait() {
        let x = ReferenceIntervalStruct{
            lower: 11,
            upper: 97,
        };
        assert_eq!(*x.lower(), 11);
        assert_eq!(*x.upper(), 97);
    }
}
