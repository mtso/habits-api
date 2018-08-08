use chrono::prelude::*; // {Utc, FixedOffset}
use ksuid::Ksuid;
use std::collections::HashSet;

use externals::HabitsStorage;
use models::Habit;

pub struct HabitsProcessor {
    storage: Box<HabitsStorage>,
}

impl HabitsProcessor {
    pub fn new(storage: Box<HabitsStorage>) -> Self {
        HabitsProcessor { storage }
    }

    pub fn create_habit(&self, user_id: String, timezone_offset: i32) -> Result<Habit, String> {
        let new_habit = Habit::new(user_id, timezone_offset);

        match self.storage.put_habit(&new_habit) {
            Ok(()) => Ok(new_habit),
            Err(e) => Err(format!("{}", e)),
        }
    }

    pub fn get_habit(&self, id: Ksuid) -> Result<Habit, String> {
        match self.storage.get_habit(id) {
            Ok(habit) => Ok(habit),
            e => e,
        }
    }

    pub fn check_habit(&self, id: Ksuid) -> Result<Habit, String> {
        let mut habit = match self.storage.get_habit(id) {
            Ok(h) => h,
            Err(e) => return Err(format!("{}", e)),
        };

        let now = Utc::now();
        let localnow = FixedOffset::west(habit.timezone_offset * 3600)
            .ymd(now.year(), now.month(), now.day())
            .and_hms(now.hour(), now.minute(), now.second());
        let naive = localnow.naive_utc();
        let datestring = format!(
            "{:04}-{:02}-{:02}",
            naive.year(),
            naive.month(),
            naive.day(),
        );

        if !habit.checks.insert(datestring) {
            return Ok(habit);
        }

        self.storage
            .put_habit(&habit)
            .map_err(|e| format!("{}", e))
            .map(|_| habit)
    }

    pub fn uncheck_habit(&self, id: Ksuid, dates: HashSet<String>) -> Result<Habit, String> {
        let mut habit = match self.storage.get_habit(id) {
            Ok(h) => h,
            Err(e) => return Err(format!("{}", e)),
        };

        let mut did_delete = false;

        for date in dates.iter() {
            if habit.checks.contains(date) {
                habit.checks.remove(date);
                did_delete = true;
            }
        }

        if did_delete {
            return self
                .storage
                .put_habit(&habit)
                .map_err(|e| format!("{}", e))
                .map(|_| habit);
        } else {
            Ok(habit)
        }
    }

    pub fn reset_habit_checks(&self, id: Ksuid) -> Result<Habit, String> {
        let mut habit = match self.storage.get_habit(id) {
            Ok(h) => h,
            Err(e) => return Err(format!("{}", e)),
        };

        habit.checks.clear();

        self.storage
            .put_habit(&habit)
            .map_err(|e| format!("{}", e))
            .map(|_| habit)
    }

    pub fn delete_habit(&self, id: Ksuid) -> Result<(), String> {
        self.storage.delete_habit(id)
    }
}
