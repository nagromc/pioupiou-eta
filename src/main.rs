use chrono::NaiveDate;
use clap::Parser;

mod eta;

/// Shows the ETA of Pioupiou
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Estimated date of birth of Pioupiou
    #[arg(short, long, value_parser = is_valid_date)]
    dob: NaiveDate,
}

fn main() {
    let args = Args::parse();

    let dob = args.dob;
    let n_of_days = eta::compute_eta(dob);
    println!("ETA: {} days", n_of_days);
}

fn is_valid_date(s: &str) -> Result<NaiveDate, String> {
    const DATE_FORMAT: &'static str = "%Y-%m-%d";
    let result = NaiveDate::parse_from_str(s, DATE_FORMAT);

    match result {
        Ok(_) => Ok(result.unwrap()),
        Err(_) => Err(format!("Must be in the following format: {}", DATE_FORMAT)),
    }
}
