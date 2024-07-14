use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::fs::OpenOptions;

use crate::dir_operations;
use crate::file_operations;
/// The `DirectorySearcher` struct in Rust provides functionality to search directories for specific
/// files while maintaining a blacklist of paths.
/// 
/// Properties:
/// 
/// * `blacklist`: The `blacklist` property in the `DirectorySearcher` struct is a HashSet that stores
/// strings representing paths that should be excluded from the search. This blacklist is used to skip
/// certain directories during the search process.
/// * `blacklist_file_write`: The `blacklist_file_write` property in the `DirectorySearcher` struct is
/// of type `File`. It is used to store a file handle for writing to the blacklist file. This file
/// handle is used to append entries to the blacklist file when needed, such as when a path is added to
pub struct DirectorySearcher{
    blacklist: HashSet<String>,
    blacklist_file_write: File,
}

impl DirectorySearcher {
    /// The function creates a new instance of DirectorySearcher with a blacklist HashSet and a file for
    /// writing blacklist entries.
    /// 
    /// Returns:
    /// 
    /// A new instance of the `DirectorySearcher` struct is being returned.
    pub fn new() -> Self {
        let file= match OpenOptions::new().append(true).open("src\\blacklist.txt") {
            Ok(file) => file,
            Err(_e) => file_operations::new_file_return_append()
        };
        DirectorySearcher{
            blacklist: HashSet::new(),
            blacklist_file_write: file,
        }
    }
    /// The function `update_blacklist` reads a file named "blacklist.txt" and updates a blacklist
    /// collection with its contents.
    pub fn update_blacklist(&mut self) {
        let file: File = match OpenOptions::new().read(true).open("src\\blacklist.txt") {
            Ok(file) => file,
            Err(_e) => file_operations::new_file_return()
        };
        let reader: BufReader<File> = BufReader::new(file);
        self.blacklist.clear();
        for line in reader.lines() {
            if let Ok(line) = line {
                self.blacklist.insert(line);
            }
        }
    }

    /// The function searches for a specific string in file paths within a given root directory while
    /// avoiding blacklisted paths.
    /// 
    /// Arguments:
    /// 
    /// * `root_path`: The `root_path` parameter is a reference to a string that represents the root
    /// path from which the search operation will start. This path is the starting point for searching
    /// for files or directories containing the specified search term.
    /// * `be_searched_`: The `be_searched_` parameter in the `search` function is a reference to a
    /// string that represents the keyword or phrase that is being searched for within the paths. This
    /// parameter is expected to be in its original form without any modifications.
    /// 
    /// Returns:
    /// 
    /// A vector of strings containing the paths that match the search criteria.
    pub(crate) fn search(&mut self, root_path: &str, be_searched_: &str) -> Vec<String> {
        if self.blacklist.contains(&root_path.to_string()) {
            return Vec::new();
        }
        let be_searched: String = be_searched_.to_lowercase();
        let paths: Vec<String> = self.get_paths(&root_path.trim());
        let mut matchers: Vec<String> = Vec::new();
        let mut dirs: Vec<String> = Vec::new();
        for i in paths.iter() {
            if dir_operations::is_dir(i) {
                dirs.push(i.to_string());
            }
            let breakers: Vec<&str> = i.split("\\").collect();
            if breakers[breakers.len() - 1].contains(&be_searched) {
                matchers.push(i.to_string());
            }
        }
        dirs.into_iter().for_each(|dir| matchers.append(&mut self.search(&dir, &be_searched)));
        matchers
    }

    /// The function `get_paths` reads directory contents and returns a vector of file paths, handling
    /// errors by writing to a blacklist file if necessary.
    /// 
    /// Arguments:
    /// 
    /// * `path`: The `path` parameter in the `get_paths` function is a reference to a string (`&str`)
    /// which represents the directory path from which you want to read the directory entries.
    /// 
    /// Returns:
    /// 
    /// The `get_paths` function returns a `Vec<String>`. If the `fs::read_dir(path)` call is
    /// successful, it populates the vector with paths obtained from the directory entries. If there is
    /// an error, it writes the path to a blacklist file and returns an empty vector.
    fn get_paths(&mut self, path: &str) -> Vec<String> {
        if let Ok(paths) = fs::read_dir(path) {
            let mut paths_: Vec<String> = Vec::new();
            for path in paths {
                paths_.push(path.unwrap().path().display().to_string());
            }
            return paths_; 
        } else {
            let _ = self.blacklist_file_write.write(path.as_bytes()).unwrap();
            let _ = self.blacklist_file_write.write(b"\n").unwrap();
            return Vec::new();
        }
    }
}


