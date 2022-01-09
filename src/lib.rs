//! This crate provides simple types to parse and operate on a
//! Nagios range as described in the [Nagios Development Guidelines](https://nagios-plugins.org/doc/guidelines.html#THRESHOLDFORMAT).
//!   
//! The main type [NagiosRange] behaves similar to a [std::ops::RangeInclusive]
//! but also provides methods that implement the extended behaviour of
//! a Nagios range, i.e. checking if a value is inside or outside the
//! range which is basically the same as the [std::ops::RangeInclusive::contains()]
//! method but extends it with the inverse behaviour.
//!
//! # Examples
//!
//! Create a `NagiosRange` from a literal string.
//! ```rust
//! use nagios_range::{NagiosRange, Error};
//!
//! fn main() -> Result<(), Error> {
//!     let range = NagiosRange::from("@0:10");
//!     assert!(range.is_ok());
//!
//!     Ok(())
//! }
//! ```
//!
//! Look if a `NagiosRange` checks for values inside
//! or outside of its range.
//! ```rust
//! use nagios_range::{NagiosRange, Error};
//!
//! fn main() -> Result<(), Error> {
//!     // This is an "inside" range.
//!     let range = NagiosRange::from("@0:10")?;
//!     assert!(range.checks_inside());
//!     assert!(!range.checks_outside());
//!
//!     Ok(())
//! }
//! ```
//!
//! Look if the start point of a `NagiosRange` (the lower bound)
//! is negatively infinite.
//! ```rust
//! use nagios_range::{NagiosRange, Error};
//!
//! fn main() -> Result<(), Error> {
//!     let range = NagiosRange::from("@~:10")?;
//!     assert!(range.start_is_infinite());
//!
//!     Ok(())
//! }
//! ```
//!
//! Probably the most important function when working with Nagios check plugins:
//! Check if a value is "contained" by the range in respect to its [CheckType].
//! ```rust
//! use nagios_range::{NagiosRange, Error};
//!
//! fn main() -> Result<(), Error> {
//!     let range = NagiosRange::from("@~:10")?;
//!     assert!(range.check(5.0));
//!
//!     let range = NagiosRange::from("20")?;
//!     assert!(range.check(30.0));
//!
//!     Ok(())
//! }
//! ```
pub mod error;
pub mod types;
pub use self::error::Error;
pub use self::types::CheckType;
pub use self::types::NagiosRange;
