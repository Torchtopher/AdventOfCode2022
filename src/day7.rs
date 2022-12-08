use std::collections::HashMap;
use std::process::{exit, Command};

#[derive(Debug)]
struct Directory {
    has_subdir: bool,
    size: i128,
    subdirs: HashMap<String, Directory>,
    name: String,
    // makes files a hashmap of name and size
    // very scared that the same files would have been listed twice
    files: HashMap<String, i128>
}

impl Directory {
    fn new(name: String) -> Directory {
        Directory { has_subdir: (false), 
                    size: (0), 
                    subdirs: HashMap::new(),
                    name: name,
                    files: HashMap::new()}
    }
    fn add_subdir(&mut self, name: String) {
        if self.has_subdir == false {
            self.has_subdir = true;
        }
        //println!("Adding subdir: {}", name);
        self.subdirs.insert(name.clone(), Directory::new(name));
    }

    fn add_file(&mut self, name: String, size: i128) {
        // handle size not here
        // self.size += size;
        // check if file already exists

        self.files.insert(name, size);
    }
}

#[derive(Debug)]
struct FileSystem {
    root: Directory,
    cwd: Vec<String>,
}

impl FileSystem {
    fn new() -> FileSystem {
        FileSystem { root: Directory::new("/".to_string()), cwd: Vec::new() }
    }

    fn run_ls(&mut self, arg: String) {
        //println!("Ran ls with arg:\n{}", arg);
        // find current dir
        let mut current_dir = &mut self.root;
        for dir in &self.cwd {
            //println!("current_dir subdirs: {:#?}", current_dir.subdirs);
            //println!("\ndir: {}", dir);
            // print cwd
            //println!("cwd: {:#?}", self.cwd);
            current_dir = current_dir.subdirs.get_mut(dir).unwrap();
        }
        // print current dir
        //println!("current_dir: {:#?}", current_dir);
        //println!("\n\n");
        for line in arg.lines() {
            let mut lineS: String = line.to_string();
            lineS = lineS.trim().to_string();
            //println!("line: {}", lineS);
            // check if the line is a file
            // get first character of lineS
            // if it is a number then it is a file
            let ch = lineS.chars().nth(0).unwrap();
            
            if ch.is_numeric() {
                let lineS = lineS.split(" ").collect::<Vec<&str>>();
                // first element is ""
                //println!("lineS: {:?}", lineS);
                let name = lineS[1].to_string();
                let size = lineS[0].parse::<i128>().unwrap();
                //println!("name: {}, size: {}", name, size);
                current_dir = &mut self.root;
                current_dir.size += size;
                for dir in &self.cwd {
                    //println!("dir: {}", dir);
                    //println!("cwd: {:#?}", self.cwd);
                    current_dir = current_dir.subdirs.get_mut(dir).unwrap();
                    //println!("current_dir: {:#?}", current_dir);
                    // sleep for 1 second
                    // std::thread::sleep(std::time::Duration::from_secs(1));
                    current_dir.size += size;
                }
                current_dir.add_file(name.clone(), size);
                //println!("Added file: {}\n{:#?}", name, current_dir.files);
            }
            else if (lineS.contains("dir")) { 
                lineS = lineS.replace("dir ", "");
                current_dir.add_subdir(lineS.to_string());
            }
            else {
                //println!("Error: line is not a file or dir");
                exit(1);
            }
        }
        // print hashmap of dirs
        //println!("List of dirs: {:#?}", current_dir.subdirs);
        
    }
    
    fn run_cd(&mut self, arg: String) {
        //println!("Ran cd with arg: {}", arg);
        // change arg to a str
        let arg = arg.as_str().trim();
        match arg {
            ".." => {
                //println!("Moving up a dir");
                if self.cwd.len() > 0 {
                    self.cwd.pop();
                }
            },
            "/" => {
                //println!("Moving to root");
                self.cwd.clear();
            },
            _ => {
                // find current dir
                let current_dir = &mut self.root;
                let mut current_dir = &mut self.root;
                for dir in &self.cwd {
                    // print cwd
                    //println!("cwd: {:#?}", self.cwd);
                    current_dir = current_dir.subdirs.get_mut(dir).unwrap();
                }
                // check if the dir exists
                if current_dir.subdirs.contains_key(arg) {
                    //println!("Dir exists");
                    self.cwd.push(arg.to_string());
                    //println!("cwd: {:?}", self.cwd);
                }
                else {
                    //println!("Dir {} does not exist", arg);
                    exit(-1)
                }
            }
        }
    }
    
    fn recurse_dirs(&self, dir: Option<&Directory>, size: &mut i128) -> i128 {
        let dir_2_loop: &Directory;
        match dir {
            Some(dir) => {
                dir_2_loop = dir;
            },
            None => {
                dir_2_loop = &self.root;
                let total_size = dir_2_loop.size;
            }
        }
        let delta_size = 700_000_000;

        for (name, dir) in &dir_2_loop.subdirs {
            //println!("{}: {}", name, dir.size);
            if dir.size <= 100000 {
                *size += dir.size;
            }
            self.recurse_dirs(Some(dir), size);
        }
        *size
    }
    fn recurse_dirspt2(&self, dir: Option<&Directory>, sizes: &mut Vec<i128>) -> Vec<i128> {
        let dir_2_loop: &Directory;
        match dir {
            Some(dir) => {
                dir_2_loop = dir;
            },
            None => {
                dir_2_loop = &self.root;
            }
        }
        
        for (name, dir) in &dir_2_loop.subdirs {
            //println!("{}: {}", name, dir.size);
            
            sizes.push(dir.size);
            self.recurse_dirspt2(Some(dir), sizes);
        }
        sizes.to_vec()
    }
}

pub fn run(input: String) -> (i128, i128) {
    // cwd is the indexs of the subdir vec from root
    // so if cwd is [0, 1, 2] then we are in root.subdir[0].subdir[1].subdir[2]
    let mut fs = FileSystem::new();
    let mut cmd: String = "".to_string();
    //println!("{:?}", fs);

    //println!("input: {:?}", input);
    let input = input.split("$").collect::<Vec<&str>>();
    //println!("input: {:?}", input);
    for command_full in input {
        ////println!("command_full: {}", command_full);
        // check if the string "ls" is in the first line of input
        if command_full.contains("ls\n") {
            // remove the first line
            let command = command_full.replace("ls\n", "");
            ////println!("command:\n{}", command.trim());
            fs.run_ls(command);
            ////println!("fs: {:#?}", fs);
        }
        else if command_full.contains("cd") {
            // remove the first line
            // ahhhhhhhhhhhh why does replace do them all
            // just cut off the first 2 characters
            let command = command_full[3..].to_string();
            // let command = command.replace("cd", "");
            //let command = command_full.replace("cd", "");
            ////println!("command:\n{}", command.trim());
            fs.run_cd(command);
            ////println!("fs: {:#?}", fs);
        }
        else {
            //println!("no command found");
        }

    }
    //println!("\n\n");
    println!("fs: {:#?}", fs);
    let mut part1 = 0i128;
    println!("{}", fs.recurse_dirs(None, &mut part1));
    let mut sizes = Vec::new();
    fs.recurse_dirspt2(None, &mut sizes);
    println!("sizes: {:#?}", sizes);
    // find max of sizes
    let max = sizes.iter().max().unwrap();
    println!("Max of fs: {}", fs.root.size);
    let delta = (70_000_000 - fs.root.size - 30_000_000).abs();
    println!("delta: {}", delta);
    // find the smallest number in sizes that is greater than delta
    let mut part2 = 100000000; // set smallest to massive, so it will be replaced
    for size in sizes {
        if size > delta && size < part2 {
            part2 = size;
        }
    }
    (part1, part2)
}