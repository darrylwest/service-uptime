use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};
use std::time::Instant;

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct DaysHoursMinutesSeconds {
    seconds: u64,
    minutes: u64,
    hours: u64,
    days: u64,
}

impl Display for DaysHoursMinutesSeconds {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{} days, {:02}:{:02}:{:02} hms",
            self.days, self.hours, self.minutes, self.seconds
        )
    }
}

impl DaysHoursMinutesSeconds {
    /// create a new zero'd struct
    pub fn new() -> DaysHoursMinutesSeconds {
        DaysHoursMinutesSeconds {
            seconds: 0u64,
            minutes: 0u64,
            hours: 0u64,
            days: 0u64,
        }
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
pub struct Uptime {
    /// set on instantiation
    started_at: Instant,
}

impl Default for Uptime {
    fn default() -> Self {
        Self::new()
    }
}

impl Uptime {
    /// create a new
    pub fn new() -> Uptime {
        Uptime {
            started_at: Instant::now(),
        }
    }

    /// return the formatted days hours minutes and seconds
    ///
    /// Example:
    ///
    pub fn get_uptime(&self) -> DaysHoursMinutesSeconds {
        seconds_to_hms(self.get_uptime_seconds())
    }

    /// returns the instant that this service was created
    pub fn get_started_at(&self) -> Instant {
        self.started_at
    }

    /// return the elapsed seconds for this service
    pub fn get_uptime_seconds(&self) -> u64 {
        self.started_at.elapsed().as_secs()
    }
}

impl Display for Uptime {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.get_uptime())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_uptime() {
        let uptime = Uptime::new();
        assert_eq!(
            uptime.to_string(),
            DaysHoursMinutesSeconds::new().to_string()
        );

        assert_eq!(uptime.get_started_at().elapsed().as_secs(), 0);
    }

    #[test]
    fn default_uptime() {
        let uptime = Uptime::default();
        assert_eq!(uptime.get_started_at().elapsed().as_secs(), 0);
        println!("{}", uptime);
    }

    #[test]
    fn get_uptime_seconds() {
        let uptime = Uptime::new();
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

        for n in 0..100000u64 {
            let p = seconds_to_hms(n);

            assert!((0..=59).contains(&p.seconds));
            assert!((0..=59).contains(&p.minutes));
            assert!((0..=23).contains(&p.hours));
            assert!((0..=3).contains(&p.days));
        }

        // ai generated
        for ts in [86400u64, 90000u64, 54000u64, 3661] {
            assert_eq!(
                seconds_to_hms(ts),
                DaysHoursMinutesSeconds::from_seconds(ts)
            );
        }

        // Test conversion of 0 seconds
        assert_eq!(seconds_to_hms(0), DaysHoursMinutesSeconds::new());
    }
}
