use chrono::{NaiveDate};
use chrono::Month::August;

mod eta;

fn main() {
    let dob = NaiveDate::from_ymd_opt(2024, August.number_from_month(), 20).unwrap();
    let n_of_days = eta::compute_eta(dob);
    println!("ETA: {} days", n_of_days);
}
