mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod util;

use anyhow::anyhow;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let days_completed = 5;

    match args.get(1) {
        Some(day_arg) => {
            if let Ok(day_num) = day_arg.parse::<usize>() {
                if let Err(e) = run_day(day_num) {
                    eprintln!("Error: {}", e);
                }
            } else {
                eprintln!("Please provide a valid day number");
            }
        }
        None => {
            for day in 1..days_completed {
                if let Err(e) = run_day(day) {
                    eprintln!("Error: {}", e);
                }
            }
        }
    }
}

fn run_day(day_number: usize) -> anyhow::Result<i32> {
    match day_number {
        1 => day1::run(),
        2 => {
            let max_red = 12;
            let max_green = 13;
            let max_blue = 14;
            day2::run(max_red, max_green, max_blue)
        }
        3 => day3::run(),
        4 => day4::run(),
        5 => day5::run(),
        _ => Err(anyhow!("Invalid day number")),
    }
}
