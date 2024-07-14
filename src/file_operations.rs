use std::fs::{File, OpenOptions};

pub fn new_file_return_append() -> File {
    File::create("src\\blacklist.txt").unwrap();
    OpenOptions::new().append(true).open("src\\blacklist.txt").unwrap()
}

pub fn new_file_return() -> File {
    File::create("src\\blacklist.txt").unwrap();
    OpenOptions::new().read(true).open("src\\blacklist.txt").unwrap()
}