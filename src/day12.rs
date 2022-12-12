use std::vec;

// ndarray
use ndarray::prelude::*;

pub fn run(input: String) -> (i128, i128) {
    // make vecofvec of numbers
    let mut vecofvec: Vec<Vec<u32>> = Vec::new();
    let mut row_size: usize = 0;
    let mut col_size: usize = 0;
    for line in input.lines() {
        row_size = line.len();
        col_size += 1;
        let mut vec: Vec<u32> = Vec::new();
        for c in line.chars() {
            vec.push(c as u32);
        }
        vecofvec.push(vec);
    }
    println!("Row size {}", row_size);
    println!("Col size {}", col_size);
    let map = Array2::from_shape_vec((row_size, col_size), vecofvec).unwrap();
    println!("Map {:?}", map);
    (0, 0)

}