#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32
}

struct CPU {
    x: i32,
    cycles: i32,
    signals: Vec<i32>,
    // make vec of vec of chars to render the cpu
    screen: Vec<Vec<char>>,
    sprite: Point, // use to index into screen use first usize for y and second for x
}

impl CPU {
    fn new() -> CPU {
        CPU {
            x: 1,
            cycles: 0,
            signals: Vec::new(),
            screen: vec![vec!['.'; 40]; 6],
            sprite: Point { x: 1, y: 0 },
        }
    }
    fn check_signal(&mut self) {
        if (self.cycles - 20) % 40 == 0 || self.cycles == 20 {
            self.signals.push(self.x * self.cycles);
        }
    }
    fn update_sprite(&mut self) {
        if self.x.is_negative() {
            self.sprite.x = 999;
        } else {
            self.sprite.x = self.x;
        }
        self.sprite.y = (self.cycles - 1) / 40;
        self.draw_sprite();
    }

    fn draw_sprite(&mut self) {
        // find which pixel the cpu is on, and draw the character if the x and y are the same
        let row = self.cycles % 40;
        let col = self.sprite.y;
        // sprite is 3 pixels wide
        if row == self.sprite.x || row == self.sprite.x + 1 || row == self.sprite.x - 1{
            self.screen[col as usize][row as usize] = '#';
        }
    }

    fn render_screen(&self) {
        // print control character to clear screen
        print!("{}[2J", 27 as char);
        for line in &self.screen {
            for char in line {
                print!("{}", char);
            }
            println!();
        }
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
}


pub fn run(input: String) -> (i128, i128) {
    let mut cpu = CPU::new();
    for line in input.lines() {
        // match first 4 chars
        match &line[..4] {
            "noop" => {
                cpu.update_sprite();
                cpu.render_screen();
                cpu.cycles += 1;
                cpu.check_signal();

            },
            "addx" => {
                for _ in 0..2 {
                    cpu.update_sprite();
                    cpu.render_screen();
                    cpu.cycles += 1;
                    cpu.check_signal();
                }
                // finish addx
                cpu.x += line[5..].parse::<i32>().unwrap();

            },
            _ => {
                println!("invalid input");
            }
        }
    }
    println!("x: {}", cpu.x);
    let part1 = cpu.signals.clone().into_iter().sum::<i32>() as i128;
    println!("signals sum: {:?}", part1);
    cpu.render_screen();
    (part1, 0)
}