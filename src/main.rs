extern crate csv;

use csv::{ReaderBuilder, Trim};
use std::collections::HashMap;
use std::error::Error;
use std::io;
use std::process;
use std::string::String;

type Record = HashMap<String, String>;

fn run() -> Result<(), Box<Error>> {
    let mut rdr = ReaderBuilder::new()
        .delimiter(b';')
        .trim(Trim::All)
        .flexible(true)
        .from_reader(io::stdin());
    for result in rdr.deserialize() {
        let r: Result<Record, csv::Error> = result;
        match r {
            Ok(record) => {
                if record["Debit Euros"] != "" {
                    let mut s = String::from("-");
                    s.push_str(&record["Debit Euros"]);
                    println!(
                        "{},{},{}",
                        record["Date"],
                        record["Libelle"].replace('\n', ""),
                        s,
                    );
                } else {
                    println!(
                        "{},{},{}",
                        record["Date"],
                        record["Libelle"].replace('\n', ""),
                        record["Credit Euros"],
                    );
                }
            }
            Err(err) => eprintln!("error reading csv line… {:?}", err),
        }
    }
    Ok(())
}

fn main() {
    println!("date,payee,amount");
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}
