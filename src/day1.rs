// use binary heap to sort the numbers
//
use std::collections::BinaryHeap;

pub fn run(input: String) -> (i128, i128) {

    println!("Hello from day 1!");
    let size = 3;
    let mut elf = 0;
    // make binary heap of length 3
    let mut heap: BinaryHeap<i128> = BinaryHeap::with_capacity(size);
    let mut temp: Vec<i128> = Vec::with_capacity(size); // probably should be array instead of vec
    // itterate over each line in the input
    for line in input.lines() {
        //println!("line: {}", line);
        if line == "" {
            //println!("Elf {} is being worked on", elf);
            //println!("heap: {:?}", heap);
            // tried to make super efficent  
            if heap.len() == size {
                for _ in 0..size {
                    temp.push(heap.pop().unwrap());
                    //println!("temp: {:?}", temp);
                    //println!("heap: {:?}", heap);
                    if elf > temp[temp.len()-1] { // will end up popping 3 times in average case
                        heap.push(elf);
                        // remove number from temp
                        temp.pop();
                        break;
                    }
                }
                // re add vec to heap
                for i in &temp {
                    heap.push(i.clone());
                }
                temp.clear();
            }
            else {
                heap.push(elf);
            }
            //println!("heap after: {:?}", heap);
            elf = 0;
        }
        else {
            // add the number to elf
            elf += line.parse::<i128>().unwrap();
        }
    }
    // println!("heap: {:?}", heap);
    let part1 = heap.peek().unwrap().clone(); // probably cheap to clone a primitive type
    // add up the numbers in the heap
    let part2: i128 = heap.iter().sum();
    return (part1, part2);  
}