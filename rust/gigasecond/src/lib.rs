use time::PrimitiveDateTime as DateTime;
use time::ext::NumericalDuration;

const GIGA_SECOND: i64 = 1e9 as i64;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    return start.checked_add(GIGA_SECOND.seconds()).unwrap();
}
