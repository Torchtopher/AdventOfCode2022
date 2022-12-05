use std::collections::HashMap;
use itertools::Itertools;  // for sorting hashmap by keys

#[derive(Debug)]
enum CraneVersion {
    v9000,
    v9001,
}

#[derive(Debug)]
struct StackYard {
    piles: HashMap<u32, Vec<char>>,
    crane: CraneVersion,
}

impl StackYard {
    fn new(version: CraneVersion) -> StackYard {
        StackYard {
            piles: HashMap::new(),
            crane: version,
        }
    }
    // add a pile to the stack yard, should be called in the order of the piles
    fn add_pile(&mut self, pile: Vec<char>) {  
        let stack_len = self.piles.len() as u32 + 1;
        self.piles.insert(stack_len, pile);
    }
    // implements the crane in the problem, take the pile to move, the depth, and then the pile to move to
    fn move_pile(&mut self, from: u32, to: u32, depth: u32) { 
        let mut to_remove: u32 = 0;
        { // scope to have the mutable borrow of self.piles end before the remove
            let pile_depth = self.piles.get(&from).unwrap().len() as u32; // how deep is the pile we're moving
            let pile = self.piles.get(&from).unwrap().clone(); // get the pile we're moving
            let pile_to = self.piles.get_mut(&to).unwrap(); // get the pile we're moving to, mutable borrow
            let range = (pile_depth - depth) as usize..pile_depth as usize; // get the range of the pile we're moving
            let mut new_pile = pile[range].to_vec(); // get the actual elements we're moving

            match self.crane {
                CraneVersion::v9000 => new_pile.reverse(), // part 1, reverse the pile when moving
                CraneVersion::v9001 => (), // part 2 can pick up the entire pile
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

    fn top_string(&mut self) -> String {
        let mut top_chars = String::new();
        for key in self.piles.keys().sorted() {
            let pile = self.piles.get(key).unwrap();
            top_chars.push(pile[pile.len() - 1]);
        }
        top_chars
    }
}

fn parse_instructions(line: &str) -> (u32, u32, u32) {
    // remove all non numeric characters
    let line = line.replace(' ',"").replace("from", " ").replace("to", " ");

    let mut split = line.split_whitespace();
    let depth = split.next().unwrap().parse::<u32>().unwrap(); // just assume input is valid and unwrap everything
    let from = split.next().unwrap().parse::<u32>().unwrap();
    let to = split.next().unwrap().parse::<u32>().unwrap();

    (from, to, depth)
}

pub fn run(input: String) -> (String, String) {
    {   // test input, part 1 works
        let mut yard = StackYard::new(CraneVersion::v9000);
        let pile: Vec<char> = vec!['Z', 'N'];
        let test_pile = vec!['M', 'C', 'D'];
        let test_pile2 = vec!['P'];
        yard.add_pile(pile);
        yard.add_pile(test_pile);
        yard.add_pile(test_pile2);

        yard.move_pile(2, 1, 1);
        yard.move_pile(1, 3, 3);
        yard.move_pile(2, 1, 2);
        yard.move_pile(1, 2, 1);
        println!("Top chars test={}", yard.top_string());
        //exit(0);
    }

    // before the first "move" and assign it to a variable called "yard_input"
    let mut yard = StackYard::new(CraneVersion::v9000);
    let mut yard2 = StackYard::new(CraneVersion::v9001);

    let vec = input.split("move").collect::<Vec<&str>>();
    // input is a pile of blocks  
    let yard_input = vec[0].to_string();
    // instructions is just moves for the crane
    let yard_instructions = vec[1..].to_vec();
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
        temp_pile.reverse();
        yard.add_pile(temp_pile.clone()); // clone to avoid use after move
        yard2.add_pile(temp_pile); // yard will be modifying the piles, so we need to clone (I hope) 
    }

    for line in yard_instructions {
        let (from, to, depth) = parse_instructions(line);
        yard.move_pile(from, to, depth);
        yard2.move_pile(from, to, depth);
    }
    let part1 = yard.top_string();
    let part2 = yard2.top_string();
    (part1, part2)

}