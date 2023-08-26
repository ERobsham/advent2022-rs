use std::{
    collections::HashMap, 
    rc::Rc, 
    cell::RefCell, fmt::Display,
};

use crate::Part;

type SharedFilePtr = Rc<File>;

enum File {
    File{
        name: String,
        size: usize,
    },
    Directory{ 
        parent: Option<SharedFilePtr>,
        name: String, 
        files: RefCell<HashMap<String, SharedFilePtr>>,
    },
}

impl File {
    
    fn add_file(&self, new_file: File) {
        match self {
            File::Directory { parent: _, name: _, files } => {
                let dir_name = { new_file.get_name() };

                if let Ok(mut files) = files.try_borrow_mut() {
                    files.insert(dir_name, Rc::new(new_file));
                }
            },
            _ => { return; },
        };
    }

    fn get_dir(&self, name: &String) -> Option<SharedFilePtr> {
        match self {
            File::Directory { parent: _, name: _, files } => {
                if let Some(dir) = files.borrow().get(name) {
                    Some(dir.clone())
                } else {
                    None
                }
            },
            _ => { None },
        }
    }

    fn get_parent(&self) -> Option<SharedFilePtr> {
        match self {
            File::Directory { parent, name: _, files: _ } => {
                if let Some(parent) = parent {
                    Some(parent.clone())
                } else {
                    None
                }
            },
            File::File { name: _, size: _ } => None,
        }
    }

    fn get_name(&self) -> String {
        match self {
            File::File{ name, size: _} => name.to_owned(),
            File::Directory { parent: _, name, files: _ } => {
                name.to_owned()
            },
        }
    }

    fn size(&self) -> usize {
        match self {
            File::File{ name: _, size} => size.to_owned(),
            File::Directory { parent: _, name: _, files } => {
                files.borrow().iter().fold(0usize, |acc, (_, file)| acc + file.size())
            },
        }
    }

}

impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            File::File { name, size } => { 
                f.write_fmt(format_args!("{} : {} \n", name, size))?; 
            },
            File::Directory { parent: _, name, files } => {
                f.write_fmt(format_args!("{} -> \n", name))?;
                files.borrow().iter().try_for_each(|(_, file)| {
                    let fmt_str = { file.to_string() };
                    f.write_fmt(format_args!("    {}", fmt_str))
                })?;
            },
        }
        Ok(())
    }
}



const THRESHOLD_PT1: usize = 100_000;

const FILESYS_MAX:usize = 70_000_000;
const UPDATE_SIZE_REQ:usize = 30_000_000;

pub(crate) fn solve(input: Box<dyn Iterator<Item = String>>, part: Part) -> String {
    
    let filesys = parse_filesys_from(input);

    let output = match part {
        Part::Part1 => sum_folders_with_max_size_in(&filesys, THRESHOLD_PT1),
        Part::Part2 => {
            let unused_space = FILESYS_MAX - filesys.size();
            let space_needed = UPDATE_SIZE_REQ - unused_space;

            min_folder_size_above_threshold_in(&filesys, space_needed, usize::MAX)
        },
    };

    format!("{}", output)
}


fn parse_filesys_from(mut lines: Box<dyn Iterator<Item=String>>) -> Rc<File> {

    let root = Rc::new(
        File::Directory { parent: None, name: "/".into(), files: RefCell::new(HashMap::new()) }
    );
    let mut cur_dir = root.clone();

    // parse the input
    while let Some(line) = lines.next() {
        
        // handle `cd`
        if let Some(dir) = line.strip_prefix("$ cd ") {
            match dir {
                "/" => { cur_dir = root.clone() },
                ".." => {
                    let new_dir: Option<SharedFilePtr> = if let Some(parent) = cur_dir.get_parent() {
                        Some(parent)
                    } else {
                        None
                    };
                    
                    if new_dir.is_some() {
                        cur_dir = new_dir.unwrap();
                    }
                },
                _ => { 
                    let dir_name = String::from(dir);
                    let new_dir: Option<SharedFilePtr> = if let Some(child) = cur_dir.get_dir(&dir_name) {
                        Some(child)
                    } else {
                        None
                    };

                    if new_dir.is_some() {
                        cur_dir = new_dir.unwrap();
                    }
                },
            }
            continue;
        }

        // handle `ls`
        if let Some(_) = line.strip_prefix("$ ls") {
            continue;
        }

        // handle parsing `ls` output
        if let Some((left, right)) = line.split_once(' ') {
            let file =  match left.parse::<usize>() {
                Ok(size) => File::File { name: right.into(), size },
                Err(_) => File::Directory { parent: Some(cur_dir.clone()), name: right.into(), files: RefCell::new(HashMap::new()) },
            };

            cur_dir.add_file(file);

        }
    }
    drop(cur_dir);

    root
}

fn sum_folders_with_max_size_in(f: &Rc<File>, threshold: usize) -> usize {
    let mut sum = 0_usize;
    match **f {
        File::Directory { parent:_, name:_, ref files } => {
            if f.size() <= threshold {
                sum += f.size();
            }

            let files_ref = files.borrow();
            let mut file_list = files_ref.values();
            while let Some(file) = file_list.next() {
                sum += sum_folders_with_max_size_in(file, threshold);
            };
        },
        _=> (),
    };

    sum
}

fn min_folder_size_above_threshold_in(f: &Rc<File>, threshold: usize, mut cur_min: usize) -> usize {
    match **f {
        File::Directory { parent:_, name:_, ref files } => { 
            if f.size() < threshold {
                return cur_min;
            }
            
            if f.size() < cur_min {
                cur_min = f.size();
            }

            let files_ref = files.borrow();
            let mut file_list = files_ref.values();
            while let Some(file) = file_list.next() {
                let c_min =  min_folder_size_above_threshold_in(file, threshold, cur_min);
                if c_min <= cur_min {
                    cur_min = c_min;
                }
            };

            cur_min
        },
        _ => cur_min,
    }
}

#[test]
// sanity check vs example input
fn test_input() {
    const EXAMPLE: &str = r"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    let lines = EXAMPLE.split('\n')
        .map(|item| String::from(item));

    let output = solve(Box::new(lines.clone()), Part::Part1);
    assert_eq!(output.as_str(), "95437");
    
    let output = solve(Box::new(lines), Part::Part2);
    assert_eq!(output.as_str(), "24933642");
}
