use chrono::{NaiveDate};
use chrono::Month::August;

mod eta;

fn main() {
    let dob = NaiveDate::parse_from_str("2024-08-20", "%Y-%m-%d").unwrap();
    let n_of_days = eta::compute_eta(dob);
    println!("ETA: {} days", n_of_days);
}
