use serde::{Deserialize, Serialize};
use serde_yaml::{self, Value};
use std::fs::File;
use std::io::{Error, Read};

pub trait ConfigTrait {
    fn new(path: &str) -> Self;
    fn load(&mut self) -> Result<(), Error>;
    fn sections(&self) -> Value;
    fn content(&self) -> &str;
}

struct Config {
    config_file: String,
    sections: String,
}

impl ConfigTrait for Config {
    fn new(config_file: &str) -> Self {
        let s = Config {
            config_file: config_file.to_string(),
            sections: String::new(),
        };
        s
    }

    fn load(&mut self) -> Result<(), Error> {
        let mut s = String::new();
        File::open(&self.config_file)
            .unwrap()
            .read_to_string(&mut s)?;
        self.sections = s;
        Ok(())
    }

    fn sections(&self) -> Value {
        let val = serde_yaml::from_str(&self.sections).unwrap();
        val
    }

    fn content(&self) -> &str {
        self.sections.as_str()
    }
}

#[test]
fn test_config() {
    #[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
    struct AppConfig {
        app_debug: bool,
        app_name: String,
        app_port: i32,
    }

    let mut c = Config::new("test.yaml");
    c.load().expect("read file failed");

    // read config to struct
    let s: AppConfig = serde_yaml::from_str(c.content()).unwrap();
    println!("{:?}", s);

    // read config from serde Value
    let s: AppConfig = serde_yaml::from_value(c.sections()).unwrap();
    println!("{:?}", s);
}
