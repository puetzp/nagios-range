use crate::error::Error;

#[derive(Debug, PartialEq, Eq)]
pub enum NagiosRange {
    Inside(Start, End),
    Outside(Start, End),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Start {
    Value(i32),
    NegInf,
}

#[derive(Debug, PartialEq, Eq)]
pub enum End {
    Value(i32),
    PosInf,
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

impl NagiosRange {
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
