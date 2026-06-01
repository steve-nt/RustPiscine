use colored::*;
use std::{fmt, time::Duration};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Position {
    Top,
    Bottom,
    Center,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Notification {
    pub size: u32,
    pub color: (u8, u8, u8),
    pub position: Position,
    pub content: String,
}

#[derive(Clone, Copy)]
pub enum Event<'a> {
    Remainder(&'a str),
    Registration(Duration),
    Appointment(&'a str),
    Holiday,
}

impl fmt::Display for Notification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (r, g, b) = self.color;
        let colored_content = self.content.truecolor(r, g, b);
        write!(f, "({:?}, {}, {})", self.position, self.size, colored_content)
    }
}

impl Event<'_> {
    pub fn notify(self) -> Notification {
        match self {
            Event::Remainder(msg) => Notification {
                size: 50,
                color: (50, 50, 50),
                position: Position::Bottom,
                content: msg.to_string(),
            },
            Event::Registration(duration) => {
                let total = duration.as_secs();
                let hours = total / 3600;
                let minutes = (total % 3600) / 60;
                let seconds = total % 60;
                Notification {
                    size: 30,
                    color: (255, 2, 22),
                    position: Position::Top,
                    content: format!(
                        "You have {}H:{}M:{}S left before the registration ends",
                        hours, minutes, seconds
                    ),
                }
            }
            Event::Appointment(msg) => Notification {
                size: 100,
                color: (200, 200, 3),
                position: Position::Center,
                content: msg.to_string(),
            },
            Event::Holiday => Notification {
                size: 25,
                color: (0, 255, 0),
                position: Position::Top,
                content: "Enjoy your holiday".to_string(),
            },
        }
    }
}
