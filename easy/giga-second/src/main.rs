use time::PrimitiveDateTime as DateTime;
use time::ext::NumericalDuration;
use time::{Date, Time, Month};

fn moment_after_giga_seconds(start: DateTime) -> DateTime {
    start + 1e9.seconds()
}

fn main() {
    let date = Date::from_calendar_date(2022, Month::February, 22);
    let time = Time::from_hms_milli(18, 00, 00, 00);
    let date_time = DateTime::new(date.unwrap(), time.unwrap());
    println!("{:#?}", moment_after_giga_seconds(date_time));
}
