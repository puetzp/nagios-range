use crate::error::Error;
use std::fmt;

/// A parsed Nagios range built from a literal string.
/// A Nagios range works similar to [std::ops::RangeInclusive]
/// in that it is bounded by the lower and upper bounds inclusively
/// (`start..=end`).
/// However it differs slightly in how it is used. It contains some
/// logic to determine if an alert should be raised when it is
/// compared to some value (either when inside or outside of the range).
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct NagiosRange {
    pub(crate) check_type: CheckType,
    pub(crate) start: f64,
    pub(crate) end: f64,
}

impl NagiosRange {
    /// Creates a [NagiosRange] from a literal string.
    ///
    /// ```rust
    /// use nagios_range::NagiosRange;
    ///
    /// fn main() -> Result<(), nagios_range::Error> {
    ///     let range = NagiosRange::from("@-10:10");
    ///     assert!(range.is_ok());
    ///     Ok(())
    /// }
    /// ```
    pub fn from(input: &str) -> Result<Self, Error> {
        if input.is_empty() {
            return Err(Error::EmptyRange);
        }

        let (check_type, input) = match input.strip_prefix('@') {
            Some(i) => (CheckType::Inside, i),
            None => (CheckType::Outside, input),
        };

        let (start, end) = parse_range(input)?;

        let range = NagiosRange {
            check_type,
            start,
            end,
        };

        Ok(range)
    }

    /// Creates a [NagiosRange] from a [CheckType], lower and
    /// (inclusive) upper bounds.
    ///
    /// ```rust
    /// use nagios_range::{NagiosRange, CheckType};
    ///
    /// fn main() -> Result<(), nagios_range::Error> {
    ///     let range = NagiosRange::new(CheckType::Inside, f64::NEG_INFINITY, 20.0);
    ///     assert!(range.is_ok());
    ///     assert_eq!(range?, NagiosRange::from("@~:20")?);
    ///     Ok(())
    /// }
    /// ```
    pub fn new(check_type: CheckType, start: f64, end: f64) -> Result<Self, Error> {
        if start > end {
            return Err(Error::StartGreaterThanEnd);
        }

        Ok(NagiosRange {
            check_type,
            start,
            end,
        })
    }

    /// Returns the lower bound of the range.
    ///
    /// ```rust
    /// use nagios_range::NagiosRange;
    ///
    /// fn main() -> Result<(), nagios_range::Error> {
    ///     let range = NagiosRange::from("@-10:10")?;
    ///     assert_eq!(range.start(), &-10.0);
    ///     Ok(())
    /// }
    /// ```
    pub fn start(&self) -> &f64 {
        &self.start
    }

    /// Returns the upper bound of the range.
    ///
    /// ```rust
    /// use nagios_range::NagiosRange;
    ///
    /// fn main() -> Result<(), nagios_range::Error> {
    ///     let range = NagiosRange::from("@-10:10")?;
    ///     assert_eq!(range.end(), &10.0);
    ///     Ok(())
    /// }
    /// ```
    pub fn end(&self) -> &f64 {
        &self.end
    }

    /// Returns `true` if the lower bound is negative infinity.
    /// This is just a convenience method that calls [f64::is_infinite()]
    /// on `start`.
    ///
    /// ```rust
    /// use nagios_range::NagiosRange;
    ///
    /// fn main() -> Result<(), nagios_range::Error> {
    ///     let range = NagiosRange::from("@~:10")?;
    ///     assert!(range.start_is_infinite());
    ///     assert_eq!(range.start_is_infinite(), range.start().is_infinite());
    ///     Ok(())
    /// }
    /// ```
    pub fn start_is_infinite(&self) -> bool {
        self.start.is_infinite()
    }

    /// Returns `true` if the upper bound is positive infinity.
    /// This is just a convenience method that calls [f64::is_infinite()]
    /// on `end`.
    ///
    /// ```rust
    /// use nagios_range::NagiosRange;
    ///
    /// fn main() -> Result<(), nagios_range::Error> {
    ///     let range = NagiosRange::from("@10:")?;
    ///     assert!(range.end_is_infinite());
    ///     assert_eq!(range.end_is_infinite(), range.end().is_infinite());
    ///     Ok(())
    /// }
    /// ```
    pub fn end_is_infinite(&self) -> bool {
        self.end.is_infinite()
    }

    /// Returns `true` if `item` is contained in the range irregardless
    /// of the type of the NagiosRange. So this behaves just like
    /// [std::ops::RangeInclusive::contains()].
    ///
    /// ```rust
    /// use nagios_range::NagiosRange;
    ///
    /// fn main() -> Result<(), nagios_range::Error> {
    ///     let range = NagiosRange::from("@~:10")?;
    ///     assert!(range.contains(-50.0));
    ///     assert!(range.contains(10.0));
    ///     assert!(!range.contains(20.0));
    ///     Ok(())
    /// }
    /// ```
    pub fn contains(&self, item: f64) -> bool {
        item >= self.start && item <= self.end
    }

    /// Returns `true` if a value is either inside or outside
    /// of the range depending on the type of Nagios range.
    ///
    /// ```rust
    /// use nagios_range::NagiosRange;
    ///
    /// fn main() -> Result<(), nagios_range::Error> {
    ///     // When it is an "inside" range...
    ///     let range = NagiosRange::from("@10:20")?;
    ///     assert!(range.check(15.0));
    ///
    ///     // ...inverted behaviour when it is an "outside" range...
    ///     let range = NagiosRange::from("10:20")?;
    ///     assert!(range.check(30.0));
    ///     Ok(())
    /// }
    /// ```
    pub fn check(&self, item: f64) -> bool {
        match self.check_type {
            CheckType::Inside => item >= self.start && item <= self.end,
            CheckType::Outside => item < self.start || item > self.end,
        }
    }

    /// Returns `true` if [NagiosRange::check()] checks
    /// if a value lies inside the range (`start <= item <= end`).
    ///
    /// ```rust
    /// use nagios_range::NagiosRange;
    ///
    /// fn main() -> Result<(), nagios_range::Error> {
    ///     let range = NagiosRange::from("@~:10")?;
    ///     assert!(range.checks_inside());
    ///     Ok(())
    /// }
    /// ```
    pub fn checks_inside(&self) -> bool {
        self.check_type == CheckType::Inside
    }

    /// Returns `true` if [NagiosRange::check()] checks
    /// if a value lies outside the range (`start > item > end`).
    ///
    /// ```rust
    /// use nagios_range::NagiosRange;
    ///
    /// fn main() -> Result<(), nagios_range::Error> {
    ///     let range = NagiosRange::from("10:50")?;
    ///     assert!(range.checks_outside());
    ///     Ok(())
    /// }
    /// ```
    pub fn checks_outside(&self) -> bool {
        self.check_type == CheckType::Outside
    }

    /// Destructures the [NagiosRange] into a [CheckType] and
    /// the lower and (inclusive) upper bounds.
    ///
    /// ```rust
    /// use nagios_range::{NagiosRange, CheckType};
    ///
    /// fn main() -> Result<(), nagios_range::Error> {
    ///     let range = NagiosRange::from("10:50")?;
    ///     let (check_type, start, end) = range.into_inner();
    ///     assert_eq!(check_type, CheckType::Outside);
    ///     assert_eq!(start, 10.0);
    ///
    ///     let new_range = NagiosRange::new(check_type, start, end);
    ///     assert!(new_range.is_ok());
    ///     Ok(())
    /// }
    /// ```
    pub fn into_inner(self) -> (CheckType, f64, f64) {
        (self.check_type, self.start, self.end)
    }
}

impl fmt::Display for NagiosRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let start = if self.start.is_infinite() {
            "~".to_string()
        } else {
            self.start.to_string()
        };
        let end = if self.end.is_infinite() {
            "~".to_string()
        } else {
            self.end.to_string()
        };
        match self.check_type {
            CheckType::Inside => write!(f, "@{}:{}", start, end),
            CheckType::Outside => write!(f, "{}:{}", start, end),
        }
    }
}

/// This enum indicates if [NagiosRange::check()] should
/// check if a value lies inside or outside of the range.
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum CheckType {
    Inside,
    Outside,
}

fn parse_range(range: &str) -> Result<(f64, f64), Error> {
    match range.split_once(':') {
        Some(parts) => {
            let start = if parts.0 == "~" {
                f64::NEG_INFINITY
            } else if parts.0.is_empty() {
                0.0
            } else {
                parts.0.parse().map_err(Error::ParseStartPoint)?
            };

            let end = if parts.1.is_empty() {
                f64::INFINITY
            } else {
                let num: f64 = parts.1.parse().map_err(Error::ParseEndPoint)?;

                if start > num {
                    return Err(Error::StartGreaterThanEnd);
                }
                num
            };

            Ok((start, end))
        }
        None => {
            let start = 0.0;
            let end: f64 = range.parse().map_err(Error::ParseEndPoint)?;
            Ok((start, end))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::error::Error;
    use crate::{CheckType, NagiosRange};

    #[test]
    fn parse_example_range_1() {
        let result = NagiosRange::from("10");
        let expect = NagiosRange {
            check_type: CheckType::Outside,
            start: 0.0,
            end: 10.0,
        };
        assert_eq!(result, Ok(expect));
    }

    #[test]
    fn parse_example_range_2() {
        let result = NagiosRange::from("10:");
        let expect = NagiosRange {
            check_type: CheckType::Outside,
            start: 10.0,
            end: f64::INFINITY,
        };
        assert_eq!(result, Ok(expect));
    }

    #[test]
    fn parse_example_range_3() {
        let result = NagiosRange::from(":10");
        let expect = NagiosRange {
            check_type: CheckType::Outside,
            start: 0.0,
            end: 10.0,
        };
        assert_eq!(result, Ok(expect));
    }

    #[test]
    fn parse_example_range_4() {
        let result = NagiosRange::from("~:10");
        let expect = NagiosRange {
            check_type: CheckType::Outside,
            start: f64::NEG_INFINITY,
            end: 10.0,
        };
        assert_eq!(result, Ok(expect));
    }

    #[test]
    fn parse_example_range_5() {
        let result = NagiosRange::from("10:20");
        let expect = NagiosRange {
            check_type: CheckType::Outside,
            start: 10.0,
            end: 20.0,
        };
        assert_eq!(result, Ok(expect));
    }

    #[test]
    fn parse_example_range_6() {
        let result = NagiosRange::from("@10:20");
        let expect = NagiosRange {
            check_type: CheckType::Inside,
            start: 10.0,
            end: 20.0,
        };
        assert_eq!(result, Ok(expect));
    }

    #[test]
    fn parse_example_range_7() {
        let result = NagiosRange::from("@-10:20");
        let expect = NagiosRange {
            check_type: CheckType::Inside,
            start: -10.0,
            end: 20.0,
        };
        assert_eq!(result, Ok(expect));
    }

    #[test]
    fn parse_example_range_8() {
        let result = NagiosRange::from("@-10:-20");
        let expect = Error::StartGreaterThanEnd;
        assert_eq!(result, Err(expect));
    }

    #[test]
    fn parse_example_range_9() {
        let result = NagiosRange::from("@20:-20");
        let expect = Error::StartGreaterThanEnd;
        assert_eq!(result, Err(expect));
    }

    #[test]
    fn parse_example_range_10() {
        let result = NagiosRange::from("");
        let expect = Error::EmptyRange;
        assert_eq!(result, Err(expect));
    }

    #[test]
    fn display_range_1() {
        let range = NagiosRange::from("@10:20").unwrap();
        let result = "@10:20".to_string();
        assert_eq!(range.to_string(), result);
    }

    #[test]
    fn display_range_2() {
        let range = NagiosRange::from("@10:").unwrap();
        let result = "@10:~".to_string();
        assert_eq!(range.to_string(), result);
    }

    #[test]
    fn display_range_3() {
        let range = NagiosRange::from("10").unwrap();
        let result = "0:10".to_string();
        assert_eq!(range.to_string(), result);
    }

    #[test]
    fn display_range_4() {
        let range = NagiosRange::from("~:10").unwrap();
        let result = "~:10".to_string();
        assert_eq!(range.to_string(), result);
    }
}
