

pub fn run(input: String) -> (i128, i128) {
    for line in input.lines() {
        println!("Half line lenght {}", line.len()/2);
        println!("Even = {}", line.len()%2==0);
        
    }


    (0,0)
}