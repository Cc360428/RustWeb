use once_cell::sync::Lazy;
use redis::{Client, Connection, RedisResult};
use std::sync::Mutex;

/// 全局 Redis 客户端实例
static REDIS_CLIENT: Lazy<Mutex<RedisClient>> = Lazy::new(|| {
    let address = "redis://172.12.10.36:6000/";
    let password = ""; // 留空表示无密码
    let db = 0; // 数据库索引

    let client =
        RedisClient::new(address, password, db).expect("Failed to initialize Redis client");
    Mutex::new(client)
});

pub struct RedisClient {
    client: Client,
}

impl RedisClient {
    pub fn new(address: &str, password: &str, db: u8) -> RedisResult<Self> {
        let client = Client::open(address)?;

        println!("{} {}", password, db);
        // if !password.is_empty() {
        //     let mut con = client.get_connection()?;
        //     redis::cmd("AUTH").arg(format!("{}\r\n", password)).execute(&mut con);
        // }

        // if db > 0 {
        //     let mut con = client.get_connection()?;
        //     redis::cmd("SELECT").arg(db)(&mut con);
        // }

        Ok(Self { client })
    }

    pub fn get_connection(&self) -> RedisResult<Connection> {
        self.client.get_connection()
    }
}

pub fn get_global_connection() -> RedisResult<Connection> {
    let client = REDIS_CLIENT.lock().expect("Failed to acquire lock");
    client.get_connection()
}
