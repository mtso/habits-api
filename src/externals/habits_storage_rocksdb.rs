use bincode;
use ksuid::Ksuid;
use rocksdb::Writable;

use models::Habit;
use DbConn;

#[derive(Debug, Clone)]
pub struct HabitsStorageRocksdb {
    conn: DbConn,
}

impl HabitsStorageRocksdb {
    pub fn new(conn: DbConn) -> Self {
        HabitsStorageRocksdb { conn }
    }
}

use super::habits_storage::HabitsStorage;

impl HabitsStorage for HabitsStorageRocksdb {
    fn put_habit(&self, habit: &Habit) -> Result<(), String> {
        use constants::CF_HABITS;

        let db = match self.conn.lock() {
            Ok(db) => db,
            Err(e) => return Err(format!("{}", e)),
        };

        let cf_handle = match db.cf_handle(CF_HABITS) {
            Some(h) => h,
            None => return Err("Storage schema error".to_string()),
        };

        let habit_bincode = match bincode::serialize(habit) {
            Ok(bc) => bc,
            Err(e) => return Err(format!("{}", e)),
        };

        db.put_cf(cf_handle, habit.id.as_bytes(), &habit_bincode)
            .map_err(|e| format!("{}", e))
    }

    fn get_habit(&self, id: Ksuid) -> Result<Habit, String> {
        use constants::CF_HABITS;

        let db = match self.conn.lock() {
            Ok(db) => db,
            Err(e) => return Err(format!("{}", e)),
        };

        let habits_cf = match db.cf_handle(CF_HABITS) {
            Some(h) => h,
            None => return Err("Storage schema error".to_string()),
        };

        let value_buf = match db.get_cf(habits_cf, id.as_bytes()) {
            Ok(Some(v)) => v,
            Ok(None) => return Err("Not found".to_string()),
            Err(e) => return Err(format!("{}", e)),
        };

        match bincode::deserialize(&value_buf) {
            Ok(h) => Ok(h),
            Err(e) => Err(format!("{}", e)),
        }
    }

    fn delete_habit(&self, id: Ksuid) -> Result<(), String> {
        use constants::CF_HABITS;

        let db = match self.conn.lock() {
            Ok(db) => db,
            Err(e) => return Err(format!("{}", e)),
        };

        let habits_cf = match db.cf_handle(CF_HABITS) {
            Some(h) => h,
            None => return Err("Storage schema error".to_string()),
        };

        db.delete_cf(habits_cf, id.as_bytes())
            .map_err(|e| format!("{}", e))
    }
}
