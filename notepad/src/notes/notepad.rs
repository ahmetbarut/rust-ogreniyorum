use std::collections::HashMap;
pub struct Notepad {
    pub notes: HashMap<u32, String>,
    pub next_id: u32,
}

impl Notepad {
    pub fn add(&mut self, note: String) -> u32 {
        let id = self.next_id;
        self.notes.insert(id, note);
        self.next_id += 1;
        id
    }

    pub fn get(&self, id: u32) -> Option<&String> {
        self.notes.get(&id)
    }

    pub fn remove(&mut self, id: u32) -> Option<String> {
        self.notes.remove(&id)
    }

    pub fn edit(&mut self, id: u32, note: String) -> bool {
        if self.notes.contains_key(&id) {
            self.notes.insert(id, note);
            true
        } else {
            false
        }
    }

    pub fn count(&self) -> usize {
        self.notes.len()
    }
}
