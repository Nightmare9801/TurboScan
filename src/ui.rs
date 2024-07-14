use crate::{dir_operations, path_manipulation, search::{self}};

/// These lines are declaring three constant variables `INCLUDE_ONLY`, `EXCLUDE_ONLY`, and `DIR`, all of
/// type `String`, and initializing them with empty strings using `String::new()`. These constants are
/// used to store user input for directory paths, include-only file extensions, and exclude file
/// extensions in the Rust code snippet provided.
const INCLUDE_ONLY: String = String::new();
const EXCLUDE_ONLY: String = String::new();
const DIR: String = String::new();

/// The function `ui` in Rust handles user input for setting directory search parameters and performing
/// searches.
/// 
/// Returns:
/// 
/// The function `ui()` returns a boolean value `true`.
pub fn ui() -> bool {
    let mut dir: String = DIR.clone();
    let mut include_only: String = INCLUDE_ONLY.clone();
    let mut exclude_only: String = EXCLUDE_ONLY.clone();
    let mut searcher = search::DirectorySearcher::new();
    searcher.update_blacklist();
    loop {
        searcher.update_blacklist();
        let input = get_input();
        if input.is_empty() {
            continue;
        } else if input.to_lowercase().contains(&"EXIT".to_lowercase()) || input.to_lowercase().contains(&"END".to_lowercase()) {
            println!("Exited");
            break;
        } else if input.to_lowercase().contains(&"Help".to_lowercase()) {
            help();
            continue;
        } else if input.to_lowercase().contains(&"Set_to_parent_dir".to_lowercase()) {
            dir = dir_operations::parent_dir();
            continue;
        } else if input.to_lowercase().starts_with(&"Set_dir".to_lowercase()) {
            let breaker: Vec<&str> = input.split(" ").collect();
            if breaker.len() > 2 {
                println!("Wrong Number Of Inputs.");
                continue;
            }
            dir = breaker[1].trim().to_owned();
            continue;
        } else if input.to_lowercase().starts_with(&"Include_only".to_lowercase()) {
            let breaker: Vec<&str> = input.split(" ").collect();
            if breaker.len() > 2 {
                println!("Wrong Number Of Inputs.");
                continue;
            }
            include_only = breaker[1].to_lowercase().trim().to_owned();
            continue;
        } else if input.to_lowercase().starts_with(&"Exclude".to_lowercase()) {
            let breaker: Vec<&str> = input.split(" ").collect();
            if breaker.len() > 2 {
                println!("Wrong Number Of Inputs.");
                continue;
            }
            exclude_only = breaker[1].to_lowercase().trim().to_owned();
            continue;
        } else if input.to_lowercase().starts_with("search") {
            let breaker: Vec<&str> = input.split(" ").collect();
            if breaker.len() > 3 {
                println!("Wrong Number Of Inputs.");
                continue;
            } else if breaker.len() == 3 {
                let mut paths = searcher.search(breaker[1], breaker[2].trim());
                if !include_only.is_empty() {
                    paths = path_manipulation::include_only_wanted_search_objects(paths, &include_only);
                }
                if !exclude_only.is_empty() {
                    paths = path_manipulation::exclude_unwanted_search_objects(paths, &exclude_only);
                }
                path_manipulation::print(paths);
                continue;
            } else if breaker.len() == 2 {
                let mut paths = searcher.search(&dir, breaker[1].trim());
                if !include_only.is_empty() {
                    paths = path_manipulation::include_only_wanted_search_objects(paths, &include_only);
                }
                if !exclude_only.is_empty() {
                    paths = path_manipulation::exclude_unwanted_search_objects(paths, &exclude_only);
                }
                path_manipulation::print(paths);
                continue;
            } else {
                println!("Empty Command");
                continue;
            }
        } else if input.to_lowercase().starts_with(&"Set_to_factory_settings".to_lowercase()) {
            dir = DIR.clone();
            include_only = INCLUDE_ONLY.clone();
            exclude_only = EXCLUDE_ONLY.clone();
            continue;
        } else if input.to_lowercase().starts_with(&"See_settings".to_lowercase()) {
            println!("Directory: {}", dir);
            println!("Include only files: {}", include_only);
            println!("Exclude only files: {}", exclude_only);
            continue;
        }
        println!("Wrong Command");
    }
    true
}

/// The function provides a list of commands and explanations for a search program in Rust.
fn help() {
    println!("search: <DIR> <NAME> ->       Searches the specified name in the specified directory.");
    println!("search: <NAME> ->             Searches the specified name in the set directory.");
    println!("Set_to_parent_dir ->          Sets the search path to the parent directory.");
    println!("Set_dir: <DIR> ->             Sets the search path to the specified directory.");
    println!("include_only: .<Extension> -> Includes only the files with the given extemsion");
    println!("exclude: .<EXTENSION> ->       Excludes the files with extensions from the search results.");
    println!("See_settings ->               Displays the settings");
    println!("Set_to_factory_settings ->    Resets the settings.");
    println!("Help ->                       Displays the help menu");
    println!("Exit/End ->                   Terminates the program.");
}

/// The `get_input` function in Rust reads a line of input from the user and returns it as a String.
/// 
/// Returns:
/// 
/// The `get_input` function returns a `String` value, which is the user input read from the standard
/// input (stdin).
fn get_input() -> String {
    let mut line: String = String::new();
    print!("\r");
    let _ = std::io::stdin().read_line(&mut line).unwrap();
    line
}