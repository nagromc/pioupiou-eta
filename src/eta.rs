use chrono::{Local, NaiveDate, TimeDelta};

pub fn compute_eta(dob: NaiveDate) -> TimeDelta {
    let now = Local::now().date_naive();

    dob - now
}
