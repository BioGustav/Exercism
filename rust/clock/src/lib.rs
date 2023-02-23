use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock { hours, minutes }.normalize()
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let m = self.minutes + minutes;

        Clock {
            hours: self.hours,
            minutes: m,
        }
        .normalize()
    }

    fn normalize(&self) -> Self {
        let mut c = *self;

        c.hours = (c.hours + c.minutes / 60) % 24 + 24;
        c.minutes %= 60;
        if c.minutes < 0 {
            c.hours -= 1;
            c.minutes += 60;
        }
        c.hours %= 24;

        c
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
