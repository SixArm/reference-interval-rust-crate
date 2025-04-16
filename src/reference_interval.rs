/// Reference interval with a lower reference limit and upper reference limit.
pub trait ReferenceInterval<T: PartialOrd> {
    /// Lower Reference Limit (LRL)
    fn lower(&self) -> &T;

    /// Upper Reference Limit (LRL)
    fn upper(&self) -> &T;

    /// Is a value inside the reference interval?
    fn includes(&self, value: T) -> bool {
        value >= *self.lower() && value <= *self.upper()
    }

    /// Is a value outside the reference interval?
    fn excludes(&self, value: T) -> bool {
        value < *self.lower() || value > *self.upper()
    }

    /// Is a value outside the lower reference interval?
    fn excludes_lower(&self, value: T) -> bool {
        value < *self.lower()
    }

    /// Is a value outside the upper reference interval?
    fn excludes_upper(&self, value: T) -> bool {
        value > *self.upper()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    struct ReferenceIntervalStruct<T: PartialOrd> {
        pub lower: T,
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

    #[test]
    fn test_includes() {
        let x = ReferenceIntervalStruct {
            lower: 11,
            upper: 97,
        };
        // Test lower boundary
        assert_eq!(x.includes(10), false);
        assert_eq!(x.includes(11), true);
        assert_eq!(x.includes(12), true);
        // Test upper boundary
        assert_eq!(x.includes(96), true);
        assert_eq!(x.includes(97), true);
        assert_eq!(x.includes(98), false);
    }

    #[test]
    fn test_excludes() {
        let x = ReferenceIntervalStruct {
            lower: 11,
            upper: 97,
        };
        // Test lower boundary
        assert_eq!(x.excludes(10), true);
        assert_eq!(x.excludes(11), false);
        assert_eq!(x.excludes(12), false);
        // Test upper boundary
        assert_eq!(x.excludes(96), false);
        assert_eq!(x.excludes(97), false);
        assert_eq!(x.excludes(98), true);
    }

    #[test]
    fn test_excludes_lower() {
        let x = ReferenceIntervalStruct {
            lower: 11,
            upper: 97,
        };
        // Test lower boundary
        assert_eq!(x.excludes_lower(10), true);
        assert_eq!(x.excludes_lower(11), false);
        assert_eq!(x.excludes_lower(12), false);
        // Test upper boundary
        assert_eq!(x.excludes_lower(96), false);
        assert_eq!(x.excludes_lower(97), false);
        assert_eq!(x.excludes_lower(98), false);
    }

    #[test]
    fn test_excludes_upper() {
        let x = ReferenceIntervalStruct {
            lower: 11,
            upper: 97,
        };
        // Test lower boundary
        assert_eq!(x.excludes_upper(10), false);
        assert_eq!(x.excludes_upper(11), false);
        assert_eq!(x.excludes_upper(12), false);
        // Test upper boundary
        assert_eq!(x.excludes_upper(96), false);
        assert_eq!(x.excludes_upper(97), false);
        assert_eq!(x.excludes_upper(98), true);
    }

}
