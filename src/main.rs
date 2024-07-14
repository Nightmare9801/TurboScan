pub mod search;
pub mod ui;
pub mod file_operations;
pub mod dir_operations;
pub mod path_manipulation;

/// The main function in Rust calls the ui function from the ui module.
fn main() {
    ui::ui();
}