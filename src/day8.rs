use ndarray::prelude::*;
/*
30373
25512
65332
33549
35390
*/
#[derive(Clone, Debug)]
struct SenicScore {
    score: i128,
    index: (usize, usize),
    up: i32,
    down: i32,
    left: i32,
    right: i32,
}

impl SenicScore {
    fn new() -> Self {
        SenicScore {
            score: 0,
            index: (0, 0),
            up: 0,
            down: 0,
            left: 0,
            right: 0,
        }
    } 
    fn find_score(&mut self) -> i128 {
        self.score = (self.up * self.down * self.left * self.right) as i128;
        self.score
    }
    fn reset(&mut self) {
        self.score = 0;
        self.up = 0;
        self.down = 0;
        self.left = 0;
        self.right = 0;
    }
}
    

pub fn run(input: String) -> (i128, i128) {
    let mut num_rows = 0;
    let mut num_cols = 0;
    let mut tempVec: Vec<u8> = Vec::with_capacity(input.len());
    for line in input.lines() {
        num_rows += 1;
        num_cols = line.len();
        //println!("line: {}", line);
        for c in line.chars() {
            //println!("c: {}", c);
            // parse the char to a u8 and push it to the tempVec
            tempVec.push(c.to_digit(10).unwrap() as u8);
        }
    }
    //println!("tempVec: {:#?}", tempVec);
    // please compiler optimize this because it is not mutable
    let ndarray = Array2::from_shape_vec((num_rows, num_cols), tempVec).unwrap();
    //println!("ndarray:\n{:#?}", ndarray);
    let mut part1 = 0;
    let mut part2: i128 = 0;
    // itterate over the 2d array and find the elements to the left right up and down
    for (i, row) in ndarray.outer_iter().enumerate() {
        for (j, col) in row.iter().enumerate() {

            //println!("\ni: {}, j: {}, col: {}", i, j, col);
            //println!("EQ? {}, {}", col, ndarray[[i, j]]);
            // check if the element is a 0
            if j == 0 || j == num_cols - 1 || i == 0 || i == num_rows - 1 {
                //println!("On edge with value: {}\n", col);
                part1 += 1;
                continue;
            }
            let ii = i;
            let jj = j;
            let mut count = 1;
            let mut found = false;
            let mut l = true;
            let mut r = true;
            let mut u = true;
            let mut d = true;
            loop {
                found = false;
                //println!("Count: {}", count);
                // out of bounds check for left
                if !(jj as i16 - (count as i16) < 0) && l { // convert to i16 to avoid underflow
                    found = true;
                    if col <= &ndarray[[ii, jj - count]] {
                        //println!("L Col {} is less than or eq {}", col, ndarray[[ii, jj - count]]);
                        l = false;
                    }
                }
                if !(jj + count > num_cols - 1) && r {
                    found = true;
                    if col <= &ndarray[[ii, jj + count]] {
                        //println!("R Col {} is less than or eq {}", col, ndarray[[ii, jj + count]]);
                        r = false;
                    }
                }
                if !(ii as i16 - (count as i16) < 0) && u { 
                    found = true;
                    if col <= &ndarray[[ii - count, jj]] {
                        //println!("U Col {} is less than or eq {}", col, ndarray[[ii - count, jj]]);
                        u = false;
                    }
                }
                if !(ii + count > num_rows - 1) && d {
                    found = true;
                    if col <= &ndarray[[ii + count, jj]] {
                        //println!("D Col {} is less than or equal to {}", col, ndarray[[ii + count, jj]]);
                        d = false;
                    }
                }
                // print center and all surrounding elements
                //println!("Center: {}, {}, {}, {}, {}", ndarray[[ii, jj]], ndarray[[ii, jj - count]], ndarray[[ii, jj + count]], ndarray[[ii - count, jj]], ndarray[[ii + count, jj]]);
                if !found {
                    //println!("Breaking");
                    break;
                }
                count += 1;
            }
            if l || r || u || d {
                //println!("Found a visible at: {}, {}", i, j);
                part1 += 1;
            }
        }
    }
    // part2 
    let mut highest_score = SenicScore::new();
    let mut score = SenicScore::new();
    for (i, row) in ndarray.outer_iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            score.reset();
            score.index = (i, j);
            // early exit if on edge, whole score goes to 0 
            if j == 0 || j == num_cols - 1 || i == 0 || i == num_rows - 1 {
                //println!("On edge with value: {}\n", col);
                continue;
            }
            let ii = i;
            let jj = j;
            let mut count = 1;
            let mut found = false;
            let mut l = true;
            let mut r = true;
            let mut u = true;
            let mut d = true;
            //println!("\n{}", ndarray);
            loop {
                found = false;
                //println!("Count: {}", count);
                // out of bounds check for left
                if !(jj as i16 - (count as i16) < 0) && l { // convert to i16 to avoid underflow
                    found = true;
                    if col <= &ndarray[[ii, jj - count]] {
                        //println!("L Col {} is less than or eq {}", col, ndarray[[ii, jj - count]]);
                        l = false;
                    }
                    score.left += 1;
                }
                if !(jj + count > num_cols - 1) && r {
                    found = true;
                    if col <= &ndarray[[ii, jj + count]] {
                        //println!("R Col {} is less than or eq {}", col, ndarray[[ii, jj + count]]);
                        r = false;
                    }
                    score.right += 1;
                }
                if !(ii as i16 - (count as i16) < 0) && u { 
                    found = true;
                    if col <= &ndarray[[ii - count, jj]] {
                        //println!("U Col {} is less than or eq {}", col, ndarray[[ii - count, jj]]);
                        u = false;
                    }
                    score.up += 1;
                    
                }
                if !(ii + count > num_rows - 1) && d {
                    found = true;
                    if col <= &ndarray[[ii + count, jj]] {
                        //println!("D Col {} is less than or equal to {}", col, ndarray[[ii + count, jj]]);
                        d = false;
                    }
                    score.down += 1;
                }
                // print center and all surrounding elements
                //println!("Center: {}, {}, {}, {}, {}", ndarray[[ii, jj]], ndarray[[ii, jj - count]], ndarray[[ii, jj + count]], ndarray[[ii - count, jj]], ndarray[[ii + count, jj]]);
                if !found {
                    //println!("Breaking");
                    break;
                }
                count += 1;
            }
            //println!("Score {:#?}", score.find_score());
            //println!("Highest Score {:#?}", highest_score.find_score());
            if score.find_score() > highest_score.find_score() {
                highest_score = score.clone();
            }
        }
    }
    println!("{:#?}", highest_score);
    part2 = highest_score.find_score();
    (part1, part2)

}