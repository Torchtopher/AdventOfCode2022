use std::collections::HashMap;
use std::process::exit;
use itertools::Itertools;  // for sorting hashmap by keys

// they don't know how to divide or subtract apparently
#[derive(Debug)]
enum Ops {
    Add,
    Mul,
    Div,
    Unknown
}
#[derive(Debug)]
struct MonkeyOp {
    op: Ops,
    val: i64 // value to perform the operation with, if -1 then its self
}
#[derive(Debug)]

struct Monkey {
    items: Vec<i64>,
    op: MonkeyOp, // add or multiply
    test: MonkeyOp, // subtract or divide
    res_true : i64, // the id of monkey that gets the item if the test is true
    res_false : i64 // the id of monkey that gets the item if the test is false
}

#[derive(Debug)]
struct MonkeySolver {
    monkey_pen: HashMap<i64, Monkey>,
    monkey_activity: HashMap<i64, i64>,
    lcm: i64
}

impl MonkeySolver {
    fn new() -> MonkeySolver {
        MonkeySolver {
            monkey_pen: HashMap::new(),
            monkey_activity: HashMap::new(),
            lcm: -999
        }
    }
    fn play_round(&mut self) {
        // make vec of keys to iterate over, bc I love the borrow checker
        let mut keys: Vec<i64> = Vec::new();
        for key in self.monkey_pen.keys().sorted() {
            keys.push(*key);
        }
        
        // go through each monkey and let them do their thing
        for monkey_id in keys {

            let mut items_to_throw: Vec<(i64, i64)> = Vec::with_capacity(10);
            { // scope to drop borrow of monkey_pen
                let monkey = self.monkey_pen.get_mut(&monkey_id).unwrap();
                //println!("monkey_id!: {:#?}", monkey_id);
                //println!("monkey!: {:#?}", monkey);
                for item in &mut monkey.items {
                    *self.monkey_activity.get_mut(&monkey_id).unwrap() += 1; // woo mutable references
                    // perform the operation
                    match monkey.op.op {
                        Ops::Add => {
                            *item += monkey.op.val;
                        },
                        Ops::Mul => {
                            if monkey.op.val == -1 {
                                *item *= *item;
                            } else {
                                *item *= monkey.op.val;
                            }
                        },
                        _ => {exit(-1);}
                    }
                    // arbitary divide by 3 (totally did not forget to add this)
                    //*item /= 3;

                    // from what I can tell, subtract the lcm as many times as possible and that keeps all the same properties
                    while *item > self.lcm {
                        *item -= self.lcm;
                    }

                    let monkey_to_throw: i64;
                    // perform the test
                    if *item % monkey.test.val == 0 {
                        monkey_to_throw = monkey.res_true;
                    } else {
                        monkey_to_throw = monkey.res_false;
                    }
                    //println!("Adding item {} to throw to monkey {}", *item, monkey_to_throw);
                    items_to_throw.push((*item, monkey_to_throw));
                } // end for
            } // end scope
            // throw the item to the correct monkey
            for (item, monkey_to_throw) in items_to_throw {
                // throw item 
                self.monkey_pen.get_mut(&monkey_to_throw).unwrap().items.push(item);
                // remove item from current monkey
                self.monkey_pen.get_mut(&monkey_id).unwrap().items.retain(|&x| x != item);
            }
        }
    }

    fn parse_monkey(&mut self, input: String) {
        /* Average monkey input:
        Monkey 2:
            Starting items: 52, 95
            Operation: new = old * old // need to handle refs to self and other monkeys
            Test: divisible by 3
                If true: throw to monkey 5
                If false: throw to monkey 4
        */
        // strip trailing newline
        let input = input.trim().to_string();
        println!("input:\n{}", input);
        // declare parts of a monkey
        let mut monkey_num: i64 = 0;
        let mut items: Vec<i64> = Vec::new();
        let mut op: MonkeyOp = MonkeyOp {op: Ops::Unknown, val: -999};
        let mut test: MonkeyOp = MonkeyOp {op: Ops::Unknown, val: -999};
        let mut res_true: i64 = -1;
        let mut res_false: i64 = -1;
        
        // enumerate over the lines
        let lines = input.lines();
        for (idx, val) in lines.enumerate() {
            println!("idx {}: val {}", idx, val);
            match idx {
                0 => {monkey_num = self.handle_num(val); println!("monkey_num: {}", monkey_num);},
                1 => {items = self.handle_items(val); println!("items: {:?}", items);},
                2 => {op = self.handle_op(val); println!("op: {:?}", op);},
                3 => {test = self.handle_test(val); println!("test: {:?}", test);},
                4 => {res_true = self.handle_res(val); println!("res_true: {}", res_true);},
                5 => {res_false = self.handle_res(val); println!("res_false: {}", res_false);},
                _ => {
                      println!("Invalid input"); 
                      exit(1);}
            
            }
        }
        self.monkey_activity.insert(monkey_num, 0);
        self.monkey_pen.insert(monkey_num, Monkey {items, op, test, res_true, res_false});
        println!("Monkey num: {}\n{:#?}", monkey_num, self.monkey_pen.get(&monkey_num).unwrap());
    }

    fn handle_num(&self, input: &str) -> i64 {
        // avg input "7:"
        let mut num: i64 = 0;
        let input = input.replace(" ", "");
        let input = input.replace(":", "");
        num = input.parse::<i64>().unwrap();
        num
    }

    fn handle_items(&self, input: &str) -> Vec<i64> {
        // avg input "Starting items: 52, 95"
        let mut items: Vec<i64> = Vec::new();
        let input = input.replace("Starting items: ", "");
        let input = input.replace(" ", "");
        // split by comma
        let input = input.split(",");
        for i in input {
            items.push(i.parse::<i64>().unwrap());
        }
        items
    }

    fn handle_op(&self, input: &str) -> MonkeyOp {
        // hard input "Operation: new = old * old"
        // avg input "Operation: new = old + 10"
        let mut monkey_op: MonkeyOp = MonkeyOp {op: Ops::Unknown, val: -999};
        let input = input.replace("Operation: new = old ", "");
        let input = input.replace(" ", "");

        // match first character
        match &input[0..1] {
            "+" => monkey_op.op = Ops::Add,
            "*" => monkey_op.op = Ops::Mul,
            "/" => monkey_op.op = Ops::Div,
            _ => {println!("Invalid operation ops"); exit(1)}
        }
        // match remaining characters
        match &input[1..] {
            "old" => monkey_op.val = -1,
            _ => monkey_op.val = input[1..].parse::<i64>().unwrap()
        }
        println!("monkey_op: {:?}", monkey_op);
        monkey_op
    }
    
    fn handle_test(&self, input: &str) -> MonkeyOp {
        // avg input "Test: divisible by 3"
        let mut monkey_op: MonkeyOp = MonkeyOp {op: Ops::Div, val: -999};
        let input = input.replace("Test: divisible by ", "");
        let input = input.replace(" ", "");
        monkey_op.val = input.parse::<i64>().unwrap();
        monkey_op
    }

    fn handle_res(&self, input: &str) -> i64 {
        // avg input "If true: throw to monkey 5"
        let mut res: i64 = -1;
        let input = input.replace("If true: throw to monkey ", "");
        let input = input.replace("If false: throw to monkey ", "");
        let input = input.replace(" ", "");
        res = input.parse::<i64>().unwrap();
        res
    }
    fn set_lcm(&mut self) {
        // find the lcm of all the monkeys
        let mut lcm: i64 = 1;
        for i in self.monkey_pen.values() {
            lcm *= i.test.val as i64;
        }
        self.lcm = lcm;
    }
}


pub fn run(input: String) -> (i128, i128) { 
    // split input by "Monkey"
    let input = input.split("Monkey");
    let mut monkey_solver: MonkeySolver = MonkeySolver::new();

    for i in input {
        // print the input
        println!("{}", i);
        monkey_solver.parse_monkey(i.to_string());
    }
    monkey_solver.set_lcm();
    // play 10000 rounds
    for _round in 0..10000 {
        //println!("round: {}", round);
        monkey_solver.play_round();
    }
    // find the two highest values in the monkey_activity hashmap
    println!("monkey_activity: {:?}", monkey_solver.monkey_activity);
    let mut part1: i64 = 0;
    for (idx, val) in monkey_solver.monkey_activity.values().sorted().rev().enumerate() {
        if idx > 1 {
            break;
        }
        if part1 == 0 {
            part1 = *val as i64;
        }
        else {
            part1 *= *val as i64;
        }
    }

    println!("monkey_activity pt1: {:?}", part1);
    // return the output
    (part1.into(), 0)
}