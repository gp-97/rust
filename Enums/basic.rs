use std::io;

enum Week {
    Monday(String),
    Tuesday(String),
    Wednesday(String),
    Thursday(String),
    Friday(String),
    Saturday(String),
    Sunday(String),
}

impl Week {
    fn today(&self) -> String {
        match self {
            Week::Monday(day) => day.to_string(),
            Week::Tuesday(day) => day.to_string(),
            Week::Wednesday(day) => day.to_string(),
            Week::Thursday(day) => day.to_string(),
            Week::Friday(day) => day.to_string(),
            Week::Saturday(day) => day.to_string(),
            Week::Sunday(day) => day.to_string(),
        }
    }
    fn day_number(&self) -> u8 {
        match self {
            Week::Monday(_day) => 1,
            Week::Tuesday(_day) => 2,
            Week::Wednesday(_day) => 3,
            Week::Thursday(_day) => 4,
            Week::Friday(_day) => 5,
            Week::Saturday(_day) => 6,
            Week::Sunday(_day) => 0,
        }
    }
    fn day_by_number(n: u8) -> String {
        match n {
            1 => "Monday".to_string(),
            2 => "Tuesday".to_string(),
            3 => "Wednesday".to_string(),
            4 => "Thursday".to_string(),
            5 => "Friday".to_string(),
            6 => "Saturday".to_string(),
            0 => "Sunday".to_string(),
            _ => "Wrong day".to_string(),
        }
    }
}

fn n_days_after_today(today: &Week, n: &u8) -> String {
    let current_day: u8 = (*today).day_number();
    let after_n_days: u8 = (*n + current_day) % 7;
    Week::day_by_number(after_n_days)
}

fn get_user_input_day() -> Week {
    let mut inp: String = String::new();
    io::stdin().read_line(&mut inp).expect("Enter a day");
    inp = inp.trim().to_string();

    let matching_slice: &str = &inp[..];
    match matching_slice {
        "m" | "M" | "mon" | "monday" | "Monday" | "MONDAY" => {
            Week::Monday("Monday".to_string()),
        }
        "t" | "T" | "tue" | "tuesday" | "Tuesday" | "TUESDAY" => {
            Week::Tuesday("Tuesday".to_string())
        }
        "w" | "W" | "wednesday" | "Wednesday" | "WEDNESDAY" => {
            Week::Wednesday("Wednesday".to_string())
        }
        "th" | "TH" | "Th" | "thursday" | "Thursday" | "THURSDAY" => {
            Week::Thursday("Thursday".to_string())
        }
        "f" | "F" | "fri" | "friday" | "Friday" | "FRIDAY" => {
            Week::Friday("Friday".to_string()),
        }
        "st" | "sat" | "ST" | "SAT" | "saturday" | "Saturday" | "SATURDAY" => {
            Week::Saturday("Saturday".to_string())
        }
        "s" | "S" | "SUN" | "sun" | "sunday" | "Sunday" | "SUNDAY" => {
            Week::Sunday("Sunday".to_string())
        }
        _ => {
            println!("Please enter valid week day");
            get_user_input_day()
        }
    }
}

fn get_number_of_days_after() -> u8 {
    let mut inp = String::new();
    io::stdin().read_line(&mut inp).expect("Invalid number");
    inp = inp.trim().to_string();

    match inp.parse::<u8>() {
        Ok(n) => n,
        Err(_e) => {
            println!("Enter valid 8-bit unsigned number");
            get_number_of_days_after()
        }
    }
}

fn main() {
    let today = get_user_input_day();
    let n: u8 = get_number_of_days_after();
    println!("Today is {}", today.today());
    println!(
        "After {} days from {} : {}",
        n,
        today.today(),
        n_days_after_today(&today, &n)
    );
}
