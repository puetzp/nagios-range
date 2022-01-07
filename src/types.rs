use crate::error::Error;

/// A parsed Nagios range built from a literal string.
#[derive(Debug, PartialEq, Eq)]
pub enum NagiosRange {
    Inside(Start, End),
    Outside(Start, End),
}

impl NagiosRange {
    /// Creates a `NagiosRange` from a literal string.
    ///
    /// ```rust
    /// use nagios_range::NagiosRange;
    ///
    /// let range = NagiosRange::from("@-10:10");
    /// assert!(range.is_ok());
    /// ```
    pub fn from(input: &str) -> Result<Self, Error> {
        if input.is_empty() {
            return Err(Error::EmptyRange);
        }

        if input.starts_with('@') {
            let rem = &input[1..];
            let (start, end) = parse_range(rem)?;
            let inside_range = NagiosRange::Inside(start, end);
            Ok(inside_range)
        } else {
            let (start, end) = parse_range(input)?;
            let outside_range = NagiosRange::Outside(start, end);
            Ok(outside_range)
        }
    }

    /// Returns the `Start` component of a NagiosRange.
    ///
    /// ```rust
    /// use nagios_range::{NagiosRange, Start};
    ///
    /// let start = NagiosRange::from("@~:10").unwrap().start();
    /// assert_eq!(start, Start::NegInf);
    /// ```
    pub fn start(&self) -> Start {
        match self {
            NagiosRange::Inside(s, _) => *s,
            NagiosRange::Outside(s, _) => *s,
        }
    }

    /// Returns the `End` component of a NagiosRange.
    ///
    /// ```rust
    /// use nagios_range::{NagiosRange, End};
    ///
    /// let end = NagiosRange::from("@10").unwrap().end();
    /// assert_eq!(end, End::Value(10));
    /// ```
    pub fn end(&self) -> End {
        match self {
            NagiosRange::Inside(_, e) => *e,
            NagiosRange::Outside(_, e) => *e,
        }
    }

    /// Returns `true` when the `NagiosRange` is supposed to
    /// check for a metric inside of its range.
    ///
    /// ```rust
    /// use nagios_range::NagiosRange;
    ///
    /// let range = NagiosRange::from("@10").unwrap();
    /// assert!(range.is_inside());
    ///
    /// let range = NagiosRange::from("10:20").unwrap();
    /// assert!(!range.is_inside());
    /// ```
    pub fn is_inside(&self) -> bool {
        match self {
            NagiosRange::Inside(_, _) => true,
            NagiosRange::Outside(_, _) => false,
        }
    }

    /// Returns `true` when the `NagiosRange` is supposed to
    /// check for a metric outside of its range.
    ///
    /// ```rust
    /// use nagios_range::NagiosRange;
    ///
    /// let range = NagiosRange::from("~:10").unwrap();
    /// assert!(range.is_outside());
    ///
    /// let range = NagiosRange::from("@20").unwrap();
    /// assert!(!range.is_outside());
    /// ```
    pub fn is_outside(&self) -> bool {
        match self {
            NagiosRange::Inside(_, _) => false,
            NagiosRange::Outside(_, _) => true,
        }
    }

    /// Returns a tuple containing the `Start` and `End`
    /// points or `None` if the range is not of type
    /// `NagiosRange::Inside`.
    ///
    /// ```rust
    /// use nagios_range::NagiosRange;
    ///
    /// let range = NagiosRange::from("@10").unwrap().as_inside();
    /// assert!(range.is_some());
    ///
    /// let range = NagiosRange::from("~:20").unwrap().as_inside();
    /// assert!(range.is_none());
    /// ```
    pub fn as_inside(&self) -> Option<(Start, End)> {
        match self {
            NagiosRange::Inside(s, e) => Some((*s, *e)),
            NagiosRange::Outside(_, _) => None,
        }
    }

    /// Returns a tuple containing the `Start` and `End`
    /// points or `None` if the range is not of type
    /// `NagiosRange::Outside`.
    ///
    /// ```rust
    /// use nagios_range::NagiosRange;
    ///
    /// let range = NagiosRange::from("~:50").unwrap().as_outside();
    /// assert!(range.is_some());
    ///
    /// let range = NagiosRange::from("@50").unwrap().as_outside();
    /// assert!(range.is_none());
    /// ```
    pub fn as_outside(&self) -> Option<(Start, End)> {
        match self {
            NagiosRange::Inside(_, _) => None,
            NagiosRange::Outside(s, e) => Some((*s, *e)),
        }
    }
}

/// The start point of a Nagios range which may specify either
/// a number (positive or negative) or negative infinity.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Start {
    Value(i32),
    NegInf,
}

impl Start {
    /// Returns a `i32` or None if the start point
    /// specifies negative infinity.
    ///
    /// ```rust
    /// use nagios_range::NagiosRange;
    ///
    /// let start = NagiosRange::from("~:50").unwrap().start();
    /// assert_eq!(start.inner(), None);
    ///
    /// let start = NagiosRange::from("@-20:50").unwrap().start();
    /// assert_eq!(start.inner(), Some(-20));
    pub fn inner(&self) -> Option<i32> {
        match self {
            Self::Value(v) => Some(*v),
            Self::NegInf => None,
        }
    }

    /// Returns `true` if the start point specifies
    /// negative infinity.
    ///
    /// ```rust
    /// use nagios_range::NagiosRange;
    ///
    /// let start = NagiosRange::from("~:50").unwrap().start();
    /// assert!(start.is_neg_inf());
    pub fn is_neg_inf(&self) -> bool {
        match self {
            Self::Value(_) => false,
            Self::NegInf => true,
        }
    }
}

/// The end point of a Nagios range which may specify either
/// a number (positive or negative) or positive infinity.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum End {
    Value(i32),
    PosInf,
}

impl End {
    pub fn inner(&self) -> Option<i32> {
        match self {
            Self::Value(v) => Some(*v),
            Self::PosInf => None,
        }
    }

    pub fn is_pos_inf(&self) -> bool {
        match self {
            Self::Value(_) => false,
            Self::PosInf => true,
        }
    }
}

fn parse_range(range: &str) -> Result<(Start, End), Error> {
    match range.split_once(':') {
        Some(parts) => {
            let start = if parts.0 == "~" {
                Start::NegInf
            } else if parts.0.is_empty() {
                Start::Value(0)
            } else {
                let num = i32::from_str_radix(parts.0, 10).map_err(Error::ParseStartPoint)?;
                Start::Value(num)
            };

            let end = if parts.1.is_empty() {
                End::PosInf
            } else {
                let num = i32::from_str_radix(parts.1, 10).map_err(Error::ParseEndPoint)?;

                match start {
                    Start::Value(v) => {
                        if v > num {
                            return Err(Error::StartGreaterThanEnd);
                        }
                    }
                    _ => {}
                }
                End::Value(num)
            };

            Ok((start, end))
        }
        None => {
            let start = Start::Value(0);
            let num = i32::from_str_radix(range, 10).map_err(Error::ParseEndPoint)?;
            let end = End::Value(num);
            Ok((start, end))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Error;

    #[test]
    fn parse_example_range_1() {
        let result = NagiosRange::from("10");
        let expect = NagiosRange::Outside(Start::Value(0), End::Value(10));
        assert_eq!(result, Ok(expect));
    }

    #[test]
    fn parse_example_range_2() {
        let result = NagiosRange::from("10:");
        let expect = NagiosRange::Outside(Start::Value(10), End::PosInf);
        assert_eq!(result, Ok(expect));
    }

    #[test]
    fn parse_example_range_3() {
        let result = NagiosRange::from(":10");
        let expect = NagiosRange::Outside(Start::Value(0), End::Value(10));
        assert_eq!(result, Ok(expect));
    }

    #[test]
    fn parse_example_range_4() {
        let result = NagiosRange::from("~:10");
        let expect = NagiosRange::Outside(Start::NegInf, End::Value(10));
        assert_eq!(result, Ok(expect));
    }

    #[test]
    fn parse_example_range_5() {
        let result = NagiosRange::from("~:10");
        let expect = NagiosRange::Outside(Start::NegInf, End::Value(10));
        assert_eq!(result, Ok(expect));
    }

    #[test]
    fn parse_example_range_6() {
        let result = NagiosRange::from("10:20");
        let expect = NagiosRange::Outside(Start::Value(10), End::Value(20));
        assert_eq!(result, Ok(expect));
    }

    #[test]
    fn parse_example_range_7() {
        let result = NagiosRange::from("@10:20");
        let expect = NagiosRange::Inside(Start::Value(10), End::Value(20));
        assert_eq!(result, Ok(expect));
    }

    #[test]
    fn parse_example_range_8() {
        let result = NagiosRange::from("@-10:20");
        let expect = NagiosRange::Inside(Start::Value(-10), End::Value(20));
        assert_eq!(result, Ok(expect));
    }

    #[test]
    fn parse_example_range_9() {
        let result = NagiosRange::from("@-10:-20");
        let expect = Error::StartGreaterThanEnd;
        assert_eq!(result, Err(expect));
    }

    #[test]
    fn parse_example_range_10() {
        let result = NagiosRange::from("@20:-20");
        let expect = Error::StartGreaterThanEnd;
        assert_eq!(result, Err(expect));
    }

    #[test]
    fn parse_example_range_11() {
        let result = NagiosRange::from("");
        let expect = Error::EmptyRange;
        assert_eq!(result, Err(expect));
    }
}
