mod core;
mod task;

fn main() {
    println!("{}", task::commit::prompt::generate());
    println!("{}", core::get_api_key());
}
