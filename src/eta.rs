use chrono::{Local, NaiveDate, TimeDelta};

pub fn compute_eta(dob: NaiveDate) -> isize {
    let now = Local::now().date_naive();
    let diff = dob - now;

    diff.num_days() as isize
}
