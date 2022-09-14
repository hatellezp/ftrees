use std::collections::HashMap;

pub trait AbstractStorage<T> {
    fn new() -> Self;
    fn contains_key(&self, k: &u64) -> bool;
    fn add(&mut self, v: T) -> u64;
    fn get_value(&self, k: &u64) -> Option<&T>;
    fn get_value_mut(&mut self, k: &u64) -> Option<&mut T>;
}

pub struct Storage<T> {
    data: HashMap<u64, T>,
    next_key: u64,
}

impl<T> AbstractStorage<T> for Storage<T> {
    fn new() -> Storage<T> {
        let data: HashMap<u64, T> = HashMap::new();
        Storage {
            data: data,
            next_key: 0,
        }
    }

    fn contains_key(&self, k: &u64) -> bool {
        self.data.contains_key(k)
    }

    fn add(&mut self, v: T) -> u64 {
        self.data.insert(self.next_key, v);
        self.next_key += 1;

        self.next_key - 1 // return the key used for the value inserted
    }

    fn get_value(&self, k: &u64) -> Option<&T> {
        self.data.get(k)
    }

    fn get_value_mut(&mut self, k: &u64) -> Option<&mut T> {
        self.data.get_mut(k)
    }
}

impl<T: PartialEq> Storage<T> {
    // this can be modified after (if I find a better way to do it)


    pub fn contains_value(&self, value: &T) -> bool {
        for (_, v) in self.data.iter() {
            if v == value {
                return true;
            }
        }

        false
    }

    pub fn get_key(&self, value: &T) -> Option<u64> {
        for (k, v) in self.data.iter() {
            if v == value {
                return Some(*k);
            }
        }

        return None;
    }
}
