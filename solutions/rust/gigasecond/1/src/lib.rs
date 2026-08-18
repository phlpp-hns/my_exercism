use time::PrimitiveDateTime as DateTime;

const GIGA_SECONDS: i64 = 1_000_000_000;
const ONE_MINUTE_IN_SECONDS: i64 = 60;
const ONE_HOUR_IN_SECONDS: i64 = ONE_MINUTE_IN_SECONDS * 60;
const ONE_DAY_IN_SECONDS: i64 = ONE_HOUR_IN_SECONDS * 24;
const ONE_YEAR_IN_SECONDS: i64 = ONE_DAY_IN_SECONDS * 365; // an this will be wrong every 4th year

const DAYS_IN_GIGA_SECONDS: i64 = GIGA_SECONDS / ONE_DAY_IN_SECONDS;
const SECONDS_LEFT: i64 = GIGA_SECONDS - (DAYS_IN_GIGA_SECONDS * ONE_DAY_IN_SECONDS);

// const DAYS_LEAP_YEAR: i64 = 366;
// const DAYS_YEAR: i64 = 365;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    use time::{Date, PrimitiveDateTime, Time};

    println!("We are heeeeeeere  {}", DAYS_IN_GIGA_SECONDS);
    println!("We are heeeeeeere  {}", SECONDS_LEFT);
    println!("We are heeeeeeere  {}", (DAYS_IN_GIGA_SECONDS * ONE_DAY_IN_SECONDS));
    println!("We are heeeeeeere  {}", SECONDS_LEFT+(DAYS_IN_GIGA_SECONDS * ONE_DAY_IN_SECONDS));
    
    let days_since_julian_day: i64 = start.to_julian_day() as i64;
    let days_since_julian_day_in_seconds = days_since_julian_day * ONE_DAY_IN_SECONDS;
    let remainder_in_seconds = 59;
    let after_in_seconds_after_julian_day = days_since_julian_day_in_seconds + remainder_in_seconds;
    let days_after_julian_day_after: i64 = days_since_julian_day_in_seconds / ONE_DAY_IN_SECONDS;
    let remainder_in_seconds = after_in_seconds_after_julian_day - (days_after_julian_day_after * ONE_DAY_IN_SECONDS);

    let date_after = Date::from_julian_day(days_after_julian_day_after as i32);

    //let year = 2026;
    //let month = 6;
    //let day = 23;
    let hour = 7;
    let minute = 20;
    let second = 10;
    let transformed_ = DateTime::new(
        date_after.unwrap(),
        Time::from_hms(hour, minute, second).unwrap(),
    );
    println!("We are heeeeeeere  {}", transformed_);


    transformed_
}
