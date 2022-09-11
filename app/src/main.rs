mod config;
mod logic;
mod middleware;
mod web;

// 默认是web层
fn main() {
    config::bootstrap::init_config();
    println!("api");
    println!("add(1,2) = {}", logic::add(1, 2));
}
