use std::env;
use chrono::NaiveDate;

fn main() {
    let api_key = env::var("USER_BIRTHDAY")
        .expect("Birthday must be set as environment variable");

    handle_birthday(&api_key);
}

fn handle_birthday(key: &str) {
    let key_date = NaiveDate::parse_from_str(key, "%Y-%m-%d")
        .expect("Invalid date format");
    let today = chrono::Utc::now().date_naive();
    let days_between = (today - key_date).num_days();

    if days_between == 10_000 {
        println!("You are 10000 days old. That's a nice, round number!");
    } else if key_date == today {
        println!("Happy birthday! Looks like you're new here.");
    } else if key_date > today {
        println!("Are you from the future?");
    } else {
        println!("You are {days_between} days old!");
    }
}
