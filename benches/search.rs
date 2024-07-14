use std::fs;
use criterion::{criterion_group, criterion_main, Criterion};

fn is_dir(path: &String) -> bool {
    if let Ok(metadata) = fs::metadata(path) {
        return metadata.is_dir();
    } 
    false
}

pub(crate) fn search(root_path: &str, be_searched: &str) -> Vec<String> {
    //println!("{}", root_path);
    let paths: Vec<String> = get_paths(&root_path.trim());
    //println!("{}", paths.len());
    let mut matchers: Vec<String> = Vec::new();
    let mut dirs: Vec<String> = Vec::new();
    for i in paths.iter() {
        if is_dir(i) {
            dirs.push(i.to_string());
        }
        let breakers: Vec<&str> = i.split("\\").collect();
        if breakers[breakers.len() - 1].contains(be_searched) {
            matchers.push(i.to_string());
        }
    }
    dirs.into_iter().for_each(|dir| matchers.append(&mut search(&dir, be_searched)));
    matchers
}

fn get_paths(path: &str) -> Vec<String> {
    if let Ok(paths) = fs::read_dir(path){
        let mut paths_: Vec<String> = Vec::new();
        for path in paths {
            paths_.push(path.unwrap().path().display().to_string());
        }
        return paths_; 
    } else {
        return Vec::new();
    }
    
}

fn search_benchmark(c: &mut Criterion) {
    c.bench_function("Search Benchmark", |b| b.iter(|| search("F:/Git/", ".exe") ));
}

// Create a benchmark group
criterion_group!(benches, search_benchmark);

// Run the benchmarks
criterion_main!(benches);