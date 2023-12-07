mod day1;
mod day2;
mod day3;
mod util;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let days = [day1::run, day2::run, day3::run /* ... other day functions */];
 
    match args.get(1) {
        Some(day_arg) => {
            if let Ok(day_num) = day_arg.parse::<usize>() {
                if day_num > 0 && day_num <= days.len() {
                    if let Err(e) = days[day_num - 1]() {
                        eprintln!("Error: {}", e);
                    }
                } else {
                    eprintln!("Invalid day number: {}", day_num);
                }
            } else {
                eprintln!("Please provide a valid day number");
            }
        }
        None => {
            for day in days.iter() {
                if let Err(e) = day() {
                    eprintln!("Error: {}", e);
                }
            }
        }
    }
}
