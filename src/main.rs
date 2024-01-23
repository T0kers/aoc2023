
pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    day1::parts();
    day2::parts();
    day3::parts();
    day4::parts();
    day5::parts();
    day6::parts();
}
