fn connection_redis() -> redis::RedisResult<Connection> {
    let client = redis::Client::open("redis://172.12.10.36:6000")?;
    let con = client.get_connection()?;
    Ok(con)
}

embed_migrations!();