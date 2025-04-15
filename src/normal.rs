use num_traits::{Float, NumCast};
use crate::mean::mean;
use crate::standard_deviation::standard_deviation;
use rand_distr::{Distribution, Normal, StandardNormal};

// Calculate the mean of two numbers, with special handling for overflow cases.
#[allow(dead_code)]
pub fn normal<F>(lower: F, upper: F) -> Normal<F> 
where
    F: Float + NumCast,
    StandardNormal: Distribution<F>
{
    Normal::new(mean(lower, upper), standard_deviation(lower, upper)).expect("Normal::new")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut rng = rand::rng();
        let normal = normal(11.0, 97.0);
        let sample = normal.sample(&mut rng);
        assert!(sample > 0.0);
    }

}
