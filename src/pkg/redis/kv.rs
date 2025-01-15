use super::client::get_global_connection;
use log::info;
use redis::{Commands, RedisResult};
use serde::{Deserialize, Serialize};
pub struct KvStore;

#[derive(Serialize, Deserialize, Debug)]

pub struct KvItem {
    pub(crate)  key: String,
    pub(crate) value: String,
}

impl KvStore {
    pub fn set_kv(key: &str, value: &str) -> RedisResult<()> {
        let mut connection = get_global_connection()?;
        info!("Setting to {} {}", key, value);
        connection.set::<_, _, ()>(key, value)?;
        Ok(())
    }

    pub fn del_kv(key: &str) -> RedisResult<()> {
        let mut connection = get_global_connection()?;
        info!("Deleting from {}", key);
        connection.del::<_, ()>(key)?;
        Ok(())
    }

    pub fn get_kv(key: &str) -> RedisResult<Option<String>> {
        let mut connection = get_global_connection()?;
        connection.get(key)
    }

    pub fn set_kv_item(item: &KvItem) -> RedisResult<()> {
        let mut connection = get_global_connection()?;
        info!("Setting to {} {}", item.key, item.value);
        connection.set::<_, _, ()>(&item.key, &item.value)?;
        Ok(())
    }

    pub fn del_kv_item(item: &KvItem) -> RedisResult<()> {
        let mut connection = get_global_connection()?;
        info!("Deleting from {}", item.key);
        connection.del::<_, ()>(&item.key)?;
        Ok(())
    }

    pub fn get_kv_item(key: &str) -> RedisResult<Option<KvItem>> {
        let mut connection = get_global_connection()?;
        let value: Option<String> = connection.get(key)?;

        match value {
            Some(v) => Ok(Some(KvItem {
                key: key.to_string(),
                value: v,
            })),
            None => Ok(None),
        }
    }

}