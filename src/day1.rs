// use binary heap to sort the numbers
//
use std::collections::BinaryHeap;

pub fn run(input: String) -> (i128, i128) {

    //println!("Hello from day 1!");
    let size = 3; // can be anything
    let mut elf = 0;
    // make binary heap of length 3
    let mut heap: BinaryHeap<i128> = BinaryHeap::with_capacity(size);
    // itterate over each line in the input
    for line in input.lines() {
        ////println!("line: {}", line);
        if line.is_empty() {
            //println!("Elf {} is being worked on", elf);
            heap.push(-elf); // negative is intentional, so that the smallest number is at the top and gets popped first
            //println!("heap: {:?}", heap);
            if heap.len() > size {
                // remove smallest element of heap
                heap.pop();
            }
            //println!("heap after: {:?}", heap);
            ////println!("heap after: {:?}", heap);
            elf = 0;
        }
        else {
            // add the number to elf
            elf += line.parse::<i128>().unwrap();
        }
    }
    // get the smallest number in the heap by taking last element in finalitter
    let part1 = -heap.iter().last().unwrap();
    // add up the numbers in the heap and take the absolute value
    let part2: i128 =  heap.iter().map(|x| x.abs()).sum();
    (part1, part2) 
}

    
// old implementation
#[allow(dead_code)]
pub fn run_old(input: String) -> (i128, i128) {

    //println!("Hello from day 1!");
    let size = 3; // can be anything
    let mut elf = 0;
    // make binary heap of length 3
    let mut heap: BinaryHeap<i128> = BinaryHeap::with_capacity(size);
    let mut temp: Vec<i128> = Vec::with_capacity(size); // probably should be array instead of vec
    // itterate over each line in the input
    for line in input.lines() {
        ////println!("line: {}", line);
        if line.is_empty() {
            ////println!("Elf {} is being worked on", elf);
            ////println!("heap: {:?}", heap);
            // tried to make super efficent  
            // remove first element of heap
            
            if heap.len() == size {
                for _ in 0..size {
                    temp.push(heap.pop().unwrap());
                    ////println!("temp: {:?}", temp);
                    ////println!("heap: {:?}", heap);
                    if elf > temp[temp.len()-1] { // will end up popping 3 times in average case
                        heap.push(elf);
                        // remove number from temp
                        temp.pop();
                        break;
                    }
                }
                // re add vec to heap
                for i in &temp {
                    heap.push(*i);
                }
                temp.clear();
            }
            else {
                heap.push(elf);
            }
            ////println!("heap after: {:?}", heap);
            elf = 0;
        }
        else {
            // add the number to elf
            elf += line.parse::<i128>().unwrap();
        }
    }
    // //println!("heap: {:?}", heap);
    let part1 = *heap.peek().unwrap(); // probably cheap to clone a primitive type
    // add up the numbers in the heap
    let part2: i128 = heap.iter().sum();
    (part1, part2)
}