use std::io;
use std::io::Write;

mod aoc24;

fn get_input(prompt: &str) -> Result<String, io::Error> {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input_buf: String = String::new();
    match io::stdin().read_line(&mut input_buf) {
        Ok(_) => Ok(input_buf.trim().to_string()),
        Err(err) => Err(err),
    }
}

fn main() {
    let choice: i32 = get_input("Enter Day number: ")
        .unwrap_or_else(|_| {
            println!("Error: Could not read input");
            std::process::exit(1);
        })
        .parse::<i32>()
        .unwrap();

    let result_option: Option<[String; 2]> = match choice {
        1 => Some(aoc24::day1::Day1::get_results()),
        2 => Some(aoc24::day2::Day2::get_results()),
        3 => Some(aoc24::day3::Day3::get_results()),
        4 => Some(aoc24::day4::Day4::get_results()),
        _ => None,
    };

    if let Some(results) = result_option {
        println!("{:?}", results);
    } else {
        println!("Solution is not implemented");
    }
}
