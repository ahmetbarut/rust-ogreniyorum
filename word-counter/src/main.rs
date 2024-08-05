use std::env::args;
use std::fs::File;
use std::io::BufReader;
use std::io::{self, *};
use std::path::Path;

use colored::Colorize;

#[derive(PartialEq)]
enum ReadingType {
    File,
    Stdin,
}

fn main() {
    let reading_type: ReadingType = args()
        .skip(1)
        .map(|arg| match arg.as_str() {
            "-f" => ReadingType::File,
            "-s" => ReadingType::Stdin,
            _ => {
                println!("Invalid argument");
                ReadingType::File // or ReadingType::Stdin, depending on your desired behavior
            }
        })
        .next()
        .unwrap_or(ReadingType::File); // provide a default value if no valid argument is found

    let mut input: String = String::new();

    if reading_type == ReadingType::File {
        input = args().nth(2).unwrap();
    } else {
        println!("Metni giriniz: ");
        io::stdin().read_line(&mut input).unwrap();
    }

    if reading_type == ReadingType::File && !Path::new(&input).exists() {
        println!("{} {} ", input.bold().blue().underline() ,"dosyası bulunamadı".red().bold());
        return;
    }

    let content = match reading_type {
        ReadingType::File => read_from_file(input).unwrap(),
        ReadingType::Stdin => read_from_stdin(input).unwrap(),
    };

    let word_count = count_words(&content);
    println!("Kelime sayısı: {}", word_count);
}

fn read_from_file(file_path: String) -> Result<String> {
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let mut content = String::new();

    reader.read_to_string(&mut content)?;

    Ok(content)
}

fn read_from_stdin(input: String) -> Result<String> {
    Ok(input)
}

fn count_words(content: &str) -> usize {
    content.split_whitespace().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_words() {
        let content = "Hello, world!";
        assert_eq!(count_words(content), 2);
    }

    #[test]
    fn test_read_from_file() {
        let content = read_from_file("src/main.rs".to_string()).unwrap();
        assert!(!content.is_empty());
    }

    #[test]
    fn test_read_from_stdin() {
        let content = read_from_stdin("Hello, world!".to_string()).unwrap();
        assert_eq!(content, "Hello, world!");
    }
}