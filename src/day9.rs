use std::collections::HashSet;

// just want to access with .x and .y
#[derive(Debug, Clone, Copy)]
#[derive(PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}
struct Grid {
    body: Vec<Point>,
    tail_positions: HashSet<Point>,
}

impl Grid {
    fn new(len: usize) -> Grid {
        let mut tail_vec = Vec::with_capacity(len);
        for _i in 0..len {
            tail_vec.push(Point {x:0, y:0});
        }
        Grid {body:tail_vec, tail_positions: HashSet::new() }
    }
    // write a function that visualizes the grid
    fn visalize_grid(&self) {
        print!("{}[2J", 27 as char);
        if self.body.is_empty() {
            //println!("The body is empty");
            return;
        }
        // find the max and min x and y values
        let mut max_x = 25;
        let mut min_x = 0;
        let mut max_y = 25;
        let mut min_y = 0;
        // dynamic scaling if the snake is too big
        for i in 0..self.body.len() {
            if self.body[i].x > max_x {
                max_x = self.body[i].x;
            }
            if self.body[i].x < min_x {
                min_x = self.body[i].x;
            }
            if self.body[i].y > max_y {
                max_y = self.body[i].y;
            }
            if self.body[i].y < min_y {
                min_y = self.body[i].y;
            }
        }
        
        let max_x = 30;
        let min_x = -30;
        let max_y = 20;
        let min_y = 0;
        let start = Point {x: 30, y: 10};
        // create a grid of size (max_x - min_x) * (max_y - min_y)
        let mut grid: Vec<Vec<String>> = vec![vec!['.'.to_string(); (max_x - min_x + 1) as usize]; (max_y - min_y + 1) as usize];

        for mut i in 0..self.body.len() {
            // basically iterate over the body backwards
            i = self.body.len() - i - 1;
            // get the row
            let tmp = &mut grid[(self.body[i].y.abs()) as usize];
            // handle the head
            if i != 0 {
                tmp[(start.x + self.body[i].x) as usize] = i.to_string();
            }
            else {
                tmp[(start.x + self.body[i].x) as usize] = "H".to_string();
            }
        }
        // print the grid
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                print!("{}", grid[i][j]);
            }
            //println!("");
        }
        // sleep for 1 second
        std::thread::sleep(std::time::Duration::from_millis(500));
    }


    fn step(&mut self , dir: char) {
        let mut starting_head = self.body[0].clone();
        ////println!("Starting position: {:?}", starting_head);
        match dir {
            'U' => {
                self.body[0].y += 1;
            },
            'D' => {
                self.body[0].y -= 1;
            },
            'L' => {
                self.body[0].x -= 1;
            },
            'R' => {
                self.body[0].x += 1;
            },
            _ => {
                //println!("Invalid direction: {}", dir);
            }
        }
        ////println!("Tail positions: {:?}", self.body[self.body.len()-1]);
        self.tail_positions.insert(self.body[self.body.len()-1]);

        ////println!("head: ({}, {})", self.body[0].x, self.body[0].y);
        //self.print_grid();
        // itterate over the rest of the body
        for i in 1..self.body.len() {
            ////println!("self.body[{}]: ({}, {})", i, self.body[i].x, self.body[i].y);
            let starting_head_new = self.body[i].clone();

            let x_diff = self.body[i-1].x - self.body[i].x;
            let y_diff = self.body[i-1].y - self.body[i].y;
            
            if (x_diff.abs() == 1 || y_diff.abs() == 1) && (x_diff.abs() > 1 || y_diff.abs() > 1) {
                ////println!("tail is diagonal from head");
                // find which diagonal it is (up right, up left, down right, down left)
                // and move the tail in that direction
                if x_diff > 0 && y_diff > 0 {
                    // up right
                    self.body[i].x += 1;
                    self.body[i].y += 1;
                }
                else if x_diff > 0 && y_diff < 0 {
                    // down right
                    self.body[i].x += 1;
                    self.body[i].y -= 1;
                }
                else if x_diff < 0 && y_diff > 0 {
                    // up left
                    self.body[i].x -= 1;
                    self.body[i].y += 1;
                }
                else if x_diff < 0 && y_diff < 0 {
                    // down left
                    self.body[i].x -= 1;
                    self.body[i].y -= 1;
                }

            }
            else if (x_diff.abs() > 1 || y_diff.abs() > 1) && (x_diff.abs() == 0 || y_diff.abs() == 0) {
                // tail is in a straight line from head
                ////println!("tail is in a straight line from head");
                if x_diff > 0 {
                    // tail is to the right of head
                    self.body[i].x += 1;
                }
                else if x_diff < 0 {
                    // tail is to the left of head
                    self.body[i].x -= 1;
                }
                else if y_diff > 0 {
                    // tail is above head
                    self.body[i].y += 1;
                }
                else if y_diff < 0 {
                    // tail is below head
                    self.body[i].y -= 1;
                }
            }
            // not even quite sure which case this handles but it works
            else if x_diff.abs() > 1 || y_diff.abs() > 1 {
                self.body[i] = starting_head.clone();
            }

            starting_head = starting_head_new;
        }
        ////println!("\n\n")
        self.tail_positions.insert(self.body[self.body.len()-1]);

    }
}


pub fn run(input: String) -> (i128, i128) { 
    let mut grid = Grid::new(2);
    let mut gridpt2 = Grid::new(10);
    for line in input.lines() {
        // parse first char as direction
        let dir = line.chars().nth(0).unwrap();
        // parse rest of line as a number
        let num = line[2..].parse::<i32>().unwrap();
        ////println!("dir: {}, num: {}", dir, num);
        for _ in 0..num {
            grid.step(dir);
            gridpt2.step(dir);
            grid.visalize_grid();
        }
        //std::thread::sleep(std::time::Duration::from_millis(5000));
    }
    let part1 = grid.tail_positions.len() as i128;
    let part2 = gridpt2.tail_positions.len() as i128;
    (part1, part2)
}