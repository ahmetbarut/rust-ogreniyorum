use clap::Parser;

mod notes;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    action: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    let mut notepad = notes::new();

    match cli.action.as_deref() {
        Some("add") => {
            let mut note = String::new();
            println!("Enter your note:");
            std::io::stdin().read_line(&mut note).unwrap();
            let id = notepad.add(note.trim().to_string());
            println!("Added note with ID: {}", id);
        }

        Some("get") => {
            let mut id = String::new();
            println!("Enter the ID of the note you want to get:");
            std::io::stdin().read_line(&mut id).unwrap();
            let id = id.trim().parse::<u32>().unwrap();

            match notepad.get(id) {
                Some(note) => {
                    println!("Note {}: {}", id, note);
                }
                _ => {
                    println!("Note {} not found.", id);
                }
            }
        }

        Some("remove") => {
            let mut id = String::new();
            println!("Enter the ID of the note you want to remove:");
            std::io::stdin().read_line(&mut id).unwrap();
            let id = id.trim().parse::<u32>().unwrap();

            match notepad.remove(id) {
                Some(note) => {
                    println!("Removed note {}: {}", id, note);
                }
                None => {
                    println!("Note {} not found.", id);
                }
            }
        }

        Some("edit") => {
            let mut id = String::new();
            println!("Enter the ID of the note you want to edit:");
            std::io::stdin().read_line(&mut id).unwrap();
            let id = id.trim().parse::<u32>().unwrap();

            let mut note = String::new();
            println!("Enter the new note:");
            std::io::stdin().read_line(&mut note).unwrap();
            let note = note.trim().to_string();

            if notepad.edit(id, note) {
                println!("Edited note {}.", id);
            } else {
                println!("Note {} not found.", id);
            }
        }

        Some("count") => {
            println!("Number of notes: {}", notepad.count());
        }

        None => {
            println!("No action provided. Use --help to see available actions.");
        }

        _ => {
            println!("Invalid action provided.");
        }
    }
}
