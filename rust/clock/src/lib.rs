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
        return self.hours.to_string() + ":" + &self.minutes.to_string();
    }

    pub fn eq(&self, other: &Self) -> bool {
        return self.hours == other.hours;
    }
}
