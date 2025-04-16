//! # Reference interval Rust crate
//!
//! This create provides several ways to help with a reference interval:
//!
//! * `ReferenceInterval` - A reference interval trait, with lower value and
//!   upper value, and helper functions such as includes and excludes.
//!
//! * `ReferenceIntervalStrict` - A reference interval structure, with lower
//!   value and upper value.
//! 
//! * `mean` - Calculate the mean of a reference interval.
//!
//! * `standard_deviation` - Calculate the standard deviation of a reference
//!   interval.
//! 
//! * `normal` - Calculate the normal distribution of a reference interval.
//! 
//! Wikipedia has an introduction to the concept of a reference interval, which
//! we've excerpted and edited below.
//!
//! * <https://en.wikipedia.org/wiki/Reference_interval>
//!
//! ## Reference interval
//!
//! A reference interval is the set of values that are deemed normal for a
//! measurement.
//!
//! A reference interval is a basis for comparison and interpretation, typically
//! comparing and interpreting one measurement result with a general group of
//! measurement results.
//!
//! Some important reference intervals are in medicine and health care. Examples
//! are a reference interval for hemoglobin blood testing, a reference interval
//! for cortisol hormone testing, a reference interval for histamine allergy
//! testing, and a reference interval for acidity urine testing.
//!
//! ## Standard reference interval
//!
//! A standard reference interval is defined as the interval between which 95%
//! of values of a reference population fall into, in such a way that 2.5% of
//! the time a value will be less than the lower limit of this interval, and
//! 2.5% of the time it will be larger than the upper limit of this interval,
//! whatever the distribution of these values.
//!
//! ## General reference interval
//!
//! A general reference interval is defined as a standard reference interval
//! based on what is most prevalent in a reference group taken from the general
//! population, and without any known aspect that directly affects the interval
//! being established.
//!
//! ## Subgroup reference interval
//!
//! A subgroup reference interval is defined as a standard reference interval
//! based on a subgroup with a known aspect that directly affects the interval
//! being established.
//!
//! ## Within Reference Interval (WRI)
//!
//! A value inside a reference interval is termed within reference interval
//! (WRI).
//!
//! A value inside a reference interval is not necessarily good, and not
//! necessarily normal in any sense other than statistically.
//!
//! ## Beyond reference interval (BRI)
//!
//! A value outside a reference interval is termed beyond reference interval
//! (BRI).
//!
//! A value outside a reference interval is not necessarily bad, and not
//! necessarily abnormal in any sense other than statistically.
//!
//! ## Lower Reference Limit (LRL) & Upper Reference Limit (URL)
//!
//! The limits are called:
//!
//! * The lower reference limit (LRL) a.k.a. lower limit of normal (LLN)
//!
//! * The upper reference limit (URL) a.k.a. upper limit of normal (ULN)
//!
//! For mass communications such as publishing, style sheets sometimes prefer
//! the word "reference" over the word "normal", to prevent the nontechnical
//! senses of normal from being conflated with the statistical sense.
//!
//! ## Binary classification
//!
//! For binary classification, a cutoff or threshold is a limit used mainly
//! between within reference interval (WRI) and beyond reference interval (BRI).
//!
//! ## Reference interval vs. reference range
//!
//! A reference interval is a.k.a. reference range. Because mathematical
//! statistics defines the term "range" as describing the interval between the
//! smallest and largest values, many, including the International Federation of
//! Clinical Chemistry prefer to use the expression reference interval rather
//! than reference range.
//!
pub mod reference_interval;
pub mod reference_interval_struct;
pub mod mean;
pub mod standard_deviation;
pub mod normal;

#[allow(unused_imports)] pub use reference_interval::ReferenceInterval;
#[allow(unused_imports)] pub use reference_interval_struct::ReferenceIntervalStruct;
#[allow(unused_imports)] pub use mean::mean;
#[allow(unused_imports)] pub use standard_deviation::standard_deviation;
#[allow(unused_imports)] pub use normal::normal;
