use reqwest::blocking::get;
use serde::Deserialize;

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

    println!("BEGIN:VCALENDAR");

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
                // let d = NaiveDate::parse_from_str(&e.date, "%Y-%m-%d")?;
                println!("BEGIN:VEVENT");
                println!("SUMMARY:{}", item.MenuName);
                println!("DTSTART:{}T{}Z",
                    lunch_starts_at.format("%Y%m%d"),
                    lunch_starts_at.format("%H%M%S"),
                );
                println!("DTEND:{}T{}Z",
                    lunch_ends_at.format("%Y%m%d"),
                    lunch_ends_at.format("%H%M%S"),
                );
                println!("END:VEVENT");
            }
        }
    }

    println!("END:VCALENDAR");
    Ok(())
}
