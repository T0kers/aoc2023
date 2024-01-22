
pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    day5::parts();
}
