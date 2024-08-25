use csv::{ReaderBuilder, WriterBuilder};
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize, serde::Serialize)]
struct Record {
    name: String,
    age: u32,
    city: String,
}

fn main() {
    let file_path = "data.csv";
    let new_records = vec![
        Record {
            name: "John".to_string(),
            age: 30,
            city: "New York".to_string(),
        },
        Record {
            name: "Jane".to_string(),
            age: 25,
            city: "San Francisco".to_string(),
        },
    ];

    write_csv(file_path, &new_records).unwrap();
}

fn write_csv(file_path: &str, records: &[Record]) -> Result<(), Box<dyn Error>> {
    let mut writer = WriterBuilder::new().from_path(file_path)?;

    for record in records {
        writer.serialize(record)?;
    }

    writer.flush()?;
    Ok(())
}
