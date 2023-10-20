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

    fn create_new(&self) {
        let res = self.database.set_by_key("test", "test");
    }

    fn get_res_by_key(&self, key: String) -> Result<String, &str> {
        let res = self.database.get_by_key(key);
    }
}
