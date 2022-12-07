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
    let mut currentFolder = &mut Folder::new("/".to_string(), None);
    for line in content.lines() {
        if line.starts_with('$') {
            let command = extract_command(line);
            match command.name.as_str() {
                "cd" => {
                    let argument = command.argument.unwrap();
                    if argument == "/" {
                        while currentFolder.parent.is_some() {
                            currentFolder = currentFolder.parent.unwrap();
                        }
                    } else if argument == ".." {
                        currentFolder = currentFolder.parent.unwrap();
                    } else {
                        currentFolder.folders.get_mut(argument.as_str()).unwrap();
                    }
                }
                "ls" => {} //doesn't really change anything
                _ => panic!("Unexpected command {}", command.name),
            }
        } else if line.starts_with("dir") {
            let name = line.strip_prefix("dir ").unwrap();
            let folder = Folder::new(name.to_string(), Some(currentFolder));
            currentFolder.add_folder(folder);
        }
    }
    0
}

fn part_two(filepath: &str) -> i64 {
    0
}

fn extract_command(line: &str) -> Command {
    let mut split = line.split(' ');
    split.next(); // discard $
    let command_name = split.next().unwrap().to_string();
    let command_argument = split.next().map(|s| s.to_string());
    Command::new(command_name, command_argument)
}

struct Folder<'this, 'parent: 'this> {
    folders: HashMap<String, Folder<'this, 'this>>,
    files: Vec<FileItem>,
    parent: Option<&'parent mut Folder<'parent, 'parent>>,
    name: String,
}

impl<'this, 'parent: 'this> Folder<'this, 'parent> {
    fn new(name: String, parent: Option<&'parent mut Folder<'parent, 'parent>>) -> Self {
        Folder {
            folders: HashMap::new(),
            files: Vec::new(),
            parent,
            name,
        }
    }

    fn add_folder(&mut self, folder: Folder<'this, 'parent>) {
        self.folders.insert(folder.name().to_string(), folder);
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn get_folder(&mut self, name: String) -> Option<&mut Folder<'this, 'this>> {
        self.folders.get_mut(name.as_str())
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
