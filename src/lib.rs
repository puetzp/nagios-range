//! ```rust
//! use nagios_range::{NagiosRange, Error};
//!
//! fn main() -> Result<(), Error> {
//!     let range = NagiosRange::from("@0:10");
//!     assert!(range.is_ok());
//!
//!     let start = range?.start().inner();
//!     assert_eq!(start, Some(0));
//!
//!     Ok(())
//! }
//! ```
pub mod error;
pub mod types;
pub use self::error::Error;
pub use self::types::End;
pub use self::types::NagiosRange;
pub use self::types::Start;
