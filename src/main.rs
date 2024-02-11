mod eta;

fn main() {
    let number_of_days = eta::compute_eta();
    println!("ETA: {} days", number_of_days);
}
