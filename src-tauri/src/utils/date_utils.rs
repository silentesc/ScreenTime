use chrono::NaiveDate;

pub fn get_today_date() -> String {
    let now = chrono::Utc::now();
    now.format("%d.%m.%Y").to_string()
}

pub fn get_specific_date(day: u32, month: u32, year: i32) -> String {
    let date = NaiveDate::from_ymd_opt(year, month, day)
        .expect("Invalid date")
        .format("%d.%m.%Y")
        .to_string();

    date
}
