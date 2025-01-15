use notify::{recommended_watcher, RecursiveMode, Watcher as NotifyWatcher};
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub platform: Vec<String>,
    pub brazil: Environment,
    pub indonesia: Environment,
    pub indonesiaback: Environment,
    pub malaysia: Environment,
}

#[derive(Debug, Deserialize)]
pub struct Environment {
    pub envinfo: Vec<String>,
    pub env: HashMap<String, EnvironmentDetail>,
}

#[derive(Debug, Deserialize)]
pub struct EnvironmentDetail {
    pub db: u32,
    pub host: String,
    pub password: String,
    pub port: u32,
}

// 全局配置实例
pub static CONFIG: Lazy<Arc<Mutex<Config>>> =
    Lazy::new(|| Arc::new(Mutex::new(load_config("conf/redis.toml").unwrap())));

// 加载配置文件
fn load_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}

// 启动配置监控
pub fn start_monitoring() {
    let (tx, rx) = channel();
    let mut watcher = recommended_watcher(tx).unwrap();
    watcher
        .watch(
            std::path::Path::new("conf/redis.toml"),
            RecursiveMode::NonRecursive,
        )
        .unwrap();

    thread::spawn(move || {
        loop {
            match rx.recv() {
                Ok(_) => {
                    let new_config = load_config("conf/redis.toml").unwrap();
                    let mut config = CONFIG.lock().unwrap();
                    *config = new_config; // 更新全局配置
                    println!("Updated config: {:?}", *config);
                }
                Err(e) => println!("Watch error: {:?}", e),
            }
        }
    });
}
