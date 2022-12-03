use std::env;
use std::fs;
use std::i128;
use std::path;
mod day1; 
mod day2; 
mod day3;

fn invalid_input(input: &String) {
    println!("Invalid argument \"{input}\", should be a day like day4");
}

fn read_input(day: &i32) -> String {
    // make path dynamic based on day
    let path = path::Path::new("inputs").join(format!("day{}.txt", day));
    println!("Reading input from {:?}", path);
    fs::read_to_string(path).expect("Unable to read file")
}

fn print_output(output: (i128, i128)) {
    println!("Part 1: {}", output.0);
    println!("Part 2: {}", output.1);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    //dbg!(args);
    let dayarg: &String = &args[1];

    // check if input is a valid day by comparing the first 3 chars to "day"
    // if not, return invalid_input
    // if it is, return the module
    if dayarg.len() < 3 || &dayarg[..3] != "day" {
        invalid_input(dayarg);
        return;
    } else {};

    let daynum = dayarg[3..].parse::<i32>();
    // match daynum and let match = daynum

    match daynum {
        Ok(n) => {
            if !(1..=25).contains(&n) {
                invalid_input(dayarg);
            } else {
                // run the day
                let input = read_input(&n);
                //println!("input: {}", input);
                //println!("day: {}", n);
                match n { // add each day as a match, wanted to use macros but couldn't figure it out so write out each day
                    1 => {
                        print_output(day1::run(input));
                    },
                    2 => {
                        print_output(day2::run(input))
                    },
                    3 => {
                        print_output(day3::run(input))
                    },
                    _ => {
                        println!("Day {} not implemented yet", n);
                    }
                }
            }
        }
        Err(_) => {
            invalid_input(dayarg);
        }
    }

}