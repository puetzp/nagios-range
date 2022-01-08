use crate::error::Error;

/// A parsed Nagios range built from a literal string.
#[derive(Debug)]
pub struct NagiosRange {
    pub alert_type: AlertType,
    pub start: f64,
    pub end: f64,
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
            let inside_range = NagiosRange {
                alert_type: AlertType::Inside,
                start,
                end,
            };
            Ok(inside_range)
        } else {
            let (start, end) = parse_range(input)?;
            let outside_range = NagiosRange {
                alert_type: AlertType::Outside,
                start,
                end,
            };
            Ok(outside_range)
        }
    }

    pub fn alerts_inside_range(&self) -> bool {
        self.alert_type == AlertType::Inside
    }

    pub fn alerts_outside_range(&self) -> bool {
        self.alert_type == AlertType::Outside
    }
}

/// The start point of a Nagios range which may specify either
/// a number (positive or negative) or negative infinity.
#[derive(Debug, PartialEq)]
pub enum AlertType {
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
