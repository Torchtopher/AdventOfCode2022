use std::env;
use std::fs;
use std::path;
mod day1;


fn invalid_input(input: &String) {
    println!("Invalid argument \"{input}\", should be a day like day4");
}

pub fn read_input(day: &i32) -> String {
    // make path dynamic based on day
    let path = path::Path::new("inputs").join(format!("day{}.txt", day));
    println!("Reading input from {:?}", path);
    let input = fs::read_to_string(path).expect("Unable to read file");
    input
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
            if n < 1 || n > 25 {
                invalid_input(dayarg);
                return;
            } else {
                // run the day
                let input = read_input(&n);
                //println!("input: {}", input);
                //println!("day: {}", n);
                match n { // add each day as a match, wanted to use macros but couldn't figure it out so write out each day
                    1 => {
                        
                        let (part1, part2) = day1::run(input);
                        println!("part1: {}", part1);
                        println!("part2: {}", part2);
                    },
                    _ => {
                        println!("Day {} not implemented yet", n);
                    }
                }
            }
        }
        Err(_) => {
            invalid_input(dayarg);
            return;
        }
    }

}