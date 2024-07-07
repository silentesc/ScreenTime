pub fn get_today_date() -> String {
    let now = chrono::Utc::now();
    now.format("%d.%m.%Y").to_string()
}
