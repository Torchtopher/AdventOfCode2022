use std::collections::HashMap;
use itertools::Itertools;  // for sorting hashmap by keys

#[derive(Debug)]
pub enum CraneVersion {
    V9000,
    V9001,
}

#[derive(Debug)]
pub struct StackYard {
    pub piles: HashMap<u32, Vec<char>>,
    pub crane: CraneVersion,
    pub instructions: Vec<String>,
}

impl StackYard {
    pub fn new(version: CraneVersion) -> StackYard {
        StackYard {
            piles: HashMap::new(),
            crane: version,
            instructions: Vec::new(),
        }
    }
    // add a pile to the stack yard, should be called in the order of the piles
    fn add_pile(&mut self, pile: Vec<char>) {  
        let stack_len = self.piles.len() as u32 + 1;
        self.piles.insert(stack_len, pile);
    }

    pub fn step(&mut self) {
        let line = self.instructions.pop().unwrap();
        println!("{}", line);
        let (from, to, depth) = self.parse_instructions(&line);
        self.move_pile(from, to, depth);
    }
    
    // implements the crane in the problem, take the pile to move, the depth, and then the pile to move to
    pub fn move_pile(&mut self, from: u32, to: u32, depth: u32) { 
        let mut to_remove: u32 = 0;
        { // scope to have the mutable borrow of self.piles end before the remove
            let pile_depth = self.piles.get(&from).unwrap().len() as u32; // how deep is the pile we're moving
            let pile = self.piles.get(&from).unwrap().clone(); // get the pile we're moving
            let pile_to = self.piles.get_mut(&to).unwrap(); // get the pile we're moving to, mutable borrow
            let range = (pile_depth - depth) as usize..pile_depth as usize; // get the range of the pile we're moving
            let mut new_pile = pile[range].to_vec(); // get the actual elements we're moving

            match self.crane {
                CraneVersion::V9000 => new_pile.reverse(), // part 1, reverse the pile when moving
                CraneVersion::V9001 => (), // part 2 can pick up the entire pile
            }
            for i in &mut new_pile {
                pile_to.push(*i); // add to the pile we're moving to
                to_remove += 1; // remember how many we've added to remove later
            }
        }
        let pile = self.piles.get_mut(&from).unwrap(); // mutable borrow of the pile we moved from
        for _ in 0..to_remove { // remove the elements we moved
            pile.pop();
        }
    }

    pub fn top_string(&mut self) -> String {
        let mut top_chars = String::new();
        for key in self.piles.keys().sorted() {
            let pile = self.piles.get(key).unwrap();
            top_chars.push(pile[pile.len() - 1]);
        }
        top_chars
    }

    pub fn parse_instructions(&self, line: &str) -> (u32, u32, u32) {
        // remove all non numeric characters
        let line = line.replace(' ',"").replace("from", " ").replace("to", " ");
    
        let mut split = line.split_whitespace();
        let depth = split.next().unwrap().parse::<u32>().unwrap(); // just assume input is valid and unwrap everything
        let from = split.next().unwrap().parse::<u32>().unwrap();
        let to = split.next().unwrap().parse::<u32>().unwrap();
        
        (from, to, depth)
    }
}



pub fn parse_input(input: String, version: CraneVersion) -> StackYard {
    let mut yard = StackYard::new(version);
    let vec = input.split("move").collect::<Vec<&str>>();
    // input is a pile of blocks  
    let yard_input = vec[0].to_string();
    // instructions is just moves for the crane
    // make it a vector of strings
    let instructions = vec[1..].iter().map(|x| x.to_string()).collect::<Vec<String>>();
    
    yard.instructions = instructions;
    // let yard_instructions = vec[1..].to_vec();
    // last digit is the length of the pile
    let yard_lenght = yard_input[yard_input.len()-4..yard_input.len()-3].to_string().parse::<u32>().unwrap();
    let mut piles: Vec<Vec<char>> = Vec::new(); // vector of piles
    for _ in 0..yard_lenght { // find how many piles and make a vector of vectors for them
        piles.push(Vec::new());
    }
    println!("piles: {:?}", piles);
    for line in yard_input.lines() { // parse the input and add the piles to the vector
        println!("line: {}", line);
        for (i, c) in line.chars().enumerate() {
            if c.is_alphabetic() {
                let pile_idx = ((i - 1) / 4) + 1; // find the pile index eg 1, 2, 3
                //println!("pile_idx: {}", pile_idx);
                piles[pile_idx-1].push(c);
            }
        }
    }

    for pile in &piles {
        let mut temp_pile = pile.clone();
        println!("temp_pile: {:?}", temp_pile);
        temp_pile.reverse();
        yard.add_pile(temp_pile); 
    }
    yard

}

pub fn run(input: String) -> (String, String) {
    let mut yard = parse_input(input.clone(), CraneVersion::V9000);
    for instruction in &yard.instructions.clone() {
        println!("instruction: {}", instruction);
        let (from, to, depth) = yard.parse_instructions(instruction);
        println!("from: {}, to: {}, depth: {}", from, to, depth);
        yard.move_pile(from, to, depth);
    }
    let mut yard2 = parse_input(input, CraneVersion::V9001);
    for instruction in &yard2.instructions.clone() {
        let (from, to, depth) = yard2.parse_instructions(instruction);
        yard2.move_pile(from, to, depth);
    }
    let part1 = yard.top_string();
    let part2 = yard2.top_string();
    (part1, part2)
}