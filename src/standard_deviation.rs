use num_traits::{Float, NumCast};
use crate::mean::mean;

// Calculate the standard deviation of a standard reference interval.
#[allow(dead_code)]
pub fn standard_deviation<F>(lower: F, upper: F) -> F 
where
    F: Float + NumCast,
{
    (mean(lower, upper) - lower) / F::from(2.0).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let x = standard_deviation(11.0, 97.0);
        assert_eq!(x as i32, 21);
    }

}
