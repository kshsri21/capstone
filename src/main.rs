mod location;
mod transaction;
use transaction::Transaction;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("./transaction.csv").unwrap();
    let reader = BufReader::new(file);
    let mut transactions: Vec<Transaction> = Vec::new();
    let mut skipped_lines = Vec::new();

    for (idx, line) in reader.lines().enumerate() {
        if idx == 0 {
            continue;
        }
        let line_str = line.unwrap();
        let parsed_transaction = Transaction::from_csv_line(&line_str);
        match parsed_transaction {
            Ok(val) => transactions.push(val),
            Err(err) => skipped_lines.push((idx, err, line_str)),
        }
    }

    for tx in transactions.iter() {
        println!("{:?}", tx);
    }
    for sl in skipped_lines.iter() {
        println!("{:?}", sl);
    }
}
