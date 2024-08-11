use std::collections::HashMap;
mod notepad;

pub fn new() -> notepad::Notepad {
    let notes = HashMap::new();
    notepad::Notepad {
        notes: notes,
        next_id: 1,
    }
}
