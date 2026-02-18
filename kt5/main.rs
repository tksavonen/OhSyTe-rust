use std::fmt;
use chrono::{NaiveDate, Datelike};

#[derive(Debug, PartialEq)]
pub struct Category {
    primary: String,
    secondary: Option<String>,
}

impl Category {
    fn new(primary: &str, secondary: Option<&str>) -> Self {
        Self {
            primary: primary.to_string(),
            secondary: secondary.map(|s| s.to_string()),
        }
    }

    fn from_primary(primary: &str) -> Self {
        Self {
            primary: primary.to_string(),
            secondary: None,
        }
    }

    fn from_str(s: &str) -> Self {
        let parts: Vec<&str> = s.split('/').collect();

        if parts.len() < 2 {
            Category {
                primary: parts[0].to_string(),
                secondary: None,
            }
        } else {
            Category {
                primary: parts[0].to_string(),
                secondary: Some(parts[1].to_string()),
            }
        }
    }
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.secondary {
            Some(sec) => write!(f, "{}/{}", self.primary, sec),
            None => write!(f, "{}", self.primary),
        }
    }
}

#[derive(Debug)]
pub struct MonthDay {
    pub month: u32,
    pub day: u32,
}

#[derive(Debug)]
enum EventKind {
    Singular(NaiveDate),
}

#[derive(Debug)]
struct Event {
    kind: EventKind,
    description: String,
    category: Category,
}

impl Event {
    fn new_singular(date: NaiveDate, description: String, category: Category) -> Self {
        Event {
            kind: EventKind::Singular(date),
            description,
            category,
        }
    }

    fn year(&self) -> i32 {
        match &self.kind {
            EventKind::Singular(date) => date.year(),
        }
    }

    fn month_day(&self) -> MonthDay {
        match &self.kind {
            EventKind::Singular(date) => MonthDay {
                month: date.month(),
                day: date.day(),
            },
        }
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}: {} ({})",
            match &self.kind {
                EventKind::Singular(date) => date.year(),
            },
            self.description,
            self.category
        )
    }
}

fn main() {
    let mut events: Vec<Event> = Vec::new();

    events.push(Event::new_singular(
        NaiveDate::from_ymd_opt(2025, 12, 11).unwrap(),
        String::from("Rust 1.92.0 released"),
        Category::from_str("programming/rust"),
    ));

    events.push(Event::new_singular(
        NaiveDate::from_ymd_opt(2015, 5, 15).unwrap(),
        String::from("Rust 1.0.0 released"),
        Category::new("programming", Some("rust")),
    ));

    for event in events {
        println!("{}: {}", event.year(), event.description);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn category_new_with_primary_and_secondary() {
        let c = Category::new("Food", Some("Fruit"));
        assert_eq!(c.primary, "Food");
        assert_eq!(c.secondary, Some("Fruit".to_string()));
    }

    #[test]
    fn category_from_primary_only() {
        let c = Category::from_primary("Books");
        assert_eq!(c.primary, "Books");
        assert_eq!(c.secondary, None);
    }

    #[test]
    fn category_from_str_with_both() {
        let c = Category::from_str("Vehicle/Car");
        assert_eq!(c.primary, "Vehicle");
        assert_eq!(c.secondary, Some("Car".to_string()));
    }

    #[test]
    fn category_from_str_with_only_primary() {
        let c = Category::from_str("Music");
        assert_eq!(c.primary, "Music");
        assert_eq!(c.secondary, None);
    }
}
