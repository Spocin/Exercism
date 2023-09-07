#[derive(PartialEq, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

pub const DAY_AS_MINUTES: i32 = 1440;

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        return Clock {
            hours,
            minutes,
        };
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut clock_as_minutes = (self.hours * 60) + self.minutes + minutes;

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
        return self.hours == other.hours;
    }
}
