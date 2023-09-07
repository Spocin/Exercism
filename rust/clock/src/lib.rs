#[derive(PartialEq, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        return Clock {
            hours,
            minutes,
        };
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        unimplemented!("Add {minutes} minutes to existing Clock time");
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
