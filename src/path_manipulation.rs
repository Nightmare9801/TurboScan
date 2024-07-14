pub fn exclude_unwanted_search_objects(paths: Vec<String>, exclude: &str) -> Vec<String> {
    let mut new_paths = Vec::new();
    for mut path in paths {
        let actual_path: String = path.clone();
        path = path.to_lowercase();
        let breakers: Vec<&str> = path.split("\\").collect();
        if !breakers[breakers.len() - 1].contains(exclude) {
            new_paths.push(actual_path);
        }
    }
    new_paths
} 

pub fn include_only_wanted_search_objects(paths: Vec<String>, include_only: &str) -> Vec<String> {
    let mut new_paths: Vec<String> = Vec::new();
    for mut path in paths {
        let actual_path: String = path.clone();
        path = path.to_lowercase();
        let breakers: Vec<&str> = path.split("\\").collect();
        if breakers[breakers.len() - 1].contains(include_only) {
            new_paths.push(actual_path);
        }
    }
    new_paths
}

pub(crate) fn print(paths: Vec<String>) {
    println!("The paths are:");
    for i in &paths {
        println!("{}", i);
    }
    println!("Number of paths: {}", paths.len());
}