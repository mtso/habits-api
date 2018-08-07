use bincode;
use ksuid::Ksuid;
use log::Level;
use rocksdb::{Writable, DB};
use std::collections::HashSet;
use std::sync::MutexGuard;

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
        use constants::{CF_HABITS_ID, CF_HABITS_USERID};

        let db = match self.conn.lock() {
            Ok(db) => db,
            Err(e) => return Err(format!("{}", e)),
        };

        let cf_handle = match db.cf_handle(CF_HABITS_ID) {
            Some(h) => h,
            None => return Err("Storage schema error".to_string()),
        };

        let habits_userid_cf = match db.cf_handle(CF_HABITS_USERID) {
            Some(h) => h,
            None => return Err("Storage schema error".to_string()),
        };

        let habit_bincode = match bincode::serialize(habit) {
            Ok(bc) => bc,
            Err(e) => return Err(format!("{}", e)),
        };

        // Put new habit.
        match db.put_cf(cf_handle, habit.id.as_bytes(), &habit_bincode) {
            Ok(_) => (),
            Err(e) => return Err(format!("{}", e)),
        };

        // FIXME: Prevent accessing user_id index if the Habit row already exists!

        // Add habit_id to set!
        let mut habit_ids = match get_habit_ids(&db, &habit.user_id) {
            Ok(ids) => ids,
            Err(e) => return Err(format!("{}", e)),
        };

        if !habit_ids.insert(habit.id) {
            log!(Level::Info, "key exists in user_id set: {:?}", habit.id);
        }

        let set_bincode = match bincode::serialize(&habit_ids) {
            Ok(bc) => bc,
            Err(e) => return Err(format!("{}", e)),
        };

        db.put_cf(habits_userid_cf, habit.user_id.as_bytes(), &set_bincode)
            .map_err(|e| format!("{}", e))
    }

    fn get_habit(&self, id: Ksuid) -> Result<Habit, String> {
        use constants::CF_HABITS_ID;

        let db = match self.conn.lock() {
            Ok(db) => db,
            Err(e) => return Err(format!("{}", e)),
        };

        let habits_cf = match db.cf_handle(CF_HABITS_ID) {
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
        use constants::{CF_HABITS_ID, CF_HABITS_USERID};

        let db = match self.conn.lock() {
            Ok(db) => db,
            Err(e) => return Err(format!("{}", e)),
        };

        let habit = match get_habit(&db, &id) {
            Ok(h) => h,
            Err(e) => return Err(format!("{}", e)),
        };

        let habits_cf = match db.cf_handle(CF_HABITS_ID) {
            Some(h) => h,
            None => return Err("Storage schema error".to_string()),
        };

        let habits_userid_cf = match db.cf_handle(CF_HABITS_USERID) {
            Some(h) => h,
            None => return Err("Storage schema error".to_string()),
        };

        match db
            .delete_cf(habits_cf, id.as_bytes())
            .map_err(|e| format!("{}", e))
        {
            Ok(()) => (),
            e => return e,
        };

        let mut habit_ids = match get_habit_ids(&db, &habit.user_id) {
            Ok(h) => h,
            Err(e) => return Err(format!("{}", e)),
        };

        if !habit_ids.remove(&habit.id) {
            warn!("Expected an ID to be removed: {:?}", habit.id);
        }

        let set_bincode = match bincode::serialize(&habit_ids) {
            Ok(bc) => bc,
            Err(e) => return Err(format!("{}", e)),
        };

        db.put_cf(habits_userid_cf, habit.user_id.as_bytes(), &set_bincode)
            .map_err(|e| format!("{}", e))
    }

    fn get_habits_by_userid(&self, id: String) -> Result<HashSet<Habit>, String> {
        use constants::CF_HABITS_USERID;

        let db = match self.conn.lock() {
            Ok(db) => db,
            Err(e) => return Err(format!("{}", e)),
        };

        let habits_userid_cf = match db.cf_handle(CF_HABITS_USERID) {
            Some(h) => h,
            None => return Err("Storage schema error".to_string()),
        };

        let habit_ids: HashSet<Ksuid> = match db.get_cf(habits_userid_cf, id.as_bytes()) {
            Ok(Some(ids)) => match bincode::deserialize(&ids) {
                Ok(h) => h,
                Err(e) => return Err(format!("{}", e)),
            },
            Ok(None) => HashSet::new(),
            Err(e) => return Err(format!("{}", e)),
        };

        habit_ids
            .iter()
            .map(|id| get_habit(&db, id))
            .filter(|r| if let Ok(_) = r { true } else { false })
            .collect()
    }
}

fn get_habit(db: &MutexGuard<DB>, id: &Ksuid) -> Result<Habit, String> {
    use constants::CF_HABITS_ID;

    let habits_cf = match db.cf_handle(CF_HABITS_ID) {
        Some(h) => h,
        None => return Err("Storage schema error".to_string()),
    };

    let value_buf = match db.get_cf(habits_cf, id.as_bytes()) {
        Ok(Some(buf)) => buf,
        Ok(None) => return Err("Not found".to_string()),
        Err(e) => return Err(format!("{}", e)),
    };

    bincode::deserialize(&value_buf).map_err(|e| format!("{}", e))
}

fn get_habit_ids(db: &MutexGuard<DB>, id: &String) -> Result<HashSet<Ksuid>, String> {
    use constants::CF_HABITS_USERID;

    let habits_cf = match db.cf_handle(CF_HABITS_USERID) {
        Some(h) => h,
        None => return Err("Storage schema error".to_string()),
    };

    let value_buf = match db.get_cf(habits_cf, id.as_bytes()) {
        Ok(Some(buf)) => buf,
        Ok(None) => return Ok(HashSet::new()),
        // If not found, return an empty set.
        Err(e) => return Err(format!("{}", e)),
    };

    bincode::deserialize(&value_buf).map_err(|e| format!("{}", e))
}
