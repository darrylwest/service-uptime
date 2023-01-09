#![doc = include_str!("../README.md")]

use std::time::Instant;

/// the current app version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DaysHoursMinutesSeconds {
    seconds: u64,
    minutes: u64,
    hours: u64,
    days: u64,
}

impl DaysHoursMinutesSeconds {
    /// create a new zero'd struct
    pub fn new() -> DaysHoursMinutesSeconds {
        let zero = 0_u64;
        DaysHoursMinutesSeconds {
            seconds: zero,
            minutes: zero,
            hours: zero,
            days: zero,
        }
    }

    /// return the string in the standard format
    pub fn to_string(&self) -> String {
        format!(
            "{} days, {:02}:{:02}:{:02} hms",
            self.days, self.hours, self.minutes, self.seconds
        )
    }

    /// create a new struct with the elapsed seconds set to any number
    pub fn from_seconds(seconds: u64) -> DaysHoursMinutesSeconds {
        seconds_to_hms(seconds)
    }
}

/// convert seconds to days, hours, minutes and seconds
///
/// Example:
///
pub fn seconds_to_hms(seconds: u64) -> DaysHoursMinutesSeconds {
    let days = seconds / 86400;
    let hours = (seconds % 86400) / 3600;
    let minutes = (seconds % 3600) / 60;
    let seconds = seconds % 60;

    DaysHoursMinutesSeconds {
        seconds,
        minutes,
        hours,
        days,
    }
}

/// state
#[derive(Debug, Clone)]
pub struct State {
    /// set on instantiation
    started_at: Instant,
    /// optional way to track error counts for the specified service
    error_count: u64,
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

impl State {
    /// create a new
    pub fn new() -> State {
        State {
            started_at: Instant::now(),
            error_count: 0_u64,
        }
    }

    /// return the formatted days hours minutes and seconds
    ///
    /// Example:
    ///
    pub fn get_uptime(&self) -> String {
        seconds_to_hms(self.get_uptime_seconds()).to_string()
    }

    /// return the elapsed seconds for this service
    pub fn get_uptime_seconds(&self) -> u64 {
        self.started_at.elapsed().as_secs()
    }

    /// returns the current error count
    pub fn get_error_count(&self) -> u64 {
        self.error_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_uptime() {
        let uptime = State::new();
        assert_eq!(uptime.get_uptime(), DaysHoursMinutesSeconds::new().to_string());
    }

    #[test]
    fn get_uptime_seconds() {
        let uptime = State::new();
        assert_eq!(uptime.get_uptime_seconds(), 0);
    }

    #[test]
    fn convert_seconds_to_hms() {
        let ts: u64 = 59;
        let p = seconds_to_hms(ts);
        assert_eq!(p.days, 0);
        assert_eq!(p.hours, 0);
        assert_eq!(p.minutes, 0);
        assert_eq!(p.seconds, 59);

        let ts: u64 = 60;
        let p = seconds_to_hms(ts);
        assert_eq!(p.days, 0);
        assert_eq!(p.hours, 0);
        assert_eq!(p.minutes, 1);
        assert_eq!(p.seconds, 0);

        let ts: u64 = 61;
        let p = seconds_to_hms(ts);
        assert_eq!(p.days, 0);
        assert_eq!(p.hours, 0);
        assert_eq!(p.minutes, 1);
        assert_eq!(p.seconds, 1);

        let ts: u64 = 60 * 60;
        let p = seconds_to_hms(ts);
        assert_eq!(p.days, 0);
        assert_eq!(p.hours, 1);
        assert_eq!(p.minutes, 0);
        assert_eq!(p.seconds, 0);

        // ai generated
        assert_eq!(
            seconds_to_hms(86400),
            DaysHoursMinutesSeconds::from_seconds(86400)
        );
        // assert_eq!(seconds_to_hms(90000), (1, 1, 0, 0));
        // assert_eq!(seconds_to_hms(54000), (0, 15, 0, 0));
        // assert_eq!(seconds_to_hms(3661), (0, 1, 1, 1));

        // Test conversion of 0 seconds
        assert_eq!(seconds_to_hms(0), DaysHoursMinutesSeconds::new());

        for n in 0..100000 {
            let p = seconds_to_hms(n);

            assert!((0..=59).contains(&p.seconds));
            assert!((0..=59).contains(&p.minutes));
            assert!((0..=23).contains(&p.hours));
            assert!((0..=3).contains(&p.days));
        }
    }
}
