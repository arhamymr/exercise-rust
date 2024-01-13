use time::PrimitiveDateTime as DateTime;
use time::Duration;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let gigasecond = Duration::seconds(1_000_000_000);
    
    let result_datetime = start + gigasecond;

    return result_datetime;
}