use std::hash::Hash;
use std::collections::HashSet;
use crate::local_search::search_types::tabu_search::tabu_list::memory::TabuListMemory;

pub struct LongTermMemory<T: Hash + Eq> {
    list: HashSet<T>
}

impl<T: Hash + Eq> TabuListMemory<T> for LongTermMemory<T> {
    fn new(_: u32) -> Self {
        Self {
            list: HashSet::new(),
        }
    }

    fn is_tabu(&self, item: &T, _: u32) -> bool {
        self.list.contains(item)
    }

    fn add(&mut self, item: T, _: u32) {
        self.list.insert(item);
    }

    fn clear(&mut self) {
        self.list.clear();
    }

    fn name() -> String {
        String::from("Long term memory")
    }

    fn snaked_name() -> String {
        String::from("long_term_memory")
    }
}