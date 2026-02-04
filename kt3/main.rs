// kt3/main.rs

#[derive(Debug, Clone)]
enum Category {
    Pacific,
    Europe,
    Africa,
    Other
}

#[derive(Debug, Clone)]
struct Date {
    year: i32,
    month: Month,
    day: MonthDay,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Month {
    January = 1,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct MonthDay(u32);

#[derive(Debug, Clone)]
struct Event {
    date: Date,
    description: String,
    category: Category,
}

fn load_events() -> Vec<Event> {
    use Category::*;
    use Month::*;

    vec![
        Event {
            date: Date { year: 1943, month: January, day: MonthDay(15) },
            description: "The Soviet counter-offensive at Voronezh begins.".into(),
            category: Europe,
        },
        Event {
            date: Date { year: 1945, month: January, day: MonthDay(16) },
            description: "Adolf Hitler moves into the Führerbunker as Berlin crumbles.".into(),
            category: Europe,
        },
        Event {
            date: Date { year: 1944, month: January, day: MonthDay(17) },
            description: "Battle of Monte Cassino begins as Allies attempt to break the Gustav Line.".into(),
            category: Europe,
        },
        Event {
            date: Date { year: 1943, month: January, day: MonthDay(18) },
            description: "First Warsaw Ghetto Uprising begins".into(),
            category: Europe,
        },
        Event {
            date: Date { year: 1941, month: January, day: MonthDay(19) },
            description: "British and Commonwealth troops retake Kassala from Italian forces".into(),
            category: Africa,
        },
        Event {
            date: Date { year: 1945, month: January, day: MonthDay(20) },
            description: "Franklin D. Roosevelt begins his fourth presidential term.".into(),
            category: Other,
        },
        Event {
            date: Date { year: 1942, month: January, day: MonthDay(21) },
            description: "Rommel launches a surprise counteroffensive in North Africa.".into(),
            category: Africa,
        },
        Event {
            date: Date { year: 1941, month: January, day: MonthDay(22) },
            description: "Tobruk falls to British and Australian forces.".into(),
            category: Africa,
        },
        Event {
            date: Date { year: 1942, month: January, day: MonthDay(23) },
            description: "Japanese forces capture Rabaul, a major strategic base.".into(),
            category: Pacific,
        },
        Event {
            date: Date { year: 1943, month: January, day: MonthDay(24) },
            description: "The Casablanca Conference ends, promising an uncompromising end to the war".into(),
            category: Africa,
        },
        Event {
            date: Date { year: 1945, month: January, day: MonthDay(25) },
            description: "Battle of the Bulge ends in Allied victory".into(),
            category: Europe,
        },
        Event {
            date: Date { year: 1942, month: January, day: MonthDay(26) },
            description: "First American forces arrive in Northern Ireland.".into(),
            category: Other,
        },
        Event {
            date: Date { year: 1940, month: January, day: MonthDay(27) },
            description: "Germany begins rationing food nationwide as war pressures increase.".into(),
            category: Europe,
        },
        Event {
            date: Date { year: 1945, month: January, day: MonthDay(28) },
            description: "U.S. 1st and 3rd Armies link up, closing the Bulge pocket.".into(),
            category: Europe,
        },
        Event {
            date: Date { year: 1944, month: January, day: MonthDay(29) },
            description: "Soviets liberate key towns in the Leningrad region as their offensive accelerates.".into(),
            category: Europe,
        },
    ]
}

fn print_events_for_date(events: &[Event], day: u32, month: Month) {
    for e in events {
        if e.date.day.0 == day && e.date.month == month {
            println!(
                "{}.{}.{} — {} ({:?})",
                e.date.day.0,
                e.date.month as u32,
                e.date.year,
                e.description,
                e.category
            );
        }
    }
}

fn print_events_for_week(events: &[Event]) {
    use Month::January;

    for day in 15..=29 {
        println!("--- {}.1. ---", day);
        for e in events {
            if e.date.month == January && e.date.day.0 == day {
                println!("{} — {} ({:?})", e.date.year, e.description, e.category);
            }
        }
        println!();
    }
}

fn main() {
    let events = load_events();

    let day = 15;
    let month = Month::January;

    print_events_for_date(&events, day, month);
    print_events_for_week(&events);
}

