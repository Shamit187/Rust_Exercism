use std::fmt;
#[derive(Debug, PartialEq)]
pub struct Clock{
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            minutes: (hours * 60 + minutes).rem_euclid(24 * 60),
        }

    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock {
            minutes: (self.minutes + minutes).rem_euclid(24 * 60),
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let hours = self.minutes / 60;
        let minutes = self.minutes % 60;
        write!(f, "{:02}:{:02}", hours, minutes)
    }
}

// impl fmt::Debug for Clock {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         f.debug_struct("Clock")
//             .field("minutes", &self.minutes)
//             .finish()
//     }
// }

// impl PartialEq for Clock {
//     fn eq(&self, other: &Self) -> bool {
//         self.minutes == other.minutes
//     }
// }

