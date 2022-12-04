use core::ops::Range;

// template this function, takes any range that has a type that implements the Ord trait
fn range_has_range<T:PartialOrd>(large_range: &Range<T>, small_range: &Range<T>) -> bool {
    large_range.start <= small_range.start && large_range.end >= small_range.end
}

fn range_overlaps_range<T:PartialOrd>(range1: &Range<T>, range2: &Range<T>) -> bool {
    range1.start <= range2.end && range2.start <= range1.end
}

pub fn run(input: String) -> (i128, i128) {
    let mut part1:i128 = 0;
    let mut part2:i128 = 0;

    for line in input.lines() {
        // println!("line: {}", line);
        // find the 4 numbers
        let mut numbers:Vec<i128> = Vec::new();
        for number in line.split(",") {
            //println!("number: {}", number);
            number.split("-").for_each(|n| numbers.push(n.parse::<i128>().unwrap()));
        }
        if numbers.len() != 4 {
            panic!("Expected 4 numbers, got {}", numbers.len());
        }
        let range1 = numbers[0]..numbers[1];
        let range2 = numbers[2]..numbers[3];
        println!("numbers: {:?}", numbers);
        println!("range1: {:?}", range1);
        println!("range2: {:?}", range2);
        // check if range2 is a subset of range
        if range_has_range(&range1, &range2) || range_has_range(&range2, &range1) {
            part1 += 1;
        }
        if range_overlaps_range(&range1, &range2) {
            part2 += 1;
        }
    }
    (part1, part2)
}