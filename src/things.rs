use chrono::NaiveDate;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Clone, Debug)]
pub struct Thing {
    pub date: NaiveDate,
    pub link: String,
    pub title: String,
    pub description: Option<String>,
}

pub fn read_things_from_file(file_path: &str) -> io::Result<Vec<Thing>> {
    let path = Path::new(file_path);
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut things = Vec::new();

    for line in reader.lines().skip(1) {
        // Skip the header line
        let line = line?;
        let parts: Vec<&str> = line.split(',').collect();

        if parts.len() < 3 {
            continue;
        }

        let date = NaiveDate::parse_from_str(parts[0], "%Y-%m-%d").expect("Invalid date format");
        let link = parts[1].to_string();
        let title = parts[2].to_string();
        let mut description = parts.get(3).map(|s| s.to_string());

        if description == Some("".to_string()) {
            description = None;
        }

        things.push(Thing {
            date,
            link,
            title,
            description,
        });
    }

    // Sort the things in descending order by date
    things.sort_by(|a, b| b.date.cmp(&a.date));

    Ok(things)
}
