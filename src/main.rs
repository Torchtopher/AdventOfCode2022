use std::env;
use std::collections::HashMap;
use std::fs;
use std::path;
mod day1;


fn invalid_input(input: &String) {
    println!("Invalid argument \"{input}\", should be a day like day4");
}

pub fn read_input(day: i32) -> String {
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
    let daystr: String = "day1".to_string();
    // make a hashmap of string to function
    let mut daymap: HashMap<String, fn(String) -> (i128, i128)> = HashMap::new();
    // add the day1 function to the hashmap
    daymap.insert(daystr, day1::run);
    // call the function
    let func = daymap.get(dayarg);
    // dynamically get the day number from the string
    // should not fail because we already know it's a valid day
    let daynum = dayarg[3..].parse::<i32>().unwrap();
    let input = read_input(daynum);
    // if function is found, call it and print out tuple of results
    // otherwise run invalid_input
    match func {
        Some(f) =>  {let (a, b) = f(input); 
                                            println!("Part A {}\nPart B {}", a, b);},
        None => invalid_input(dayarg),
    }
    println!("Done")

}