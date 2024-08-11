#[cfg(test)]
#[path = "../src/notes/mod.rs"]
mod notes;

#[test]
fn test_add_note() {
    let mut notepad = notes::new();
    let id = notepad.add("Hello, world!".to_string());
    assert_eq!(id, 1);

    assert!(notepad.get(1).is_some());
}

#[test]
fn test_get_note() {
    let mut notepad = notes::new();
    notepad.add("Hello, world!".to_string());
    let note = notepad.get(1);
    assert_eq!(note, Some(&"Hello, world!".to_string()));
}

#[test]
fn test_remove_note() {
    let mut notepad = notes::new();
    notepad.add("Hello, world!".to_string());
    let note = notepad.remove(1);
    assert_eq!(note, Some("Hello, world!".to_string()));
    assert!(notepad.get(1).is_none());
}

#[test]
fn test_edit_note() {
    let mut notepad = notes::new();
    notepad.add("Hello, world!".to_string());
    notepad.edit(1, "Hello, Rust!".to_string());
    let note = notepad.get(1);
    assert_eq!(note, Some(&"Hello, Rust!".to_string()));
}

#[test]
fn test_count_notes() {
    let mut notepad = notes::new();
    notepad.add("Hello, world!".to_string());
    notepad.add("Hello, Rust!".to_string());
    assert_eq!(notepad.count(), 2);
}