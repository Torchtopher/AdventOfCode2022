use std::collections::VecDeque;

pub fn run(input: String) -> (i128, i128) {
    // make deque of len 4
    let mut deque: VecDeque<char> = VecDeque::with_capacity(4);
    let mut deque2: VecDeque<char> = VecDeque::with_capacity(14);
    let mut part1: i128 = 0;
    let mut part2: i128 = 0;
    for line in input.lines() {
        println!("{}", line);
        // part 1
        for (idx, c) in line.chars().enumerate() {
            //println!("{}: {}", idx, c);
            if deque.len() < 4 {
                deque.push_back(c);
            } else {
                deque.pop_front();
                deque.push_back(c);
                // check if deque has only unique chars
                if deque.iter().all(|&x| deque.iter().filter(|&y| x == *y).count() == 1) {
                    println!("deque: {:?}", deque);
                    part1 = 1 + idx as i128; 
                    break;
                }
            }
        }
        // part 2
        for (idx, c) in line.chars().enumerate() {
            if deque2.len() < 14 {
                deque2.push_back(c);
            } 
            else {
                deque2.pop_front();
                deque2.push_back(c);
                // check if deque has only unique chars
                if deque2.iter().all(|&x| deque2.iter().filter(|&y| x == *y).count() == 1) {
                    println!("deque2: {:?}", deque2);
                    part2 = 1 + idx as i128; 
                    break;
                }
            }
        }
    }
    (part1, part2)
}