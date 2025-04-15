use num_traits::{Float, NumCast};

// Calculate the mean of two numbers, with special handling for overflow cases.
#[allow(dead_code)]
pub fn mean<F>(a: F, b: F) -> F 
where
    F: Float + NumCast,
{
    if (b > F::zero()) && (a > (F::max_value() - b)) {
      // Will overflow, so use difference method.
      // Both a and b > 0.
      // We want difference also > 0 so rounding works correctly.
      if a >= b {
        b + (a - b) / F::from(2.0).unwrap()
      } else {
        a + (b - a) / F::from(2.0).unwrap()
      }
    } else {
        if (b < F::zero()) && (a < (F::min_value() - b)) {
            // Will overflow, so use difference method.
            // Both a and b < 0.
            // We want difference also < 0 so rounding works correctly.
            if a <= b {
                b + (a - b) / F::from(2.0).unwrap()
            } else {
                a + (b - a) / F::from(2.0).unwrap()
            }
        } else {
            // Will not overflow.
            (a + b) / F::from(2.0).unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let x = mean(11.0, 97.0);
        assert_eq!(x as i32, 54);
    }

}
