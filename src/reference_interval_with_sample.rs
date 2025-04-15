use num_traits::{Float, NumCast};
use rand::Rng;
use rand_distr::{Distribution, Normal, StandardNormal};
use crate::reference_interval::ReferenceInterval;
use crate::mean::mean;


/// ReferenceIntervalWithSample decorates a ReferenceInterval with a random sample
/// function, suitable for generating plausible ReferenceInterval normal values.
pub struct ReferenceIntervalWithSample<T, F> 
where
    T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + Copy,
    F: Float,
    StandardNormal: Distribution<F>
{    
    // The reference interval, which may use an integer or float or real etc.
    #[allow(dead_code)]
    pub reference_interval: ReferenceInterval<T>,

    // Memoize as (lower + upper) / 2 as a heuristic.
    #[allow(dead_code)]
    pub mean: F,

    // Memoize as (mean - lower) / 2 as a heuristic because of standard interval of 95%.
    #[allow(dead_code)]
    pub standard_deviation: F,

    // Memoize as (upper - lower) / 2 as a heuristic.
    #[allow(dead_code)]
    pub normal: Normal<F>,
}

impl<T, F> ReferenceIntervalWithSample<T, F>
where
    T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + Copy + Into<F>,
    F: Float + NumCast,
    StandardNormal: Distribution<F>
{
    /// The new function is a builder that calculates the resource interval
    /// mean and standard deviation, then creates a normal distribution that is
    /// ready for the random sample function.
    #[allow(dead_code)]
    pub fn new(reference_interval: ReferenceInterval<T>) -> Self {
        let lower: F = reference_interval.lower.into();
        let upper: F = reference_interval.upper.into();
        let mean: F = mean(lower, upper);
        let standard_deviation = (mean - lower) / F::from(2.0).unwrap();
        let normal = Normal::new(mean, standard_deviation).expect("Normal::new");
        Self { 
            reference_interval: reference_interval,
            mean: mean,
            standard_deviation: standard_deviation,
            normal: normal,
        }
    }
}

/// Implement the typical rand sample function.
impl<T, F> Distribution<F> for ReferenceIntervalWithSample<T, F> 
where
    T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + Copy + Into<F>,
    F: Float + NumCast,
    StandardNormal: Distribution<F>
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> F {
        self.normal.sample(rng)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn subject() -> ReferenceIntervalWithSample<i32, f64> {
        let reference_interval = ReferenceInterval{
            units: "units".to_string(),
            lower: 11,
            upper: 97,
        };
        ReferenceIntervalWithSample::new(reference_interval)
    }

    #[test]
    fn test_new() {
        let x = subject();
        assert_eq!(x.mean as i32, 54);
        assert_eq!(x.standard_deviation as i32, 21);
    }

    #[test]
    fn test_sample() {
        let subject = subject();
        let mut rng = rand::rng();
        let x: f64 = subject.normal.sample(&mut rng); 
        assert!(x >= 0.0);
    }

}
