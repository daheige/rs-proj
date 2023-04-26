use config::{Config, ConfigTrait};

fn main() {
    let mut c = Config::new("app.yaml");
    c.load().expect("read config failed");

    println!("content: {}", c.content());

    println!("Hello, world!");
}
