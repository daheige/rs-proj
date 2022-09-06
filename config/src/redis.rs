use r2d2::Pool;
use std::time::Duration;
use redis;

#[derive(Default, Debug)]
struct RedisConf {
    dsn: String,
    max_size:u32,
    min_idle: u32,
    max_lifetime: Duration,
    idle_timeout: Duration,
    connection_timeout: Duration,
}

impl RedisConf{
    fn new(dsn: &str) ->Self{
        Self{
            dsn: dsn.to_string(),
            max_size:20,
            min_idle:3,
            max_lifetime: Duration::from_secs(1800),
            idle_timeout: Duration::from_secs(300),
            connection_timeout: Duration::from_secs(10),
        }
    }

    pub fn with_max_size(mut self, max: u32) -> Self {
        self.max_size = max;
        self
    }

    pub fn with_min_idle(mut self, min: u32) -> Self {
        self.min_idle = min;
        self
    }

    pub fn with_max_lifetime(mut self, lifetime: Duration) -> Self {
        self.max_lifetime = lifetime;
        self
    }

    pub fn with_idle_timeout(mut self, timeout: Duration) -> Self {
        self.idle_timeout = timeout;
        self
    }

    pub fn with_connect_timeout(mut self, timeout: Duration) -> Self {
        self.connection_timeout = timeout;
        self
    }

    fn init_pool(&self) -> Pool<redis::Client> {
        let client = redis::Client::open(self.dsn.as_str()).unwrap();
        let pool: Pool<redis::Client> = Pool::builder()
            .max_size(self.max_size)
            .max_lifetime(Some(self.max_lifetime))
            .idle_timeout(Some(self.idle_timeout))
            .min_idle(Some(self.min_idle))
            .connection_timeout(self.connection_timeout)
            .build(client)
            .unwrap();
        pool
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use redis::{self, Commands};

    #[test]
    fn test_redis() {
        let dsn = "redis://:@127.0.0.1:6379/0";
        let pool = RedisConf::new(dsn).init_pool();
        let mut conn = pool.get().unwrap(); // 默认超时是 connection_timeout 参数

        // 设置单个pool timeout
        // let mut conn = pool.get_timeout(Duration::from_secs(2)).unwrap();
        let res: redis::RedisResult<String> = conn.set("my_user", "daheige");
        if res.is_err() {
            println!("redis set error:{}", res.err().unwrap().to_string());
        }else{
            println!("set success");
        }
    }
}
