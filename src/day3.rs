

fn char_to_score(c: char) -> i128 {
    // convert a-z to 1-26
    // convert A-Z to 27-52
    // convert space to 53

    let score = match c {
        'a'..='z' => c as u8 - 96,
        'A'..='Z' => c as u8 - 38,
        _ => panic!("Invalid character"),
    };
    // convert score to i128
    score as i128
}

pub fn run(input: String) -> (i128, i128) {
    let mut part1: i128 = 0;
    let mut part2: i128 = 0;
    // part 1
    for line in input.lines() {
        let line_size = line.len();
        println!("Sack={}", line);
        let compartment1: &String = &line[..line_size/2].to_string(); // 1st half
        println!("Sack after 1={}", line);
        let compartment2: &String = &line[line_size/2..].to_string(); // 2nd half

        let compartment1 = compartment1.chars().collect::<Vec<char>>(); // convert to vector of chars
        let compartment2 = compartment2.chars().collect::<Vec<char>>();

        // make byte set of ruck
        println!("Compartment1={:?}", compartment1);
        println!("Compartment2={:?}", compartment2);
        for c in compartment1 {
            if compartment2.contains(&c) {
                part1 += char_to_score(c);
                break;
            }
        }
    }
    // part 2
    for i in (0..input.lines().count()).step_by(3) {
        println!("i={}", i);
        let rucksack1 = input.lines().nth(i)
                                        .unwrap().to_string()
                                        .chars().collect::<Vec<char>>();

        let rucksack2 = input.lines().nth(i+1)
                                        .unwrap().to_string()
                                        .chars().collect::<Vec<char>>();

        let rucksack3 = input.lines().nth(i+2)
                                        .unwrap().to_string()
                                        .chars().collect::<Vec<char>>();

        for c in rucksack1 {
            if rucksack2.contains(&c) && rucksack3.contains(&c) { // hopefully its smart and doesn't check the second if the first is false
                part2 += char_to_score(c); // think that is how it works in c++
                break;
            }
        }
    }
    (part1, part2)
}