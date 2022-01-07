//! This crate provides simple types to parse and operate on a
//! Nagios range as described in the [Nagios Development Guidelines](https://nagios-plugins.org/doc/guidelines.html#THRESHOLDFORMAT).
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
//! Check if a `NagiosRange` codes for an inclusive or exclusive
//! range.
//! ```rust
//! use nagios_range::{NagiosRange, Error};
//!
//! fn main() -> Result<(), Error> {
//!     let range = NagiosRange::from("@0:10")?;
//!     assert!(range.is_inside());
//!     assert!(!range.is_outside());
//!
//!     Ok(())
//! }
//! ```
//!
//! Check if the start point of a `NagiosRange` (the lower bound)
//! matches negative infinity.
//! ```rust
//! use nagios_range::{NagiosRange, Error};
//!
//! fn main() -> Result<(), Error> {
//!     let range = NagiosRange::from("@~:10")?;
//!     assert!(range.start().is_neg_inf());
//!
//!     Ok(())
//! }
//! ```
//!
//! Extract the inner value of an end point.
//! ```rust
//! use nagios_range::{NagiosRange, Error};
//!
//! fn main() -> Result<(), Error> {
//!     let range = NagiosRange::from("@~:10")?;
//!     assert_eq!(range.end().inner(), Some(10));
//!
//!     Ok(())
//! }
//! ```
//!
//! # Planned future enhancements
//!
//! * The most important function when working with ranges
//! => `NagiosRange.contains()`.
pub mod error;
pub mod types;
pub use self::error::Error;
pub use self::types::End;
pub use self::types::NagiosRange;
pub use self::types::Start;
