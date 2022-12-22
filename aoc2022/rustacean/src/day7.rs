use std::collections::HashMap;

fn main() {
    let filename = "data/7.prod";
    find_file_size(std::fs::read_to_string(filename).unwrap());
}

fn find_file_size(s: String) -> TreeItem {
    let mut root = TreeItem::new(FileType::Diretory, 0, Vec::new());
    let mut cwd = Vec::new();
    for line in s.lines() {
        let split_line = line.split_whitespace().collect::<Vec<&str>>();
        match (split_line[0], split_line[1]){
            ("$", "cd") => {
                if split_line[2] == ".."{
                    cwd.pop();
                }else {
                cwd.push(split_line[2]);
                }

                root.content.push(TreeItem { 
                    file_type: FileType::Diretory,
                    size: 0, content: Vec::new() 
                })
            },
            ("$", "ls") => {},
            ("dir", _dir_name) => {
                root.content.push(TreeItem { 
                    file_type: FileType::Diretory,
                    size: 0, content: Vec::new() 
                })
            },
            (file_size, _file_name) => {
                root.size += file_size.parse::<i32>().unwrap();
            },
        }
        println!("split_line = {:?}", split_line);
    }
    let mut sum = 0;
    for item in &root.content{
        println!("item = {:?}", &item);
        sum += item.get_file_size();
    }
    println!("sum = {:?}", sum);
    println!("cwd = {:?}", cwd);
    return root;
}

#[derive(Debug)]
enum FileType {
    File,
    Diretory,
}
#[derive(Debug)]
struct TreeItem {
    file_type: FileType,
    size: i32,
    content: Vec<TreeItem>,
}

impl TreeItem {
    fn new(file_type: FileType, size: i32, content: Vec<TreeItem>) -> TreeItem {
        TreeItem {
            file_type,
            size,
            content,
        }
    }

    fn get_file_size(&self) -> i32 {
        match &self.file_type {
            FileType::File => return self.size,
            FileType::Diretory => {
                let mut total_size: i32 = 0;
                for item in &self.content {
                    total_size += item.size;
                }
                return total_size;
            }
        }
    }
}
