mod config;
mod logic;
mod worker;

fn main() {
    config::bootstrap::init_config();

    println!("worker/job");
}
