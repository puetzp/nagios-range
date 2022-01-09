# nagios-range

This is a very small Rust library that simply parses a Nagios range as defined in the [Nagios development guidelines](https://nagios-plugins.org/doc/guidelines.html#THRESHOLDFORMAT).

## Example

```rust
use nagios_range::{NagiosRange, Error};

fn main() -> Result<(), Error>{
    let range = NagiosRange::from("@~:10");
    assert!(range.is_ok());
    assert!(range?.checks_inside());
    assert!(range?.start_is_infinite());
    assert!(range?.check(5.0));
}
```