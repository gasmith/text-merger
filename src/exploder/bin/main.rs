use std::{collections::BTreeMap, fs::File, io::Write};

const BOOK: &str = include_str!("iliad.txt");
const JAN_01_2000_MS: u64 = 946645200000;
const REPEAT_N: u64 = 100;

fn write_line(file: &mut File, timestamp: u64, line: &str) {
    writeln!(file, "{timestamp} | {line}").expect("failed write");
}

fn main() {
    let mut outputs: BTreeMap<char, File> = BTreeMap::new();
    let mut other_file = None;
    let mut cur_timestamp = JAN_01_2000_MS;

    std::fs::create_dir_all("data").expect("failed to create data dir");

    for n in 0..REPEAT_N {
        if n % 10 == 0 {
            println!("writing out the Iliad for the {n}th time...");
        }
        for line in BOOK.lines() {
            let Some(first_character) = line.chars().next() else {
                continue;
            };
            let lowercase = first_character.to_ascii_lowercase();
            if !lowercase.is_alphabetic() {
                let file = other_file.get_or_insert_with(|| {
                    File::create("data/other.txt").expect("failed to create other.txt")
                });
                write_line(file, cur_timestamp, line);
                continue;
            }
            let file = outputs.entry(lowercase).or_insert_with(|| {
                File::create(format!("data/{lowercase}.txt")).expect("failed to create letter file")
            });
            write_line(file, cur_timestamp, line);
            cur_timestamp += rand::random_range(1u64..1000u64);
        }
    }
}
