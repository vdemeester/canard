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
                if record["Crédit Euros"] != "" {
                    let mut s = String::from("-");
                    s.push_str(&record["Crédit Euros"]);
                    print_line(&record["Date"], &record["Libellé"], &s)
                } else {
                    print_line(&record["Date"], &record["Libellé"], &record["Débit Euros"])
                }
            }
            Err(err) => eprintln!("error reading csv line… {:?}", err),
        }
    }
    Ok(())
}

fn print_line(date: &String, label: &String, amount: &String) {
    println!("{},{},€{}", date, label.replace('\n', ""), amount.replace(',', "."));
}

fn main() {
    println!("date,payee,amount");
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}
