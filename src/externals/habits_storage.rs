use ksuid::Ksuid;
use std::collections::HashSet;

use models::Habit;

pub trait HabitsStorage: Sync + Send {
    fn put_habit(&self, habit: &Habit) -> Result<(), String>;
    fn get_habit(&self, id: Ksuid) -> Result<Habit, String>;
    fn delete_habit(&self, id: Ksuid) -> Result<(), String>;
    fn get_habits_by_userid(&self, id: String) -> Result<HashSet<Habit>, String>;
}
