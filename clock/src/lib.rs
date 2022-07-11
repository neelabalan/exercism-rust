use std::fmt;
use std::fmt::Debug;

#[derive(Debug, Eq, PartialEq)]
pub struct Clock {
    hours: i32,
    minute: i32,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{hour:0>2}:{minute:0>2}",
            hour = self.hours,
            minute = self.minute
        )
    }
}

fn div_rem(x: i32, y: i32) -> (i32, i32) {
    let quot = x / y;
    let rem = x % y;
    (quot, rem)
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let (_, hour) = div_rem(hours, 24);
        let (mut additional_hours, mut minute) = div_rem(minutes, 60);
        if minute.is_negative() {
            minute = 60 + minute;
            additional_hours = additional_hours - 1;
        }
        let (_, mut final_hour) = div_rem(hour + additional_hours, 24);
        if final_hour.is_negative() {
            final_hour = 24 + final_hour
        }

        return Clock {
            hours: final_hour,
            minute: minute,
        };
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        return Clock::new(self.hours, self.minute + minutes);
    }
}
