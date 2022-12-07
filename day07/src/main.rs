use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    println!("part_one: {}", part_one("input1.txt"));
    println!("part_two: {}", part_two("input1.txt"));
}

fn part_one(filepath: &str) -> i64 {
    let content = get_content(filepath.to_string());
    let mut root = Folder::new("/".to_string());
    let mut current_folder: Vec<String> = Vec::new();
    for line in content.lines() {
        if line.starts_with('$') {
            let command = extract_command(line);
            match command.name.as_str() {
                "cd" => {
                    let argument = command.argument.unwrap();
                    if argument == "/" {
                        current_folder.clear();
                    } else if argument == ".." {
                        current_folder.pop();
                    } else {
                        current_folder.push(argument);
                    }
                }
                "ls" => {} //doesn't really change anything
                _ => panic!("Unexpected command {}", command.name),
            }
        } else if line.starts_with("dir") {
            let name = line.strip_prefix("dir ").unwrap();
            let folder = Folder::new(name.to_string());
            let mut target_folder = &mut root;
            for name in &current_folder {
                target_folder = target_folder.folders.get_mut(name.as_str()).unwrap();
            }
            target_folder.add_folder(folder);
        } else {
            let mut split = line.split(' ');
            let size: i64 = split.next().unwrap().parse().unwrap();
            let name = split.next().unwrap().to_string();
            let mut target_folder = &mut root;
            let file = FileItem { name, size };
            for name in &current_folder {
                target_folder = target_folder.folders.get_mut(name.as_str()).unwrap();
            }
            target_folder.files.push(file);
        }
    }
    calculate_size(&root)
}

fn part_two(filepath: &str) -> i64 {
    let content = get_content(filepath.to_string());
    let mut root = Folder::new("/".to_string());
    let mut current_folder: Vec<String> = Vec::new();
    for line in content.lines() {
        if line.starts_with('$') {
            let command = extract_command(line);
            match command.name.as_str() {
                "cd" => {
                    let argument = command.argument.unwrap();
                    if argument == "/" {
                        current_folder.clear();
                    } else if argument == ".." {
                        current_folder.pop();
                    } else {
                        current_folder.push(argument);
                    }
                }
                "ls" => {} //doesn't really change anything
                _ => panic!("Unexpected command {}", command.name),
            }
        } else if line.starts_with("dir") {
            let name = line.strip_prefix("dir ").unwrap();
            let folder = Folder::new(name.to_string());
            let mut target_folder = &mut root;
            for name in &current_folder {
                target_folder = target_folder.folders.get_mut(name.as_str()).unwrap();
            }
            target_folder.add_folder(folder);
        } else {
            let mut split = line.split(' ');
            let size: i64 = split.next().unwrap().parse().unwrap();
            let name = split.next().unwrap().to_string();
            let mut target_folder = &mut root;
            let file = FileItem { name, size };
            for name in &current_folder {
                target_folder = target_folder.folders.get_mut(name.as_str()).unwrap();
            }
            target_folder.files.push(file);
        }
    }
    calculate_smallest_that_frees_up_enough(&root)
}

fn extract_command(line: &str) -> Command {
    let mut split = line.split(' ');
    split.next(); // discard $
    let command_name = split.next().unwrap().to_string();
    let command_argument = split.next().map(|s| s.to_string());
    Command::new(command_name, command_argument)
}

fn calculate_size(root: &Folder) -> i64 {
    let mut sum = match root.size() {
        x if x > 100000 => 0,
        x => x,
    };
    for (_, folder) in &root.folders {
        sum += calculate_size(folder);
    }
    sum
}

fn calculate_smallest_that_frees_up_enough(root: &Folder) -> i64 {
    let max: i64 = 70000000;
    let target = 30000000;
    let curr_size = root.size();
    let required = target - (max - curr_size);
    min_that_satisfies(root, required)
}

fn min_that_satisfies(root: &Folder, required: i64) -> i64 {
    let mut min = i64::MAX;
    if root.size() >= required {
        min = root.size();
    }
    for (_, folder) in &root.folders {
        min = min.min(min_that_satisfies(folder, required));
    }
    min
}
struct Folder {
    folders: HashMap<String, Folder>,
    files: Vec<FileItem>,
    name: String,
}

impl Folder {
    fn new(name: String) -> Self {
        Folder {
            folders: HashMap::new(),
            files: Vec::new(),
            name,
        }
    }

    fn add_folder(&mut self, folder: Folder) {
        self.folders.insert(folder.name().to_string(), folder);
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn size(&self) -> i64 {
        let folder_sum: i64 = self.folders.iter().map(|(_, folder)| folder.size()).sum();
        let file_sum: i64 = self.files.iter().map(|file| file.size).sum();
        folder_sum + file_sum
    }
}

struct FileItem {
    name: String,
    size: i64,
}

struct Command {
    name: String,
    argument: Option<String>,
}

impl Command {
    fn new(name: String, argument: Option<String>) -> Self {
        Command { name, argument }
    }
}

fn get_content(filepath: String) -> String {
    let path = Path::new(&filepath);
    let mut file = File::open(path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    content
}
