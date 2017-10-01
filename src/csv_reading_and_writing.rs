extern crate csv;

use std::process;
use std::path::Path;
use self::csv::Writer;

pub fn write_data(file_path: &str) {
    let dollar_films = vec![
        ("A Fistful of Dollars", "Rojo", 1964),
        ("For a Few Dollars More", "El Indio", 1965),
        ("The Good, the Bad and the Ugly", "Tuco", 1966),
    ];
    let path = Path::new(file_path);
    let mut writer = Writer::from_file(path).unwrap();
    for row in dollar_films.into_iter() {
        writer.encode(row).ok().expect("CSV writer error");
    }
}

pub fn read_data(file_path: &str) {
    let mut csv_reader = csv::Reader::from_file(file_path).unwrap();
    for record in csv_reader.records() {
        match record {
            Ok(item) => println!("{:?}", item),
            Err(err) => {
                println!("error reading CSV from <stdin>: {}", err);
                process::exit(1);
            }
        }
    }
}
