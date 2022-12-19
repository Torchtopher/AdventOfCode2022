use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}
struct Map { 
    // (0, 0) is top left, x increases to the right, y increases down
    points: HashMap<(i32, i32), char>,
    lowest_y: i32, // if sand goes below this, its done
    num_sands: i32, // how much sand has been added
    part2 : bool,
}

impl Map {
    fn new(part2: bool) -> Map {
        Map {
            points: HashMap::new(),
            lowest_y: 8,
            num_sands: 0,
            part2: part2,
        }
    }
    fn get_sand_move(&self, pt: &Point) -> Option<Point> {
                        
        // check down
        let down = self.points.get(&(pt.x, pt.y+1));
        // check if its none
        if down == None {
            // check if its the lowest y
            if pt.y+1 > self.lowest_y && self.part2 == false {
                return Some(Point { x: -1, y: -1 }); // sand is falling to abyss
            }
            // sand goes down
            return Some(Point {x: pt.x, y: pt.y+1});
        }
        let downleft = self.points.get(&(pt.x-1, pt.y+1));
        if downleft == None {
            if pt.y+1 > self.lowest_y && self.part2 == false {
                return Some(Point { x: -1, y: -1}); // sand is falling to abyss
            }
            // sand goes downleft
            return Some(Point {x: pt.x-1, y: pt.y+1});
        }
        let downright = self.points.get(&(pt.x+1, pt.y+1));
        if downright == None {
            if pt.y+1 > self.lowest_y && self.part2 == false {
                return Some(Point { x: -1, y: -1 }); // sand is falling to abyss
            }
            // sand goes downright
            return Some(Point {x: pt.x+1, y: pt.y+1});
        }
        None
    }

    fn add_sand(&mut self) -> bool {
        
        if self.points.get(&(500, 0)) == None {

        }
        else {
            return true;
        }
        let mut current_sand = Point {x: 500, y: 0};
        self.visualize();

        while let Some(next_sand) = self.get_sand_move(&current_sand) {
            if next_sand.x == -1 && self.part2 == false {
                //println!("Sand is falling to abyss");
                //std::thread::sleep(std::time::Duration::from_millis(1000));
                current_sand = next_sand;
                break;
            } 
            //println!("current_sand: {:?}", current_sand);
            current_sand = next_sand;
        }
        if current_sand.y == -1 && self.part2 == false{
            //println!("Sand is falling to abyss");
            //std::thread::sleep(std::time::Duration::from_millis(1000));
            return true;
        }
        self.points.insert((current_sand.x, current_sand.y), 'O');
        self.num_sands += 1;
        //self.visualize();
        return false;
    }

    fn parse_points(&mut self, pt1: Point, pt2: Point) {
        if pt1.y - 1 > self.lowest_y {
            self.lowest_y = pt1.y;
        }
        if pt2.y - 1 > self.lowest_y {
            self.lowest_y = pt2.y;
        }
        //println!("pt1: {:?}, pt2: {:?}", pt1, pt2);
        if pt1.x == pt2.x { // means y is diffrent
            let y_diff = (pt2.y - pt1.y).abs();
            //println!("y_diff: {}", y_diff);
            //println!("pt1.y: {}, pt2.y: {}", pt1.y, pt2.y);
            if pt1.y < pt2.y {
                for y in pt1.y..=pt1.y+y_diff {

                    self.points.insert((pt1.x, y), '#');
                    //println!("Adding point4: ({}, {})", pt1.x, y);
                }
            } 
            else {
                for y in pt2.y..=pt2.y+y_diff {
                    //println!("Adding point3: ({}, {})", pt1.x, y);
                    self.points.insert((pt1.x, y), '#');
                }
            }
        }
        else if pt1.y == pt2.y { // means x is diffrent
            let x_diff = (pt2.x - pt1.x).abs();
            //println!("x_diff: {}", x_diff);
            //println!("pt1.x: {}, pt2.x: {}", pt1.x, pt2.x);
            if pt1.x < pt2.x {
                for x in pt1.x..=pt1.x+x_diff {
                    //println!("Adding point: ({}, {})", x, pt1.y);
                    self.points.insert((x, pt1.y), '#');
                }
            } 
            else {
                for x in pt2.x..=pt2.x+x_diff {
                    //println!("Adding point2: ({}, {})", x, pt1.y);
                    self.points.insert((x, pt1.y), '#');
                }
            }
        }
    }

    fn visualize(&self) {
        // print control characters
        println!("{}[2J", 27 as char);
    // only show from x=490 to x=505
        for y in 0..=12{
            for x in 480..=520 {
                let point = self.points.get(&(x, y));
                match point {
                    Some('#') => print!("#"),
                    Some('O') => print!("O"),
                    Some('|') => print!("|"),
                    Some('.') => print!("."),
                    None => print!("."),
                    _ => print!("."),
                }
            }
            println!("");
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    fn add_floor_pt2(&mut self) {
        for x in 0..=10000 { // "infinite" floor
            self.points.insert((x, self.lowest_y + 3), '#');
        }
    }
}



pub fn run(input: String) -> (i128, i128) {
    let mut map = Map::new(false);
    let mut map2 = Map::new(true);
    for line in input.lines() {
        let split = line.split(" -> ");
        let splitvec = split.collect::<Vec<&str>>();
        for (idx, pt) in splitvec.iter().enumerate() {
            if idx + 1 == splitvec.len() {
                break;
            }
            // pt looks like "498,4"
            // transform to a Point with x and y
            let pt1 = splitvec[idx];
            let pt2 = splitvec[idx + 1];
            let pt1 = pt1.split(",").collect::<Vec<&str>>();
            let pt2 = pt2.split(",").collect::<Vec<&str>>();
            let pt1 = Point {
                x: pt1[0].parse::<i32>().unwrap(),
                y: pt1[1].parse::<i32>().unwrap(),
            };
            let pt2 = Point {
                x: pt2[0].parse::<i32>().unwrap(),
                y: pt2[1].parse::<i32>().unwrap(),
            };
            map.parse_points(pt1.clone(), pt2.clone());
            map2.parse_points(pt1, pt2);
        }
    }
    map2.add_floor_pt2();
    //map.visualize();
    loop {
        map.visualize();
        if map.add_sand() {
            break;
        }
    }
    loop {
        if map2.add_sand() {
            break;
        }
    }
    map.visualize();
    (map.num_sands as i128, map2.num_sands as i128)
}


