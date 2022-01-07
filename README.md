# nagios-range

This is a very small Rust library that simply parses a Nagios range as defined in the [Nagios development guidelines](https://nagios-plugins.org/doc/guidelines.html#THRESHOLDFORMAT).

## Example

```rust
use nagios_range::NagiosRange;

fn main() {
    let range = NagiosRange::from("@~:10");
    assert!(range.is_ok());
    assert!(range.is_inside());

    let start = range.start();
    assert!(start.is_neg_inf());

    let end = range.end().inner();
    assert_eq!(end, Some(10));
}
```