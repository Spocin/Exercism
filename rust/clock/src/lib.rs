use std::fmt;
use std::fmt::Formatter;

#[derive(PartialEq, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

pub const DAY_AS_MINUTES: i32 = 1440;

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        return Clock::process(hours, minutes);
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        return Clock::process(self.hours, self.minutes + minutes)
    }

    pub fn to_string(&self) -> String {
        let mut formatted: String = String::from("");

        if self.hours < 10 {
            formatted.push('0');
            formatted.push_str(&self.hours.to_string());
        } else {
            formatted.push_str(&self.hours.to_string());
        }

        formatted.push(':');

        if self.minutes < 10 {
            formatted.push('0');
            formatted.push_str(&self.minutes.to_string());
        } else {
            formatted.push_str(&self.minutes.to_string());
        }

        return formatted;
    }

    pub fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }

    fn process (hours: i32, minutes: i32) -> Self {
        let mut clock_as_minutes = (hours * 60) + minutes;

        /* Handle negative values */
        if clock_as_minutes < 0 {
            clock_as_minutes = clock_as_minutes % DAY_AS_MINUTES;
            clock_as_minutes = DAY_AS_MINUTES - (clock_as_minutes * -1);
        }

        /* Normalize to day */
        if clock_as_minutes >= DAY_AS_MINUTES {
            clock_as_minutes = clock_as_minutes % DAY_AS_MINUTES;
        }

        let hour = clock_as_minutes / 60;
        let minute = clock_as_minutes - (hour * 60);

        return Self {
            hours: hour,
            minutes: minute,
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut hour_formatted = String::from("00");
        let mut minute_formatted = String::from("00");

        if self.hours >= 10 { hour_formatted = self.hours.to_string() }
        if self.minutes >= 10 { minute_formatted = self.minutes.to_string() }

        write!(f, "{}:{}", hour_formatted, minute_formatted)
    }
}
