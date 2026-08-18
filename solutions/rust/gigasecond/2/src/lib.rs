use time::PrimitiveDateTime as DateTime;
use time::SignedDuration;

const GIGA_SECONDS: i64 = 1_000_000_000;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {

    let transformed_ =         start + SignedDuration::seconds(GIGA_SECONDS);
    transformed_
}
