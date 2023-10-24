use crate::db::Database;

pub struct Processor<T> {
    database: T,
}

impl<T> Processor<T>
where
    T: Database,
{
    pub fn new(database: T) -> Processor<T> {
        return Processor { database };
    }

    fn create_new(&mut self) {
        let res = self.database.set_by_key("test", "test");
        if res.is_err() {
            println!("Failed to set key");
        }
    }

    fn get_res_by_key(&self, key: String) -> Result<String, &str> {
        self.database.get_by_key(key)
    }
}
