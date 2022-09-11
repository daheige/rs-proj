mod config;
mod logic;
mod rpc;

fn main() {
    config::bootstrap::init_config();

    println!("rpc");
}
