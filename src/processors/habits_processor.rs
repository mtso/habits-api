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

    pub fn create_habit(
        &self,
        user_id: String,
        timezone_offset: i32,
        title: String,
    ) -> Result<Habit, String> {
        let habit = Habit::new(user_id, timezone_offset, title);

        match self.storage.put_habit(&habit) {
            Ok(()) => Ok(habit),
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
        let naivenow = FixedOffset::west(habit.timezone_offset * 3600)
            .ymd(now.year(), now.month(), now.day())
            .and_hms(now.hour(), now.minute(), now.second())
            .naive_utc();

        let datestring = format!(
            "{:04}-{:02}-{:02}",
            naivenow.year(),
            naivenow.month(),
            naivenow.day(),
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

    pub fn get_habits_by_userid(&self, id: String) -> Result<HashSet<Habit>, String> {
        self.storage.get_habits_by_userid(id)
    }

    pub fn update_habit(
        &self,
        id: Ksuid,
        title: Option<String>,
        timezone_offset: Option<i32>,
    ) -> Result<Habit, String> {
        let mut habit = match self.storage.get_habit(id) {
            Ok(h) => h,
            Err(e) => return Err(format!("{}", e)),
        };

        if let (None, None) = (&title, &timezone_offset) {
            return Ok(habit);
        }

        if let Some(title) = title {
            habit.title = title;
        }

        if let Some(timezone_offset) = timezone_offset {
            habit.timezone_offset = timezone_offset;
        }

        self.storage.put_habit(&habit).map(|()| habit)
    }
}
