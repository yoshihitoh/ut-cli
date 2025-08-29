use chrono::{DateTime, Datelike, Duration, TimeZone};

pub trait ApplyDateTime<Tz: TimeZone> {
    fn apply_datetime(&self, dt: DateTime<Tz>) -> Option<DateTime<Tz>>;
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TimeDelta {
    values: DeltaValues,
}

impl TimeDelta {
    #[allow(dead_code)]
    pub fn new(
        years: i32,
        months: i32,
        days: i32,
        hours: i32,
        minutes: i32,
        seconds: i32,
        microseconds: i32,
    ) -> Self {
        // microseconds
        let sign = sign_of(microseconds);
        let (d, m) = div_mod(microseconds * sign, 1_000_000);
        let seconds = seconds + d * sign;
        let microseconds = m * sign;

        // seconds
        let sign = sign_of(seconds);
        let (d, m) = div_mod(seconds * sign, 60);
        let minutes = minutes + d * sign;
        let seconds = m * sign;

        // minutes
        let sign = sign_of(minutes);
        let (d, m) = div_mod(minutes * sign, 60);
        let hours = hours + d * sign;
        let minutes = m * sign;

        // hours
        let sign = sign_of(hours);
        let (d, m) = div_mod(hours * sign, 24);
        let days = days + d * sign;
        let hours = m * sign;

        // NOTE: cannot convert days to months.

        // months
        let sign = sign_of(months);
        let (d, m) = div_mod(months * sign, 12);
        let years = years + d * sign;
        let months = m * sign;

        TimeDelta {
            values: DeltaValues {
                years,
                months,
                days,
                hours,
                minutes,
                seconds,
                microseconds,
            },
        }
    }

    pub fn years(&self) -> i32 {
        self.values.years
    }

    pub fn months(&self) -> i32 {
        self.values.months
    }

    pub fn days(&self) -> i32 {
        self.values.days
    }

    pub fn hours(&self) -> i32 {
        self.values.hours
    }

    pub fn minutes(&self) -> i32 {
        self.values.minutes
    }

    pub fn seconds(&self) -> i32 {
        self.values.seconds
    }

    pub fn microseconds(&self) -> i32 {
        self.values.microseconds
    }
}

impl<Tz: TimeZone> ApplyDateTime<Tz> for TimeDelta {
    fn apply_datetime(&self, target: DateTime<Tz>) -> Option<DateTime<Tz>> {
        let duration = Duration::microseconds(i64::from(self.microseconds()))
            + Duration::seconds(i64::from(self.seconds()))
            + Duration::minutes(i64::from(self.minutes()))
            + Duration::hours(i64::from(self.hours()))
            + Duration::days(i64::from(self.days()));

        let duration_applied: DateTime<Tz> = target + duration;

        let delta_months = self.years() * 12 + self.months();
        let sum_months = duration_applied.month() as i32 + delta_months;

        let delta_years = if sum_months > 0 {
            (sum_months - 1) / 12
        } else {
            (sum_months / 12) - 1
        };
        let result_year = duration_applied.year() + delta_years;

        let result_month = if sum_months > 0 {
            ((sum_months - 1) % 12) + 1
        } else {
            (sum_months % 12) + 12
        } as u32;

        duration_applied
            .with_year(result_year)
            .and_then(|dt| dt.with_month(result_month))
    }
}

pub struct TimeDeltaBuilder {
    values: DeltaValues,
}

impl Default for TimeDeltaBuilder {
    fn default() -> Self {
        TimeDeltaBuilder {
            values: DeltaValues {
                years: 0,
                months: 0,
                days: 0,
                hours: 0,
                minutes: 0,
                seconds: 0,
                microseconds: 0,
            },
        }
    }
}

impl TimeDeltaBuilder {
    pub fn years(mut self, value: i32) -> Self {
        self.values.years = value;
        self
    }

    pub fn add_years(self, value: i32) -> Self {
        let y = self.values.years + value;
        self.years(y)
    }

    pub fn months(mut self, value: i32) -> Self {
        self.values.months = value;
        self
    }

    pub fn add_months(self, value: i32) -> Self {
        let m = self.values.months + value;
        self.months(m)
    }

    pub fn days(mut self, d: i32) -> Self {
        self.values.days = d;
        self
    }

    pub fn add_days(self, value: i32) -> Self {
        let d = self.values.days + value;
        self.days(d)
    }

    pub fn hours(mut self, h: i32) -> Self {
        self.values.hours = h;
        self
    }

    pub fn add_hours(self, value: i32) -> Self {
        let h = self.values.hours + value;
        self.hours(h)
    }

    pub fn minutes(mut self, m: i32) -> Self {
        self.values.minutes = m;
        self
    }

    pub fn add_minutes(self, value: i32) -> Self {
        let m = self.values.minutes + value;
        self.minutes(m)
    }

    pub fn seconds(mut self, s: i32) -> Self {
        self.values.seconds = s;
        self
    }

    pub fn add_seconds(self, value: i32) -> Self {
        let s = self.values.seconds + value;
        self.seconds(s)
    }

    #[allow(dead_code)]
    pub fn milliseconds(self, value: i32) -> Self {
        let s = value / 1000;
        let us = (value % 1000) * 1000;
        self.seconds(s).microseconds(us)
    }

    pub fn add_milliseconds(self, value: i32) -> Self {
        let s = value / 1000;
        let us = (value % 1000) * 1000;

        self.add_seconds(s).add_microseconds(us)
    }

    pub fn microseconds(mut self, value: i32) -> Self {
        self.values.microseconds = value;
        self
    }

    pub fn add_microseconds(self, value: i32) -> Self {
        let us = self.values.microseconds + value;
        self.microseconds(us)
    }

    pub fn build(self) -> TimeDelta {
        TimeDelta {
            values: self.values,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct DeltaValues {
    years: i32,
    months: i32,
    days: i32,
    hours: i32,
    minutes: i32,
    seconds: i32,
    microseconds: i32,
}

#[allow(dead_code)]
fn sign_of(x: i32) -> i32 {
    if x > 0 {
        1
    } else {
        -1
    }
}

#[allow(dead_code)]
fn div_mod(x: i32, y: i32) -> (i32, i32) {
    (x / y, x % y)
}

#[cfg(test)]
mod time_delta_tests {
    use chrono::offset::TimeZone;
    use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};

    use super::{ApplyDateTime, TimeDelta, TimeDeltaBuilder};

    fn naive_date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    fn naive_time(h: u32, m: u32, s: u32, us: u32) -> NaiveTime {
        NaiveTime::from_hms_micro_opt(h, m, s, us).unwrap()
    }

    fn utc_datetime(naive_date: NaiveDate, naive_time: NaiveTime) -> DateTime<Utc> {
        Utc.from_utc_datetime(&NaiveDateTime::new(naive_date, naive_time))
    }

    #[derive(Copy, Clone)]
    struct UtcBuilder;
    impl UtcBuilder {
        fn ymd(self, y: i32, m: u32, d: u32) -> UtcBuilderWithDate {
            UtcBuilderWithDate {
                date: naive_date(y, m, d),
            }
        }
    }

    #[derive(Copy, Clone)]
    struct UtcBuilderWithDate {
        date: NaiveDate,
    }

    impl UtcBuilderWithDate {
        fn and_hms(self, h: u32, m: u32, s: u32) -> DateTime<Utc> {
            utc_datetime(self.date, naive_time(h, m, s, 0))
        }

        fn and_hms_micro(self, h: u32, m: u32, s: u32, us: u32) -> DateTime<Utc> {
            utc_datetime(self.date, naive_time(h, m, s, us))
        }
    }

    #[test]
    fn time_delta_new_basics() {
        let delta = TimeDelta::new(0, 0, 0, 0, 0, 0, 0);
        assert_eq!(delta.years(), 0);
        assert_eq!(delta.months(), 0);
        assert_eq!(delta.days(), 0);
        assert_eq!(delta.hours(), 0);
        assert_eq!(delta.minutes(), 0);
        assert_eq!(delta.seconds(), 0);
        assert_eq!(delta.microseconds(), 0);

        let delta = TimeDelta::new(1234, 11, 365, 23, 59, 59, 999_999);
        assert_eq!(delta.years(), 1234);
        assert_eq!(delta.months(), 11);
        assert_eq!(delta.days(), 365);
        assert_eq!(delta.hours(), 23);
        assert_eq!(delta.minutes(), 59);
        assert_eq!(delta.seconds(), 59);
        assert_eq!(delta.microseconds(), 999_999);

        let delta = TimeDelta::new(1234, 11, 365, 23, 59, 59, 1_000_000);
        assert_eq!(delta.years(), 1234);
        assert_eq!(delta.months(), 11);
        assert_eq!(delta.days(), 366);
        assert_eq!(delta.hours(), 0);
        assert_eq!(delta.minutes(), 0);
        assert_eq!(delta.seconds(), 0);
        assert_eq!(delta.microseconds(), 0);

        let delta = TimeDelta::new(-1234, -11, -365, -23, -59, -59, -999_999);
        assert_eq!(delta.years(), -1234);
        assert_eq!(delta.months(), -11);
        assert_eq!(delta.days(), -365);
        assert_eq!(delta.hours(), -23);
        assert_eq!(delta.minutes(), -59);
        assert_eq!(delta.seconds(), -59);
        assert_eq!(delta.microseconds(), -999_999);

        let delta = TimeDelta::new(-1234, -11, -365, -23, -59, -59, -1_000_000);
        assert_eq!(delta.years(), -1234);
        assert_eq!(delta.months(), -11);
        assert_eq!(delta.days(), -366);
        assert_eq!(delta.hours(), 0);
        assert_eq!(delta.minutes(), 0);
        assert_eq!(delta.seconds(), 0);
        assert_eq!(delta.microseconds(), 0);
    }

    #[test]
    fn time_delta_new_microseconds() {
        // plus
        let delta = TimeDelta::new(0, 0, 0, 0, 0, 0, 999_999);
        assert_eq!(delta.seconds(), 0);
        assert_eq!(delta.microseconds(), 999_999);

        let delta = TimeDelta::new(0, 0, 0, 0, 0, 0, 1_000_000);
        assert_eq!(delta.seconds(), 1);
        assert_eq!(delta.microseconds(), 0);

        // minus
        let delta = TimeDelta::new(0, 0, 0, 0, 0, 0, -999_999);
        assert_eq!(delta.seconds(), 0);
        assert_eq!(delta.microseconds(), -999_999);

        let delta = TimeDelta::new(0, 0, 0, 0, 0, 0, -1_000_000);
        assert_eq!(delta.seconds(), -1);
        assert_eq!(delta.microseconds(), 0);
    }

    #[test]
    fn time_delta_new_seconds() {
        // plus
        let delta = TimeDelta::new(0, 0, 0, 0, 0, 59, 0);
        assert_eq!(delta.minutes(), 0);
        assert_eq!(delta.seconds(), 59);

        let delta = TimeDelta::new(0, 0, 0, 0, 0, 60, 0);
        assert_eq!(delta.minutes(), 1);
        assert_eq!(delta.seconds(), 0);

        // minus
        let delta = TimeDelta::new(0, 0, 0, 0, 0, -59, 0);
        assert_eq!(delta.minutes(), 0);
        assert_eq!(delta.seconds(), -59);

        let delta = TimeDelta::new(0, 0, 0, 0, 0, -60, 0);
        assert_eq!(delta.minutes(), -1);
        assert_eq!(delta.seconds(), 0);
    }

    #[test]
    fn time_delta_new_minutes() {
        // minutes
        let delta = TimeDelta::new(0, 0, 0, 0, 59, 0, 0);
        assert_eq!(delta.hours(), 0);
        assert_eq!(delta.minutes(), 59);

        let delta = TimeDelta::new(0, 0, 0, 1, 0, 0, 0);
        assert_eq!(delta.hours(), 1);
        assert_eq!(delta.minutes(), 0);

        // minutes
        let delta = TimeDelta::new(0, 0, 0, 0, -59, 0, 0);
        assert_eq!(delta.hours(), 0);
        assert_eq!(delta.minutes(), -59);

        let delta = TimeDelta::new(0, 0, 0, 0, -60, 0, 0);
        assert_eq!(delta.hours(), -1);
        assert_eq!(delta.minutes(), 0);
    }

    #[test]
    fn time_delta_new_hours() {
        // plus
        let delta = TimeDelta::new(0, 0, 0, 23, 0, 0, 0);
        assert_eq!(delta.days(), 0);
        assert_eq!(delta.hours(), 23);

        let delta = TimeDelta::new(0, 0, 1, 0, 0, 0, 0);
        assert_eq!(delta.days(), 1);
        assert_eq!(delta.hours(), 0);

        // minus
        let delta = TimeDelta::new(0, 0, 0, -23, 0, 0, 0);
        assert_eq!(delta.days(), 0);
        assert_eq!(delta.hours(), -23);

        let delta = TimeDelta::new(0, 0, 0, -24, 0, 0, 0);
        assert_eq!(delta.days(), -1);
        assert_eq!(delta.hours(), 0);
    }

    #[test]
    fn time_delta_new_days() {
        // plus
        let delta = TimeDelta::new(0, 0, 364, 0, 0, 0, 0);
        assert_eq!(delta.months(), 0);
        assert_eq!(delta.days(), 364);

        let delta = TimeDelta::new(0, 0, 365, 0, 0, 0, 0);
        assert_eq!(delta.months(), 0); // NOTE: cannot calculate months from days.
        assert_eq!(delta.days(), 365);

        // minus
        let delta = TimeDelta::new(0, 0, -364, 0, 0, 0, 0);
        assert_eq!(delta.months(), 0);
        assert_eq!(delta.days(), -364);

        let delta = TimeDelta::new(0, 0, -365, 0, 0, 0, 0);
        assert_eq!(delta.months(), 0); // NOTE: cannot calculate months from days.
        assert_eq!(delta.days(), -365);
    }

    #[test]
    fn time_delta_new_months() {
        // plus
        let delta = TimeDelta::new(0, 11, 0, 0, 0, 0, 0);
        assert_eq!(delta.years(), 0);
        assert_eq!(delta.months(), 11);

        let delta = TimeDelta::new(0, 12, 0, 0, 0, 0, 0);
        assert_eq!(delta.years(), 1);
        assert_eq!(delta.months(), 0);

        // minus
        let delta = TimeDelta::new(0, -11, 0, 0, 0, 0, 0);
        assert_eq!(delta.years(), 0);
        assert_eq!(delta.months(), -11);

        let delta = TimeDelta::new(0, -12, 0, 0, 0, 0, 0);
        assert_eq!(delta.years(), -1);
        assert_eq!(delta.months(), 0);
    }

    #[test]
    fn time_delta_new_years() {
        // plus
        let delta = TimeDelta::new(1, 0, 0, 0, 0, 0, 0);
        assert_eq!(delta.years(), 1);

        // minus
        let delta = TimeDelta::new(-1, 0, 0, 0, 0, 0, 0);
        assert_eq!(delta.years(), -1);
    }

    #[test]
    fn time_delta_apply_microseconds() {
        let date = UtcBuilder.ymd(1, 1, 1);

        // plus
        assert_eq!(
            TimeDeltaBuilder::default()
                .microseconds(111_222)
                .build()
                .apply_datetime(date.and_hms_micro(0, 0, 0, 12_234)),
            Some(date.and_hms_micro(0, 0, 0, 123_456))
        );

        assert_eq!(
            TimeDeltaBuilder::default()
                .microseconds(999_999)
                .build()
                .apply_datetime(date.and_hms_micro(0, 0, 0, 1)),
            Some(date.and_hms_micro(0, 0, 1, 0))
        );

        // minus
        assert_eq!(
            TimeDeltaBuilder::default()
                .microseconds(-1)
                .build()
                .apply_datetime(date.and_hms_micro(0, 0, 0, 1)),
            Some(date.and_hms_micro(0, 0, 0, 0))
        );

        assert_eq!(
            TimeDeltaBuilder::default()
                .microseconds(-1)
                .build()
                .apply_datetime(date.and_hms_micro(0, 0, 0, 0)),
            Some(UtcBuilder.ymd(0, 12, 31).and_hms_micro(23, 59, 59, 999_999))
        );

        let date = UtcBuilder.ymd(0, 1, 1);
        assert_eq!(
            TimeDeltaBuilder::default()
                .microseconds(-1)
                .build()
                .apply_datetime(date.and_hms_micro(0, 0, 0, 0)),
            Some(
                UtcBuilder
                    .ymd(-1, 12, 31)
                    .and_hms_micro(23, 59, 59, 999_999)
            )
        );
    }

    #[test]
    fn time_delta_apply_seconds() {
        let date = UtcBuilder.ymd(2019, 6, 12);

        // plus
        assert_eq!(
            TimeDeltaBuilder::default()
                .seconds(1)
                .build()
                .apply_datetime(date.and_hms(0, 0, 58)),
            Some(date.and_hms(0, 0, 59))
        );

        assert_eq!(
            TimeDeltaBuilder::default()
                .seconds(2)
                .build()
                .apply_datetime(date.and_hms(0, 0, 58)),
            Some(date.and_hms(0, 1, 0))
        );

        // minus
        assert_eq!(
            TimeDeltaBuilder::default()
                .seconds(-1)
                .build()
                .apply_datetime(date.and_hms(0, 0, 1)),
            Some(date.and_hms(0, 0, 0))
        );

        assert_eq!(
            TimeDeltaBuilder::default()
                .seconds(-1)
                .build()
                .apply_datetime(date.and_hms(0, 0, 0)),
            Some(UtcBuilder.ymd(2019, 6, 11).and_hms(23, 59, 59))
        );
    }

    #[test]
    fn time_delta_apply_minutes() {
        let date = UtcBuilder.ymd(2019, 6, 12);

        // plus
        assert_eq!(
            TimeDeltaBuilder::default()
                .minutes(1)
                .build()
                .apply_datetime(date.and_hms(0, 58, 0)),
            Some(date.and_hms(0, 59, 0))
        );

        assert_eq!(
            TimeDeltaBuilder::default()
                .minutes(1)
                .build()
                .apply_datetime(date.and_hms(0, 59, 0)),
            Some(date.and_hms(1, 0, 0))
        );

        // minus
        assert_eq!(
            TimeDeltaBuilder::default()
                .minutes(-1)
                .build()
                .apply_datetime(date.and_hms(0, 1, 0)),
            Some(date.and_hms(0, 0, 0))
        );

        assert_eq!(
            TimeDeltaBuilder::default()
                .minutes(-2)
                .build()
                .apply_datetime(date.and_hms(0, 1, 0)),
            Some(UtcBuilder.ymd(2019, 6, 11).and_hms(23, 59, 0))
        );
    }

    #[test]
    fn time_delta_apply_hours() {
        let date = UtcBuilder.ymd(2019, 6, 12);

        // plus
        assert_eq!(
            TimeDeltaBuilder::default()
                .hours(1)
                .build()
                .apply_datetime(date.and_hms(22, 0, 0)),
            Some(date.and_hms(23, 0, 0))
        );

        assert_eq!(
            TimeDeltaBuilder::default()
                .hours(2)
                .build()
                .apply_datetime(date.and_hms(22, 0, 0)),
            Some(UtcBuilder.ymd(2019, 6, 13).and_hms(0, 0, 0))
        );

        // minus
        assert_eq!(
            TimeDeltaBuilder::default()
                .hours(-1)
                .build()
                .apply_datetime(date.and_hms(1, 0, 0)),
            Some(date.and_hms(0, 0, 0))
        );

        assert_eq!(
            TimeDeltaBuilder::default()
                .hours(-2)
                .build()
                .apply_datetime(date.and_hms(1, 0, 0)),
            Some(UtcBuilder.ymd(2019, 6, 11).and_hms(23, 0, 0))
        );
    }

    #[test]
    fn time_delta_apply_days() {
        // plus
        assert_eq!(
            TimeDeltaBuilder::default()
                .days(28)
                .build()
                .apply_datetime(UtcBuilder.ymd(2019, 6, 2).and_hms(0, 0, 0)),
            Some(UtcBuilder.ymd(2019, 6, 30).and_hms(0, 0, 0))
        );

        assert_eq!(
            TimeDeltaBuilder::default()
                .days(29)
                .build()
                .apply_datetime(UtcBuilder.ymd(2019, 6, 2).and_hms(0, 0, 0)),
            Some(UtcBuilder.ymd(2019, 7, 1).and_hms(0, 0, 0))
        );

        assert_eq!(
            TimeDeltaBuilder::default()
                .days(28)
                .build()
                .apply_datetime(UtcBuilder.ymd(2019, 2, 1).and_hms(0, 0, 0)),
            Some(UtcBuilder.ymd(2019, 3, 1).and_hms(0, 0, 0))
        );

        // minus
        assert_eq!(
            TimeDeltaBuilder::default()
                .days(-1)
                .build()
                .apply_datetime(UtcBuilder.ymd(2019, 6, 2).and_hms(0, 0, 0)),
            Some(UtcBuilder.ymd(2019, 6, 1).and_hms(0, 0, 0))
        );

        assert_eq!(
            TimeDeltaBuilder::default()
                .days(-2)
                .build()
                .apply_datetime(UtcBuilder.ymd(2019, 6, 2).and_hms(0, 0, 0)),
            Some(UtcBuilder.ymd(2019, 5, 31).and_hms(0, 0, 0))
        );

        assert_eq!(
            TimeDeltaBuilder::default()
                .days(-1)
                .build()
                .apply_datetime(UtcBuilder.ymd(2019, 3, 1).and_hms(0, 0, 0)),
            Some(UtcBuilder.ymd(2019, 2, 28).and_hms(0, 0, 0))
        );
    }

    #[test]
    fn time_delta_apply_months() {
        // plus
        assert_eq!(
            TimeDeltaBuilder::default()
                .months(1)
                .build()
                .apply_datetime(UtcBuilder.ymd(2019, 11, 1).and_hms(0, 0, 0)),
            Some(UtcBuilder.ymd(2019, 12, 1).and_hms(0, 0, 0))
        );

        assert_eq!(
            TimeDeltaBuilder::default()
                .months(2)
                .build()
                .apply_datetime(UtcBuilder.ymd(2019, 11, 1).and_hms(0, 0, 0)),
            Some(UtcBuilder.ymd(2020, 1, 1).and_hms(0, 0, 0))
        );

        assert_eq!(
            TimeDeltaBuilder::default()
                .months(2)
                .build()
                .apply_datetime(UtcBuilder.ymd(2019, 10, 31).and_hms(0, 0, 0)),
            Some(UtcBuilder.ymd(2019, 12, 31).and_hms(0, 0, 0))
        );

        assert_eq!(
            TimeDeltaBuilder::default()
                .months(1)
                .build()
                .apply_datetime(UtcBuilder.ymd(2019, 10, 31).and_hms(0, 0, 0)),
            None
        );

        // minus
        assert_eq!(
            TimeDeltaBuilder::default()
                .months(-1)
                .build()
                .apply_datetime(UtcBuilder.ymd(2019, 2, 1).and_hms(0, 0, 0)),
            Some(UtcBuilder.ymd(2019, 1, 1).and_hms(0, 0, 0))
        );

        assert_eq!(
            TimeDeltaBuilder::default()
                .months(-2)
                .build()
                .apply_datetime(UtcBuilder.ymd(2019, 2, 1).and_hms(0, 0, 0)),
            Some(UtcBuilder.ymd(2018, 12, 1).and_hms(0, 0, 0))
        );

        assert_eq!(
            TimeDeltaBuilder::default()
                .months(-1)
                .build()
                .apply_datetime(UtcBuilder.ymd(2019, 1, 31).and_hms(0, 0, 0)),
            Some(UtcBuilder.ymd(2018, 12, 31).and_hms(0, 0, 0))
        );

        assert_eq!(
            TimeDeltaBuilder::default()
                .months(-2)
                .build()
                .apply_datetime(UtcBuilder.ymd(2019, 1, 31).and_hms(0, 0, 0)),
            None
        );
    }

    #[test]
    fn time_delta_apply_years() {
        // plus
        assert_eq!(
            TimeDeltaBuilder::default()
                .years(1)
                .build()
                .apply_datetime(UtcBuilder.ymd(2019, 1, 1).and_hms(0, 0, 0)),
            Some(UtcBuilder.ymd(2020, 1, 1).and_hms(0, 0, 0))
        );

        assert_eq!(
            TimeDeltaBuilder::default()
                .years(1)
                .build()
                .apply_datetime(UtcBuilder.ymd(2020, 2, 29).and_hms(0, 0, 0)),
            None
        );

        // minus
        assert_eq!(
            TimeDeltaBuilder::default()
                .years(-1)
                .build()
                .apply_datetime(UtcBuilder.ymd(2019, 1, 1).and_hms(0, 0, 0)),
            Some(UtcBuilder.ymd(2018, 1, 1).and_hms(0, 0, 0))
        );

        assert_eq!(
            TimeDeltaBuilder::default()
                .years(-1)
                .build()
                .apply_datetime(UtcBuilder.ymd(2020, 2, 29).and_hms(0, 0, 0)),
            None
        );
    }
}

#[cfg(test)]
mod builder_tests {
    use super::{TimeDelta, TimeDeltaBuilder};

    #[test]
    fn time_delta_builder() {
        assert_eq!(
            TimeDeltaBuilder::default()
                .years(2019)
                .months(6)
                .days(10)
                .hours(20)
                .minutes(12)
                .seconds(34)
                .microseconds(56)
                .build(),
            TimeDelta::new(2019, 6, 10, 20, 12, 34, 56)
        );
    }
}
