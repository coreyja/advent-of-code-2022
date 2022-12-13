use std::{collections::HashMap, path::PathBuf};

#[derive(Debug)]
enum FileLike {
    Dir { path: PathBuf },
    File { path: PathBuf, size: usize },
}

impl FileLike {
    fn path(&self) -> PathBuf {
        match self {
            FileLike::Dir { path } => path,
            FileLike::File { path, .. } => path,
        }
        .clone()
    }

    fn file_size(&self) -> usize {
        match self {
            FileLike::Dir { .. } => 0,
            FileLike::File { size, .. } => *size,
        }
    }
}

#[derive(Debug)]
struct FileSystem {
    files: HashMap<PathBuf, FileLike>,
}

impl FileSystem {
    fn parse(input: &str) -> Self {
        let mut files = HashMap::new();

        let mut current_directory: PathBuf = "/".into();

        for line in input.lines() {
            if let Some(command) = line.strip_prefix("$ ") {
                if let Some(cd_location) = command.strip_prefix("cd ") {
                    if cd_location == "/" {
                        current_directory = "/".into();
                    } else if cd_location == ".." {
                        current_directory.pop();
                    } else {
                        current_directory.push(cd_location);
                    }
                } else if command == "ls" {
                    // We don't need to do anything here,
                    // but we know we can read until the next command
                }
            } else {
                let mut split = line.split(' ');

                let file_size_or_dir = split.next().unwrap();
                let path = split.next().unwrap();

                let mut new_file_path = current_directory.clone();
                new_file_path.push(path);

                let filelike: FileLike = match file_size_or_dir {
                    "dir" => FileLike::Dir {
                        path: new_file_path.clone(),
                    },
                    size => FileLike::File {
                        path: new_file_path.clone(),
                        size: size.parse().unwrap(),
                    },
                };

                files.insert(new_file_path, filelike);
            }
        }

        Self { files }
    }

    fn total_size(&self, filelike: &FileLike) -> usize {
        match filelike {
            FileLike::File { size, .. } => *size,
            FileLike::Dir {
                path: directory_path,
            } => self
                .files
                .values()
                .filter(|f| {
                    let p = f.path();

                    p.starts_with(directory_path)
                })
                .map(|f| f.file_size())
                .sum(),
        }
    }
}

pub fn part_1(input: &str) -> usize {
    let filesystem = FileSystem::parse(input);

    filesystem
        .files
        .values()
        .filter(|f| matches!(f, FileLike::Dir { .. }))
        .filter(|dir| filesystem.total_size(dir) < 100_000)
        .map(|dir| filesystem.total_size(dir))
        .sum()
}

pub fn part_2(input: &str) -> usize {
    let filesystem = FileSystem::parse(input);

    let root = FileLike::Dir { path: "/".into() };

    const TOTAL_SPACE: usize = 70_000_000;
    const SPACE_NEEDED: usize = 30_000_000;

    let used_space = filesystem.total_size(&root);
    let space_available = TOTAL_SPACE - used_space;

    let space_needed_to_delete = if space_available > SPACE_NEEDED {
        panic!("We don't need to delete anything");
    } else {
        SPACE_NEEDED - space_available
    };

    filesystem
        .files
        .values()
        .filter(|f| matches!(f, FileLike::Dir { .. }))
        .filter(|dir| filesystem.total_size(dir) > space_needed_to_delete)
        .map(|dir| filesystem.total_size(dir))
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_input_part_1() {
        let input = include_str!("example.input");
        let ans = part_1(input);

        assert_eq!(ans, 95437);
    }

    #[test]
    fn my_input_part_1() {
        let input = include_str!("my.input");
        let ans = part_1(input);

        assert_eq!(ans, 1086293);
    }

    #[test]
    fn example_input_part_2() {
        let input = include_str!("example.input");
        let ans = part_2(input);

        assert_eq!(ans, 24933642);
    }

    #[test]
    fn my_input_part_2() {
        let input = include_str!("my.input");
        let ans = part_2(input);

        assert_eq!(ans, 366028);
    }
}
