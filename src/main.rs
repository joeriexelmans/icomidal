use reqwest::blocking::get;
use serde::Deserialize;
use icalendar::Component;
use icalendar::EventLike;

const location_id: &str = "8199"; // Middelheim?
const customer_id: &str = "7622"; // Me?

#[derive(Deserialize)]
struct MenuItem {
    SectionName: String,
    MenuName: String,
}

#[derive(Deserialize)]
struct Data {
    menuPlannerList: Vec<MenuItem>,
}

#[derive(Deserialize)]
struct Response {
    data: Data,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let now = chrono::Local::now();

    let mut calendar = icalendar::Calendar::new();

    calendar.name("Komida Menu");

    for days_in_future in 0..14 {
        let date = now + chrono::Days::new(days_in_future);

        // komida API expects local time
        let date_komida_fmt = date.format("%Y-%m-%d");

        let res: Response = get(format!(
            "https://app.growzer.be/MenuPlanner/GetMenuPlanner?locationId={}&stringDate={}&customerId={}",
            location_id,
            date_komida_fmt,
            customer_id,
        ))?.json()?;

        // ical uses UTC
        let lunch_starts_at = date.with_time(chrono::NaiveTime::from_hms(11, 30, 00)).unwrap().to_utc();
        let lunch_ends_at = date.with_time(chrono::NaiveTime::from_hms(13, 45, 00)).unwrap().to_utc();

        for item in res.data.menuPlannerList.iter() {
            if item.SectionName.contains("Streetfood")
             || item.SectionName.contains("Dailyfood")
             || item.SectionName.contains("Soep") {
                calendar.push(
                    icalendar::Event::new()
                        .summary(item.MenuName.as_str())
                        .starts(lunch_starts_at)
                        .ends(lunch_ends_at)
                );
            }
        }
    }

    println!("{}", calendar);

    Ok(())
}
