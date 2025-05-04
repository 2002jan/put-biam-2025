use std::hash::Hash;
use std::collections::HashMap;
use crate::local_search::search_types::tabu_search::tabu_list::memory::TabuListMemory;

pub struct ShortTermMemory<T: Hash + Eq> {
    list: HashMap<T, u32>,
    tenure: u32,
}

impl<T: Hash + Eq> TabuListMemory<T> for ShortTermMemory<T> {
    fn new(tenure: u32) -> Self {
        Self {
            list: HashMap::new(),
            tenure,
        }
    }

    fn is_tabu(&self, item: &T, iteration: u32) -> bool {
        self.list.contains_key(item) && iteration < self.list[&item]
    }

    fn add(&mut self, item: T, iteration: u32) {
        self.list.insert(item, iteration + self.tenure);
    }


    fn clear(&mut self) {
        return;
    }

    fn name() -> String {
        String::from("Short term memory")
    }

    fn snaked_name() -> String {
        String::from("short_term_memory")
    }
}