use chrono::NaiveDate;
use clap::{Parser, ValueEnum};

mod eta;

/// Shows the ETA of Pioupiou
#[derive(Parser)]
#[command(version, about)]
struct Args {
    /// Estimated date of birth of Pioupiou
    #[arg(short, long, value_parser = is_valid_date)]
    dob: NaiveDate,

    #[arg(short, long)]
    /// Display mode
    format: Format,
}

#[derive(Clone, ValueEnum)]
enum Format {
    /// Number of days
    Days,

    /// Number of months and days
    MonthsAndDays,
}

fn main() {
    let args = Args::parse();

    let eta = eta::compute_eta(args.dob);

    let output: String = match args.format {
        Format::Days => {
            format!("{} days", eta.num_days().to_string())
        }
        Format::MonthsAndDays => {
            format!(
                "{} months, {} days",
                eta.num_weeks() / 4,
                eta.num_days() / eta.num_weeks() / 4
            )
        }
    };

    println!("{}", output);
}

fn is_valid_date(s: &str) -> Result<NaiveDate, String> {
    const DATE_FORMAT: &'static str = "%Y-%m-%d";
    let result = NaiveDate::parse_from_str(s, DATE_FORMAT);

    match result {
        Ok(_) => Ok(result.unwrap()),
        Err(_) => Err(format!("Must be in the following format: {}", DATE_FORMAT)),
    }
}
