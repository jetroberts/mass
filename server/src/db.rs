use std::env;

use redis::{Commands, RedisResult};

pub trait Database {
    fn connect(&mut self);
    fn get_by_key(&self, key: String) -> Result<String, &str>;
    fn set_by_key(&mut self, key: &str, value: &str) -> Result<(), &str>;
}

pub struct RedisDatabase {
    client: Option<redis::Client>,
    connection: Option<redis::Connection>,
}

impl RedisDatabase {
    pub fn new() -> RedisDatabase {
        RedisDatabase {
            client: None,
            connection: None,
        }
    }
}

impl Database for RedisDatabase {
    fn connect(&mut self) {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL was not set");
        let client = redis::Client::open(database_url).expect("Failed to create client");
        let connection = client.get_connection().expect("Failed to get connection");

        self.client = Some(client);
        self.connection = Some(connection);
    }

    // should this return an option rather than a result? 
    fn get_by_key(&self, key: String) -> Result<String, &str> {
        let conn = match self.connection {
            Some(ref mut conn) => conn,
            None => return Err("Failed to get connection"),
        };

        match conn.get(key) {
            Ok(value) => Ok(value),
            Err(_) => Err("Failed to get value"),
        }
    }

    fn set_by_key(&mut self, key: &str, value: &str) -> Result<(), &str> {
        let conn = match self.connection {
            Some(ref mut conn) => conn,
            None => return Err("Failed to get connection"),
        };

        let res: RedisResult<()> = conn.set(key, value);
        if res.is_err() {
            return Err("Failed to set key");
        };

        Ok(())
    }
}
