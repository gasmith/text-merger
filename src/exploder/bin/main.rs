use std::{collections::BTreeMap, fs::File, io::Write};

const BOOK: &'static str = include_str!("iliad.txt");
const JAN_01_2000_MS: u64 = 946645200000;

fn write_line(file: &mut File, timestamp: u64, line: &str) {
    write!(file, "{timestamp} | {line}\n").expect("failed write");
}

fn main() {
    let mut outputs: BTreeMap<char, File> = BTreeMap::new();
    let mut other_file = None;
    let mut cur_timestamp = JAN_01_2000_MS;

    for line in BOOK.lines() {
        let Some(first_character) = line.chars().next() else {
            continue;
        };
        let lowercase = first_character.to_ascii_lowercase();
        if !lowercase.is_alphabetic() {
            let mut file = match other_file {
                Some(f) => f,
                None => File::create("data/other.txt").expect("failed to create other.txt"),
            };
            write_line(&mut file, cur_timestamp, line);
            other_file = Some(file);
            continue;
        }
        let mut file = outputs.entry(lowercase).or_insert_with(|| {
            File::create(format!("data/{lowercase}.txt")).expect("failed to create letter file")
        });
        write_line(&mut file, cur_timestamp, line);
        cur_timestamp += rand::random_range(1u64..1000u64);
    }
}
