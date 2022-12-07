#[derive(Debug)]
struct Directory {
    has_subdir: bool,
    size: i128,
    subdir: Vec<Option<Box<Directory>>>
}

impl Directory {
    fn new() -> Directory {
        Directory { has_subdir: (false), 
                    size: (0), 
                    subdir: (Vec::new()) }
    }


}


pub fn run(input: String) -> (i128, i128) {
    let root: Directory = Directory::new();
    println!("{:?}", root);
    (0,0)
}